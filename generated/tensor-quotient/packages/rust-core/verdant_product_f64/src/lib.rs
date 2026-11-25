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
    C0(u64),
    C1(u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Fn0 {
    F14,
}

impl Fn0 {
    pub fn apply(&self, p0: Adt1) -> R<Option<Adt1>> {
        match self {
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

pub fn f0(v0: u64, v1: u64) -> R<Adt0> {
    let v2: u64 = f1(v0, v1)?;
    let v3: bool = {
        let a1 = f2(v0);
        let a2 = 0u64;
        if nat_eq(a1, a2) {
            true
        } else {
            let a3 = f2(v1);
            let a4 = 0u64;
            nat_eq(a3, a4)
        }
    };
    let v4: bool = if f3(v0) {
        true
    } else {
        f3(v1)
    };
    if if if f4(v0) {
        true
    } else {
        f4(v1)
    } {
        true
    } else {
        if v3 {
            v4
        } else {
            false
        }
    } {
        Ok(Adt0::C0(9221120237041090560u64))
    } else {
        if v4 {
            Ok(Adt0::C0(f5(v2)))
        } else {
            if v3 {
                Ok(Adt0::C0({
                    let a5 = v2;
                    let a6 = 0u64;
                    if nat_eq(a5, a6) {
                        0u64
                    } else {
                        9223372036854775808u64
                    }
                }))
            } else {
                f6(v2, v0, v1)
            }
        }
    }
}

pub fn f1(v0: u64, v1: u64) -> R<u64> {
    let a9 = {
        let a7 = f7(v0);
        let a8 = f7(v1);
        nat_add(a7, a8)?
    };
    let a10 = 2u64;
    let a11 = 0u64;
    Ok(nat_rem(a9, a10, a11))
}

pub fn f2(v0: u64) -> u64 {
    let a12 = v0;
    let a13 = 9223372036854775808u64;
    let a14 = 0u64;
    nat_rem(a12, a13, a14)
}

pub fn f3(v0: u64) -> bool {
    let a15 = f8(v0);
    let a16 = 2047u64;
    if nat_eq(a15, a16) {
        let a17 = f9(v0);
        let a18 = 0u64;
        nat_eq(a17, a18)
    } else {
        false
    }
}

pub fn f4(v0: u64) -> bool {
    let a19 = f8(v0);
    let a20 = 2047u64;
    if nat_eq(a19, a20) {
        let a21 = 0u64;
        let a22 = f9(v0);
        nat_lt(a21, a22)
    } else {
        false
    }
}

pub fn f5(v0: u64) -> u64 {
    let a23 = v0;
    let a24 = 0u64;
    if nat_eq(a23, a24) {
        9218868437227405312u64
    } else {
        18442240474082181120u64
    }
}

pub fn f6(v0: u64, v1: u64, v2: u64) -> R<Adt0> {
    let v3: Adt1 = f10(v1)?;
    let v4: Adt1 = f10(v2)?;
    let v5: (u64, u64) = f11(match v3.clone() {
        Adt1::C0(_, h25, _, _) => {
            h25
        }
    }, match v4.clone() {
        Adt1::C0(_, h26, _, _) => {
            h26
        }
    })?;
    f12(v0, {
        let a29 = match v3 {
            Adt1::C0(h27, _, _, _) => {
                h27
            }
        };
        let a30 = match v4 {
            Adt1::C0(h28, _, _, _) => {
                h28
            }
        };
        nat_add(a29, a30)?
    }, {
        let (h31, _) = v5;
        h31
    }, {
        let (_, h32) = v5;
        h32
    })
}

pub fn f7(v0: u64) -> u64 {
    let a33 = v0;
    let a34 = 9223372036854775808u64;
    let a35 = 0u64;
    nat_quot(a33, a34, a35)
}

pub fn f8(v0: u64) -> u64 {
    let a36 = f2(v0);
    let a37 = 4503599627370496u64;
    let a38 = 0u64;
    nat_quot(a36, a37, a38)
}

pub fn f9(v0: u64) -> u64 {
    let a39 = v0;
    let a40 = 4503599627370496u64;
    let a41 = 0u64;
    nat_rem(a39, a40, a41)
}

pub fn f10(v0: u64) -> R<Adt1> {
    let (h42, _) = f13(Fn0::F14, 52u64, Adt1::C0({
        let a43 = f15(v0);
        let a44 = 4096u64;
        nat_add(a43, a44)?
    }, f16(v0)?, 0u64, 4503599627370496u64))?;
    Ok(h42)
}

pub fn f11(v0: u64, v1: u64) -> R<(u64, u64)> {
    let v2: u64 = {
        let a45 = v0;
        let a46 = 67108864u64;
        let a47 = 0u64;
        nat_quot(a45, a46, a47)
    };
    let v3: u64 = {
        let a48 = v0;
        let a49 = 67108864u64;
        let a50 = 0u64;
        nat_rem(a48, a49, a50)
    };
    let v4: u64 = {
        let a51 = v1;
        let a52 = 67108864u64;
        let a53 = 0u64;
        nat_quot(a51, a52, a53)
    };
    let v5: u64 = {
        let a54 = v1;
        let a55 = 67108864u64;
        let a56 = 0u64;
        nat_rem(a54, a55, a56)
    };
    let v6: u64 = {
        let a57 = v3;
        let a58 = v5;
        nat_mul(a57, a58)?
    };
    let v7: u64 = {
        let a68 = {
            let a63 = {
                let a59 = v2;
                let a60 = v5;
                nat_mul(a59, a60)?
            };
            let a64 = {
                let a61 = v3;
                let a62 = v4;
                nat_mul(a61, a62)?
            };
            nat_add(a63, a64)?
        };
        let a69 = {
            let a65 = v6;
            let a66 = 67108864u64;
            let a67 = 0u64;
            nat_quot(a65, a66, a67)
        };
        nat_add(a68, a69)?
    };
    Ok(({
        let a75 = {
            let a70 = v2;
            let a71 = v4;
            nat_mul(a70, a71)?
        };
        let a76 = {
            let a72 = v7;
            let a73 = 67108864u64;
            let a74 = 0u64;
            nat_quot(a72, a73, a74)
        };
        nat_add(a75, a76)?
    }, {
        let a85 = {
            let a80 = {
                let a77 = v7;
                let a78 = 67108864u64;
                let a79 = 0u64;
                nat_rem(a77, a78, a79)
            };
            let a81 = 67108864u64;
            nat_mul(a80, a81)?
        };
        let a86 = {
            let a82 = v6;
            let a83 = 67108864u64;
            let a84 = 0u64;
            nat_rem(a82, a83, a84)
        };
        nat_add(a85, a86)?
    }))
}

pub fn f12(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt0> {
    let v4: bool = {
        let a87 = 9007199254740992u64;
        let a88 = v2;
        nat_le(a87, a88)
    };
    let v5: u64 = {
        let a89 = v1;
        let a90 = if v4 {
            1u64
        } else {
            0u64
        };
        nat_add(a89, a90)?
    };
    let v6: u64 = f17(v2, v3, if v4 {
        4u64
    } else {
        8u64
    })?;
    let a91 = 11262u64;
    let a92 = v5;
    if nat_le(a91, a92) {
        Ok(Adt0::C0(f5(v0)))
    } else {
        Ok(Adt0::C1(v0, {
            let a93 = v5;
            let a94 = 9215u64;
            if nat_le(a93, a94) {
                1u64
            } else {
                let a95 = v5;
                let a96 = 9215u64;
                nat_sub(a95, a96)
            }
        }, f18(v6, {
            let a97 = v5;
            let a98 = 9215u64;
            if nat_le(a97, a98) {
                let a99 = 9216u64;
                let a100 = v5;
                nat_sub(a99, a100)
            } else {
                0u64
            }
        })?))
    }
}

pub fn f13(v0: Fn0, p105: u64, p106: Adt1) -> R<(Adt1, bool)> {
    let m104 = (p105, p106);
    {
        let mut m104 = m104;
        loop {
            let step = {
                let (v1, v2) = m104;
                let m101 = v1;
                if m101 == 0 {
                    Err::<(u64, Adt1), R<(Adt1, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m101 - 1;
                    let m103 = {
                        let c102: Fn0 = v0.clone();
                        c102.apply(v2.clone())?
                    };
                    match m103 {
                        None => {
                            Err::<(u64, Adt1), R<(Adt1, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt1), R<(Adt1, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m104 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f14(v0: Adt1) -> R<Option<Adt1>> {
    f19(v0)
}

pub fn f15(v0: u64) -> u64 {
    let a107 = f8(v0);
    let a108 = 0u64;
    if nat_eq(a107, a108) {
        1u64
    } else {
        f8(v0)
    }
}

pub fn f16(v0: u64) -> R<u64> {
    let a111 = f9(v0);
    let a112 = {
        let a109 = f8(v0);
        let a110 = 0u64;
        if nat_eq(a109, a110) {
            0u64
        } else {
            4503599627370496u64
        }
    };
    nat_add(a111, a112)
}

pub fn f17(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let v3: u64 = {
        let a113 = 4503599627370496u64;
        let a114 = v2;
        let a115 = 0u64;
        nat_quot(a113, a114, a115)
    };
    let v4: u64 = {
        let a121 = {
            let a116 = v0;
            let a117 = v2;
            nat_mul(a116, a117)?
        };
        let a122 = {
            let a118 = v1;
            let a119 = v3;
            let a120 = 0u64;
            nat_quot(a118, a119, a120)
        };
        nat_add(a121, a122)?
    };
    let a133 = v4;
    let a134 = {
        let a126 = 0u64;
        let a127 = {
            let a123 = v1;
            let a124 = v3;
            let a125 = 0u64;
            nat_rem(a123, a124, a125)
        };
        if if nat_lt(a126, a127) {
            let a131 = {
                let a128 = v4;
                let a129 = 2u64;
                let a130 = 0u64;
                nat_rem(a128, a129, a130)
            };
            let a132 = 0u64;
            nat_eq(a131, a132)
        } else {
            false
        } {
            1u64
        } else {
            0u64
        }
    };
    nat_add(a133, a134)
}

pub fn f18(v0: u64, v1: u64) -> R<u64> {
    let a135 = v1;
    let a136 = 0u64;
    if nat_eq(a135, a136) {
        Ok(v0)
    } else {
        let a137 = 64u64;
        let a138 = v1;
        if nat_le(a137, a138) {
            let a139 = v0;
            let a140 = 0u64;
            if nat_eq(a139, a140) {
                Ok(0u64)
            } else {
                Ok(1u64)
            }
        } else {
            let v2: u64 = f20(v1)?;
            let v3: u64 = {
                let a141 = v0;
                let a142 = v2;
                let a143 = 0u64;
                nat_quot(a141, a142, a143)
            };
            let a154 = v3;
            let a155 = {
                let a147 = 0u64;
                let a148 = {
                    let a144 = v0;
                    let a145 = v2;
                    let a146 = 0u64;
                    nat_rem(a144, a145, a146)
                };
                if if nat_lt(a147, a148) {
                    let a152 = {
                        let a149 = v3;
                        let a150 = 2u64;
                        let a151 = 0u64;
                        nat_rem(a149, a150, a151)
                    };
                    let a153 = 0u64;
                    nat_eq(a152, a153)
                } else {
                    false
                } {
                    1u64
                } else {
                    0u64
                }
            };
            nat_add(a154, a155)
        }
    }
}

pub fn f19(v0: Adt1) -> R<Option<Adt1>> {
    let a157 = 0u64;
    let a158 = match v0.clone() {
        Adt1::C0(_, h156, _, _) => {
            h156
        }
    };
    if if if nat_lt(a157, a158) {
        let a161 = match v0.clone() {
            Adt1::C0(_, h159, _, _) => {
                h159
            }
        };
        let a162 = match v0.clone() {
            Adt1::C0(_, _, _, h160) => {
                h160
            }
        };
        nat_lt(a161, a162)
    } else {
        false
    } {
        let a165 = match v0.clone() {
            Adt1::C0(_, _, h163, _) => {
                h163
            }
        };
        let a166 = match v0.clone() {
            Adt1::C0(h164, _, _, _) => {
                h164
            }
        };
        nat_lt(a165, a166)
    } else {
        false
    } {
        Ok(Some(Adt1::C0({
            let a168 = match v0.clone() {
                Adt1::C0(h167, _, _, _) => {
                    h167
                }
            };
            let a169 = 1u64;
            nat_sub(a168, a169)
        }, {
            let a172 = match v0.clone() {
                Adt1::C0(_, h170, _, _) => {
                    h170
                }
            };
            let a173 = match v0.clone() {
                Adt1::C0(_, h171, _, _) => {
                    h171
                }
            };
            nat_add(a172, a173)?
        }, match v0.clone() {
            Adt1::C0(_, _, h174, _) => {
                h174
            }
        }, match v0 {
            Adt1::C0(_, _, _, h175) => {
                h175
            }
        })))
    } else {
        Ok(None::<Adt1>)
    }
}

pub fn f20(v0: u64) -> R<u64> {
    f21(Fn1::F22, v0, 1u64)
}

pub fn f21(v0: Fn1, p179: u64, p180: u64) -> R<u64> {
    let m178 = (p179, p180);
    {
        let mut m178 = m178;
        loop {
            let step = {
                let (v1, v2) = m178;
                let m176 = v1;
                if m176 == 0 {
                    Err::<(u64, u64), R<u64>>(Ok(v2))
                } else {
                    let v3 = m176 - 1;
                    Ok::<(u64, u64), R<u64>>((v3, {
                        let c177: Fn1 = v0.clone();
                        c177.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m178 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f22(v0: u64) -> R<u64> {
    let a181 = v0;
    let a182 = v0;
    nat_add(a181, a182)
}

pub fn run(p0: u64, p1: u64) -> R<Adt0> {
    f0(p0, p1)
}
