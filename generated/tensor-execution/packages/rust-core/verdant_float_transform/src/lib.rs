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
    F28,
}

impl Fn0 {
    pub fn apply(&self, p0: u64) -> R<u64> {
        match self {
            Fn0::F28 => f28(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F34,
}

impl Fn1 {
    pub fn apply(&self, p0: Adt0) -> R<Option<Adt0>> {
        match self {
            Fn1::F34 => f34(p0),
        }
    }
}

pub fn f0(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Option<u64>> {
    let a1 = v0;
    let a2 = 3u64;
    if if if nat_lt(a1, a2) {
        let a3 = v3;
        let a4 = 4294967296u64;
        nat_lt(a3, a4)
    } else {
        false
    } {
        let a5 = v4;
        let a6 = 4294967296u64;
        nat_lt(a5, a6)
    } else {
        false
    } {
        let m7 = f1(v1, 0u64, v2)?;
        match m7 {
            None => {
                Ok(None::<u64>)
            }
            Some(v5) => {
                f1(0u64, v1, f2(v0, v1, v5, v3, v4)?)
            }
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f1(v0: u64, v1: u64, v2: u64) -> R<Option<u64>> {
    if if if f3(v0) {
        f3(v1)
    } else {
        false
    } {
        let a10 = v2;
        let a11 = {
            let a8 = v0;
            let a9 = 0u64;
            if nat_eq(a8, a9) {
                4294967296u64
            } else {
                65536u64
            }
        };
        nat_lt(a10, a11)
    } else {
        false
    } {
        Ok(Some({
            let a12 = v0;
            let a13 = v1;
            if nat_eq(a12, a13) {
                v2
            } else {
                let v3: u64 = f4(v0, v2)?;
                let a14 = v1;
                let a15 = 0u64;
                if nat_eq(a14, a15) {
                    v3
                } else {
                    let a16 = v1;
                    let a17 = 1u64;
                    if nat_eq(a16, a17) {
                        f5(v3)?
                    } else {
                        f6(v3)?
                    }
                }
            }
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f2(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    let a18 = v0;
    let a19 = 0u64;
    if nat_eq(a18, a19) {
        let v5: u64 = f7(v2, v3)?;
        let a20 = v1;
        let a21 = 0u64;
        if if nat_eq(a20, a21) {
            f8(v4)?
        } else {
            false
        } {
            Ok(v5)
        } else {
            f9(v5, v4)
        }
    } else {
        let a22 = v0;
        let a23 = 1u64;
        if nat_eq(a22, a23) {
            let v6: u64 = if f10(v2, v4)? {
                v2
            } else {
                v4
            };
            if f10(v3, v6)? {
                Ok(v6)
            } else {
                Ok(v3)
            }
        } else {
            f9(if f10(0u64, v2)? {
                v2
            } else {
                0u64
            }, f7(v3, if f10(v2, 0u64)? {
                v2
            } else {
                0u64
            })?)
        }
    }
}

pub fn f3(v0: u64) -> bool {
    let a24 = v0;
    let a25 = 0u64;
    if if nat_eq(a24, a25) {
        true
    } else {
        let a26 = v0;
        let a27 = 1u64;
        nat_eq(a26, a27)
    } {
        true
    } else {
        let a28 = v0;
        let a29 = 30u64;
        nat_eq(a28, a29)
    }
}

pub fn f4(v0: u64, v1: u64) -> R<u64> {
    let a30 = v0;
    let a31 = 0u64;
    if nat_eq(a30, a31) {
        Ok(v1)
    } else {
        let a32 = v0;
        let a33 = 1u64;
        if nat_eq(a32, a33) {
            f11(v1)
        } else {
            let a34 = v1;
            let a35 = 65536u64;
            nat_mul(a34, a35)
        }
    }
}

pub fn f5(v0: u64) -> R<u64> {
    let v1: u64 = {
        let a36 = f12(v0)?;
        let a37 = 32768u64;
        nat_mul(a36, a37)?
    };
    let v2: u64 = f13(v0)?;
    let a56 = v1;
    let a57 = {
        let a38 = v2;
        let a39 = 255u64;
        if nat_eq(a38, a39) {
            let a40 = f14(v0)?;
            let a41 = 0u64;
            if nat_eq(a40, a41) {
                31744u64
            } else {
                32256u64
            }
        } else {
            let a42 = 143u64;
            let a43 = v2;
            if nat_le(a42, a43) {
                31744u64
            } else {
                let a44 = v2;
                let a45 = 102u64;
                if nat_lt(a44, a45) {
                    0u64
                } else {
                    let a46 = 113u64;
                    let a47 = v2;
                    if nat_le(a46, a47) {
                        let a52 = {
                            let a50 = {
                                let a48 = v2;
                                let a49 = 113u64;
                                nat_sub(a48, a49)
                            };
                            let a51 = 1024u64;
                            nat_mul(a50, a51)?
                        };
                        let a53 = f15(f16(v0)?, 8192u64)?;
                        nat_add(a52, a53)?
                    } else {
                        f15(f16(v0)?, f17({
                            let a54 = 126u64;
                            let a55 = v2;
                            nat_sub(a54, a55)
                        })?)?
                    }
                }
            }
        }
    };
    nat_add(a56, a57)
}

pub fn f6(v0: u64) -> R<u64> {
    if f18(v0)? {
        let v1: u64 = {
            let a58 = v0;
            let a59 = 65536u64;
            let a60 = 0u64;
            nat_quot(a58, a59, a60)
        };
        let a69 = v1;
        let a70 = {
            let a67 = {
                let a64 = {
                    let a61 = v1;
                    let a62 = 64u64;
                    let a63 = 0u64;
                    nat_quot(a61, a62, a63)
                };
                let a65 = 2u64;
                let a66 = 0u64;
                nat_rem(a64, a65, a66)
            };
            let a68 = 0u64;
            if nat_eq(a67, a68) {
                64u64
            } else {
                0u64
            }
        };
        nat_add(a69, a70)
    } else {
        let a81 = {
            let a79 = {
                let a71 = v0;
                let a72 = 32767u64;
                nat_add(a71, a72)?
            };
            let a80 = {
                let a76 = {
                    let a73 = v0;
                    let a74 = 65536u64;
                    let a75 = 0u64;
                    nat_quot(a73, a74, a75)
                };
                let a77 = 2u64;
                let a78 = 0u64;
                nat_rem(a76, a77, a78)
            };
            nat_add(a79, a80)?
        };
        let a82 = 65536u64;
        let a83 = 0u64;
        Ok(nat_quot(a81, a82, a83))
    }
}

pub fn f7(v0: u64, v1: u64) -> R<u64> {
    if if if if f18(v0)? {
        true
    } else {
        f18(v1)?
    } {
        true
    } else {
        if f19(v0)? {
            f8(v1)?
        } else {
            false
        }
    } {
        true
    } else {
        if f19(v1)? {
            f8(v0)?
        } else {
            false
        }
    } {
        let a170 = {
            let a148 = {
                let a124 = {
                    let a122 = {
                        let a84 = 1u64;
                        let a85 = 1u64;
                        nat_add(a84, a85)?
                    };
                    let a123 = {
                        let a120 = {
                            let a118 = {
                                let a86 = 1u64;
                                let a87 = 1u64;
                                nat_add(a86, a87)?
                            };
                            let a119 = {
                                let a116 = {
                                    let a114 = {
                                        let a88 = 1u64;
                                        let a89 = 1u64;
                                        nat_add(a88, a89)?
                                    };
                                    let a115 = {
                                        let a112 = {
                                            let a110 = {
                                                let a90 = 1u64;
                                                let a91 = 1u64;
                                                nat_add(a90, a91)?
                                            };
                                            let a111 = {
                                                let a108 = {
                                                    let a106 = {
                                                        let a92 = 1u64;
                                                        let a93 = 1u64;
                                                        nat_add(a92, a93)?
                                                    };
                                                    let a107 = {
                                                        let a104 = {
                                                            let a102 = {
                                                                let a94 = 1u64;
                                                                let a95 = 1u64;
                                                                nat_add(a94, a95)?
                                                            };
                                                            let a103 = {
                                                                let a100 = {
                                                                    let a98 = {
                                                                        let a96 = 1u64;
                                                                        let a97 = 1u64;
                                                                        nat_add(a96, a97)?
                                                                    };
                                                                    let a99 = 1u64;
                                                                    nat_mul(a98, a99)?
                                                                };
                                                                let a101 = 1u64;
                                                                nat_add(a100, a101)?
                                                            };
                                                            nat_mul(a102, a103)?
                                                        };
                                                        let a105 = 1u64;
                                                        nat_add(a104, a105)?
                                                    };
                                                    nat_mul(a106, a107)?
                                                };
                                                let a109 = 1u64;
                                                nat_add(a108, a109)?
                                            };
                                            nat_mul(a110, a111)?
                                        };
                                        let a113 = 1u64;
                                        nat_add(a112, a113)?
                                    };
                                    nat_mul(a114, a115)?
                                };
                                let a117 = 1u64;
                                nat_add(a116, a117)?
                            };
                            nat_mul(a118, a119)?
                        };
                        let a121 = 1u64;
                        nat_add(a120, a121)?
                    };
                    nat_mul(a122, a123)?
                };
                let a125 = 1u64;
                nat_add(a124, a125)?
            };
            let a149 = f17({
                let a146 = {
                    let a144 = {
                        let a126 = 1u64;
                        let a127 = 1u64;
                        nat_add(a126, a127)?
                    };
                    let a145 = {
                        let a142 = {
                            let a140 = {
                                let a128 = 1u64;
                                let a129 = 1u64;
                                nat_add(a128, a129)?
                            };
                            let a141 = {
                                let a138 = {
                                    let a136 = {
                                        let a130 = 1u64;
                                        let a131 = 1u64;
                                        nat_add(a130, a131)?
                                    };
                                    let a137 = {
                                        let a134 = {
                                            let a132 = 1u64;
                                            let a133 = 1u64;
                                            nat_add(a132, a133)?
                                        };
                                        let a135 = 1u64;
                                        nat_mul(a134, a135)?
                                    };
                                    nat_mul(a136, a137)?
                                };
                                let a139 = 1u64;
                                nat_add(a138, a139)?
                            };
                            nat_mul(a140, a141)?
                        };
                        let a143 = 1u64;
                        nat_add(a142, a143)?
                    };
                    nat_mul(a144, a145)?
                };
                let a147 = 1u64;
                nat_add(a146, a147)?
            })?;
            nat_mul(a148, a149)?
        };
        let a171 = f17({
            let a168 = {
                let a150 = 1u64;
                let a151 = 1u64;
                nat_add(a150, a151)?
            };
            let a169 = {
                let a166 = {
                    let a164 = {
                        let a152 = 1u64;
                        let a153 = 1u64;
                        nat_add(a152, a153)?
                    };
                    let a165 = {
                        let a162 = {
                            let a160 = {
                                let a154 = 1u64;
                                let a155 = 1u64;
                                nat_add(a154, a155)?
                            };
                            let a161 = {
                                let a158 = {
                                    let a156 = 1u64;
                                    let a157 = 1u64;
                                    nat_add(a156, a157)?
                                };
                                let a159 = 1u64;
                                nat_mul(a158, a159)?
                            };
                            nat_mul(a160, a161)?
                        };
                        let a163 = 1u64;
                        nat_add(a162, a163)?
                    };
                    nat_mul(a164, a165)?
                };
                let a167 = 1u64;
                nat_add(a166, a167)?
            };
            nat_mul(a168, a169)?
        })?;
        nat_add(a170, a171)
    } else {
        if if f19(v0)? {
            true
        } else {
            f19(v1)?
        } {
            f20(f21(v0, v1)?)
        } else {
            if if f8(v0)? {
                true
            } else {
                f8(v1)?
            } {
                f22(f21(v0, v1)?)
            } else {
                f23(v0, v1)
            }
        }
    }
}

pub fn f8(v0: u64) -> R<bool> {
    let a172 = f24(v0)?;
    let a173 = 0u64;
    Ok(nat_eq(a172, a173))
}

pub fn f9(v0: u64, v1: u64) -> R<u64> {
    if if f18(v0)? {
        true
    } else {
        f18(v1)?
    } {
        let a260 = {
            let a238 = {
                let a214 = {
                    let a212 = {
                        let a174 = 1u64;
                        let a175 = 1u64;
                        nat_add(a174, a175)?
                    };
                    let a213 = {
                        let a210 = {
                            let a208 = {
                                let a176 = 1u64;
                                let a177 = 1u64;
                                nat_add(a176, a177)?
                            };
                            let a209 = {
                                let a206 = {
                                    let a204 = {
                                        let a178 = 1u64;
                                        let a179 = 1u64;
                                        nat_add(a178, a179)?
                                    };
                                    let a205 = {
                                        let a202 = {
                                            let a200 = {
                                                let a180 = 1u64;
                                                let a181 = 1u64;
                                                nat_add(a180, a181)?
                                            };
                                            let a201 = {
                                                let a198 = {
                                                    let a196 = {
                                                        let a182 = 1u64;
                                                        let a183 = 1u64;
                                                        nat_add(a182, a183)?
                                                    };
                                                    let a197 = {
                                                        let a194 = {
                                                            let a192 = {
                                                                let a184 = 1u64;
                                                                let a185 = 1u64;
                                                                nat_add(a184, a185)?
                                                            };
                                                            let a193 = {
                                                                let a190 = {
                                                                    let a188 = {
                                                                        let a186 = 1u64;
                                                                        let a187 = 1u64;
                                                                        nat_add(a186, a187)?
                                                                    };
                                                                    let a189 = 1u64;
                                                                    nat_mul(a188, a189)?
                                                                };
                                                                let a191 = 1u64;
                                                                nat_add(a190, a191)?
                                                            };
                                                            nat_mul(a192, a193)?
                                                        };
                                                        let a195 = 1u64;
                                                        nat_add(a194, a195)?
                                                    };
                                                    nat_mul(a196, a197)?
                                                };
                                                let a199 = 1u64;
                                                nat_add(a198, a199)?
                                            };
                                            nat_mul(a200, a201)?
                                        };
                                        let a203 = 1u64;
                                        nat_add(a202, a203)?
                                    };
                                    nat_mul(a204, a205)?
                                };
                                let a207 = 1u64;
                                nat_add(a206, a207)?
                            };
                            nat_mul(a208, a209)?
                        };
                        let a211 = 1u64;
                        nat_add(a210, a211)?
                    };
                    nat_mul(a212, a213)?
                };
                let a215 = 1u64;
                nat_add(a214, a215)?
            };
            let a239 = f17({
                let a236 = {
                    let a234 = {
                        let a216 = 1u64;
                        let a217 = 1u64;
                        nat_add(a216, a217)?
                    };
                    let a235 = {
                        let a232 = {
                            let a230 = {
                                let a218 = 1u64;
                                let a219 = 1u64;
                                nat_add(a218, a219)?
                            };
                            let a231 = {
                                let a228 = {
                                    let a226 = {
                                        let a220 = 1u64;
                                        let a221 = 1u64;
                                        nat_add(a220, a221)?
                                    };
                                    let a227 = {
                                        let a224 = {
                                            let a222 = 1u64;
                                            let a223 = 1u64;
                                            nat_add(a222, a223)?
                                        };
                                        let a225 = 1u64;
                                        nat_mul(a224, a225)?
                                    };
                                    nat_mul(a226, a227)?
                                };
                                let a229 = 1u64;
                                nat_add(a228, a229)?
                            };
                            nat_mul(a230, a231)?
                        };
                        let a233 = 1u64;
                        nat_add(a232, a233)?
                    };
                    nat_mul(a234, a235)?
                };
                let a237 = 1u64;
                nat_add(a236, a237)?
            })?;
            nat_mul(a238, a239)?
        };
        let a261 = f17({
            let a258 = {
                let a240 = 1u64;
                let a241 = 1u64;
                nat_add(a240, a241)?
            };
            let a259 = {
                let a256 = {
                    let a254 = {
                        let a242 = 1u64;
                        let a243 = 1u64;
                        nat_add(a242, a243)?
                    };
                    let a255 = {
                        let a252 = {
                            let a250 = {
                                let a244 = 1u64;
                                let a245 = 1u64;
                                nat_add(a244, a245)?
                            };
                            let a251 = {
                                let a248 = {
                                    let a246 = 1u64;
                                    let a247 = 1u64;
                                    nat_add(a246, a247)?
                                };
                                let a249 = 1u64;
                                nat_mul(a248, a249)?
                            };
                            nat_mul(a250, a251)?
                        };
                        let a253 = 1u64;
                        nat_add(a252, a253)?
                    };
                    nat_mul(a254, a255)?
                };
                let a257 = 1u64;
                nat_add(a256, a257)?
            };
            nat_mul(a258, a259)?
        })?;
        nat_add(a260, a261)
    } else {
        if if f19(v0)? {
            f19(v1)?
        } else {
            false
        } {
            let a262 = f12(v0)?;
            let a263 = f12(v1)?;
            if nat_eq(a262, a263) {
                Ok(v0)
            } else {
                let a350 = {
                    let a328 = {
                        let a304 = {
                            let a302 = {
                                let a264 = 1u64;
                                let a265 = 1u64;
                                nat_add(a264, a265)?
                            };
                            let a303 = {
                                let a300 = {
                                    let a298 = {
                                        let a266 = 1u64;
                                        let a267 = 1u64;
                                        nat_add(a266, a267)?
                                    };
                                    let a299 = {
                                        let a296 = {
                                            let a294 = {
                                                let a268 = 1u64;
                                                let a269 = 1u64;
                                                nat_add(a268, a269)?
                                            };
                                            let a295 = {
                                                let a292 = {
                                                    let a290 = {
                                                        let a270 = 1u64;
                                                        let a271 = 1u64;
                                                        nat_add(a270, a271)?
                                                    };
                                                    let a291 = {
                                                        let a288 = {
                                                            let a286 = {
                                                                let a272 = 1u64;
                                                                let a273 = 1u64;
                                                                nat_add(a272, a273)?
                                                            };
                                                            let a287 = {
                                                                let a284 = {
                                                                    let a282 = {
                                                                        let a274 = 1u64;
                                                                        let a275 = 1u64;
                                                                        nat_add(a274, a275)?
                                                                    };
                                                                    let a283 = {
                                                                        let a280 = {
                                                                            let a278 = {
                                                                                let a276 = 1u64;
                                                                                let a277 = 1u64;
                                                                                nat_add(a276, a277)?
                                                                            };
                                                                            let a279 = 1u64;
                                                                            nat_mul(a278, a279)?
                                                                        };
                                                                        let a281 = 1u64;
                                                                        nat_add(a280, a281)?
                                                                    };
                                                                    nat_mul(a282, a283)?
                                                                };
                                                                let a285 = 1u64;
                                                                nat_add(a284, a285)?
                                                            };
                                                            nat_mul(a286, a287)?
                                                        };
                                                        let a289 = 1u64;
                                                        nat_add(a288, a289)?
                                                    };
                                                    nat_mul(a290, a291)?
                                                };
                                                let a293 = 1u64;
                                                nat_add(a292, a293)?
                                            };
                                            nat_mul(a294, a295)?
                                        };
                                        let a297 = 1u64;
                                        nat_add(a296, a297)?
                                    };
                                    nat_mul(a298, a299)?
                                };
                                let a301 = 1u64;
                                nat_add(a300, a301)?
                            };
                            nat_mul(a302, a303)?
                        };
                        let a305 = 1u64;
                        nat_add(a304, a305)?
                    };
                    let a329 = f17({
                        let a326 = {
                            let a324 = {
                                let a306 = 1u64;
                                let a307 = 1u64;
                                nat_add(a306, a307)?
                            };
                            let a325 = {
                                let a322 = {
                                    let a320 = {
                                        let a308 = 1u64;
                                        let a309 = 1u64;
                                        nat_add(a308, a309)?
                                    };
                                    let a321 = {
                                        let a318 = {
                                            let a316 = {
                                                let a310 = 1u64;
                                                let a311 = 1u64;
                                                nat_add(a310, a311)?
                                            };
                                            let a317 = {
                                                let a314 = {
                                                    let a312 = 1u64;
                                                    let a313 = 1u64;
                                                    nat_add(a312, a313)?
                                                };
                                                let a315 = 1u64;
                                                nat_mul(a314, a315)?
                                            };
                                            nat_mul(a316, a317)?
                                        };
                                        let a319 = 1u64;
                                        nat_add(a318, a319)?
                                    };
                                    nat_mul(a320, a321)?
                                };
                                let a323 = 1u64;
                                nat_add(a322, a323)?
                            };
                            nat_mul(a324, a325)?
                        };
                        let a327 = 1u64;
                        nat_add(a326, a327)?
                    })?;
                    nat_mul(a328, a329)?
                };
                let a351 = f17({
                    let a348 = {
                        let a330 = 1u64;
                        let a331 = 1u64;
                        nat_add(a330, a331)?
                    };
                    let a349 = {
                        let a346 = {
                            let a344 = {
                                let a332 = 1u64;
                                let a333 = 1u64;
                                nat_add(a332, a333)?
                            };
                            let a345 = {
                                let a342 = {
                                    let a340 = {
                                        let a334 = 1u64;
                                        let a335 = 1u64;
                                        nat_add(a334, a335)?
                                    };
                                    let a341 = {
                                        let a338 = {
                                            let a336 = 1u64;
                                            let a337 = 1u64;
                                            nat_add(a336, a337)?
                                        };
                                        let a339 = 1u64;
                                        nat_mul(a338, a339)?
                                    };
                                    nat_mul(a340, a341)?
                                };
                                let a343 = 1u64;
                                nat_add(a342, a343)?
                            };
                            nat_mul(a344, a345)?
                        };
                        let a347 = 1u64;
                        nat_add(a346, a347)?
                    };
                    nat_mul(a348, a349)?
                })?;
                nat_add(a350, a351)
            }
        } else {
            if f19(v0)? {
                Ok(v0)
            } else {
                if f19(v1)? {
                    Ok(v1)
                } else {
                    let a352 = f24(v1)?;
                    let a353 = f24(v0)?;
                    if nat_le(a352, a353) {
                        f25(v0, v1)
                    } else {
                        f25(v1, v0)
                    }
                }
            }
        }
    }
}

pub fn f10(v0: u64, v1: u64) -> R<bool> {
    if if f18(v0)? {
        true
    } else {
        f18(v1)?
    } {
        Ok(false)
    } else {
        if if f8(v0)? {
            f8(v1)?
        } else {
            false
        } {
            Ok(false)
        } else {
            let a354 = f12(v0)?;
            let a355 = f12(v1)?;
            if nat_eq(a354, a355) {
                let a356 = f12(v0)?;
                let a357 = 0u64;
                if nat_eq(a356, a357) {
                    let a358 = f24(v0)?;
                    let a359 = f24(v1)?;
                    Ok(nat_lt(a358, a359))
                } else {
                    let a360 = f24(v1)?;
                    let a361 = f24(v0)?;
                    Ok(nat_lt(a360, a361))
                }
            } else {
                let a362 = f12(v0)?;
                let a363 = 1u64;
                Ok(nat_eq(a362, a363))
            }
        }
    }
}

pub fn f11(v0: u64) -> R<u64> {
    let v1: u64 = {
        let a367 = {
            let a364 = v0;
            let a365 = 32768u64;
            let a366 = 0u64;
            nat_quot(a364, a365, a366)
        };
        let a368 = 2147483648u64;
        nat_mul(a367, a368)?
    };
    let v2: u64 = {
        let a372 = {
            let a369 = v0;
            let a370 = 32768u64;
            let a371 = 0u64;
            nat_rem(a369, a370, a371)
        };
        let a373 = 1024u64;
        let a374 = 0u64;
        nat_quot(a372, a373, a374)
    };
    let v3: u64 = {
        let a375 = v0;
        let a376 = 1024u64;
        let a377 = 0u64;
        nat_rem(a375, a376, a377)
    };
    let a412 = v1;
    let a413 = {
        let a378 = v2;
        let a379 = 31u64;
        if nat_eq(a378, a379) {
            let a388 = 2139095040u64;
            let a389 = {
                let a386 = {
                    let a380 = 0u64;
                    let a381 = v3;
                    if if nat_lt(a380, a381) {
                        let a382 = v3;
                        let a383 = 512u64;
                        nat_lt(a382, a383)
                    } else {
                        false
                    } {
                        let a384 = v3;
                        let a385 = 512u64;
                        nat_add(a384, a385)?
                    } else {
                        v3
                    }
                };
                let a387 = 8192u64;
                nat_mul(a386, a387)?
            };
            nat_add(a388, a389)?
        } else {
            let a390 = v2;
            let a391 = 0u64;
            if nat_eq(a390, a391) {
                let a392 = v3;
                let a393 = 0u64;
                if nat_eq(a392, a393) {
                    0u64
                } else {
                    let v4: Adt0 = f26(113u64, v3, 0u64, 1024u64)?;
                    let a402 = {
                        let a395 = match v4.clone() {
                            Adt0::C0(h394, _, _, _) => {
                                h394
                            }
                        };
                        let a396 = 8388608u64;
                        nat_mul(a395, a396)?
                    };
                    let a403 = {
                        let a400 = {
                            let a398 = match v4 {
                                Adt0::C0(_, h397, _, _) => {
                                    h397
                                }
                            };
                            let a399 = 1024u64;
                            nat_sub(a398, a399)
                        };
                        let a401 = 8192u64;
                        nat_mul(a400, a401)?
                    };
                    nat_add(a402, a403)?
                }
            } else {
                let a410 = {
                    let a406 = {
                        let a404 = v2;
                        let a405 = 112u64;
                        nat_add(a404, a405)?
                    };
                    let a407 = 8388608u64;
                    nat_mul(a406, a407)?
                };
                let a411 = {
                    let a408 = v3;
                    let a409 = 8192u64;
                    nat_mul(a408, a409)?
                };
                nat_add(a410, a411)?
            }
        }
    };
    nat_add(a412, a413)
}

pub fn f12(v0: u64) -> R<u64> {
    let a438 = v0;
    let a439 = f17({
        let a436 = {
            let a434 = {
                let a414 = 1u64;
                let a415 = 1u64;
                nat_add(a414, a415)?
            };
            let a435 = {
                let a432 = {
                    let a430 = {
                        let a416 = 1u64;
                        let a417 = 1u64;
                        nat_add(a416, a417)?
                    };
                    let a431 = {
                        let a428 = {
                            let a426 = {
                                let a418 = 1u64;
                                let a419 = 1u64;
                                nat_add(a418, a419)?
                            };
                            let a427 = {
                                let a424 = {
                                    let a422 = {
                                        let a420 = 1u64;
                                        let a421 = 1u64;
                                        nat_add(a420, a421)?
                                    };
                                    let a423 = 1u64;
                                    nat_mul(a422, a423)?
                                };
                                let a425 = 1u64;
                                nat_add(a424, a425)?
                            };
                            nat_mul(a426, a427)?
                        };
                        let a429 = 1u64;
                        nat_add(a428, a429)?
                    };
                    nat_mul(a430, a431)?
                };
                let a433 = 1u64;
                nat_add(a432, a433)?
            };
            nat_mul(a434, a435)?
        };
        let a437 = 1u64;
        nat_add(a436, a437)?
    })?;
    let a440 = 0u64;
    Ok(nat_quot(a438, a439, a440))
}

pub fn f13(v0: u64) -> R<u64> {
    let a463 = f24(v0)?;
    let a464 = f17({
        let a461 = {
            let a459 = {
                let a441 = 1u64;
                let a442 = 1u64;
                nat_add(a441, a442)?
            };
            let a460 = {
                let a457 = {
                    let a455 = {
                        let a443 = 1u64;
                        let a444 = 1u64;
                        nat_add(a443, a444)?
                    };
                    let a456 = {
                        let a453 = {
                            let a451 = {
                                let a445 = 1u64;
                                let a446 = 1u64;
                                nat_add(a445, a446)?
                            };
                            let a452 = {
                                let a449 = {
                                    let a447 = 1u64;
                                    let a448 = 1u64;
                                    nat_add(a447, a448)?
                                };
                                let a450 = 1u64;
                                nat_mul(a449, a450)?
                            };
                            nat_mul(a451, a452)?
                        };
                        let a454 = 1u64;
                        nat_add(a453, a454)?
                    };
                    nat_mul(a455, a456)?
                };
                let a458 = 1u64;
                nat_add(a457, a458)?
            };
            nat_mul(a459, a460)?
        };
        let a462 = 1u64;
        nat_add(a461, a462)?
    })?;
    let a465 = 0u64;
    Ok(nat_quot(a463, a464, a465))
}

pub fn f14(v0: u64) -> R<u64> {
    let a488 = v0;
    let a489 = f17({
        let a486 = {
            let a484 = {
                let a466 = 1u64;
                let a467 = 1u64;
                nat_add(a466, a467)?
            };
            let a485 = {
                let a482 = {
                    let a480 = {
                        let a468 = 1u64;
                        let a469 = 1u64;
                        nat_add(a468, a469)?
                    };
                    let a481 = {
                        let a478 = {
                            let a476 = {
                                let a470 = 1u64;
                                let a471 = 1u64;
                                nat_add(a470, a471)?
                            };
                            let a477 = {
                                let a474 = {
                                    let a472 = 1u64;
                                    let a473 = 1u64;
                                    nat_add(a472, a473)?
                                };
                                let a475 = 1u64;
                                nat_mul(a474, a475)?
                            };
                            nat_mul(a476, a477)?
                        };
                        let a479 = 1u64;
                        nat_add(a478, a479)?
                    };
                    nat_mul(a480, a481)?
                };
                let a483 = 1u64;
                nat_add(a482, a483)?
            };
            nat_mul(a484, a485)?
        };
        let a487 = 1u64;
        nat_add(a486, a487)?
    })?;
    let a490 = 0u64;
    Ok(nat_rem(a488, a489, a490))
}

pub fn f15(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = {
        let a491 = v0;
        let a492 = v1;
        let a493 = 0u64;
        nat_quot(a491, a492, a493)
    };
    let v3: u64 = {
        let a497 = {
            let a494 = v0;
            let a495 = v1;
            let a496 = 0u64;
            nat_rem(a494, a495, a496)
        };
        let a498 = 2u64;
        nat_mul(a497, a498)?
    };
    let a508 = v2;
    let a509 = {
        let a499 = v1;
        let a500 = v3;
        if if nat_lt(a499, a500) {
            true
        } else {
            let a501 = v3;
            let a502 = v1;
            if nat_eq(a501, a502) {
                let a506 = {
                    let a503 = v2;
                    let a504 = 2u64;
                    let a505 = 0u64;
                    nat_rem(a503, a504, a505)
                };
                let a507 = 1u64;
                nat_eq(a506, a507)
            } else {
                false
            }
        } {
            1u64
        } else {
            0u64
        }
    };
    nat_add(a508, a509)
}

pub fn f16(v0: u64) -> R<u64> {
    let a534 = f14(v0)?;
    let a535 = {
        let a510 = f13(v0)?;
        let a511 = 0u64;
        if nat_eq(a510, a511) {
            0u64
        } else {
            f17({
                let a532 = {
                    let a530 = {
                        let a512 = 1u64;
                        let a513 = 1u64;
                        nat_add(a512, a513)?
                    };
                    let a531 = {
                        let a528 = {
                            let a526 = {
                                let a514 = 1u64;
                                let a515 = 1u64;
                                nat_add(a514, a515)?
                            };
                            let a527 = {
                                let a524 = {
                                    let a522 = {
                                        let a516 = 1u64;
                                        let a517 = 1u64;
                                        nat_add(a516, a517)?
                                    };
                                    let a523 = {
                                        let a520 = {
                                            let a518 = 1u64;
                                            let a519 = 1u64;
                                            nat_add(a518, a519)?
                                        };
                                        let a521 = 1u64;
                                        nat_mul(a520, a521)?
                                    };
                                    nat_mul(a522, a523)?
                                };
                                let a525 = 1u64;
                                nat_add(a524, a525)?
                            };
                            nat_mul(a526, a527)?
                        };
                        let a529 = 1u64;
                        nat_add(a528, a529)?
                    };
                    nat_mul(a530, a531)?
                };
                let a533 = 1u64;
                nat_add(a532, a533)?
            })?
        }
    };
    nat_add(a534, a535)
}

pub fn f17(v0: u64) -> R<u64> {
    f27(Fn0::F28, v0, 1u64)
}

pub fn f18(v0: u64) -> R<bool> {
    let a578 = f13(v0)?;
    let a579 = {
        let a576 = {
            let a574 = {
                let a536 = 1u64;
                let a537 = 1u64;
                nat_add(a536, a537)?
            };
            let a575 = {
                let a572 = {
                    let a570 = {
                        let a538 = 1u64;
                        let a539 = 1u64;
                        nat_add(a538, a539)?
                    };
                    let a571 = {
                        let a568 = {
                            let a566 = {
                                let a540 = 1u64;
                                let a541 = 1u64;
                                nat_add(a540, a541)?
                            };
                            let a567 = {
                                let a564 = {
                                    let a562 = {
                                        let a542 = 1u64;
                                        let a543 = 1u64;
                                        nat_add(a542, a543)?
                                    };
                                    let a563 = {
                                        let a560 = {
                                            let a558 = {
                                                let a544 = 1u64;
                                                let a545 = 1u64;
                                                nat_add(a544, a545)?
                                            };
                                            let a559 = {
                                                let a556 = {
                                                    let a554 = {
                                                        let a546 = 1u64;
                                                        let a547 = 1u64;
                                                        nat_add(a546, a547)?
                                                    };
                                                    let a555 = {
                                                        let a552 = {
                                                            let a550 = {
                                                                let a548 = 1u64;
                                                                let a549 = 1u64;
                                                                nat_add(a548, a549)?
                                                            };
                                                            let a551 = 1u64;
                                                            nat_mul(a550, a551)?
                                                        };
                                                        let a553 = 1u64;
                                                        nat_add(a552, a553)?
                                                    };
                                                    nat_mul(a554, a555)?
                                                };
                                                let a557 = 1u64;
                                                nat_add(a556, a557)?
                                            };
                                            nat_mul(a558, a559)?
                                        };
                                        let a561 = 1u64;
                                        nat_add(a560, a561)?
                                    };
                                    nat_mul(a562, a563)?
                                };
                                let a565 = 1u64;
                                nat_add(a564, a565)?
                            };
                            nat_mul(a566, a567)?
                        };
                        let a569 = 1u64;
                        nat_add(a568, a569)?
                    };
                    nat_mul(a570, a571)?
                };
                let a573 = 1u64;
                nat_add(a572, a573)?
            };
            nat_mul(a574, a575)?
        };
        let a577 = 1u64;
        nat_add(a576, a577)?
    };
    if nat_eq(a578, a579) {
        let a580 = 0u64;
        let a581 = f14(v0)?;
        Ok(nat_lt(a580, a581))
    } else {
        Ok(false)
    }
}

pub fn f19(v0: u64) -> R<bool> {
    let a624 = f13(v0)?;
    let a625 = {
        let a622 = {
            let a620 = {
                let a582 = 1u64;
                let a583 = 1u64;
                nat_add(a582, a583)?
            };
            let a621 = {
                let a618 = {
                    let a616 = {
                        let a584 = 1u64;
                        let a585 = 1u64;
                        nat_add(a584, a585)?
                    };
                    let a617 = {
                        let a614 = {
                            let a612 = {
                                let a586 = 1u64;
                                let a587 = 1u64;
                                nat_add(a586, a587)?
                            };
                            let a613 = {
                                let a610 = {
                                    let a608 = {
                                        let a588 = 1u64;
                                        let a589 = 1u64;
                                        nat_add(a588, a589)?
                                    };
                                    let a609 = {
                                        let a606 = {
                                            let a604 = {
                                                let a590 = 1u64;
                                                let a591 = 1u64;
                                                nat_add(a590, a591)?
                                            };
                                            let a605 = {
                                                let a602 = {
                                                    let a600 = {
                                                        let a592 = 1u64;
                                                        let a593 = 1u64;
                                                        nat_add(a592, a593)?
                                                    };
                                                    let a601 = {
                                                        let a598 = {
                                                            let a596 = {
                                                                let a594 = 1u64;
                                                                let a595 = 1u64;
                                                                nat_add(a594, a595)?
                                                            };
                                                            let a597 = 1u64;
                                                            nat_mul(a596, a597)?
                                                        };
                                                        let a599 = 1u64;
                                                        nat_add(a598, a599)?
                                                    };
                                                    nat_mul(a600, a601)?
                                                };
                                                let a603 = 1u64;
                                                nat_add(a602, a603)?
                                            };
                                            nat_mul(a604, a605)?
                                        };
                                        let a607 = 1u64;
                                        nat_add(a606, a607)?
                                    };
                                    nat_mul(a608, a609)?
                                };
                                let a611 = 1u64;
                                nat_add(a610, a611)?
                            };
                            nat_mul(a612, a613)?
                        };
                        let a615 = 1u64;
                        nat_add(a614, a615)?
                    };
                    nat_mul(a616, a617)?
                };
                let a619 = 1u64;
                nat_add(a618, a619)?
            };
            nat_mul(a620, a621)?
        };
        let a623 = 1u64;
        nat_add(a622, a623)?
    };
    if nat_eq(a624, a625) {
        let a626 = f14(v0)?;
        let a627 = 0u64;
        Ok(nat_eq(a626, a627))
    } else {
        Ok(false)
    }
}

pub fn f20(v0: u64) -> R<u64> {
    let a694 = f22(v0)?;
    let a695 = {
        let a692 = {
            let a668 = {
                let a666 = {
                    let a628 = 1u64;
                    let a629 = 1u64;
                    nat_add(a628, a629)?
                };
                let a667 = {
                    let a664 = {
                        let a662 = {
                            let a630 = 1u64;
                            let a631 = 1u64;
                            nat_add(a630, a631)?
                        };
                        let a663 = {
                            let a660 = {
                                let a658 = {
                                    let a632 = 1u64;
                                    let a633 = 1u64;
                                    nat_add(a632, a633)?
                                };
                                let a659 = {
                                    let a656 = {
                                        let a654 = {
                                            let a634 = 1u64;
                                            let a635 = 1u64;
                                            nat_add(a634, a635)?
                                        };
                                        let a655 = {
                                            let a652 = {
                                                let a650 = {
                                                    let a636 = 1u64;
                                                    let a637 = 1u64;
                                                    nat_add(a636, a637)?
                                                };
                                                let a651 = {
                                                    let a648 = {
                                                        let a646 = {
                                                            let a638 = 1u64;
                                                            let a639 = 1u64;
                                                            nat_add(a638, a639)?
                                                        };
                                                        let a647 = {
                                                            let a644 = {
                                                                let a642 = {
                                                                    let a640 = 1u64;
                                                                    let a641 = 1u64;
                                                                    nat_add(a640, a641)?
                                                                };
                                                                let a643 = 1u64;
                                                                nat_mul(a642, a643)?
                                                            };
                                                            let a645 = 1u64;
                                                            nat_add(a644, a645)?
                                                        };
                                                        nat_mul(a646, a647)?
                                                    };
                                                    let a649 = 1u64;
                                                    nat_add(a648, a649)?
                                                };
                                                nat_mul(a650, a651)?
                                            };
                                            let a653 = 1u64;
                                            nat_add(a652, a653)?
                                        };
                                        nat_mul(a654, a655)?
                                    };
                                    let a657 = 1u64;
                                    nat_add(a656, a657)?
                                };
                                nat_mul(a658, a659)?
                            };
                            let a661 = 1u64;
                            nat_add(a660, a661)?
                        };
                        nat_mul(a662, a663)?
                    };
                    let a665 = 1u64;
                    nat_add(a664, a665)?
                };
                nat_mul(a666, a667)?
            };
            let a669 = 1u64;
            nat_add(a668, a669)?
        };
        let a693 = f17({
            let a690 = {
                let a688 = {
                    let a670 = 1u64;
                    let a671 = 1u64;
                    nat_add(a670, a671)?
                };
                let a689 = {
                    let a686 = {
                        let a684 = {
                            let a672 = 1u64;
                            let a673 = 1u64;
                            nat_add(a672, a673)?
                        };
                        let a685 = {
                            let a682 = {
                                let a680 = {
                                    let a674 = 1u64;
                                    let a675 = 1u64;
                                    nat_add(a674, a675)?
                                };
                                let a681 = {
                                    let a678 = {
                                        let a676 = 1u64;
                                        let a677 = 1u64;
                                        nat_add(a676, a677)?
                                    };
                                    let a679 = 1u64;
                                    nat_mul(a678, a679)?
                                };
                                nat_mul(a680, a681)?
                            };
                            let a683 = 1u64;
                            nat_add(a682, a683)?
                        };
                        nat_mul(a684, a685)?
                    };
                    let a687 = 1u64;
                    nat_add(a686, a687)?
                };
                nat_mul(a688, a689)?
            };
            let a691 = 1u64;
            nat_add(a690, a691)?
        })?;
        nat_mul(a692, a693)?
    };
    nat_add(a694, a695)
}

pub fn f21(v0: u64, v1: u64) -> R<u64> {
    let a702 = {
        let a696 = f12(v0)?;
        let a697 = f12(v1)?;
        nat_add(a696, a697)?
    };
    let a703 = {
        let a700 = {
            let a698 = 1u64;
            let a699 = 1u64;
            nat_add(a698, a699)?
        };
        let a701 = 1u64;
        nat_mul(a700, a701)?
    };
    let a704 = 0u64;
    Ok(nat_rem(a702, a703, a704))
}

pub fn f22(v0: u64) -> R<u64> {
    let a729 = v0;
    let a730 = f17({
        let a727 = {
            let a725 = {
                let a705 = 1u64;
                let a706 = 1u64;
                nat_add(a705, a706)?
            };
            let a726 = {
                let a723 = {
                    let a721 = {
                        let a707 = 1u64;
                        let a708 = 1u64;
                        nat_add(a707, a708)?
                    };
                    let a722 = {
                        let a719 = {
                            let a717 = {
                                let a709 = 1u64;
                                let a710 = 1u64;
                                nat_add(a709, a710)?
                            };
                            let a718 = {
                                let a715 = {
                                    let a713 = {
                                        let a711 = 1u64;
                                        let a712 = 1u64;
                                        nat_add(a711, a712)?
                                    };
                                    let a714 = 1u64;
                                    nat_mul(a713, a714)?
                                };
                                let a716 = 1u64;
                                nat_add(a715, a716)?
                            };
                            nat_mul(a717, a718)?
                        };
                        let a720 = 1u64;
                        nat_add(a719, a720)?
                    };
                    nat_mul(a721, a722)?
                };
                let a724 = 1u64;
                nat_add(a723, a724)?
            };
            nat_mul(a725, a726)?
        };
        let a728 = 1u64;
        nat_add(a727, a728)?
    })?;
    nat_mul(a729, a730)
}

pub fn f23(v0: u64, v1: u64) -> R<u64> {
    let v2: Adt0 = f29(v0)?;
    let v3: Adt0 = f29(v1)?;
    f30(f21(v0, v1)?, {
        let a785 = {
            let a733 = match v2.clone() {
                Adt0::C0(h731, _, _, _) => {
                    h731
                }
            };
            let a734 = match v3.clone() {
                Adt0::C0(h732, _, _, _) => {
                    h732
                }
            };
            nat_add(a733, a734)?
        };
        let a786 = {
            let a783 = {
                let a781 = {
                    let a735 = 1u64;
                    let a736 = 1u64;
                    nat_add(a735, a736)?
                };
                let a782 = {
                    let a779 = {
                        let a777 = {
                            let a737 = 1u64;
                            let a738 = 1u64;
                            nat_add(a737, a738)?
                        };
                        let a778 = {
                            let a775 = {
                                let a773 = {
                                    let a739 = 1u64;
                                    let a740 = 1u64;
                                    nat_add(a739, a740)?
                                };
                                let a774 = {
                                    let a771 = {
                                        let a769 = {
                                            let a741 = 1u64;
                                            let a742 = 1u64;
                                            nat_add(a741, a742)?
                                        };
                                        let a770 = {
                                            let a767 = {
                                                let a765 = {
                                                    let a743 = 1u64;
                                                    let a744 = 1u64;
                                                    nat_add(a743, a744)?
                                                };
                                                let a766 = {
                                                    let a763 = {
                                                        let a761 = {
                                                            let a745 = 1u64;
                                                            let a746 = 1u64;
                                                            nat_add(a745, a746)?
                                                        };
                                                        let a762 = {
                                                            let a759 = {
                                                                let a757 = {
                                                                    let a747 = 1u64;
                                                                    let a748 = 1u64;
                                                                    nat_add(a747, a748)?
                                                                };
                                                                let a758 = {
                                                                    let a755 = {
                                                                        let a749 = 1u64;
                                                                        let a750 = 1u64;
                                                                        nat_add(a749, a750)?
                                                                    };
                                                                    let a756 = {
                                                                        let a753 = {
                                                                            let a751 = 1u64;
                                                                            let a752 = 1u64;
                                                                            nat_add(a751, a752)?
                                                                        };
                                                                        let a754 = 1u64;
                                                                        nat_mul(a753, a754)?
                                                                    };
                                                                    nat_mul(a755, a756)?
                                                                };
                                                                nat_mul(a757, a758)?
                                                            };
                                                            let a760 = 1u64;
                                                            nat_add(a759, a760)?
                                                        };
                                                        nat_mul(a761, a762)?
                                                    };
                                                    let a764 = 1u64;
                                                    nat_add(a763, a764)?
                                                };
                                                nat_mul(a765, a766)?
                                            };
                                            let a768 = 1u64;
                                            nat_add(a767, a768)?
                                        };
                                        nat_mul(a769, a770)?
                                    };
                                    let a772 = 1u64;
                                    nat_add(a771, a772)?
                                };
                                nat_mul(a773, a774)?
                            };
                            let a776 = 1u64;
                            nat_add(a775, a776)?
                        };
                        nat_mul(a777, a778)?
                    };
                    let a780 = 1u64;
                    nat_add(a779, a780)?
                };
                nat_mul(a781, a782)?
            };
            let a784 = 1u64;
            nat_add(a783, a784)?
        };
        nat_sub(a785, a786)
    }, f31({
        let a789 = match v2 {
            Adt0::C0(_, h787, _, _) => {
                h787
            }
        };
        let a790 = match v3 {
            Adt0::C0(_, h788, _, _) => {
                h788
            }
        };
        nat_mul(a789, a790)?
    }, {
        let a807 = {
            let a791 = 1u64;
            let a792 = 1u64;
            nat_add(a791, a792)?
        };
        let a808 = {
            let a805 = {
                let a793 = 1u64;
                let a794 = 1u64;
                nat_add(a793, a794)?
            };
            let a806 = {
                let a803 = {
                    let a801 = {
                        let a795 = 1u64;
                        let a796 = 1u64;
                        nat_add(a795, a796)?
                    };
                    let a802 = {
                        let a799 = {
                            let a797 = 1u64;
                            let a798 = 1u64;
                            nat_add(a797, a798)?
                        };
                        let a800 = 1u64;
                        nat_mul(a799, a800)?
                    };
                    nat_mul(a801, a802)?
                };
                let a804 = 1u64;
                nat_add(a803, a804)?
            };
            nat_mul(a805, a806)?
        };
        nat_mul(a807, a808)?
    })?)
}

pub fn f24(v0: u64) -> R<u64> {
    let a833 = v0;
    let a834 = f17({
        let a831 = {
            let a829 = {
                let a809 = 1u64;
                let a810 = 1u64;
                nat_add(a809, a810)?
            };
            let a830 = {
                let a827 = {
                    let a825 = {
                        let a811 = 1u64;
                        let a812 = 1u64;
                        nat_add(a811, a812)?
                    };
                    let a826 = {
                        let a823 = {
                            let a821 = {
                                let a813 = 1u64;
                                let a814 = 1u64;
                                nat_add(a813, a814)?
                            };
                            let a822 = {
                                let a819 = {
                                    let a817 = {
                                        let a815 = 1u64;
                                        let a816 = 1u64;
                                        nat_add(a815, a816)?
                                    };
                                    let a818 = 1u64;
                                    nat_mul(a817, a818)?
                                };
                                let a820 = 1u64;
                                nat_add(a819, a820)?
                            };
                            nat_mul(a821, a822)?
                        };
                        let a824 = 1u64;
                        nat_add(a823, a824)?
                    };
                    nat_mul(a825, a826)?
                };
                let a828 = 1u64;
                nat_add(a827, a828)?
            };
            nat_mul(a829, a830)?
        };
        let a832 = 1u64;
        nat_add(a831, a832)?
    })?;
    let a835 = 0u64;
    Ok(nat_rem(a833, a834, a835))
}

pub fn f25(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f32(v0)?;
    let v3: u64 = {
        let a848 = f16(v0)?;
        let a849 = {
            let a846 = {
                let a836 = 1u64;
                let a837 = 1u64;
                nat_add(a836, a837)?
            };
            let a847 = {
                let a844 = {
                    let a838 = 1u64;
                    let a839 = 1u64;
                    nat_add(a838, a839)?
                };
                let a845 = {
                    let a842 = {
                        let a840 = 1u64;
                        let a841 = 1u64;
                        nat_add(a840, a841)?
                    };
                    let a843 = 1u64;
                    nat_mul(a842, a843)?
                };
                nat_mul(a844, a845)?
            };
            nat_mul(a846, a847)?
        };
        nat_mul(a848, a849)?
    };
    let v4: u64 = f31({
        let a862 = f16(v1)?;
        let a863 = {
            let a860 = {
                let a850 = 1u64;
                let a851 = 1u64;
                nat_add(a850, a851)?
            };
            let a861 = {
                let a858 = {
                    let a852 = 1u64;
                    let a853 = 1u64;
                    nat_add(a852, a853)?
                };
                let a859 = {
                    let a856 = {
                        let a854 = 1u64;
                        let a855 = 1u64;
                        nat_add(a854, a855)?
                    };
                    let a857 = 1u64;
                    nat_mul(a856, a857)?
                };
                nat_mul(a858, a859)?
            };
            nat_mul(a860, a861)?
        };
        nat_mul(a862, a863)?
    }, {
        let a864 = v2;
        let a865 = f32(v1)?;
        nat_sub(a864, a865)
    })?;
    let a866 = f12(v0)?;
    let a867 = f12(v1)?;
    if nat_eq(a866, a867) {
        f30(f12(v0)?, v2, {
            let a868 = v3;
            let a869 = v4;
            nat_add(a868, a869)?
        })
    } else {
        let a870 = v3;
        let a871 = v4;
        if nat_eq(a870, a871) {
            Ok(0u64)
        } else {
            f30(f12(v0)?, v2, {
                let a872 = v3;
                let a873 = v4;
                nat_sub(a872, a873)
            })
        }
    }
}

pub fn f26(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt0> {
    let (h874, _) = f33(Fn1::F34, {
        let a893 = {
            let a875 = 1u64;
            let a876 = 1u64;
            nat_add(a875, a876)?
        };
        let a894 = {
            let a891 = {
                let a877 = 1u64;
                let a878 = 1u64;
                nat_add(a877, a878)?
            };
            let a892 = {
                let a889 = {
                    let a879 = 1u64;
                    let a880 = 1u64;
                    nat_add(a879, a880)?
                };
                let a890 = {
                    let a887 = {
                        let a881 = 1u64;
                        let a882 = 1u64;
                        nat_add(a881, a882)?
                    };
                    let a888 = {
                        let a885 = {
                            let a883 = 1u64;
                            let a884 = 1u64;
                            nat_add(a883, a884)?
                        };
                        let a886 = 1u64;
                        nat_mul(a885, a886)?
                    };
                    nat_mul(a887, a888)?
                };
                nat_mul(a889, a890)?
            };
            nat_mul(a891, a892)?
        };
        nat_mul(a893, a894)?
    }, Adt0::C0(v0, v1, v2, v3))?;
    Ok(h874)
}

pub fn f27(v0: Fn0, p898: u64, p899: u64) -> R<u64> {
    let m897 = (p898, p899);
    {
        let mut m897 = m897;
        loop {
            let step = {
                let (v1, v2) = m897;
                let m895 = v1;
                if m895 == 0 {
                    Err::<(u64, u64), R<u64>>(Ok(v2))
                } else {
                    let v3 = m895 - 1;
                    Ok::<(u64, u64), R<u64>>((v3, {
                        let c896: Fn0 = v0.clone();
                        c896.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m897 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f28(v0: u64) -> R<u64> {
    let a900 = v0;
    let a901 = v0;
    nat_add(a900, a901)
}

pub fn f29(v0: u64) -> R<Adt0> {
    f26(f32(v0)?, f16(v0)?, 0u64, f17({
        let a922 = {
            let a920 = {
                let a902 = 1u64;
                let a903 = 1u64;
                nat_add(a902, a903)?
            };
            let a921 = {
                let a918 = {
                    let a916 = {
                        let a904 = 1u64;
                        let a905 = 1u64;
                        nat_add(a904, a905)?
                    };
                    let a917 = {
                        let a914 = {
                            let a912 = {
                                let a906 = 1u64;
                                let a907 = 1u64;
                                nat_add(a906, a907)?
                            };
                            let a913 = {
                                let a910 = {
                                    let a908 = 1u64;
                                    let a909 = 1u64;
                                    nat_add(a908, a909)?
                                };
                                let a911 = 1u64;
                                nat_mul(a910, a911)?
                            };
                            nat_mul(a912, a913)?
                        };
                        let a915 = 1u64;
                        nat_add(a914, a915)?
                    };
                    nat_mul(a916, a917)?
                };
                let a919 = 1u64;
                nat_add(a918, a919)?
            };
            nat_mul(a920, a921)?
        };
        let a923 = 1u64;
        nat_add(a922, a923)?
    })?)
}

pub fn f30(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a946 = f17({
        let a944 = {
            let a942 = {
                let a924 = 1u64;
                let a925 = 1u64;
                nat_add(a924, a925)?
            };
            let a943 = {
                let a940 = {
                    let a938 = {
                        let a926 = 1u64;
                        let a927 = 1u64;
                        nat_add(a926, a927)?
                    };
                    let a939 = {
                        let a936 = {
                            let a928 = 1u64;
                            let a929 = 1u64;
                            nat_add(a928, a929)?
                        };
                        let a937 = {
                            let a934 = {
                                let a932 = {
                                    let a930 = 1u64;
                                    let a931 = 1u64;
                                    nat_add(a930, a931)?
                                };
                                let a933 = 1u64;
                                nat_mul(a932, a933)?
                            };
                            let a935 = 1u64;
                            nat_add(a934, a935)?
                        };
                        nat_mul(a936, a937)?
                    };
                    nat_mul(a938, a939)?
                };
                let a941 = 1u64;
                nat_add(a940, a941)?
            };
            nat_mul(a942, a943)?
        };
        let a945 = 1u64;
        nat_add(a944, a945)?
    })?;
    let a947 = v2;
    if nat_le(a946, a947) {
        f35(v0, {
            let a948 = v1;
            let a949 = 1u64;
            nat_add(a948, a949)?
        }, f31(v2, 1u64)?)
    } else {
        let v3: Adt0 = f26(v1, v2, {
            let a986 = {
                let a984 = {
                    let a950 = 1u64;
                    let a951 = 1u64;
                    nat_add(a950, a951)?
                };
                let a985 = {
                    let a982 = {
                        let a952 = 1u64;
                        let a953 = 1u64;
                        nat_add(a952, a953)?
                    };
                    let a983 = {
                        let a980 = {
                            let a954 = 1u64;
                            let a955 = 1u64;
                            nat_add(a954, a955)?
                        };
                        let a981 = {
                            let a978 = {
                                let a956 = 1u64;
                                let a957 = 1u64;
                                nat_add(a956, a957)?
                            };
                            let a979 = {
                                let a976 = {
                                    let a958 = 1u64;
                                    let a959 = 1u64;
                                    nat_add(a958, a959)?
                                };
                                let a977 = {
                                    let a974 = {
                                        let a960 = 1u64;
                                        let a961 = 1u64;
                                        nat_add(a960, a961)?
                                    };
                                    let a975 = {
                                        let a972 = {
                                            let a962 = 1u64;
                                            let a963 = 1u64;
                                            nat_add(a962, a963)?
                                        };
                                        let a973 = {
                                            let a970 = {
                                                let a964 = 1u64;
                                                let a965 = 1u64;
                                                nat_add(a964, a965)?
                                            };
                                            let a971 = {
                                                let a968 = {
                                                    let a966 = 1u64;
                                                    let a967 = 1u64;
                                                    nat_add(a966, a967)?
                                                };
                                                let a969 = 1u64;
                                                nat_mul(a968, a969)?
                                            };
                                            nat_mul(a970, a971)?
                                        };
                                        nat_mul(a972, a973)?
                                    };
                                    nat_mul(a974, a975)?
                                };
                                nat_mul(a976, a977)?
                            };
                            nat_mul(a978, a979)?
                        };
                        nat_mul(a980, a981)?
                    };
                    nat_mul(a982, a983)?
                };
                nat_mul(a984, a985)?
            };
            let a987 = 1u64;
            nat_add(a986, a987)?
        }, f17({
            let a1006 = {
                let a988 = 1u64;
                let a989 = 1u64;
                nat_add(a988, a989)?
            };
            let a1007 = {
                let a1004 = {
                    let a1002 = {
                        let a990 = 1u64;
                        let a991 = 1u64;
                        nat_add(a990, a991)?
                    };
                    let a1003 = {
                        let a1000 = {
                            let a992 = 1u64;
                            let a993 = 1u64;
                            nat_add(a992, a993)?
                        };
                        let a1001 = {
                            let a998 = {
                                let a996 = {
                                    let a994 = 1u64;
                                    let a995 = 1u64;
                                    nat_add(a994, a995)?
                                };
                                let a997 = 1u64;
                                nat_mul(a996, a997)?
                            };
                            let a999 = 1u64;
                            nat_add(a998, a999)?
                        };
                        nat_mul(a1000, a1001)?
                    };
                    nat_mul(a1002, a1003)?
                };
                let a1005 = 1u64;
                nat_add(a1004, a1005)?
            };
            nat_mul(a1006, a1007)?
        })?)?;
        f35(v0, match v3.clone() {
            Adt0::C0(h1008, _, _, _) => {
                h1008
            }
        }, match v3 {
            Adt0::C0(_, h1009, _, _) => {
                h1009
            }
        })
    }
}

pub fn f31(v0: u64, v1: u64) -> R<u64> {
    let a1010 = v1;
    let a1011 = 0u64;
    if nat_eq(a1010, a1011) {
        Ok(v0)
    } else {
        let a1036 = {
            let a1034 = {
                let a1012 = 1u64;
                let a1013 = 1u64;
                nat_add(a1012, a1013)?
            };
            let a1035 = {
                let a1032 = {
                    let a1014 = 1u64;
                    let a1015 = 1u64;
                    nat_add(a1014, a1015)?
                };
                let a1033 = {
                    let a1030 = {
                        let a1016 = 1u64;
                        let a1017 = 1u64;
                        nat_add(a1016, a1017)?
                    };
                    let a1031 = {
                        let a1028 = {
                            let a1018 = 1u64;
                            let a1019 = 1u64;
                            nat_add(a1018, a1019)?
                        };
                        let a1029 = {
                            let a1026 = {
                                let a1020 = 1u64;
                                let a1021 = 1u64;
                                nat_add(a1020, a1021)?
                            };
                            let a1027 = {
                                let a1024 = {
                                    let a1022 = 1u64;
                                    let a1023 = 1u64;
                                    nat_add(a1022, a1023)?
                                };
                                let a1025 = 1u64;
                                nat_mul(a1024, a1025)?
                            };
                            nat_mul(a1026, a1027)?
                        };
                        nat_mul(a1028, a1029)?
                    };
                    nat_mul(a1030, a1031)?
                };
                nat_mul(a1032, a1033)?
            };
            nat_mul(a1034, a1035)?
        };
        let a1037 = v1;
        if nat_le(a1036, a1037) {
            let a1038 = v0;
            let a1039 = 0u64;
            if nat_eq(a1038, a1039) {
                Ok(0u64)
            } else {
                Ok(1u64)
            }
        } else {
            let v2: u64 = f17(v1)?;
            f36({
                let a1040 = v0;
                let a1041 = v2;
                let a1042 = 0u64;
                nat_quot(a1040, a1041, a1042)
            }, {
                let a1046 = 0u64;
                let a1047 = {
                    let a1043 = v0;
                    let a1044 = v2;
                    let a1045 = 0u64;
                    nat_rem(a1043, a1044, a1045)
                };
                nat_lt(a1046, a1047)
            })
        }
    }
}

pub fn f32(v0: u64) -> R<u64> {
    let a1086 = {
        let a1048 = f13(v0)?;
        let a1049 = 0u64;
        if nat_eq(a1048, a1049) {
            1u64
        } else {
            f13(v0)?
        }
    };
    let a1087 = {
        let a1084 = {
            let a1050 = 1u64;
            let a1051 = 1u64;
            nat_add(a1050, a1051)?
        };
        let a1085 = {
            let a1082 = {
                let a1052 = 1u64;
                let a1053 = 1u64;
                nat_add(a1052, a1053)?
            };
            let a1083 = {
                let a1080 = {
                    let a1054 = 1u64;
                    let a1055 = 1u64;
                    nat_add(a1054, a1055)?
                };
                let a1081 = {
                    let a1078 = {
                        let a1056 = 1u64;
                        let a1057 = 1u64;
                        nat_add(a1056, a1057)?
                    };
                    let a1079 = {
                        let a1076 = {
                            let a1058 = 1u64;
                            let a1059 = 1u64;
                            nat_add(a1058, a1059)?
                        };
                        let a1077 = {
                            let a1074 = {
                                let a1060 = 1u64;
                                let a1061 = 1u64;
                                nat_add(a1060, a1061)?
                            };
                            let a1075 = {
                                let a1072 = {
                                    let a1062 = 1u64;
                                    let a1063 = 1u64;
                                    nat_add(a1062, a1063)?
                                };
                                let a1073 = {
                                    let a1070 = {
                                        let a1064 = 1u64;
                                        let a1065 = 1u64;
                                        nat_add(a1064, a1065)?
                                    };
                                    let a1071 = {
                                        let a1068 = {
                                            let a1066 = 1u64;
                                            let a1067 = 1u64;
                                            nat_add(a1066, a1067)?
                                        };
                                        let a1069 = 1u64;
                                        nat_mul(a1068, a1069)?
                                    };
                                    nat_mul(a1070, a1071)?
                                };
                                nat_mul(a1072, a1073)?
                            };
                            nat_mul(a1074, a1075)?
                        };
                        nat_mul(a1076, a1077)?
                    };
                    nat_mul(a1078, a1079)?
                };
                nat_mul(a1080, a1081)?
            };
            nat_mul(a1082, a1083)?
        };
        nat_mul(a1084, a1085)?
    };
    nat_add(a1086, a1087)
}

pub fn f33(v0: Fn1, p1092: u64, p1093: Adt0) -> R<(Adt0, bool)> {
    let m1091 = (p1092, p1093);
    {
        let mut m1091 = m1091;
        loop {
            let step = {
                let (v1, v2) = m1091;
                let m1088 = v1;
                if m1088 == 0 {
                    Err::<(u64, Adt0), R<(Adt0, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1088 - 1;
                    let m1090 = {
                        let c1089: Fn1 = v0.clone();
                        c1089.apply(v2.clone())?
                    };
                    match m1090 {
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
                Ok(next) => { m1091 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f34(v0: Adt0) -> R<Option<Adt0>> {
    f37(v0)
}

pub fn f35(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1132 = v1;
    let a1133 = {
        let a1130 = {
            let a1128 = {
                let a1094 = 1u64;
                let a1095 = 1u64;
                nat_add(a1094, a1095)?
            };
            let a1129 = {
                let a1126 = {
                    let a1096 = 1u64;
                    let a1097 = 1u64;
                    nat_add(a1096, a1097)?
                };
                let a1127 = {
                    let a1124 = {
                        let a1098 = 1u64;
                        let a1099 = 1u64;
                        nat_add(a1098, a1099)?
                    };
                    let a1125 = {
                        let a1122 = {
                            let a1100 = 1u64;
                            let a1101 = 1u64;
                            nat_add(a1100, a1101)?
                        };
                        let a1123 = {
                            let a1120 = {
                                let a1102 = 1u64;
                                let a1103 = 1u64;
                                nat_add(a1102, a1103)?
                            };
                            let a1121 = {
                                let a1118 = {
                                    let a1104 = 1u64;
                                    let a1105 = 1u64;
                                    nat_add(a1104, a1105)?
                                };
                                let a1119 = {
                                    let a1116 = {
                                        let a1106 = 1u64;
                                        let a1107 = 1u64;
                                        nat_add(a1106, a1107)?
                                    };
                                    let a1117 = {
                                        let a1114 = {
                                            let a1108 = 1u64;
                                            let a1109 = 1u64;
                                            nat_add(a1108, a1109)?
                                        };
                                        let a1115 = {
                                            let a1112 = {
                                                let a1110 = 1u64;
                                                let a1111 = 1u64;
                                                nat_add(a1110, a1111)?
                                            };
                                            let a1113 = 1u64;
                                            nat_mul(a1112, a1113)?
                                        };
                                        nat_mul(a1114, a1115)?
                                    };
                                    nat_mul(a1116, a1117)?
                                };
                                nat_mul(a1118, a1119)?
                            };
                            nat_mul(a1120, a1121)?
                        };
                        nat_mul(a1122, a1123)?
                    };
                    nat_mul(a1124, a1125)?
                };
                nat_mul(a1126, a1127)?
            };
            nat_mul(a1128, a1129)?
        };
        let a1131 = 1u64;
        nat_add(a1130, a1131)?
    };
    if nat_lt(a1132, a1133) {
        f38(v0, {
            let a1170 = {
                let a1168 = {
                    let a1134 = 1u64;
                    let a1135 = 1u64;
                    nat_add(a1134, a1135)?
                };
                let a1169 = {
                    let a1166 = {
                        let a1136 = 1u64;
                        let a1137 = 1u64;
                        nat_add(a1136, a1137)?
                    };
                    let a1167 = {
                        let a1164 = {
                            let a1138 = 1u64;
                            let a1139 = 1u64;
                            nat_add(a1138, a1139)?
                        };
                        let a1165 = {
                            let a1162 = {
                                let a1140 = 1u64;
                                let a1141 = 1u64;
                                nat_add(a1140, a1141)?
                            };
                            let a1163 = {
                                let a1160 = {
                                    let a1142 = 1u64;
                                    let a1143 = 1u64;
                                    nat_add(a1142, a1143)?
                                };
                                let a1161 = {
                                    let a1158 = {
                                        let a1144 = 1u64;
                                        let a1145 = 1u64;
                                        nat_add(a1144, a1145)?
                                    };
                                    let a1159 = {
                                        let a1156 = {
                                            let a1146 = 1u64;
                                            let a1147 = 1u64;
                                            nat_add(a1146, a1147)?
                                        };
                                        let a1157 = {
                                            let a1154 = {
                                                let a1148 = 1u64;
                                                let a1149 = 1u64;
                                                nat_add(a1148, a1149)?
                                            };
                                            let a1155 = {
                                                let a1152 = {
                                                    let a1150 = 1u64;
                                                    let a1151 = 1u64;
                                                    nat_add(a1150, a1151)?
                                                };
                                                let a1153 = 1u64;
                                                nat_mul(a1152, a1153)?
                                            };
                                            nat_mul(a1154, a1155)?
                                        };
                                        nat_mul(a1156, a1157)?
                                    };
                                    nat_mul(a1158, a1159)?
                                };
                                nat_mul(a1160, a1161)?
                            };
                            nat_mul(a1162, a1163)?
                        };
                        nat_mul(a1164, a1165)?
                    };
                    nat_mul(a1166, a1167)?
                };
                nat_mul(a1168, a1169)?
            };
            let a1171 = 1u64;
            nat_add(a1170, a1171)?
        }, f31(v2, {
            let a1210 = {
                let a1208 = {
                    let a1206 = {
                        let a1172 = 1u64;
                        let a1173 = 1u64;
                        nat_add(a1172, a1173)?
                    };
                    let a1207 = {
                        let a1204 = {
                            let a1174 = 1u64;
                            let a1175 = 1u64;
                            nat_add(a1174, a1175)?
                        };
                        let a1205 = {
                            let a1202 = {
                                let a1176 = 1u64;
                                let a1177 = 1u64;
                                nat_add(a1176, a1177)?
                            };
                            let a1203 = {
                                let a1200 = {
                                    let a1178 = 1u64;
                                    let a1179 = 1u64;
                                    nat_add(a1178, a1179)?
                                };
                                let a1201 = {
                                    let a1198 = {
                                        let a1180 = 1u64;
                                        let a1181 = 1u64;
                                        nat_add(a1180, a1181)?
                                    };
                                    let a1199 = {
                                        let a1196 = {
                                            let a1182 = 1u64;
                                            let a1183 = 1u64;
                                            nat_add(a1182, a1183)?
                                        };
                                        let a1197 = {
                                            let a1194 = {
                                                let a1184 = 1u64;
                                                let a1185 = 1u64;
                                                nat_add(a1184, a1185)?
                                            };
                                            let a1195 = {
                                                let a1192 = {
                                                    let a1186 = 1u64;
                                                    let a1187 = 1u64;
                                                    nat_add(a1186, a1187)?
                                                };
                                                let a1193 = {
                                                    let a1190 = {
                                                        let a1188 = 1u64;
                                                        let a1189 = 1u64;
                                                        nat_add(a1188, a1189)?
                                                    };
                                                    let a1191 = 1u64;
                                                    nat_mul(a1190, a1191)?
                                                };
                                                nat_mul(a1192, a1193)?
                                            };
                                            nat_mul(a1194, a1195)?
                                        };
                                        nat_mul(a1196, a1197)?
                                    };
                                    nat_mul(a1198, a1199)?
                                };
                                nat_mul(a1200, a1201)?
                            };
                            nat_mul(a1202, a1203)?
                        };
                        nat_mul(a1204, a1205)?
                    };
                    nat_mul(a1206, a1207)?
                };
                let a1209 = 1u64;
                nat_add(a1208, a1209)?
            };
            let a1211 = v1;
            nat_sub(a1210, a1211)
        })?)
    } else {
        f38(v0, v1, v2)
    }
}

pub fn f36(v0: u64, v1: bool) -> R<u64> {
    if if v1 {
        let a1219 = {
            let a1216 = v0;
            let a1217 = {
                let a1214 = {
                    let a1212 = 1u64;
                    let a1213 = 1u64;
                    nat_add(a1212, a1213)?
                };
                let a1215 = 1u64;
                nat_mul(a1214, a1215)?
            };
            let a1218 = 0u64;
            nat_rem(a1216, a1217, a1218)
        };
        let a1220 = 0u64;
        nat_eq(a1219, a1220)
    } else {
        false
    } {
        let a1221 = v0;
        let a1222 = 1u64;
        nat_add(a1221, a1222)
    } else {
        Ok(v0)
    }
}

pub fn f37(v0: Adt0) -> R<Option<Adt0>> {
    let a1224 = 0u64;
    let a1225 = match v0.clone() {
        Adt0::C0(_, h1223, _, _) => {
            h1223
        }
    };
    if if if nat_lt(a1224, a1225) {
        let a1228 = match v0.clone() {
            Adt0::C0(_, h1226, _, _) => {
                h1226
            }
        };
        let a1229 = match v0.clone() {
            Adt0::C0(_, _, _, h1227) => {
                h1227
            }
        };
        nat_lt(a1228, a1229)
    } else {
        false
    } {
        let a1232 = match v0.clone() {
            Adt0::C0(_, _, h1230, _) => {
                h1230
            }
        };
        let a1233 = match v0.clone() {
            Adt0::C0(h1231, _, _, _) => {
                h1231
            }
        };
        nat_lt(a1232, a1233)
    } else {
        false
    } {
        Ok(Some(Adt0::C0({
            let a1235 = match v0.clone() {
                Adt0::C0(h1234, _, _, _) => {
                    h1234
                }
            };
            let a1236 = 1u64;
            nat_sub(a1235, a1236)
        }, {
            let a1239 = match v0.clone() {
                Adt0::C0(_, h1237, _, _) => {
                    h1237
                }
            };
            let a1240 = match v0.clone() {
                Adt0::C0(_, h1238, _, _) => {
                    h1238
                }
            };
            nat_add(a1239, a1240)?
        }, match v0.clone() {
            Adt0::C0(_, _, h1241, _) => {
                h1241
            }
        }, match v0 {
            Adt0::C0(_, _, _, h1242) => {
                h1242
            }
        })))
    } else {
        Ok(None::<Adt0>)
    }
}

pub fn f38(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let v3: u64 = {
        let a1255 = v2;
        let a1256 = {
            let a1253 = {
                let a1243 = 1u64;
                let a1244 = 1u64;
                nat_add(a1243, a1244)?
            };
            let a1254 = {
                let a1251 = {
                    let a1245 = 1u64;
                    let a1246 = 1u64;
                    nat_add(a1245, a1246)?
                };
                let a1252 = {
                    let a1249 = {
                        let a1247 = 1u64;
                        let a1248 = 1u64;
                        nat_add(a1247, a1248)?
                    };
                    let a1250 = 1u64;
                    nat_mul(a1249, a1250)?
                };
                nat_mul(a1251, a1252)?
            };
            nat_mul(a1253, a1254)?
        };
        let a1257 = 0u64;
        nat_rem(a1255, a1256, a1257)
    };
    let v4: u64 = {
        let a1270 = v2;
        let a1271 = {
            let a1268 = {
                let a1258 = 1u64;
                let a1259 = 1u64;
                nat_add(a1258, a1259)?
            };
            let a1269 = {
                let a1266 = {
                    let a1260 = 1u64;
                    let a1261 = 1u64;
                    nat_add(a1260, a1261)?
                };
                let a1267 = {
                    let a1264 = {
                        let a1262 = 1u64;
                        let a1263 = 1u64;
                        nat_add(a1262, a1263)?
                    };
                    let a1265 = 1u64;
                    nat_mul(a1264, a1265)?
                };
                nat_mul(a1266, a1267)?
            };
            nat_mul(a1268, a1269)?
        };
        let a1272 = 0u64;
        nat_quot(a1270, a1271, a1272)
    };
    let v5: bool = {
        let a1281 = {
            let a1279 = {
                let a1273 = 1u64;
                let a1274 = 1u64;
                nat_add(a1273, a1274)?
            };
            let a1280 = {
                let a1277 = {
                    let a1275 = 1u64;
                    let a1276 = 1u64;
                    nat_add(a1275, a1276)?
                };
                let a1278 = 1u64;
                nat_mul(a1277, a1278)?
            };
            nat_mul(a1279, a1280)?
        };
        let a1282 = v3;
        if nat_lt(a1281, a1282) {
            true
        } else {
            let a1291 = v3;
            let a1292 = {
                let a1289 = {
                    let a1283 = 1u64;
                    let a1284 = 1u64;
                    nat_add(a1283, a1284)?
                };
                let a1290 = {
                    let a1287 = {
                        let a1285 = 1u64;
                        let a1286 = 1u64;
                        nat_add(a1285, a1286)?
                    };
                    let a1288 = 1u64;
                    nat_mul(a1287, a1288)?
                };
                nat_mul(a1289, a1290)?
            };
            if nat_eq(a1291, a1292) {
                let a1300 = {
                    let a1297 = v4;
                    let a1298 = {
                        let a1295 = {
                            let a1293 = 1u64;
                            let a1294 = 1u64;
                            nat_add(a1293, a1294)?
                        };
                        let a1296 = 1u64;
                        nat_mul(a1295, a1296)?
                    };
                    let a1299 = 0u64;
                    nat_rem(a1297, a1298, a1299)
                };
                let a1301 = 1u64;
                nat_eq(a1300, a1301)
            } else {
                false
            }
        }
    };
    let v6: u64 = {
        let a1302 = v4;
        let a1303 = if v5 {
            1u64
        } else {
            0u64
        };
        nat_add(a1302, a1303)?
    };
    let v7: bool = {
        let a1332 = {
            let a1330 = {
                let a1306 = {
                    let a1304 = 1u64;
                    let a1305 = 1u64;
                    nat_add(a1304, a1305)?
                };
                let a1307 = 1u64;
                nat_mul(a1306, a1307)?
            };
            let a1331 = f17({
                let a1328 = {
                    let a1326 = {
                        let a1308 = 1u64;
                        let a1309 = 1u64;
                        nat_add(a1308, a1309)?
                    };
                    let a1327 = {
                        let a1324 = {
                            let a1322 = {
                                let a1310 = 1u64;
                                let a1311 = 1u64;
                                nat_add(a1310, a1311)?
                            };
                            let a1323 = {
                                let a1320 = {
                                    let a1318 = {
                                        let a1312 = 1u64;
                                        let a1313 = 1u64;
                                        nat_add(a1312, a1313)?
                                    };
                                    let a1319 = {
                                        let a1316 = {
                                            let a1314 = 1u64;
                                            let a1315 = 1u64;
                                            nat_add(a1314, a1315)?
                                        };
                                        let a1317 = 1u64;
                                        nat_mul(a1316, a1317)?
                                    };
                                    nat_mul(a1318, a1319)?
                                };
                                let a1321 = 1u64;
                                nat_add(a1320, a1321)?
                            };
                            nat_mul(a1322, a1323)?
                        };
                        let a1325 = 1u64;
                        nat_add(a1324, a1325)?
                    };
                    nat_mul(a1326, a1327)?
                };
                let a1329 = 1u64;
                nat_add(a1328, a1329)?
            })?;
            nat_mul(a1330, a1331)?
        };
        let a1333 = v6;
        nat_le(a1332, a1333)
    };
    let v8: u64 = if v7 {
        let a1338 = v6;
        let a1339 = {
            let a1336 = {
                let a1334 = 1u64;
                let a1335 = 1u64;
                nat_add(a1334, a1335)?
            };
            let a1337 = 1u64;
            nat_mul(a1336, a1337)?
        };
        let a1340 = 0u64;
        nat_quot(a1338, a1339, a1340)
    } else {
        v6
    };
    let v9: u64 = if v7 {
        let a1341 = v1;
        let a1342 = 1u64;
        nat_add(a1341, a1342)?
    } else {
        v1
    };
    let v10: u64 = {
        let a1381 = v9;
        let a1382 = {
            let a1379 = {
                let a1377 = {
                    let a1343 = 1u64;
                    let a1344 = 1u64;
                    nat_add(a1343, a1344)?
                };
                let a1378 = {
                    let a1375 = {
                        let a1345 = 1u64;
                        let a1346 = 1u64;
                        nat_add(a1345, a1346)?
                    };
                    let a1376 = {
                        let a1373 = {
                            let a1347 = 1u64;
                            let a1348 = 1u64;
                            nat_add(a1347, a1348)?
                        };
                        let a1374 = {
                            let a1371 = {
                                let a1349 = 1u64;
                                let a1350 = 1u64;
                                nat_add(a1349, a1350)?
                            };
                            let a1372 = {
                                let a1369 = {
                                    let a1351 = 1u64;
                                    let a1352 = 1u64;
                                    nat_add(a1351, a1352)?
                                };
                                let a1370 = {
                                    let a1367 = {
                                        let a1353 = 1u64;
                                        let a1354 = 1u64;
                                        nat_add(a1353, a1354)?
                                    };
                                    let a1368 = {
                                        let a1365 = {
                                            let a1355 = 1u64;
                                            let a1356 = 1u64;
                                            nat_add(a1355, a1356)?
                                        };
                                        let a1366 = {
                                            let a1363 = {
                                                let a1357 = 1u64;
                                                let a1358 = 1u64;
                                                nat_add(a1357, a1358)?
                                            };
                                            let a1364 = {
                                                let a1361 = {
                                                    let a1359 = 1u64;
                                                    let a1360 = 1u64;
                                                    nat_add(a1359, a1360)?
                                                };
                                                let a1362 = 1u64;
                                                nat_mul(a1361, a1362)?
                                            };
                                            nat_mul(a1363, a1364)?
                                        };
                                        nat_mul(a1365, a1366)?
                                    };
                                    nat_mul(a1367, a1368)?
                                };
                                nat_mul(a1369, a1370)?
                            };
                            nat_mul(a1371, a1372)?
                        };
                        nat_mul(a1373, a1374)?
                    };
                    nat_mul(a1375, a1376)?
                };
                nat_mul(a1377, a1378)?
            };
            let a1380 = 1u64;
            nat_add(a1379, a1380)?
        };
        if if nat_eq(a1381, a1382) {
            let a1405 = v8;
            let a1406 = f17({
                let a1403 = {
                    let a1401 = {
                        let a1383 = 1u64;
                        let a1384 = 1u64;
                        nat_add(a1383, a1384)?
                    };
                    let a1402 = {
                        let a1399 = {
                            let a1397 = {
                                let a1385 = 1u64;
                                let a1386 = 1u64;
                                nat_add(a1385, a1386)?
                            };
                            let a1398 = {
                                let a1395 = {
                                    let a1393 = {
                                        let a1387 = 1u64;
                                        let a1388 = 1u64;
                                        nat_add(a1387, a1388)?
                                    };
                                    let a1394 = {
                                        let a1391 = {
                                            let a1389 = 1u64;
                                            let a1390 = 1u64;
                                            nat_add(a1389, a1390)?
                                        };
                                        let a1392 = 1u64;
                                        nat_mul(a1391, a1392)?
                                    };
                                    nat_mul(a1393, a1394)?
                                };
                                let a1396 = 1u64;
                                nat_add(a1395, a1396)?
                            };
                            nat_mul(a1397, a1398)?
                        };
                        let a1400 = 1u64;
                        nat_add(a1399, a1400)?
                    };
                    nat_mul(a1401, a1402)?
                };
                let a1404 = 1u64;
                nat_add(a1403, a1404)?
            })?;
            nat_lt(a1405, a1406)
        } else {
            false
        } {
            0u64
        } else {
            let a1443 = v9;
            let a1444 = {
                let a1441 = {
                    let a1407 = 1u64;
                    let a1408 = 1u64;
                    nat_add(a1407, a1408)?
                };
                let a1442 = {
                    let a1439 = {
                        let a1409 = 1u64;
                        let a1410 = 1u64;
                        nat_add(a1409, a1410)?
                    };
                    let a1440 = {
                        let a1437 = {
                            let a1411 = 1u64;
                            let a1412 = 1u64;
                            nat_add(a1411, a1412)?
                        };
                        let a1438 = {
                            let a1435 = {
                                let a1413 = 1u64;
                                let a1414 = 1u64;
                                nat_add(a1413, a1414)?
                            };
                            let a1436 = {
                                let a1433 = {
                                    let a1415 = 1u64;
                                    let a1416 = 1u64;
                                    nat_add(a1415, a1416)?
                                };
                                let a1434 = {
                                    let a1431 = {
                                        let a1417 = 1u64;
                                        let a1418 = 1u64;
                                        nat_add(a1417, a1418)?
                                    };
                                    let a1432 = {
                                        let a1429 = {
                                            let a1419 = 1u64;
                                            let a1420 = 1u64;
                                            nat_add(a1419, a1420)?
                                        };
                                        let a1430 = {
                                            let a1427 = {
                                                let a1421 = 1u64;
                                                let a1422 = 1u64;
                                                nat_add(a1421, a1422)?
                                            };
                                            let a1428 = {
                                                let a1425 = {
                                                    let a1423 = 1u64;
                                                    let a1424 = 1u64;
                                                    nat_add(a1423, a1424)?
                                                };
                                                let a1426 = 1u64;
                                                nat_mul(a1425, a1426)?
                                            };
                                            nat_mul(a1427, a1428)?
                                        };
                                        nat_mul(a1429, a1430)?
                                    };
                                    nat_mul(a1431, a1432)?
                                };
                                nat_mul(a1433, a1434)?
                            };
                            nat_mul(a1435, a1436)?
                        };
                        nat_mul(a1437, a1438)?
                    };
                    nat_mul(a1439, a1440)?
                };
                nat_mul(a1441, a1442)?
            };
            nat_sub(a1443, a1444)
        }
    };
    let a1445 = v8;
    let a1446 = 0u64;
    if nat_eq(a1445, a1446) {
        f22(v0)
    } else {
        let a1499 = {
            let a1497 = {
                let a1495 = {
                    let a1447 = 1u64;
                    let a1448 = 1u64;
                    nat_add(a1447, a1448)?
                };
                let a1496 = {
                    let a1493 = {
                        let a1491 = {
                            let a1449 = 1u64;
                            let a1450 = 1u64;
                            nat_add(a1449, a1450)?
                        };
                        let a1492 = {
                            let a1489 = {
                                let a1487 = {
                                    let a1451 = 1u64;
                                    let a1452 = 1u64;
                                    nat_add(a1451, a1452)?
                                };
                                let a1488 = {
                                    let a1485 = {
                                        let a1483 = {
                                            let a1453 = 1u64;
                                            let a1454 = 1u64;
                                            nat_add(a1453, a1454)?
                                        };
                                        let a1484 = {
                                            let a1481 = {
                                                let a1479 = {
                                                    let a1455 = 1u64;
                                                    let a1456 = 1u64;
                                                    nat_add(a1455, a1456)?
                                                };
                                                let a1480 = {
                                                    let a1477 = {
                                                        let a1475 = {
                                                            let a1457 = 1u64;
                                                            let a1458 = 1u64;
                                                            nat_add(a1457, a1458)?
                                                        };
                                                        let a1476 = {
                                                            let a1473 = {
                                                                let a1471 = {
                                                                    let a1459 = 1u64;
                                                                    let a1460 = 1u64;
                                                                    nat_add(a1459, a1460)?
                                                                };
                                                                let a1472 = {
                                                                    let a1469 = {
                                                                        let a1467 = {
                                                                            let a1461 = 1u64;
                                                                            let a1462 = 1u64;
                                                                            nat_add(a1461, a1462)?
                                                                        };
                                                                        let a1468 = {
                                                                            let a1465 = {
                                                                                let a1463 = 1u64;
                                                                                let a1464 = 1u64;
                                                                                nat_add(a1463, a1464)?
                                                                            };
                                                                            let a1466 = 1u64;
                                                                            nat_mul(a1465, a1466)?
                                                                        };
                                                                        nat_mul(a1467, a1468)?
                                                                    };
                                                                    let a1470 = 1u64;
                                                                    nat_add(a1469, a1470)?
                                                                };
                                                                nat_mul(a1471, a1472)?
                                                            };
                                                            let a1474 = 1u64;
                                                            nat_add(a1473, a1474)?
                                                        };
                                                        nat_mul(a1475, a1476)?
                                                    };
                                                    let a1478 = 1u64;
                                                    nat_add(a1477, a1478)?
                                                };
                                                nat_mul(a1479, a1480)?
                                            };
                                            let a1482 = 1u64;
                                            nat_add(a1481, a1482)?
                                        };
                                        nat_mul(a1483, a1484)?
                                    };
                                    let a1486 = 1u64;
                                    nat_add(a1485, a1486)?
                                };
                                nat_mul(a1487, a1488)?
                            };
                            let a1490 = 1u64;
                            nat_add(a1489, a1490)?
                        };
                        nat_mul(a1491, a1492)?
                    };
                    let a1494 = 1u64;
                    nat_add(a1493, a1494)?
                };
                nat_mul(a1495, a1496)?
            };
            let a1498 = 1u64;
            nat_add(a1497, a1498)?
        };
        let a1500 = v9;
        if nat_le(a1499, a1500) {
            f20(v0)
        } else {
            let a1552 = {
                let a1525 = f22(v0)?;
                let a1526 = {
                    let a1523 = v10;
                    let a1524 = f17({
                        let a1521 = {
                            let a1519 = {
                                let a1501 = 1u64;
                                let a1502 = 1u64;
                                nat_add(a1501, a1502)?
                            };
                            let a1520 = {
                                let a1517 = {
                                    let a1515 = {
                                        let a1503 = 1u64;
                                        let a1504 = 1u64;
                                        nat_add(a1503, a1504)?
                                    };
                                    let a1516 = {
                                        let a1513 = {
                                            let a1511 = {
                                                let a1505 = 1u64;
                                                let a1506 = 1u64;
                                                nat_add(a1505, a1506)?
                                            };
                                            let a1512 = {
                                                let a1509 = {
                                                    let a1507 = 1u64;
                                                    let a1508 = 1u64;
                                                    nat_add(a1507, a1508)?
                                                };
                                                let a1510 = 1u64;
                                                nat_mul(a1509, a1510)?
                                            };
                                            nat_mul(a1511, a1512)?
                                        };
                                        let a1514 = 1u64;
                                        nat_add(a1513, a1514)?
                                    };
                                    nat_mul(a1515, a1516)?
                                };
                                let a1518 = 1u64;
                                nat_add(a1517, a1518)?
                            };
                            nat_mul(a1519, a1520)?
                        };
                        let a1522 = 1u64;
                        nat_add(a1521, a1522)?
                    })?;
                    nat_mul(a1523, a1524)?
                };
                nat_add(a1525, a1526)?
            };
            let a1553 = {
                let a1549 = v8;
                let a1550 = f17({
                    let a1547 = {
                        let a1545 = {
                            let a1527 = 1u64;
                            let a1528 = 1u64;
                            nat_add(a1527, a1528)?
                        };
                        let a1546 = {
                            let a1543 = {
                                let a1541 = {
                                    let a1529 = 1u64;
                                    let a1530 = 1u64;
                                    nat_add(a1529, a1530)?
                                };
                                let a1542 = {
                                    let a1539 = {
                                        let a1537 = {
                                            let a1531 = 1u64;
                                            let a1532 = 1u64;
                                            nat_add(a1531, a1532)?
                                        };
                                        let a1538 = {
                                            let a1535 = {
                                                let a1533 = 1u64;
                                                let a1534 = 1u64;
                                                nat_add(a1533, a1534)?
                                            };
                                            let a1536 = 1u64;
                                            nat_mul(a1535, a1536)?
                                        };
                                        nat_mul(a1537, a1538)?
                                    };
                                    let a1540 = 1u64;
                                    nat_add(a1539, a1540)?
                                };
                                nat_mul(a1541, a1542)?
                            };
                            let a1544 = 1u64;
                            nat_add(a1543, a1544)?
                        };
                        nat_mul(a1545, a1546)?
                    };
                    let a1548 = 1u64;
                    nat_add(a1547, a1548)?
                })?;
                let a1551 = 0u64;
                nat_rem(a1549, a1550, a1551)
            };
            nat_add(a1552, a1553)
        }
    }
}

pub fn run(p0: u64, p1: u64, p2: u64, p3: u64, p4: u64) -> R<Option<u64>> {
    f0(p0, p1, p2, p3, p4)
}
