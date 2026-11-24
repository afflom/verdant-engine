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
    C1(u64, u64),
    C2(u64, u64, u64),
    C3(u64, u64, u64),
    C4(u64, u64),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(u64, u64),
}

#[derive(Clone)]
pub enum Fn0 {
    F23,
}

impl Fn0 {
    pub fn apply(&self, p0: u64) -> R<u64> {
        match self {
            Fn0::F23 => f23(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F28,
}

impl Fn1 {
    pub fn apply(&self, p0: Adt2) -> R<Option<Adt2>> {
        match self {
            Fn1::F28 => f28(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F37,
}

impl Fn2 {
    pub fn apply(&self, p0: Adt1) -> R<Option<Adt1>> {
        match self {
            Fn2::F37 => f37(p0),
        }
    }
}

pub fn f0(v0: Adt0) -> R<u64> {
    let m1 = v0;
    match m1 {
        Adt0::C0(v1) => {
            f1(1065353216u64, f2(v1)?)
        }
        Adt0::C1(v2, v3) => {
            f3(v2, v3)
        }
        Adt0::C2(v4, v5, v6) => {
            f3(f4(v4, f5(v5)?)?, v6)
        }
        Adt0::C3(v7, v8, v9) => {
            f4(v7, f3(v8, v9)?)
        }
        Adt0::C4(v10, v11) => {
            f3(v10, v11)
        }
    }
}

pub fn f1(v0: u64, v1: u64) -> R<u64> {
    if if if if f6(v0)? {
        true
    } else {
        f6(v1)?
    } {
        true
    } else {
        if f7(v0)? {
            f7(v1)?
        } else {
            false
        }
    } {
        true
    } else {
        if f8(v0)? {
            f8(v1)?
        } else {
            false
        }
    } {
        let a88 = {
            let a66 = {
                let a42 = {
                    let a40 = {
                        let a2 = 1u64;
                        let a3 = 1u64;
                        nat_add(a2, a3)?
                    };
                    let a41 = {
                        let a38 = {
                            let a36 = {
                                let a4 = 1u64;
                                let a5 = 1u64;
                                nat_add(a4, a5)?
                            };
                            let a37 = {
                                let a34 = {
                                    let a32 = {
                                        let a6 = 1u64;
                                        let a7 = 1u64;
                                        nat_add(a6, a7)?
                                    };
                                    let a33 = {
                                        let a30 = {
                                            let a28 = {
                                                let a8 = 1u64;
                                                let a9 = 1u64;
                                                nat_add(a8, a9)?
                                            };
                                            let a29 = {
                                                let a26 = {
                                                    let a24 = {
                                                        let a10 = 1u64;
                                                        let a11 = 1u64;
                                                        nat_add(a10, a11)?
                                                    };
                                                    let a25 = {
                                                        let a22 = {
                                                            let a20 = {
                                                                let a12 = 1u64;
                                                                let a13 = 1u64;
                                                                nat_add(a12, a13)?
                                                            };
                                                            let a21 = {
                                                                let a18 = {
                                                                    let a16 = {
                                                                        let a14 = 1u64;
                                                                        let a15 = 1u64;
                                                                        nat_add(a14, a15)?
                                                                    };
                                                                    let a17 = 1u64;
                                                                    nat_mul(a16, a17)?
                                                                };
                                                                let a19 = 1u64;
                                                                nat_add(a18, a19)?
                                                            };
                                                            nat_mul(a20, a21)?
                                                        };
                                                        let a23 = 1u64;
                                                        nat_add(a22, a23)?
                                                    };
                                                    nat_mul(a24, a25)?
                                                };
                                                let a27 = 1u64;
                                                nat_add(a26, a27)?
                                            };
                                            nat_mul(a28, a29)?
                                        };
                                        let a31 = 1u64;
                                        nat_add(a30, a31)?
                                    };
                                    nat_mul(a32, a33)?
                                };
                                let a35 = 1u64;
                                nat_add(a34, a35)?
                            };
                            nat_mul(a36, a37)?
                        };
                        let a39 = 1u64;
                        nat_add(a38, a39)?
                    };
                    nat_mul(a40, a41)?
                };
                let a43 = 1u64;
                nat_add(a42, a43)?
            };
            let a67 = f9({
                let a64 = {
                    let a62 = {
                        let a44 = 1u64;
                        let a45 = 1u64;
                        nat_add(a44, a45)?
                    };
                    let a63 = {
                        let a60 = {
                            let a58 = {
                                let a46 = 1u64;
                                let a47 = 1u64;
                                nat_add(a46, a47)?
                            };
                            let a59 = {
                                let a56 = {
                                    let a54 = {
                                        let a48 = 1u64;
                                        let a49 = 1u64;
                                        nat_add(a48, a49)?
                                    };
                                    let a55 = {
                                        let a52 = {
                                            let a50 = 1u64;
                                            let a51 = 1u64;
                                            nat_add(a50, a51)?
                                        };
                                        let a53 = 1u64;
                                        nat_mul(a52, a53)?
                                    };
                                    nat_mul(a54, a55)?
                                };
                                let a57 = 1u64;
                                nat_add(a56, a57)?
                            };
                            nat_mul(a58, a59)?
                        };
                        let a61 = 1u64;
                        nat_add(a60, a61)?
                    };
                    nat_mul(a62, a63)?
                };
                let a65 = 1u64;
                nat_add(a64, a65)?
            })?;
            nat_mul(a66, a67)?
        };
        let a89 = f9({
            let a86 = {
                let a68 = 1u64;
                let a69 = 1u64;
                nat_add(a68, a69)?
            };
            let a87 = {
                let a84 = {
                    let a82 = {
                        let a70 = 1u64;
                        let a71 = 1u64;
                        nat_add(a70, a71)?
                    };
                    let a83 = {
                        let a80 = {
                            let a78 = {
                                let a72 = 1u64;
                                let a73 = 1u64;
                                nat_add(a72, a73)?
                            };
                            let a79 = {
                                let a76 = {
                                    let a74 = 1u64;
                                    let a75 = 1u64;
                                    nat_add(a74, a75)?
                                };
                                let a77 = 1u64;
                                nat_mul(a76, a77)?
                            };
                            nat_mul(a78, a79)?
                        };
                        let a81 = 1u64;
                        nat_add(a80, a81)?
                    };
                    nat_mul(a82, a83)?
                };
                let a85 = 1u64;
                nat_add(a84, a85)?
            };
            nat_mul(a86, a87)?
        })?;
        nat_add(a88, a89)
    } else {
        if if f7(v0)? {
            true
        } else {
            f8(v1)?
        } {
            f10(f11(v0, v1)?)
        } else {
            if if f7(v1)? {
                true
            } else {
                f8(v0)?
            } {
                f12(f11(v0, v1)?)
            } else {
                f13(v0, v1)
            }
        }
    }
}

pub fn f2(v0: u64) -> R<u64> {
    let m90 = f14(v0)?;
    match m90 {
        None => {
            f15(v0)
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f3(v0: u64, v1: u64) -> R<u64> {
    if if if if f6(v0)? {
        true
    } else {
        f6(v1)?
    } {
        true
    } else {
        if f7(v0)? {
            f8(v1)?
        } else {
            false
        }
    } {
        true
    } else {
        if f7(v1)? {
            f8(v0)?
        } else {
            false
        }
    } {
        let a177 = {
            let a155 = {
                let a131 = {
                    let a129 = {
                        let a91 = 1u64;
                        let a92 = 1u64;
                        nat_add(a91, a92)?
                    };
                    let a130 = {
                        let a127 = {
                            let a125 = {
                                let a93 = 1u64;
                                let a94 = 1u64;
                                nat_add(a93, a94)?
                            };
                            let a126 = {
                                let a123 = {
                                    let a121 = {
                                        let a95 = 1u64;
                                        let a96 = 1u64;
                                        nat_add(a95, a96)?
                                    };
                                    let a122 = {
                                        let a119 = {
                                            let a117 = {
                                                let a97 = 1u64;
                                                let a98 = 1u64;
                                                nat_add(a97, a98)?
                                            };
                                            let a118 = {
                                                let a115 = {
                                                    let a113 = {
                                                        let a99 = 1u64;
                                                        let a100 = 1u64;
                                                        nat_add(a99, a100)?
                                                    };
                                                    let a114 = {
                                                        let a111 = {
                                                            let a109 = {
                                                                let a101 = 1u64;
                                                                let a102 = 1u64;
                                                                nat_add(a101, a102)?
                                                            };
                                                            let a110 = {
                                                                let a107 = {
                                                                    let a105 = {
                                                                        let a103 = 1u64;
                                                                        let a104 = 1u64;
                                                                        nat_add(a103, a104)?
                                                                    };
                                                                    let a106 = 1u64;
                                                                    nat_mul(a105, a106)?
                                                                };
                                                                let a108 = 1u64;
                                                                nat_add(a107, a108)?
                                                            };
                                                            nat_mul(a109, a110)?
                                                        };
                                                        let a112 = 1u64;
                                                        nat_add(a111, a112)?
                                                    };
                                                    nat_mul(a113, a114)?
                                                };
                                                let a116 = 1u64;
                                                nat_add(a115, a116)?
                                            };
                                            nat_mul(a117, a118)?
                                        };
                                        let a120 = 1u64;
                                        nat_add(a119, a120)?
                                    };
                                    nat_mul(a121, a122)?
                                };
                                let a124 = 1u64;
                                nat_add(a123, a124)?
                            };
                            nat_mul(a125, a126)?
                        };
                        let a128 = 1u64;
                        nat_add(a127, a128)?
                    };
                    nat_mul(a129, a130)?
                };
                let a132 = 1u64;
                nat_add(a131, a132)?
            };
            let a156 = f9({
                let a153 = {
                    let a151 = {
                        let a133 = 1u64;
                        let a134 = 1u64;
                        nat_add(a133, a134)?
                    };
                    let a152 = {
                        let a149 = {
                            let a147 = {
                                let a135 = 1u64;
                                let a136 = 1u64;
                                nat_add(a135, a136)?
                            };
                            let a148 = {
                                let a145 = {
                                    let a143 = {
                                        let a137 = 1u64;
                                        let a138 = 1u64;
                                        nat_add(a137, a138)?
                                    };
                                    let a144 = {
                                        let a141 = {
                                            let a139 = 1u64;
                                            let a140 = 1u64;
                                            nat_add(a139, a140)?
                                        };
                                        let a142 = 1u64;
                                        nat_mul(a141, a142)?
                                    };
                                    nat_mul(a143, a144)?
                                };
                                let a146 = 1u64;
                                nat_add(a145, a146)?
                            };
                            nat_mul(a147, a148)?
                        };
                        let a150 = 1u64;
                        nat_add(a149, a150)?
                    };
                    nat_mul(a151, a152)?
                };
                let a154 = 1u64;
                nat_add(a153, a154)?
            })?;
            nat_mul(a155, a156)?
        };
        let a178 = f9({
            let a175 = {
                let a157 = 1u64;
                let a158 = 1u64;
                nat_add(a157, a158)?
            };
            let a176 = {
                let a173 = {
                    let a171 = {
                        let a159 = 1u64;
                        let a160 = 1u64;
                        nat_add(a159, a160)?
                    };
                    let a172 = {
                        let a169 = {
                            let a167 = {
                                let a161 = 1u64;
                                let a162 = 1u64;
                                nat_add(a161, a162)?
                            };
                            let a168 = {
                                let a165 = {
                                    let a163 = 1u64;
                                    let a164 = 1u64;
                                    nat_add(a163, a164)?
                                };
                                let a166 = 1u64;
                                nat_mul(a165, a166)?
                            };
                            nat_mul(a167, a168)?
                        };
                        let a170 = 1u64;
                        nat_add(a169, a170)?
                    };
                    nat_mul(a171, a172)?
                };
                let a174 = 1u64;
                nat_add(a173, a174)?
            };
            nat_mul(a175, a176)?
        })?;
        nat_add(a177, a178)
    } else {
        if if f7(v0)? {
            true
        } else {
            f7(v1)?
        } {
            f10(f11(v0, v1)?)
        } else {
            if if f8(v0)? {
                true
            } else {
                f8(v1)?
            } {
                f12(f11(v0, v1)?)
            } else {
                f16(v0, v1)
            }
        }
    }
}

pub fn f4(v0: u64, v1: u64) -> R<u64> {
    if if f6(v0)? {
        true
    } else {
        f6(v1)?
    } {
        let a265 = {
            let a243 = {
                let a219 = {
                    let a217 = {
                        let a179 = 1u64;
                        let a180 = 1u64;
                        nat_add(a179, a180)?
                    };
                    let a218 = {
                        let a215 = {
                            let a213 = {
                                let a181 = 1u64;
                                let a182 = 1u64;
                                nat_add(a181, a182)?
                            };
                            let a214 = {
                                let a211 = {
                                    let a209 = {
                                        let a183 = 1u64;
                                        let a184 = 1u64;
                                        nat_add(a183, a184)?
                                    };
                                    let a210 = {
                                        let a207 = {
                                            let a205 = {
                                                let a185 = 1u64;
                                                let a186 = 1u64;
                                                nat_add(a185, a186)?
                                            };
                                            let a206 = {
                                                let a203 = {
                                                    let a201 = {
                                                        let a187 = 1u64;
                                                        let a188 = 1u64;
                                                        nat_add(a187, a188)?
                                                    };
                                                    let a202 = {
                                                        let a199 = {
                                                            let a197 = {
                                                                let a189 = 1u64;
                                                                let a190 = 1u64;
                                                                nat_add(a189, a190)?
                                                            };
                                                            let a198 = {
                                                                let a195 = {
                                                                    let a193 = {
                                                                        let a191 = 1u64;
                                                                        let a192 = 1u64;
                                                                        nat_add(a191, a192)?
                                                                    };
                                                                    let a194 = 1u64;
                                                                    nat_mul(a193, a194)?
                                                                };
                                                                let a196 = 1u64;
                                                                nat_add(a195, a196)?
                                                            };
                                                            nat_mul(a197, a198)?
                                                        };
                                                        let a200 = 1u64;
                                                        nat_add(a199, a200)?
                                                    };
                                                    nat_mul(a201, a202)?
                                                };
                                                let a204 = 1u64;
                                                nat_add(a203, a204)?
                                            };
                                            nat_mul(a205, a206)?
                                        };
                                        let a208 = 1u64;
                                        nat_add(a207, a208)?
                                    };
                                    nat_mul(a209, a210)?
                                };
                                let a212 = 1u64;
                                nat_add(a211, a212)?
                            };
                            nat_mul(a213, a214)?
                        };
                        let a216 = 1u64;
                        nat_add(a215, a216)?
                    };
                    nat_mul(a217, a218)?
                };
                let a220 = 1u64;
                nat_add(a219, a220)?
            };
            let a244 = f9({
                let a241 = {
                    let a239 = {
                        let a221 = 1u64;
                        let a222 = 1u64;
                        nat_add(a221, a222)?
                    };
                    let a240 = {
                        let a237 = {
                            let a235 = {
                                let a223 = 1u64;
                                let a224 = 1u64;
                                nat_add(a223, a224)?
                            };
                            let a236 = {
                                let a233 = {
                                    let a231 = {
                                        let a225 = 1u64;
                                        let a226 = 1u64;
                                        nat_add(a225, a226)?
                                    };
                                    let a232 = {
                                        let a229 = {
                                            let a227 = 1u64;
                                            let a228 = 1u64;
                                            nat_add(a227, a228)?
                                        };
                                        let a230 = 1u64;
                                        nat_mul(a229, a230)?
                                    };
                                    nat_mul(a231, a232)?
                                };
                                let a234 = 1u64;
                                nat_add(a233, a234)?
                            };
                            nat_mul(a235, a236)?
                        };
                        let a238 = 1u64;
                        nat_add(a237, a238)?
                    };
                    nat_mul(a239, a240)?
                };
                let a242 = 1u64;
                nat_add(a241, a242)?
            })?;
            nat_mul(a243, a244)?
        };
        let a266 = f9({
            let a263 = {
                let a245 = 1u64;
                let a246 = 1u64;
                nat_add(a245, a246)?
            };
            let a264 = {
                let a261 = {
                    let a259 = {
                        let a247 = 1u64;
                        let a248 = 1u64;
                        nat_add(a247, a248)?
                    };
                    let a260 = {
                        let a257 = {
                            let a255 = {
                                let a249 = 1u64;
                                let a250 = 1u64;
                                nat_add(a249, a250)?
                            };
                            let a256 = {
                                let a253 = {
                                    let a251 = 1u64;
                                    let a252 = 1u64;
                                    nat_add(a251, a252)?
                                };
                                let a254 = 1u64;
                                nat_mul(a253, a254)?
                            };
                            nat_mul(a255, a256)?
                        };
                        let a258 = 1u64;
                        nat_add(a257, a258)?
                    };
                    nat_mul(a259, a260)?
                };
                let a262 = 1u64;
                nat_add(a261, a262)?
            };
            nat_mul(a263, a264)?
        })?;
        nat_add(a265, a266)
    } else {
        if if f7(v0)? {
            f7(v1)?
        } else {
            false
        } {
            let a267 = f17(v0)?;
            let a268 = f17(v1)?;
            if nat_eq(a267, a268) {
                Ok(v0)
            } else {
                let a355 = {
                    let a333 = {
                        let a309 = {
                            let a307 = {
                                let a269 = 1u64;
                                let a270 = 1u64;
                                nat_add(a269, a270)?
                            };
                            let a308 = {
                                let a305 = {
                                    let a303 = {
                                        let a271 = 1u64;
                                        let a272 = 1u64;
                                        nat_add(a271, a272)?
                                    };
                                    let a304 = {
                                        let a301 = {
                                            let a299 = {
                                                let a273 = 1u64;
                                                let a274 = 1u64;
                                                nat_add(a273, a274)?
                                            };
                                            let a300 = {
                                                let a297 = {
                                                    let a295 = {
                                                        let a275 = 1u64;
                                                        let a276 = 1u64;
                                                        nat_add(a275, a276)?
                                                    };
                                                    let a296 = {
                                                        let a293 = {
                                                            let a291 = {
                                                                let a277 = 1u64;
                                                                let a278 = 1u64;
                                                                nat_add(a277, a278)?
                                                            };
                                                            let a292 = {
                                                                let a289 = {
                                                                    let a287 = {
                                                                        let a279 = 1u64;
                                                                        let a280 = 1u64;
                                                                        nat_add(a279, a280)?
                                                                    };
                                                                    let a288 = {
                                                                        let a285 = {
                                                                            let a283 = {
                                                                                let a281 = 1u64;
                                                                                let a282 = 1u64;
                                                                                nat_add(a281, a282)?
                                                                            };
                                                                            let a284 = 1u64;
                                                                            nat_mul(a283, a284)?
                                                                        };
                                                                        let a286 = 1u64;
                                                                        nat_add(a285, a286)?
                                                                    };
                                                                    nat_mul(a287, a288)?
                                                                };
                                                                let a290 = 1u64;
                                                                nat_add(a289, a290)?
                                                            };
                                                            nat_mul(a291, a292)?
                                                        };
                                                        let a294 = 1u64;
                                                        nat_add(a293, a294)?
                                                    };
                                                    nat_mul(a295, a296)?
                                                };
                                                let a298 = 1u64;
                                                nat_add(a297, a298)?
                                            };
                                            nat_mul(a299, a300)?
                                        };
                                        let a302 = 1u64;
                                        nat_add(a301, a302)?
                                    };
                                    nat_mul(a303, a304)?
                                };
                                let a306 = 1u64;
                                nat_add(a305, a306)?
                            };
                            nat_mul(a307, a308)?
                        };
                        let a310 = 1u64;
                        nat_add(a309, a310)?
                    };
                    let a334 = f9({
                        let a331 = {
                            let a329 = {
                                let a311 = 1u64;
                                let a312 = 1u64;
                                nat_add(a311, a312)?
                            };
                            let a330 = {
                                let a327 = {
                                    let a325 = {
                                        let a313 = 1u64;
                                        let a314 = 1u64;
                                        nat_add(a313, a314)?
                                    };
                                    let a326 = {
                                        let a323 = {
                                            let a321 = {
                                                let a315 = 1u64;
                                                let a316 = 1u64;
                                                nat_add(a315, a316)?
                                            };
                                            let a322 = {
                                                let a319 = {
                                                    let a317 = 1u64;
                                                    let a318 = 1u64;
                                                    nat_add(a317, a318)?
                                                };
                                                let a320 = 1u64;
                                                nat_mul(a319, a320)?
                                            };
                                            nat_mul(a321, a322)?
                                        };
                                        let a324 = 1u64;
                                        nat_add(a323, a324)?
                                    };
                                    nat_mul(a325, a326)?
                                };
                                let a328 = 1u64;
                                nat_add(a327, a328)?
                            };
                            nat_mul(a329, a330)?
                        };
                        let a332 = 1u64;
                        nat_add(a331, a332)?
                    })?;
                    nat_mul(a333, a334)?
                };
                let a356 = f9({
                    let a353 = {
                        let a335 = 1u64;
                        let a336 = 1u64;
                        nat_add(a335, a336)?
                    };
                    let a354 = {
                        let a351 = {
                            let a349 = {
                                let a337 = 1u64;
                                let a338 = 1u64;
                                nat_add(a337, a338)?
                            };
                            let a350 = {
                                let a347 = {
                                    let a345 = {
                                        let a339 = 1u64;
                                        let a340 = 1u64;
                                        nat_add(a339, a340)?
                                    };
                                    let a346 = {
                                        let a343 = {
                                            let a341 = 1u64;
                                            let a342 = 1u64;
                                            nat_add(a341, a342)?
                                        };
                                        let a344 = 1u64;
                                        nat_mul(a343, a344)?
                                    };
                                    nat_mul(a345, a346)?
                                };
                                let a348 = 1u64;
                                nat_add(a347, a348)?
                            };
                            nat_mul(a349, a350)?
                        };
                        let a352 = 1u64;
                        nat_add(a351, a352)?
                    };
                    nat_mul(a353, a354)?
                })?;
                nat_add(a355, a356)
            }
        } else {
            if f7(v0)? {
                Ok(v0)
            } else {
                if f7(v1)? {
                    Ok(v1)
                } else {
                    let a357 = f18(v1)?;
                    let a358 = f18(v0)?;
                    if nat_le(a357, a358) {
                        f19(v0, v1)
                    } else {
                        f19(v1, v0)
                    }
                }
            }
        }
    }
}

pub fn f5(v0: u64) -> R<u64> {
    let a405 = {
        let a383 = v0;
        let a384 = f9({
            let a381 = {
                let a379 = {
                    let a359 = 1u64;
                    let a360 = 1u64;
                    nat_add(a359, a360)?
                };
                let a380 = {
                    let a377 = {
                        let a375 = {
                            let a361 = 1u64;
                            let a362 = 1u64;
                            nat_add(a361, a362)?
                        };
                        let a376 = {
                            let a373 = {
                                let a371 = {
                                    let a363 = 1u64;
                                    let a364 = 1u64;
                                    nat_add(a363, a364)?
                                };
                                let a372 = {
                                    let a369 = {
                                        let a367 = {
                                            let a365 = 1u64;
                                            let a366 = 1u64;
                                            nat_add(a365, a366)?
                                        };
                                        let a368 = 1u64;
                                        nat_mul(a367, a368)?
                                    };
                                    let a370 = 1u64;
                                    nat_add(a369, a370)?
                                };
                                nat_mul(a371, a372)?
                            };
                            let a374 = 1u64;
                            nat_add(a373, a374)?
                        };
                        nat_mul(a375, a376)?
                    };
                    let a378 = 1u64;
                    nat_add(a377, a378)?
                };
                nat_mul(a379, a380)?
            };
            let a382 = 1u64;
            nat_add(a381, a382)?
        })?;
        nat_add(a383, a384)?
    };
    let a406 = f9({
        let a403 = {
            let a385 = 1u64;
            let a386 = 1u64;
            nat_add(a385, a386)?
        };
        let a404 = {
            let a401 = {
                let a387 = 1u64;
                let a388 = 1u64;
                nat_add(a387, a388)?
            };
            let a402 = {
                let a399 = {
                    let a389 = 1u64;
                    let a390 = 1u64;
                    nat_add(a389, a390)?
                };
                let a400 = {
                    let a397 = {
                        let a391 = 1u64;
                        let a392 = 1u64;
                        nat_add(a391, a392)?
                    };
                    let a398 = {
                        let a395 = {
                            let a393 = 1u64;
                            let a394 = 1u64;
                            nat_add(a393, a394)?
                        };
                        let a396 = 1u64;
                        nat_mul(a395, a396)?
                    };
                    nat_mul(a397, a398)?
                };
                nat_mul(a399, a400)?
            };
            nat_mul(a401, a402)?
        };
        nat_mul(a403, a404)?
    })?;
    let a407 = 0u64;
    Ok(nat_rem(a405, a406, a407))
}

pub fn f6(v0: u64) -> R<bool> {
    let a450 = f20(v0)?;
    let a451 = {
        let a448 = {
            let a446 = {
                let a408 = 1u64;
                let a409 = 1u64;
                nat_add(a408, a409)?
            };
            let a447 = {
                let a444 = {
                    let a442 = {
                        let a410 = 1u64;
                        let a411 = 1u64;
                        nat_add(a410, a411)?
                    };
                    let a443 = {
                        let a440 = {
                            let a438 = {
                                let a412 = 1u64;
                                let a413 = 1u64;
                                nat_add(a412, a413)?
                            };
                            let a439 = {
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
                            };
                            nat_mul(a438, a439)?
                        };
                        let a441 = 1u64;
                        nat_add(a440, a441)?
                    };
                    nat_mul(a442, a443)?
                };
                let a445 = 1u64;
                nat_add(a444, a445)?
            };
            nat_mul(a446, a447)?
        };
        let a449 = 1u64;
        nat_add(a448, a449)?
    };
    if nat_eq(a450, a451) {
        let a452 = 0u64;
        let a453 = f21(v0)?;
        Ok(nat_lt(a452, a453))
    } else {
        Ok(false)
    }
}

pub fn f7(v0: u64) -> R<bool> {
    let a496 = f20(v0)?;
    let a497 = {
        let a494 = {
            let a492 = {
                let a454 = 1u64;
                let a455 = 1u64;
                nat_add(a454, a455)?
            };
            let a493 = {
                let a490 = {
                    let a488 = {
                        let a456 = 1u64;
                        let a457 = 1u64;
                        nat_add(a456, a457)?
                    };
                    let a489 = {
                        let a486 = {
                            let a484 = {
                                let a458 = 1u64;
                                let a459 = 1u64;
                                nat_add(a458, a459)?
                            };
                            let a485 = {
                                let a482 = {
                                    let a480 = {
                                        let a460 = 1u64;
                                        let a461 = 1u64;
                                        nat_add(a460, a461)?
                                    };
                                    let a481 = {
                                        let a478 = {
                                            let a476 = {
                                                let a462 = 1u64;
                                                let a463 = 1u64;
                                                nat_add(a462, a463)?
                                            };
                                            let a477 = {
                                                let a474 = {
                                                    let a472 = {
                                                        let a464 = 1u64;
                                                        let a465 = 1u64;
                                                        nat_add(a464, a465)?
                                                    };
                                                    let a473 = {
                                                        let a470 = {
                                                            let a468 = {
                                                                let a466 = 1u64;
                                                                let a467 = 1u64;
                                                                nat_add(a466, a467)?
                                                            };
                                                            let a469 = 1u64;
                                                            nat_mul(a468, a469)?
                                                        };
                                                        let a471 = 1u64;
                                                        nat_add(a470, a471)?
                                                    };
                                                    nat_mul(a472, a473)?
                                                };
                                                let a475 = 1u64;
                                                nat_add(a474, a475)?
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
                    };
                    nat_mul(a488, a489)?
                };
                let a491 = 1u64;
                nat_add(a490, a491)?
            };
            nat_mul(a492, a493)?
        };
        let a495 = 1u64;
        nat_add(a494, a495)?
    };
    if nat_eq(a496, a497) {
        let a498 = f21(v0)?;
        let a499 = 0u64;
        Ok(nat_eq(a498, a499))
    } else {
        Ok(false)
    }
}

pub fn f8(v0: u64) -> R<bool> {
    let a500 = f18(v0)?;
    let a501 = 0u64;
    Ok(nat_eq(a500, a501))
}

pub fn f9(v0: u64) -> R<u64> {
    f22(Fn0::F23, v0, 1u64)
}

pub fn f10(v0: u64) -> R<u64> {
    let a568 = f12(v0)?;
    let a569 = {
        let a566 = {
            let a542 = {
                let a540 = {
                    let a502 = 1u64;
                    let a503 = 1u64;
                    nat_add(a502, a503)?
                };
                let a541 = {
                    let a538 = {
                        let a536 = {
                            let a504 = 1u64;
                            let a505 = 1u64;
                            nat_add(a504, a505)?
                        };
                        let a537 = {
                            let a534 = {
                                let a532 = {
                                    let a506 = 1u64;
                                    let a507 = 1u64;
                                    nat_add(a506, a507)?
                                };
                                let a533 = {
                                    let a530 = {
                                        let a528 = {
                                            let a508 = 1u64;
                                            let a509 = 1u64;
                                            nat_add(a508, a509)?
                                        };
                                        let a529 = {
                                            let a526 = {
                                                let a524 = {
                                                    let a510 = 1u64;
                                                    let a511 = 1u64;
                                                    nat_add(a510, a511)?
                                                };
                                                let a525 = {
                                                    let a522 = {
                                                        let a520 = {
                                                            let a512 = 1u64;
                                                            let a513 = 1u64;
                                                            nat_add(a512, a513)?
                                                        };
                                                        let a521 = {
                                                            let a518 = {
                                                                let a516 = {
                                                                    let a514 = 1u64;
                                                                    let a515 = 1u64;
                                                                    nat_add(a514, a515)?
                                                                };
                                                                let a517 = 1u64;
                                                                nat_mul(a516, a517)?
                                                            };
                                                            let a519 = 1u64;
                                                            nat_add(a518, a519)?
                                                        };
                                                        nat_mul(a520, a521)?
                                                    };
                                                    let a523 = 1u64;
                                                    nat_add(a522, a523)?
                                                };
                                                nat_mul(a524, a525)?
                                            };
                                            let a527 = 1u64;
                                            nat_add(a526, a527)?
                                        };
                                        nat_mul(a528, a529)?
                                    };
                                    let a531 = 1u64;
                                    nat_add(a530, a531)?
                                };
                                nat_mul(a532, a533)?
                            };
                            let a535 = 1u64;
                            nat_add(a534, a535)?
                        };
                        nat_mul(a536, a537)?
                    };
                    let a539 = 1u64;
                    nat_add(a538, a539)?
                };
                nat_mul(a540, a541)?
            };
            let a543 = 1u64;
            nat_add(a542, a543)?
        };
        let a567 = f9({
            let a564 = {
                let a562 = {
                    let a544 = 1u64;
                    let a545 = 1u64;
                    nat_add(a544, a545)?
                };
                let a563 = {
                    let a560 = {
                        let a558 = {
                            let a546 = 1u64;
                            let a547 = 1u64;
                            nat_add(a546, a547)?
                        };
                        let a559 = {
                            let a556 = {
                                let a554 = {
                                    let a548 = 1u64;
                                    let a549 = 1u64;
                                    nat_add(a548, a549)?
                                };
                                let a555 = {
                                    let a552 = {
                                        let a550 = 1u64;
                                        let a551 = 1u64;
                                        nat_add(a550, a551)?
                                    };
                                    let a553 = 1u64;
                                    nat_mul(a552, a553)?
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
        })?;
        nat_mul(a566, a567)?
    };
    nat_add(a568, a569)
}

pub fn f11(v0: u64, v1: u64) -> R<u64> {
    let a576 = {
        let a570 = f17(v0)?;
        let a571 = f17(v1)?;
        nat_add(a570, a571)?
    };
    let a577 = {
        let a574 = {
            let a572 = 1u64;
            let a573 = 1u64;
            nat_add(a572, a573)?
        };
        let a575 = 1u64;
        nat_mul(a574, a575)?
    };
    let a578 = 0u64;
    Ok(nat_rem(a576, a577, a578))
}

pub fn f12(v0: u64) -> R<u64> {
    let a603 = v0;
    let a604 = f9({
        let a601 = {
            let a599 = {
                let a579 = 1u64;
                let a580 = 1u64;
                nat_add(a579, a580)?
            };
            let a600 = {
                let a597 = {
                    let a595 = {
                        let a581 = 1u64;
                        let a582 = 1u64;
                        nat_add(a581, a582)?
                    };
                    let a596 = {
                        let a593 = {
                            let a591 = {
                                let a583 = 1u64;
                                let a584 = 1u64;
                                nat_add(a583, a584)?
                            };
                            let a592 = {
                                let a589 = {
                                    let a587 = {
                                        let a585 = 1u64;
                                        let a586 = 1u64;
                                        nat_add(a585, a586)?
                                    };
                                    let a588 = 1u64;
                                    nat_mul(a587, a588)?
                                };
                                let a590 = 1u64;
                                nat_add(a589, a590)?
                            };
                            nat_mul(a591, a592)?
                        };
                        let a594 = 1u64;
                        nat_add(a593, a594)?
                    };
                    nat_mul(a595, a596)?
                };
                let a598 = 1u64;
                nat_add(a597, a598)?
            };
            nat_mul(a599, a600)?
        };
        let a602 = 1u64;
        nat_add(a601, a602)?
    })?;
    nat_mul(a603, a604)
}

pub fn f13(v0: u64, v1: u64) -> R<u64> {
    let v2: Adt1 = f24(v0)?;
    let v3: Adt1 = f24(v1)?;
    let v4: bool = {
        let a607 = match v2.clone() {
            Adt1::C0(_, h605, _, _) => {
                h605
            }
        };
        let a608 = match v3.clone() {
            Adt1::C0(_, h606, _, _) => {
                h606
            }
        };
        nat_lt(a607, a608)
    };
    let v5: u64 = {
        let a652 = match v2.clone() {
            Adt1::C0(_, h609, _, _) => {
                h609
            }
        };
        let a653 = f9(if v4 {
            let a630 = {
                let a628 = {
                    let a610 = 1u64;
                    let a611 = 1u64;
                    nat_add(a610, a611)?
                };
                let a629 = {
                    let a626 = {
                        let a624 = {
                            let a612 = 1u64;
                            let a613 = 1u64;
                            nat_add(a612, a613)?
                        };
                        let a625 = {
                            let a622 = {
                                let a614 = 1u64;
                                let a615 = 1u64;
                                nat_add(a614, a615)?
                            };
                            let a623 = {
                                let a620 = {
                                    let a618 = {
                                        let a616 = 1u64;
                                        let a617 = 1u64;
                                        nat_add(a616, a617)?
                                    };
                                    let a619 = 1u64;
                                    nat_mul(a618, a619)?
                                };
                                let a621 = 1u64;
                                nat_add(a620, a621)?
                            };
                            nat_mul(a622, a623)?
                        };
                        nat_mul(a624, a625)?
                    };
                    let a627 = 1u64;
                    nat_add(a626, a627)?
                };
                nat_mul(a628, a629)?
            };
            let a631 = 1u64;
            nat_add(a630, a631)?
        } else {
            let a650 = {
                let a632 = 1u64;
                let a633 = 1u64;
                nat_add(a632, a633)?
            };
            let a651 = {
                let a648 = {
                    let a646 = {
                        let a634 = 1u64;
                        let a635 = 1u64;
                        nat_add(a634, a635)?
                    };
                    let a647 = {
                        let a644 = {
                            let a636 = 1u64;
                            let a637 = 1u64;
                            nat_add(a636, a637)?
                        };
                        let a645 = {
                            let a642 = {
                                let a640 = {
                                    let a638 = 1u64;
                                    let a639 = 1u64;
                                    nat_add(a638, a639)?
                                };
                                let a641 = 1u64;
                                nat_mul(a640, a641)?
                            };
                            let a643 = 1u64;
                            nat_add(a642, a643)?
                        };
                        nat_mul(a644, a645)?
                    };
                    nat_mul(a646, a647)?
                };
                let a649 = 1u64;
                nat_add(a648, a649)?
            };
            nat_mul(a650, a651)?
        })?;
        nat_mul(a652, a653)?
    };
    let v6: u64 = {
        let a710 = {
            let a708 = {
                let a705 = match v2 {
                    Adt1::C0(h654, _, _, _) => {
                        h654
                    }
                };
                let a706 = {
                    let a703 = {
                        let a701 = {
                            let a655 = 1u64;
                            let a656 = 1u64;
                            nat_add(a655, a656)?
                        };
                        let a702 = {
                            let a699 = {
                                let a697 = {
                                    let a657 = 1u64;
                                    let a658 = 1u64;
                                    nat_add(a657, a658)?
                                };
                                let a698 = {
                                    let a695 = {
                                        let a693 = {
                                            let a659 = 1u64;
                                            let a660 = 1u64;
                                            nat_add(a659, a660)?
                                        };
                                        let a694 = {
                                            let a691 = {
                                                let a689 = {
                                                    let a661 = 1u64;
                                                    let a662 = 1u64;
                                                    nat_add(a661, a662)?
                                                };
                                                let a690 = {
                                                    let a687 = {
                                                        let a685 = {
                                                            let a663 = 1u64;
                                                            let a664 = 1u64;
                                                            nat_add(a663, a664)?
                                                        };
                                                        let a686 = {
                                                            let a683 = {
                                                                let a681 = {
                                                                    let a665 = 1u64;
                                                                    let a666 = 1u64;
                                                                    nat_add(a665, a666)?
                                                                };
                                                                let a682 = {
                                                                    let a679 = {
                                                                        let a677 = {
                                                                            let a667 = 1u64;
                                                                            let a668 = 1u64;
                                                                            nat_add(a667, a668)?
                                                                        };
                                                                        let a678 = {
                                                                            let a675 = {
                                                                                let a669 = 1u64;
                                                                                let a670 = 1u64;
                                                                                nat_add(a669, a670)?
                                                                            };
                                                                            let a676 = {
                                                                                let a673 = {
                                                                                    let a671 = 1u64;
                                                                                    let a672 = 1u64;
                                                                                    nat_add(a671, a672)?
                                                                                };
                                                                                let a674 = 1u64;
                                                                                nat_mul(a673, a674)?
                                                                            };
                                                                            nat_mul(a675, a676)?
                                                                        };
                                                                        nat_mul(a677, a678)?
                                                                    };
                                                                    let a680 = 1u64;
                                                                    nat_add(a679, a680)?
                                                                };
                                                                nat_mul(a681, a682)?
                                                            };
                                                            let a684 = 1u64;
                                                            nat_add(a683, a684)?
                                                        };
                                                        nat_mul(a685, a686)?
                                                    };
                                                    let a688 = 1u64;
                                                    nat_add(a687, a688)?
                                                };
                                                nat_mul(a689, a690)?
                                            };
                                            let a692 = 1u64;
                                            nat_add(a691, a692)?
                                        };
                                        nat_mul(a693, a694)?
                                    };
                                    let a696 = 1u64;
                                    nat_add(a695, a696)?
                                };
                                nat_mul(a697, a698)?
                            };
                            let a700 = 1u64;
                            nat_add(a699, a700)?
                        };
                        nat_mul(a701, a702)?
                    };
                    let a704 = 1u64;
                    nat_add(a703, a704)?
                };
                nat_add(a705, a706)?
            };
            let a709 = match v3.clone() {
                Adt1::C0(h707, _, _, _) => {
                    h707
                }
            };
            nat_sub(a708, a709)
        };
        let a711 = if v4 {
            1u64
        } else {
            0u64
        };
        nat_sub(a710, a711)
    };
    let v7: u64 = {
        let a713 = v5;
        let a714 = match v3.clone() {
            Adt1::C0(_, h712, _, _) => {
                h712
            }
        };
        let a715 = 0u64;
        nat_quot(a713, a714, a715)
    };
    let v8: bool = {
        let a720 = 0u64;
        let a721 = {
            let a717 = v5;
            let a718 = match v3 {
                Adt1::C0(_, h716, _, _) => {
                    h716
                }
            };
            let a719 = 0u64;
            nat_rem(a717, a718, a719)
        };
        nat_lt(a720, a721)
    };
    f25(f11(v0, v1)?, v6, f26(v7, v8)?)
}

pub fn f14(v0: u64) -> R<Option<u64>> {
    if if f6(v0)? {
        true
    } else {
        let a722 = f17(v0)?;
        let a723 = 1u64;
        if nat_eq(a722, a723) {
            let a724 = 0u64;
            let a725 = f18(v0)?;
            nat_lt(a724, a725)
        } else {
            false
        }
    } {
        Ok(Some({
            let a812 = {
                let a790 = {
                    let a766 = {
                        let a764 = {
                            let a726 = 1u64;
                            let a727 = 1u64;
                            nat_add(a726, a727)?
                        };
                        let a765 = {
                            let a762 = {
                                let a760 = {
                                    let a728 = 1u64;
                                    let a729 = 1u64;
                                    nat_add(a728, a729)?
                                };
                                let a761 = {
                                    let a758 = {
                                        let a756 = {
                                            let a730 = 1u64;
                                            let a731 = 1u64;
                                            nat_add(a730, a731)?
                                        };
                                        let a757 = {
                                            let a754 = {
                                                let a752 = {
                                                    let a732 = 1u64;
                                                    let a733 = 1u64;
                                                    nat_add(a732, a733)?
                                                };
                                                let a753 = {
                                                    let a750 = {
                                                        let a748 = {
                                                            let a734 = 1u64;
                                                            let a735 = 1u64;
                                                            nat_add(a734, a735)?
                                                        };
                                                        let a749 = {
                                                            let a746 = {
                                                                let a744 = {
                                                                    let a736 = 1u64;
                                                                    let a737 = 1u64;
                                                                    nat_add(a736, a737)?
                                                                };
                                                                let a745 = {
                                                                    let a742 = {
                                                                        let a740 = {
                                                                            let a738 = 1u64;
                                                                            let a739 = 1u64;
                                                                            nat_add(a738, a739)?
                                                                        };
                                                                        let a741 = 1u64;
                                                                        nat_mul(a740, a741)?
                                                                    };
                                                                    let a743 = 1u64;
                                                                    nat_add(a742, a743)?
                                                                };
                                                                nat_mul(a744, a745)?
                                                            };
                                                            let a747 = 1u64;
                                                            nat_add(a746, a747)?
                                                        };
                                                        nat_mul(a748, a749)?
                                                    };
                                                    let a751 = 1u64;
                                                    nat_add(a750, a751)?
                                                };
                                                nat_mul(a752, a753)?
                                            };
                                            let a755 = 1u64;
                                            nat_add(a754, a755)?
                                        };
                                        nat_mul(a756, a757)?
                                    };
                                    let a759 = 1u64;
                                    nat_add(a758, a759)?
                                };
                                nat_mul(a760, a761)?
                            };
                            let a763 = 1u64;
                            nat_add(a762, a763)?
                        };
                        nat_mul(a764, a765)?
                    };
                    let a767 = 1u64;
                    nat_add(a766, a767)?
                };
                let a791 = f9({
                    let a788 = {
                        let a786 = {
                            let a768 = 1u64;
                            let a769 = 1u64;
                            nat_add(a768, a769)?
                        };
                        let a787 = {
                            let a784 = {
                                let a782 = {
                                    let a770 = 1u64;
                                    let a771 = 1u64;
                                    nat_add(a770, a771)?
                                };
                                let a783 = {
                                    let a780 = {
                                        let a778 = {
                                            let a772 = 1u64;
                                            let a773 = 1u64;
                                            nat_add(a772, a773)?
                                        };
                                        let a779 = {
                                            let a776 = {
                                                let a774 = 1u64;
                                                let a775 = 1u64;
                                                nat_add(a774, a775)?
                                            };
                                            let a777 = 1u64;
                                            nat_mul(a776, a777)?
                                        };
                                        nat_mul(a778, a779)?
                                    };
                                    let a781 = 1u64;
                                    nat_add(a780, a781)?
                                };
                                nat_mul(a782, a783)?
                            };
                            let a785 = 1u64;
                            nat_add(a784, a785)?
                        };
                        nat_mul(a786, a787)?
                    };
                    let a789 = 1u64;
                    nat_add(a788, a789)?
                })?;
                nat_mul(a790, a791)?
            };
            let a813 = f9({
                let a810 = {
                    let a792 = 1u64;
                    let a793 = 1u64;
                    nat_add(a792, a793)?
                };
                let a811 = {
                    let a808 = {
                        let a806 = {
                            let a794 = 1u64;
                            let a795 = 1u64;
                            nat_add(a794, a795)?
                        };
                        let a807 = {
                            let a804 = {
                                let a802 = {
                                    let a796 = 1u64;
                                    let a797 = 1u64;
                                    nat_add(a796, a797)?
                                };
                                let a803 = {
                                    let a800 = {
                                        let a798 = 1u64;
                                        let a799 = 1u64;
                                        nat_add(a798, a799)?
                                    };
                                    let a801 = 1u64;
                                    nat_mul(a800, a801)?
                                };
                                nat_mul(a802, a803)?
                            };
                            let a805 = 1u64;
                            nat_add(a804, a805)?
                        };
                        nat_mul(a806, a807)?
                    };
                    let a809 = 1u64;
                    nat_add(a808, a809)?
                };
                nat_mul(a810, a811)?
            })?;
            nat_add(a812, a813)?
        }))
    } else {
        if if f8(v0)? {
            true
        } else {
            f7(v0)?
        } {
            Ok(Some(v0))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f15(v0: u64) -> R<u64> {
    let v1: Adt1 = f24(v0)?;
    let v2: u64 = {
        let a822 = {
            let a819 = match v1.clone() {
                Adt1::C0(h814, _, _, _) => {
                    h814
                }
            };
            let a820 = {
                let a817 = {
                    let a815 = 1u64;
                    let a816 = 1u64;
                    nat_add(a815, a816)?
                };
                let a818 = 1u64;
                nat_mul(a817, a818)?
            };
            let a821 = 0u64;
            nat_rem(a819, a820, a821)
        };
        let a823 = 0u64;
        if nat_eq(a822, a823) {
            let a829 = match v1.clone() {
                Adt1::C0(_, h824, _, _) => {
                    h824
                }
            };
            let a830 = {
                let a827 = {
                    let a825 = 1u64;
                    let a826 = 1u64;
                    nat_add(a825, a826)?
                };
                let a828 = 1u64;
                nat_mul(a827, a828)?
            };
            nat_mul(a829, a830)?
        } else {
            match v1.clone() {
                Adt1::C0(_, h831, _, _) => {
                    h831
                }
            }
        }
    };
    let v3: u64 = {
        let a854 = v2;
        let a855 = f9({
            let a852 = {
                let a850 = {
                    let a832 = 1u64;
                    let a833 = 1u64;
                    nat_add(a832, a833)?
                };
                let a851 = {
                    let a848 = {
                        let a834 = 1u64;
                        let a835 = 1u64;
                        nat_add(a834, a835)?
                    };
                    let a849 = {
                        let a846 = {
                            let a844 = {
                                let a836 = 1u64;
                                let a837 = 1u64;
                                nat_add(a836, a837)?
                            };
                            let a845 = {
                                let a842 = {
                                    let a840 = {
                                        let a838 = 1u64;
                                        let a839 = 1u64;
                                        nat_add(a838, a839)?
                                    };
                                    let a841 = 1u64;
                                    nat_mul(a840, a841)?
                                };
                                let a843 = 1u64;
                                nat_add(a842, a843)?
                            };
                            nat_mul(a844, a845)?
                        };
                        let a847 = 1u64;
                        nat_add(a846, a847)?
                    };
                    nat_mul(a848, a849)?
                };
                nat_mul(a850, a851)?
            };
            let a853 = 1u64;
            nat_add(a852, a853)?
        })?;
        nat_mul(a854, a855)?
    };
    let v4: Adt2 = {
        let (h856, _) = f27(Fn1::F28, {
            let a875 = {
                let a857 = 1u64;
                let a858 = 1u64;
                nat_add(a857, a858)?
            };
            let a876 = {
                let a873 = {
                    let a859 = 1u64;
                    let a860 = 1u64;
                    nat_add(a859, a860)?
                };
                let a874 = {
                    let a871 = {
                        let a861 = 1u64;
                        let a862 = 1u64;
                        nat_add(a861, a862)?
                    };
                    let a872 = {
                        let a869 = {
                            let a863 = 1u64;
                            let a864 = 1u64;
                            nat_add(a863, a864)?
                        };
                        let a870 = {
                            let a867 = {
                                let a865 = 1u64;
                                let a866 = 1u64;
                                nat_add(a865, a866)?
                            };
                            let a868 = 1u64;
                            nat_mul(a867, a868)?
                        };
                        nat_mul(a869, a870)?
                    };
                    nat_mul(a871, a872)?
                };
                nat_mul(a873, a874)?
            };
            nat_mul(a875, a876)?
        }, Adt2::C0(v3, f9({
            let a897 = {
                let a895 = {
                    let a877 = 1u64;
                    let a878 = 1u64;
                    nat_add(a877, a878)?
                };
                let a896 = {
                    let a893 = {
                        let a891 = {
                            let a879 = 1u64;
                            let a880 = 1u64;
                            nat_add(a879, a880)?
                        };
                        let a892 = {
                            let a889 = {
                                let a881 = 1u64;
                                let a882 = 1u64;
                                nat_add(a881, a882)?
                            };
                            let a890 = {
                                let a887 = {
                                    let a885 = {
                                        let a883 = 1u64;
                                        let a884 = 1u64;
                                        nat_add(a883, a884)?
                                    };
                                    let a886 = 1u64;
                                    nat_mul(a885, a886)?
                                };
                                let a888 = 1u64;
                                nat_add(a887, a888)?
                            };
                            nat_mul(a889, a890)?
                        };
                        nat_mul(a891, a892)?
                    };
                    let a894 = 1u64;
                    nat_add(a893, a894)?
                };
                nat_mul(a895, a896)?
            };
            let a898 = 1u64;
            nat_add(a897, a898)?
        })?))?;
        h856
    };
    let v5: bool = {
        let a903 = {
            let a901 = match v4.clone() {
                Adt2::C0(_, h899) => {
                    h899
                }
            };
            let a902 = match v4.clone() {
                Adt2::C0(_, h900) => {
                    h900
                }
            };
            nat_mul(a901, a902)?
        };
        let a904 = v3;
        nat_lt(a903, a904)
    };
    f29(0u64, {
        let a962 = {
            let a956 = match v1 {
                Adt1::C0(h905, _, _, _) => {
                    h905
                }
            };
            let a957 = {
                let a954 = {
                    let a952 = {
                        let a906 = 1u64;
                        let a907 = 1u64;
                        nat_add(a906, a907)?
                    };
                    let a953 = {
                        let a950 = {
                            let a948 = {
                                let a908 = 1u64;
                                let a909 = 1u64;
                                nat_add(a908, a909)?
                            };
                            let a949 = {
                                let a946 = {
                                    let a944 = {
                                        let a910 = 1u64;
                                        let a911 = 1u64;
                                        nat_add(a910, a911)?
                                    };
                                    let a945 = {
                                        let a942 = {
                                            let a940 = {
                                                let a912 = 1u64;
                                                let a913 = 1u64;
                                                nat_add(a912, a913)?
                                            };
                                            let a941 = {
                                                let a938 = {
                                                    let a936 = {
                                                        let a914 = 1u64;
                                                        let a915 = 1u64;
                                                        nat_add(a914, a915)?
                                                    };
                                                    let a937 = {
                                                        let a934 = {
                                                            let a932 = {
                                                                let a916 = 1u64;
                                                                let a917 = 1u64;
                                                                nat_add(a916, a917)?
                                                            };
                                                            let a933 = {
                                                                let a930 = {
                                                                    let a928 = {
                                                                        let a918 = 1u64;
                                                                        let a919 = 1u64;
                                                                        nat_add(a918, a919)?
                                                                    };
                                                                    let a929 = {
                                                                        let a926 = {
                                                                            let a920 = 1u64;
                                                                            let a921 = 1u64;
                                                                            nat_add(a920, a921)?
                                                                        };
                                                                        let a927 = {
                                                                            let a924 = {
                                                                                let a922 = 1u64;
                                                                                let a923 = 1u64;
                                                                                nat_add(a922, a923)?
                                                                            };
                                                                            let a925 = 1u64;
                                                                            nat_mul(a924, a925)?
                                                                        };
                                                                        nat_mul(a926, a927)?
                                                                    };
                                                                    nat_mul(a928, a929)?
                                                                };
                                                                let a931 = 1u64;
                                                                nat_add(a930, a931)?
                                                            };
                                                            nat_mul(a932, a933)?
                                                        };
                                                        let a935 = 1u64;
                                                        nat_add(a934, a935)?
                                                    };
                                                    nat_mul(a936, a937)?
                                                };
                                                let a939 = 1u64;
                                                nat_add(a938, a939)?
                                            };
                                            nat_mul(a940, a941)?
                                        };
                                        let a943 = 1u64;
                                        nat_add(a942, a943)?
                                    };
                                    nat_mul(a944, a945)?
                                };
                                let a947 = 1u64;
                                nat_add(a946, a947)?
                            };
                            nat_mul(a948, a949)?
                        };
                        let a951 = 1u64;
                        nat_add(a950, a951)?
                    };
                    nat_mul(a952, a953)?
                };
                let a955 = 1u64;
                nat_add(a954, a955)?
            };
            nat_add(a956, a957)?
        };
        let a963 = {
            let a960 = {
                let a958 = 1u64;
                let a959 = 1u64;
                nat_add(a958, a959)?
            };
            let a961 = 1u64;
            nat_mul(a960, a961)?
        };
        let a964 = 0u64;
        nat_quot(a962, a963, a964)
    }, f26(match v4 {
        Adt2::C0(_, h965) => {
            h965
        }
    }, v5)?)
}

pub fn f16(v0: u64, v1: u64) -> R<u64> {
    let v2: Adt1 = f24(v0)?;
    let v3: Adt1 = f24(v1)?;
    f25(f11(v0, v1)?, {
        let a1020 = {
            let a968 = match v2.clone() {
                Adt1::C0(h966, _, _, _) => {
                    h966
                }
            };
            let a969 = match v3.clone() {
                Adt1::C0(h967, _, _, _) => {
                    h967
                }
            };
            nat_add(a968, a969)?
        };
        let a1021 = {
            let a1018 = {
                let a1016 = {
                    let a970 = 1u64;
                    let a971 = 1u64;
                    nat_add(a970, a971)?
                };
                let a1017 = {
                    let a1014 = {
                        let a1012 = {
                            let a972 = 1u64;
                            let a973 = 1u64;
                            nat_add(a972, a973)?
                        };
                        let a1013 = {
                            let a1010 = {
                                let a1008 = {
                                    let a974 = 1u64;
                                    let a975 = 1u64;
                                    nat_add(a974, a975)?
                                };
                                let a1009 = {
                                    let a1006 = {
                                        let a1004 = {
                                            let a976 = 1u64;
                                            let a977 = 1u64;
                                            nat_add(a976, a977)?
                                        };
                                        let a1005 = {
                                            let a1002 = {
                                                let a1000 = {
                                                    let a978 = 1u64;
                                                    let a979 = 1u64;
                                                    nat_add(a978, a979)?
                                                };
                                                let a1001 = {
                                                    let a998 = {
                                                        let a996 = {
                                                            let a980 = 1u64;
                                                            let a981 = 1u64;
                                                            nat_add(a980, a981)?
                                                        };
                                                        let a997 = {
                                                            let a994 = {
                                                                let a992 = {
                                                                    let a982 = 1u64;
                                                                    let a983 = 1u64;
                                                                    nat_add(a982, a983)?
                                                                };
                                                                let a993 = {
                                                                    let a990 = {
                                                                        let a984 = 1u64;
                                                                        let a985 = 1u64;
                                                                        nat_add(a984, a985)?
                                                                    };
                                                                    let a991 = {
                                                                        let a988 = {
                                                                            let a986 = 1u64;
                                                                            let a987 = 1u64;
                                                                            nat_add(a986, a987)?
                                                                        };
                                                                        let a989 = 1u64;
                                                                        nat_mul(a988, a989)?
                                                                    };
                                                                    nat_mul(a990, a991)?
                                                                };
                                                                nat_mul(a992, a993)?
                                                            };
                                                            let a995 = 1u64;
                                                            nat_add(a994, a995)?
                                                        };
                                                        nat_mul(a996, a997)?
                                                    };
                                                    let a999 = 1u64;
                                                    nat_add(a998, a999)?
                                                };
                                                nat_mul(a1000, a1001)?
                                            };
                                            let a1003 = 1u64;
                                            nat_add(a1002, a1003)?
                                        };
                                        nat_mul(a1004, a1005)?
                                    };
                                    let a1007 = 1u64;
                                    nat_add(a1006, a1007)?
                                };
                                nat_mul(a1008, a1009)?
                            };
                            let a1011 = 1u64;
                            nat_add(a1010, a1011)?
                        };
                        nat_mul(a1012, a1013)?
                    };
                    let a1015 = 1u64;
                    nat_add(a1014, a1015)?
                };
                nat_mul(a1016, a1017)?
            };
            let a1019 = 1u64;
            nat_add(a1018, a1019)?
        };
        nat_sub(a1020, a1021)
    }, f30({
        let a1024 = match v2 {
            Adt1::C0(_, h1022, _, _) => {
                h1022
            }
        };
        let a1025 = match v3 {
            Adt1::C0(_, h1023, _, _) => {
                h1023
            }
        };
        nat_mul(a1024, a1025)?
    }, {
        let a1042 = {
            let a1026 = 1u64;
            let a1027 = 1u64;
            nat_add(a1026, a1027)?
        };
        let a1043 = {
            let a1040 = {
                let a1028 = 1u64;
                let a1029 = 1u64;
                nat_add(a1028, a1029)?
            };
            let a1041 = {
                let a1038 = {
                    let a1036 = {
                        let a1030 = 1u64;
                        let a1031 = 1u64;
                        nat_add(a1030, a1031)?
                    };
                    let a1037 = {
                        let a1034 = {
                            let a1032 = 1u64;
                            let a1033 = 1u64;
                            nat_add(a1032, a1033)?
                        };
                        let a1035 = 1u64;
                        nat_mul(a1034, a1035)?
                    };
                    nat_mul(a1036, a1037)?
                };
                let a1039 = 1u64;
                nat_add(a1038, a1039)?
            };
            nat_mul(a1040, a1041)?
        };
        nat_mul(a1042, a1043)?
    })?)
}

pub fn f17(v0: u64) -> R<u64> {
    let a1068 = v0;
    let a1069 = f9({
        let a1066 = {
            let a1064 = {
                let a1044 = 1u64;
                let a1045 = 1u64;
                nat_add(a1044, a1045)?
            };
            let a1065 = {
                let a1062 = {
                    let a1060 = {
                        let a1046 = 1u64;
                        let a1047 = 1u64;
                        nat_add(a1046, a1047)?
                    };
                    let a1061 = {
                        let a1058 = {
                            let a1056 = {
                                let a1048 = 1u64;
                                let a1049 = 1u64;
                                nat_add(a1048, a1049)?
                            };
                            let a1057 = {
                                let a1054 = {
                                    let a1052 = {
                                        let a1050 = 1u64;
                                        let a1051 = 1u64;
                                        nat_add(a1050, a1051)?
                                    };
                                    let a1053 = 1u64;
                                    nat_mul(a1052, a1053)?
                                };
                                let a1055 = 1u64;
                                nat_add(a1054, a1055)?
                            };
                            nat_mul(a1056, a1057)?
                        };
                        let a1059 = 1u64;
                        nat_add(a1058, a1059)?
                    };
                    nat_mul(a1060, a1061)?
                };
                let a1063 = 1u64;
                nat_add(a1062, a1063)?
            };
            nat_mul(a1064, a1065)?
        };
        let a1067 = 1u64;
        nat_add(a1066, a1067)?
    })?;
    let a1070 = 0u64;
    Ok(nat_quot(a1068, a1069, a1070))
}

pub fn f18(v0: u64) -> R<u64> {
    let a1095 = v0;
    let a1096 = f9({
        let a1093 = {
            let a1091 = {
                let a1071 = 1u64;
                let a1072 = 1u64;
                nat_add(a1071, a1072)?
            };
            let a1092 = {
                let a1089 = {
                    let a1087 = {
                        let a1073 = 1u64;
                        let a1074 = 1u64;
                        nat_add(a1073, a1074)?
                    };
                    let a1088 = {
                        let a1085 = {
                            let a1083 = {
                                let a1075 = 1u64;
                                let a1076 = 1u64;
                                nat_add(a1075, a1076)?
                            };
                            let a1084 = {
                                let a1081 = {
                                    let a1079 = {
                                        let a1077 = 1u64;
                                        let a1078 = 1u64;
                                        nat_add(a1077, a1078)?
                                    };
                                    let a1080 = 1u64;
                                    nat_mul(a1079, a1080)?
                                };
                                let a1082 = 1u64;
                                nat_add(a1081, a1082)?
                            };
                            nat_mul(a1083, a1084)?
                        };
                        let a1086 = 1u64;
                        nat_add(a1085, a1086)?
                    };
                    nat_mul(a1087, a1088)?
                };
                let a1090 = 1u64;
                nat_add(a1089, a1090)?
            };
            nat_mul(a1091, a1092)?
        };
        let a1094 = 1u64;
        nat_add(a1093, a1094)?
    })?;
    let a1097 = 0u64;
    Ok(nat_rem(a1095, a1096, a1097))
}

pub fn f19(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f31(v0)?;
    let v3: u64 = {
        let a1110 = f32(v0)?;
        let a1111 = {
            let a1108 = {
                let a1098 = 1u64;
                let a1099 = 1u64;
                nat_add(a1098, a1099)?
            };
            let a1109 = {
                let a1106 = {
                    let a1100 = 1u64;
                    let a1101 = 1u64;
                    nat_add(a1100, a1101)?
                };
                let a1107 = {
                    let a1104 = {
                        let a1102 = 1u64;
                        let a1103 = 1u64;
                        nat_add(a1102, a1103)?
                    };
                    let a1105 = 1u64;
                    nat_mul(a1104, a1105)?
                };
                nat_mul(a1106, a1107)?
            };
            nat_mul(a1108, a1109)?
        };
        nat_mul(a1110, a1111)?
    };
    let v4: u64 = f30({
        let a1124 = f32(v1)?;
        let a1125 = {
            let a1122 = {
                let a1112 = 1u64;
                let a1113 = 1u64;
                nat_add(a1112, a1113)?
            };
            let a1123 = {
                let a1120 = {
                    let a1114 = 1u64;
                    let a1115 = 1u64;
                    nat_add(a1114, a1115)?
                };
                let a1121 = {
                    let a1118 = {
                        let a1116 = 1u64;
                        let a1117 = 1u64;
                        nat_add(a1116, a1117)?
                    };
                    let a1119 = 1u64;
                    nat_mul(a1118, a1119)?
                };
                nat_mul(a1120, a1121)?
            };
            nat_mul(a1122, a1123)?
        };
        nat_mul(a1124, a1125)?
    }, {
        let a1126 = v2;
        let a1127 = f31(v1)?;
        nat_sub(a1126, a1127)
    })?;
    let a1128 = f17(v0)?;
    let a1129 = f17(v1)?;
    if nat_eq(a1128, a1129) {
        f25(f17(v0)?, v2, {
            let a1130 = v3;
            let a1131 = v4;
            nat_add(a1130, a1131)?
        })
    } else {
        let a1132 = v3;
        let a1133 = v4;
        if nat_eq(a1132, a1133) {
            Ok(0u64)
        } else {
            f25(f17(v0)?, v2, {
                let a1134 = v3;
                let a1135 = v4;
                nat_sub(a1134, a1135)
            })
        }
    }
}

pub fn f20(v0: u64) -> R<u64> {
    let a1158 = f18(v0)?;
    let a1159 = f9({
        let a1156 = {
            let a1154 = {
                let a1136 = 1u64;
                let a1137 = 1u64;
                nat_add(a1136, a1137)?
            };
            let a1155 = {
                let a1152 = {
                    let a1150 = {
                        let a1138 = 1u64;
                        let a1139 = 1u64;
                        nat_add(a1138, a1139)?
                    };
                    let a1151 = {
                        let a1148 = {
                            let a1146 = {
                                let a1140 = 1u64;
                                let a1141 = 1u64;
                                nat_add(a1140, a1141)?
                            };
                            let a1147 = {
                                let a1144 = {
                                    let a1142 = 1u64;
                                    let a1143 = 1u64;
                                    nat_add(a1142, a1143)?
                                };
                                let a1145 = 1u64;
                                nat_mul(a1144, a1145)?
                            };
                            nat_mul(a1146, a1147)?
                        };
                        let a1149 = 1u64;
                        nat_add(a1148, a1149)?
                    };
                    nat_mul(a1150, a1151)?
                };
                let a1153 = 1u64;
                nat_add(a1152, a1153)?
            };
            nat_mul(a1154, a1155)?
        };
        let a1157 = 1u64;
        nat_add(a1156, a1157)?
    })?;
    let a1160 = 0u64;
    Ok(nat_quot(a1158, a1159, a1160))
}

pub fn f21(v0: u64) -> R<u64> {
    let a1183 = v0;
    let a1184 = f9({
        let a1181 = {
            let a1179 = {
                let a1161 = 1u64;
                let a1162 = 1u64;
                nat_add(a1161, a1162)?
            };
            let a1180 = {
                let a1177 = {
                    let a1175 = {
                        let a1163 = 1u64;
                        let a1164 = 1u64;
                        nat_add(a1163, a1164)?
                    };
                    let a1176 = {
                        let a1173 = {
                            let a1171 = {
                                let a1165 = 1u64;
                                let a1166 = 1u64;
                                nat_add(a1165, a1166)?
                            };
                            let a1172 = {
                                let a1169 = {
                                    let a1167 = 1u64;
                                    let a1168 = 1u64;
                                    nat_add(a1167, a1168)?
                                };
                                let a1170 = 1u64;
                                nat_mul(a1169, a1170)?
                            };
                            nat_mul(a1171, a1172)?
                        };
                        let a1174 = 1u64;
                        nat_add(a1173, a1174)?
                    };
                    nat_mul(a1175, a1176)?
                };
                let a1178 = 1u64;
                nat_add(a1177, a1178)?
            };
            nat_mul(a1179, a1180)?
        };
        let a1182 = 1u64;
        nat_add(a1181, a1182)?
    })?;
    let a1185 = 0u64;
    Ok(nat_rem(a1183, a1184, a1185))
}

pub fn f22(v0: Fn0, p1189: u64, p1190: u64) -> R<u64> {
    let m1188 = (p1189, p1190);
    {
        let mut m1188 = m1188;
        loop {
            let step = {
                let (v1, v2) = m1188;
                let m1186 = v1;
                if m1186 == 0 {
                    Err::<(u64, u64), R<u64>>(Ok(v2))
                } else {
                    let v3 = m1186 - 1;
                    Ok::<(u64, u64), R<u64>>((v3, {
                        let c1187: Fn0 = v0.clone();
                        c1187.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1188 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f23(v0: u64) -> R<u64> {
    let a1191 = v0;
    let a1192 = v0;
    nat_add(a1191, a1192)
}

pub fn f24(v0: u64) -> R<Adt1> {
    f33(f31(v0)?, f32(v0)?, 0u64, f9({
        let a1213 = {
            let a1211 = {
                let a1193 = 1u64;
                let a1194 = 1u64;
                nat_add(a1193, a1194)?
            };
            let a1212 = {
                let a1209 = {
                    let a1207 = {
                        let a1195 = 1u64;
                        let a1196 = 1u64;
                        nat_add(a1195, a1196)?
                    };
                    let a1208 = {
                        let a1205 = {
                            let a1203 = {
                                let a1197 = 1u64;
                                let a1198 = 1u64;
                                nat_add(a1197, a1198)?
                            };
                            let a1204 = {
                                let a1201 = {
                                    let a1199 = 1u64;
                                    let a1200 = 1u64;
                                    nat_add(a1199, a1200)?
                                };
                                let a1202 = 1u64;
                                nat_mul(a1201, a1202)?
                            };
                            nat_mul(a1203, a1204)?
                        };
                        let a1206 = 1u64;
                        nat_add(a1205, a1206)?
                    };
                    nat_mul(a1207, a1208)?
                };
                let a1210 = 1u64;
                nat_add(a1209, a1210)?
            };
            nat_mul(a1211, a1212)?
        };
        let a1214 = 1u64;
        nat_add(a1213, a1214)?
    })?)
}

pub fn f25(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1237 = f9({
        let a1235 = {
            let a1233 = {
                let a1215 = 1u64;
                let a1216 = 1u64;
                nat_add(a1215, a1216)?
            };
            let a1234 = {
                let a1231 = {
                    let a1229 = {
                        let a1217 = 1u64;
                        let a1218 = 1u64;
                        nat_add(a1217, a1218)?
                    };
                    let a1230 = {
                        let a1227 = {
                            let a1219 = 1u64;
                            let a1220 = 1u64;
                            nat_add(a1219, a1220)?
                        };
                        let a1228 = {
                            let a1225 = {
                                let a1223 = {
                                    let a1221 = 1u64;
                                    let a1222 = 1u64;
                                    nat_add(a1221, a1222)?
                                };
                                let a1224 = 1u64;
                                nat_mul(a1223, a1224)?
                            };
                            let a1226 = 1u64;
                            nat_add(a1225, a1226)?
                        };
                        nat_mul(a1227, a1228)?
                    };
                    nat_mul(a1229, a1230)?
                };
                let a1232 = 1u64;
                nat_add(a1231, a1232)?
            };
            nat_mul(a1233, a1234)?
        };
        let a1236 = 1u64;
        nat_add(a1235, a1236)?
    })?;
    let a1238 = v2;
    if nat_le(a1237, a1238) {
        f29(v0, {
            let a1239 = v1;
            let a1240 = 1u64;
            nat_add(a1239, a1240)?
        }, f30(v2, 1u64)?)
    } else {
        let v3: Adt1 = f33(v1, v2, {
            let a1277 = {
                let a1275 = {
                    let a1241 = 1u64;
                    let a1242 = 1u64;
                    nat_add(a1241, a1242)?
                };
                let a1276 = {
                    let a1273 = {
                        let a1243 = 1u64;
                        let a1244 = 1u64;
                        nat_add(a1243, a1244)?
                    };
                    let a1274 = {
                        let a1271 = {
                            let a1245 = 1u64;
                            let a1246 = 1u64;
                            nat_add(a1245, a1246)?
                        };
                        let a1272 = {
                            let a1269 = {
                                let a1247 = 1u64;
                                let a1248 = 1u64;
                                nat_add(a1247, a1248)?
                            };
                            let a1270 = {
                                let a1267 = {
                                    let a1249 = 1u64;
                                    let a1250 = 1u64;
                                    nat_add(a1249, a1250)?
                                };
                                let a1268 = {
                                    let a1265 = {
                                        let a1251 = 1u64;
                                        let a1252 = 1u64;
                                        nat_add(a1251, a1252)?
                                    };
                                    let a1266 = {
                                        let a1263 = {
                                            let a1253 = 1u64;
                                            let a1254 = 1u64;
                                            nat_add(a1253, a1254)?
                                        };
                                        let a1264 = {
                                            let a1261 = {
                                                let a1255 = 1u64;
                                                let a1256 = 1u64;
                                                nat_add(a1255, a1256)?
                                            };
                                            let a1262 = {
                                                let a1259 = {
                                                    let a1257 = 1u64;
                                                    let a1258 = 1u64;
                                                    nat_add(a1257, a1258)?
                                                };
                                                let a1260 = 1u64;
                                                nat_mul(a1259, a1260)?
                                            };
                                            nat_mul(a1261, a1262)?
                                        };
                                        nat_mul(a1263, a1264)?
                                    };
                                    nat_mul(a1265, a1266)?
                                };
                                nat_mul(a1267, a1268)?
                            };
                            nat_mul(a1269, a1270)?
                        };
                        nat_mul(a1271, a1272)?
                    };
                    nat_mul(a1273, a1274)?
                };
                nat_mul(a1275, a1276)?
            };
            let a1278 = 1u64;
            nat_add(a1277, a1278)?
        }, f9({
            let a1297 = {
                let a1279 = 1u64;
                let a1280 = 1u64;
                nat_add(a1279, a1280)?
            };
            let a1298 = {
                let a1295 = {
                    let a1293 = {
                        let a1281 = 1u64;
                        let a1282 = 1u64;
                        nat_add(a1281, a1282)?
                    };
                    let a1294 = {
                        let a1291 = {
                            let a1283 = 1u64;
                            let a1284 = 1u64;
                            nat_add(a1283, a1284)?
                        };
                        let a1292 = {
                            let a1289 = {
                                let a1287 = {
                                    let a1285 = 1u64;
                                    let a1286 = 1u64;
                                    nat_add(a1285, a1286)?
                                };
                                let a1288 = 1u64;
                                nat_mul(a1287, a1288)?
                            };
                            let a1290 = 1u64;
                            nat_add(a1289, a1290)?
                        };
                        nat_mul(a1291, a1292)?
                    };
                    nat_mul(a1293, a1294)?
                };
                let a1296 = 1u64;
                nat_add(a1295, a1296)?
            };
            nat_mul(a1297, a1298)?
        })?)?;
        f29(v0, match v3.clone() {
            Adt1::C0(h1299, _, _, _) => {
                h1299
            }
        }, match v3 {
            Adt1::C0(_, h1300, _, _) => {
                h1300
            }
        })
    }
}

pub fn f26(v0: u64, v1: bool) -> R<u64> {
    if if v1 {
        let a1308 = {
            let a1305 = v0;
            let a1306 = {
                let a1303 = {
                    let a1301 = 1u64;
                    let a1302 = 1u64;
                    nat_add(a1301, a1302)?
                };
                let a1304 = 1u64;
                nat_mul(a1303, a1304)?
            };
            let a1307 = 0u64;
            nat_rem(a1305, a1306, a1307)
        };
        let a1309 = 0u64;
        nat_eq(a1308, a1309)
    } else {
        false
    } {
        let a1310 = v0;
        let a1311 = 1u64;
        nat_add(a1310, a1311)
    } else {
        Ok(v0)
    }
}

pub fn f27(v0: Fn1, p1316: u64, p1317: Adt2) -> R<(Adt2, bool)> {
    let m1315 = (p1316, p1317);
    {
        let mut m1315 = m1315;
        loop {
            let step = {
                let (v1, v2) = m1315;
                let m1312 = v1;
                if m1312 == 0 {
                    Err::<(u64, Adt2), R<(Adt2, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1312 - 1;
                    let m1314 = {
                        let c1313: Fn1 = v0.clone();
                        c1313.apply(v2.clone())?
                    };
                    match m1314 {
                        None => {
                            Err::<(u64, Adt2), R<(Adt2, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt2), R<(Adt2, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1315 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f28(v0: Adt2) -> R<Option<Adt2>> {
    f34(v0)
}

pub fn f29(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1356 = v1;
    let a1357 = {
        let a1354 = {
            let a1352 = {
                let a1318 = 1u64;
                let a1319 = 1u64;
                nat_add(a1318, a1319)?
            };
            let a1353 = {
                let a1350 = {
                    let a1320 = 1u64;
                    let a1321 = 1u64;
                    nat_add(a1320, a1321)?
                };
                let a1351 = {
                    let a1348 = {
                        let a1322 = 1u64;
                        let a1323 = 1u64;
                        nat_add(a1322, a1323)?
                    };
                    let a1349 = {
                        let a1346 = {
                            let a1324 = 1u64;
                            let a1325 = 1u64;
                            nat_add(a1324, a1325)?
                        };
                        let a1347 = {
                            let a1344 = {
                                let a1326 = 1u64;
                                let a1327 = 1u64;
                                nat_add(a1326, a1327)?
                            };
                            let a1345 = {
                                let a1342 = {
                                    let a1328 = 1u64;
                                    let a1329 = 1u64;
                                    nat_add(a1328, a1329)?
                                };
                                let a1343 = {
                                    let a1340 = {
                                        let a1330 = 1u64;
                                        let a1331 = 1u64;
                                        nat_add(a1330, a1331)?
                                    };
                                    let a1341 = {
                                        let a1338 = {
                                            let a1332 = 1u64;
                                            let a1333 = 1u64;
                                            nat_add(a1332, a1333)?
                                        };
                                        let a1339 = {
                                            let a1336 = {
                                                let a1334 = 1u64;
                                                let a1335 = 1u64;
                                                nat_add(a1334, a1335)?
                                            };
                                            let a1337 = 1u64;
                                            nat_mul(a1336, a1337)?
                                        };
                                        nat_mul(a1338, a1339)?
                                    };
                                    nat_mul(a1340, a1341)?
                                };
                                nat_mul(a1342, a1343)?
                            };
                            nat_mul(a1344, a1345)?
                        };
                        nat_mul(a1346, a1347)?
                    };
                    nat_mul(a1348, a1349)?
                };
                nat_mul(a1350, a1351)?
            };
            nat_mul(a1352, a1353)?
        };
        let a1355 = 1u64;
        nat_add(a1354, a1355)?
    };
    if nat_lt(a1356, a1357) {
        f35(v0, {
            let a1394 = {
                let a1392 = {
                    let a1358 = 1u64;
                    let a1359 = 1u64;
                    nat_add(a1358, a1359)?
                };
                let a1393 = {
                    let a1390 = {
                        let a1360 = 1u64;
                        let a1361 = 1u64;
                        nat_add(a1360, a1361)?
                    };
                    let a1391 = {
                        let a1388 = {
                            let a1362 = 1u64;
                            let a1363 = 1u64;
                            nat_add(a1362, a1363)?
                        };
                        let a1389 = {
                            let a1386 = {
                                let a1364 = 1u64;
                                let a1365 = 1u64;
                                nat_add(a1364, a1365)?
                            };
                            let a1387 = {
                                let a1384 = {
                                    let a1366 = 1u64;
                                    let a1367 = 1u64;
                                    nat_add(a1366, a1367)?
                                };
                                let a1385 = {
                                    let a1382 = {
                                        let a1368 = 1u64;
                                        let a1369 = 1u64;
                                        nat_add(a1368, a1369)?
                                    };
                                    let a1383 = {
                                        let a1380 = {
                                            let a1370 = 1u64;
                                            let a1371 = 1u64;
                                            nat_add(a1370, a1371)?
                                        };
                                        let a1381 = {
                                            let a1378 = {
                                                let a1372 = 1u64;
                                                let a1373 = 1u64;
                                                nat_add(a1372, a1373)?
                                            };
                                            let a1379 = {
                                                let a1376 = {
                                                    let a1374 = 1u64;
                                                    let a1375 = 1u64;
                                                    nat_add(a1374, a1375)?
                                                };
                                                let a1377 = 1u64;
                                                nat_mul(a1376, a1377)?
                                            };
                                            nat_mul(a1378, a1379)?
                                        };
                                        nat_mul(a1380, a1381)?
                                    };
                                    nat_mul(a1382, a1383)?
                                };
                                nat_mul(a1384, a1385)?
                            };
                            nat_mul(a1386, a1387)?
                        };
                        nat_mul(a1388, a1389)?
                    };
                    nat_mul(a1390, a1391)?
                };
                nat_mul(a1392, a1393)?
            };
            let a1395 = 1u64;
            nat_add(a1394, a1395)?
        }, f30(v2, {
            let a1434 = {
                let a1432 = {
                    let a1430 = {
                        let a1396 = 1u64;
                        let a1397 = 1u64;
                        nat_add(a1396, a1397)?
                    };
                    let a1431 = {
                        let a1428 = {
                            let a1398 = 1u64;
                            let a1399 = 1u64;
                            nat_add(a1398, a1399)?
                        };
                        let a1429 = {
                            let a1426 = {
                                let a1400 = 1u64;
                                let a1401 = 1u64;
                                nat_add(a1400, a1401)?
                            };
                            let a1427 = {
                                let a1424 = {
                                    let a1402 = 1u64;
                                    let a1403 = 1u64;
                                    nat_add(a1402, a1403)?
                                };
                                let a1425 = {
                                    let a1422 = {
                                        let a1404 = 1u64;
                                        let a1405 = 1u64;
                                        nat_add(a1404, a1405)?
                                    };
                                    let a1423 = {
                                        let a1420 = {
                                            let a1406 = 1u64;
                                            let a1407 = 1u64;
                                            nat_add(a1406, a1407)?
                                        };
                                        let a1421 = {
                                            let a1418 = {
                                                let a1408 = 1u64;
                                                let a1409 = 1u64;
                                                nat_add(a1408, a1409)?
                                            };
                                            let a1419 = {
                                                let a1416 = {
                                                    let a1410 = 1u64;
                                                    let a1411 = 1u64;
                                                    nat_add(a1410, a1411)?
                                                };
                                                let a1417 = {
                                                    let a1414 = {
                                                        let a1412 = 1u64;
                                                        let a1413 = 1u64;
                                                        nat_add(a1412, a1413)?
                                                    };
                                                    let a1415 = 1u64;
                                                    nat_mul(a1414, a1415)?
                                                };
                                                nat_mul(a1416, a1417)?
                                            };
                                            nat_mul(a1418, a1419)?
                                        };
                                        nat_mul(a1420, a1421)?
                                    };
                                    nat_mul(a1422, a1423)?
                                };
                                nat_mul(a1424, a1425)?
                            };
                            nat_mul(a1426, a1427)?
                        };
                        nat_mul(a1428, a1429)?
                    };
                    nat_mul(a1430, a1431)?
                };
                let a1433 = 1u64;
                nat_add(a1432, a1433)?
            };
            let a1435 = v1;
            nat_sub(a1434, a1435)
        })?)
    } else {
        f35(v0, v1, v2)
    }
}

pub fn f30(v0: u64, v1: u64) -> R<u64> {
    let a1436 = v1;
    let a1437 = 0u64;
    if nat_eq(a1436, a1437) {
        Ok(v0)
    } else {
        let a1462 = {
            let a1460 = {
                let a1438 = 1u64;
                let a1439 = 1u64;
                nat_add(a1438, a1439)?
            };
            let a1461 = {
                let a1458 = {
                    let a1440 = 1u64;
                    let a1441 = 1u64;
                    nat_add(a1440, a1441)?
                };
                let a1459 = {
                    let a1456 = {
                        let a1442 = 1u64;
                        let a1443 = 1u64;
                        nat_add(a1442, a1443)?
                    };
                    let a1457 = {
                        let a1454 = {
                            let a1444 = 1u64;
                            let a1445 = 1u64;
                            nat_add(a1444, a1445)?
                        };
                        let a1455 = {
                            let a1452 = {
                                let a1446 = 1u64;
                                let a1447 = 1u64;
                                nat_add(a1446, a1447)?
                            };
                            let a1453 = {
                                let a1450 = {
                                    let a1448 = 1u64;
                                    let a1449 = 1u64;
                                    nat_add(a1448, a1449)?
                                };
                                let a1451 = 1u64;
                                nat_mul(a1450, a1451)?
                            };
                            nat_mul(a1452, a1453)?
                        };
                        nat_mul(a1454, a1455)?
                    };
                    nat_mul(a1456, a1457)?
                };
                nat_mul(a1458, a1459)?
            };
            nat_mul(a1460, a1461)?
        };
        let a1463 = v1;
        if nat_le(a1462, a1463) {
            let a1464 = v0;
            let a1465 = 0u64;
            if nat_eq(a1464, a1465) {
                Ok(0u64)
            } else {
                Ok(1u64)
            }
        } else {
            let v2: u64 = f9(v1)?;
            f26({
                let a1466 = v0;
                let a1467 = v2;
                let a1468 = 0u64;
                nat_quot(a1466, a1467, a1468)
            }, {
                let a1472 = 0u64;
                let a1473 = {
                    let a1469 = v0;
                    let a1470 = v2;
                    let a1471 = 0u64;
                    nat_rem(a1469, a1470, a1471)
                };
                nat_lt(a1472, a1473)
            })
        }
    }
}

pub fn f31(v0: u64) -> R<u64> {
    let a1512 = {
        let a1474 = f20(v0)?;
        let a1475 = 0u64;
        if nat_eq(a1474, a1475) {
            1u64
        } else {
            f20(v0)?
        }
    };
    let a1513 = {
        let a1510 = {
            let a1476 = 1u64;
            let a1477 = 1u64;
            nat_add(a1476, a1477)?
        };
        let a1511 = {
            let a1508 = {
                let a1478 = 1u64;
                let a1479 = 1u64;
                nat_add(a1478, a1479)?
            };
            let a1509 = {
                let a1506 = {
                    let a1480 = 1u64;
                    let a1481 = 1u64;
                    nat_add(a1480, a1481)?
                };
                let a1507 = {
                    let a1504 = {
                        let a1482 = 1u64;
                        let a1483 = 1u64;
                        nat_add(a1482, a1483)?
                    };
                    let a1505 = {
                        let a1502 = {
                            let a1484 = 1u64;
                            let a1485 = 1u64;
                            nat_add(a1484, a1485)?
                        };
                        let a1503 = {
                            let a1500 = {
                                let a1486 = 1u64;
                                let a1487 = 1u64;
                                nat_add(a1486, a1487)?
                            };
                            let a1501 = {
                                let a1498 = {
                                    let a1488 = 1u64;
                                    let a1489 = 1u64;
                                    nat_add(a1488, a1489)?
                                };
                                let a1499 = {
                                    let a1496 = {
                                        let a1490 = 1u64;
                                        let a1491 = 1u64;
                                        nat_add(a1490, a1491)?
                                    };
                                    let a1497 = {
                                        let a1494 = {
                                            let a1492 = 1u64;
                                            let a1493 = 1u64;
                                            nat_add(a1492, a1493)?
                                        };
                                        let a1495 = 1u64;
                                        nat_mul(a1494, a1495)?
                                    };
                                    nat_mul(a1496, a1497)?
                                };
                                nat_mul(a1498, a1499)?
                            };
                            nat_mul(a1500, a1501)?
                        };
                        nat_mul(a1502, a1503)?
                    };
                    nat_mul(a1504, a1505)?
                };
                nat_mul(a1506, a1507)?
            };
            nat_mul(a1508, a1509)?
        };
        nat_mul(a1510, a1511)?
    };
    nat_add(a1512, a1513)
}

pub fn f32(v0: u64) -> R<u64> {
    let a1538 = f21(v0)?;
    let a1539 = {
        let a1514 = f20(v0)?;
        let a1515 = 0u64;
        if nat_eq(a1514, a1515) {
            0u64
        } else {
            f9({
                let a1536 = {
                    let a1534 = {
                        let a1516 = 1u64;
                        let a1517 = 1u64;
                        nat_add(a1516, a1517)?
                    };
                    let a1535 = {
                        let a1532 = {
                            let a1530 = {
                                let a1518 = 1u64;
                                let a1519 = 1u64;
                                nat_add(a1518, a1519)?
                            };
                            let a1531 = {
                                let a1528 = {
                                    let a1526 = {
                                        let a1520 = 1u64;
                                        let a1521 = 1u64;
                                        nat_add(a1520, a1521)?
                                    };
                                    let a1527 = {
                                        let a1524 = {
                                            let a1522 = 1u64;
                                            let a1523 = 1u64;
                                            nat_add(a1522, a1523)?
                                        };
                                        let a1525 = 1u64;
                                        nat_mul(a1524, a1525)?
                                    };
                                    nat_mul(a1526, a1527)?
                                };
                                let a1529 = 1u64;
                                nat_add(a1528, a1529)?
                            };
                            nat_mul(a1530, a1531)?
                        };
                        let a1533 = 1u64;
                        nat_add(a1532, a1533)?
                    };
                    nat_mul(a1534, a1535)?
                };
                let a1537 = 1u64;
                nat_add(a1536, a1537)?
            })?
        }
    };
    nat_add(a1538, a1539)
}

pub fn f33(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt1> {
    let (h1540, _) = f36(Fn2::F37, {
        let a1559 = {
            let a1541 = 1u64;
            let a1542 = 1u64;
            nat_add(a1541, a1542)?
        };
        let a1560 = {
            let a1557 = {
                let a1543 = 1u64;
                let a1544 = 1u64;
                nat_add(a1543, a1544)?
            };
            let a1558 = {
                let a1555 = {
                    let a1545 = 1u64;
                    let a1546 = 1u64;
                    nat_add(a1545, a1546)?
                };
                let a1556 = {
                    let a1553 = {
                        let a1547 = 1u64;
                        let a1548 = 1u64;
                        nat_add(a1547, a1548)?
                    };
                    let a1554 = {
                        let a1551 = {
                            let a1549 = 1u64;
                            let a1550 = 1u64;
                            nat_add(a1549, a1550)?
                        };
                        let a1552 = 1u64;
                        nat_mul(a1551, a1552)?
                    };
                    nat_mul(a1553, a1554)?
                };
                nat_mul(a1555, a1556)?
            };
            nat_mul(a1557, a1558)?
        };
        nat_mul(a1559, a1560)?
    }, Adt1::C0(v0, v1, v2, v3))?;
    Ok(h1540)
}

pub fn f34(v0: Adt2) -> R<Option<Adt2>> {
    let v1: u64 = {
        let a1573 = {
            let a1567 = match v0.clone() {
                Adt2::C0(_, h1561) => {
                    h1561
                }
            };
            let a1568 = {
                let a1564 = match v0.clone() {
                    Adt2::C0(h1562, _) => {
                        h1562
                    }
                };
                let a1565 = match v0.clone() {
                    Adt2::C0(_, h1563) => {
                        h1563
                    }
                };
                let a1566 = 0u64;
                nat_quot(a1564, a1565, a1566)
            };
            nat_add(a1567, a1568)?
        };
        let a1574 = {
            let a1571 = {
                let a1569 = 1u64;
                let a1570 = 1u64;
                nat_add(a1569, a1570)?
            };
            let a1572 = 1u64;
            nat_mul(a1571, a1572)?
        };
        let a1575 = 0u64;
        nat_quot(a1573, a1574, a1575)
    };
    let a1577 = v1;
    let a1578 = match v0.clone() {
        Adt2::C0(_, h1576) => {
            h1576
        }
    };
    if nat_lt(a1577, a1578) {
        Ok(Some(Adt2::C0(match v0 {
            Adt2::C0(h1579, _) => {
                h1579
            }
        }, v1)))
    } else {
        Ok(None::<Adt2>)
    }
}

pub fn f35(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let v3: u64 = {
        let a1592 = v2;
        let a1593 = {
            let a1590 = {
                let a1580 = 1u64;
                let a1581 = 1u64;
                nat_add(a1580, a1581)?
            };
            let a1591 = {
                let a1588 = {
                    let a1582 = 1u64;
                    let a1583 = 1u64;
                    nat_add(a1582, a1583)?
                };
                let a1589 = {
                    let a1586 = {
                        let a1584 = 1u64;
                        let a1585 = 1u64;
                        nat_add(a1584, a1585)?
                    };
                    let a1587 = 1u64;
                    nat_mul(a1586, a1587)?
                };
                nat_mul(a1588, a1589)?
            };
            nat_mul(a1590, a1591)?
        };
        let a1594 = 0u64;
        nat_rem(a1592, a1593, a1594)
    };
    let v4: u64 = {
        let a1607 = v2;
        let a1608 = {
            let a1605 = {
                let a1595 = 1u64;
                let a1596 = 1u64;
                nat_add(a1595, a1596)?
            };
            let a1606 = {
                let a1603 = {
                    let a1597 = 1u64;
                    let a1598 = 1u64;
                    nat_add(a1597, a1598)?
                };
                let a1604 = {
                    let a1601 = {
                        let a1599 = 1u64;
                        let a1600 = 1u64;
                        nat_add(a1599, a1600)?
                    };
                    let a1602 = 1u64;
                    nat_mul(a1601, a1602)?
                };
                nat_mul(a1603, a1604)?
            };
            nat_mul(a1605, a1606)?
        };
        let a1609 = 0u64;
        nat_quot(a1607, a1608, a1609)
    };
    let v5: bool = {
        let a1618 = {
            let a1616 = {
                let a1610 = 1u64;
                let a1611 = 1u64;
                nat_add(a1610, a1611)?
            };
            let a1617 = {
                let a1614 = {
                    let a1612 = 1u64;
                    let a1613 = 1u64;
                    nat_add(a1612, a1613)?
                };
                let a1615 = 1u64;
                nat_mul(a1614, a1615)?
            };
            nat_mul(a1616, a1617)?
        };
        let a1619 = v3;
        if nat_lt(a1618, a1619) {
            true
        } else {
            let a1628 = v3;
            let a1629 = {
                let a1626 = {
                    let a1620 = 1u64;
                    let a1621 = 1u64;
                    nat_add(a1620, a1621)?
                };
                let a1627 = {
                    let a1624 = {
                        let a1622 = 1u64;
                        let a1623 = 1u64;
                        nat_add(a1622, a1623)?
                    };
                    let a1625 = 1u64;
                    nat_mul(a1624, a1625)?
                };
                nat_mul(a1626, a1627)?
            };
            if nat_eq(a1628, a1629) {
                let a1637 = {
                    let a1634 = v4;
                    let a1635 = {
                        let a1632 = {
                            let a1630 = 1u64;
                            let a1631 = 1u64;
                            nat_add(a1630, a1631)?
                        };
                        let a1633 = 1u64;
                        nat_mul(a1632, a1633)?
                    };
                    let a1636 = 0u64;
                    nat_rem(a1634, a1635, a1636)
                };
                let a1638 = 1u64;
                nat_eq(a1637, a1638)
            } else {
                false
            }
        }
    };
    let v6: u64 = {
        let a1639 = v4;
        let a1640 = if v5 {
            1u64
        } else {
            0u64
        };
        nat_add(a1639, a1640)?
    };
    let v7: bool = {
        let a1669 = {
            let a1667 = {
                let a1643 = {
                    let a1641 = 1u64;
                    let a1642 = 1u64;
                    nat_add(a1641, a1642)?
                };
                let a1644 = 1u64;
                nat_mul(a1643, a1644)?
            };
            let a1668 = f9({
                let a1665 = {
                    let a1663 = {
                        let a1645 = 1u64;
                        let a1646 = 1u64;
                        nat_add(a1645, a1646)?
                    };
                    let a1664 = {
                        let a1661 = {
                            let a1659 = {
                                let a1647 = 1u64;
                                let a1648 = 1u64;
                                nat_add(a1647, a1648)?
                            };
                            let a1660 = {
                                let a1657 = {
                                    let a1655 = {
                                        let a1649 = 1u64;
                                        let a1650 = 1u64;
                                        nat_add(a1649, a1650)?
                                    };
                                    let a1656 = {
                                        let a1653 = {
                                            let a1651 = 1u64;
                                            let a1652 = 1u64;
                                            nat_add(a1651, a1652)?
                                        };
                                        let a1654 = 1u64;
                                        nat_mul(a1653, a1654)?
                                    };
                                    nat_mul(a1655, a1656)?
                                };
                                let a1658 = 1u64;
                                nat_add(a1657, a1658)?
                            };
                            nat_mul(a1659, a1660)?
                        };
                        let a1662 = 1u64;
                        nat_add(a1661, a1662)?
                    };
                    nat_mul(a1663, a1664)?
                };
                let a1666 = 1u64;
                nat_add(a1665, a1666)?
            })?;
            nat_mul(a1667, a1668)?
        };
        let a1670 = v6;
        nat_le(a1669, a1670)
    };
    let v8: u64 = if v7 {
        let a1675 = v6;
        let a1676 = {
            let a1673 = {
                let a1671 = 1u64;
                let a1672 = 1u64;
                nat_add(a1671, a1672)?
            };
            let a1674 = 1u64;
            nat_mul(a1673, a1674)?
        };
        let a1677 = 0u64;
        nat_quot(a1675, a1676, a1677)
    } else {
        v6
    };
    let v9: u64 = if v7 {
        let a1678 = v1;
        let a1679 = 1u64;
        nat_add(a1678, a1679)?
    } else {
        v1
    };
    let v10: u64 = {
        let a1718 = v9;
        let a1719 = {
            let a1716 = {
                let a1714 = {
                    let a1680 = 1u64;
                    let a1681 = 1u64;
                    nat_add(a1680, a1681)?
                };
                let a1715 = {
                    let a1712 = {
                        let a1682 = 1u64;
                        let a1683 = 1u64;
                        nat_add(a1682, a1683)?
                    };
                    let a1713 = {
                        let a1710 = {
                            let a1684 = 1u64;
                            let a1685 = 1u64;
                            nat_add(a1684, a1685)?
                        };
                        let a1711 = {
                            let a1708 = {
                                let a1686 = 1u64;
                                let a1687 = 1u64;
                                nat_add(a1686, a1687)?
                            };
                            let a1709 = {
                                let a1706 = {
                                    let a1688 = 1u64;
                                    let a1689 = 1u64;
                                    nat_add(a1688, a1689)?
                                };
                                let a1707 = {
                                    let a1704 = {
                                        let a1690 = 1u64;
                                        let a1691 = 1u64;
                                        nat_add(a1690, a1691)?
                                    };
                                    let a1705 = {
                                        let a1702 = {
                                            let a1692 = 1u64;
                                            let a1693 = 1u64;
                                            nat_add(a1692, a1693)?
                                        };
                                        let a1703 = {
                                            let a1700 = {
                                                let a1694 = 1u64;
                                                let a1695 = 1u64;
                                                nat_add(a1694, a1695)?
                                            };
                                            let a1701 = {
                                                let a1698 = {
                                                    let a1696 = 1u64;
                                                    let a1697 = 1u64;
                                                    nat_add(a1696, a1697)?
                                                };
                                                let a1699 = 1u64;
                                                nat_mul(a1698, a1699)?
                                            };
                                            nat_mul(a1700, a1701)?
                                        };
                                        nat_mul(a1702, a1703)?
                                    };
                                    nat_mul(a1704, a1705)?
                                };
                                nat_mul(a1706, a1707)?
                            };
                            nat_mul(a1708, a1709)?
                        };
                        nat_mul(a1710, a1711)?
                    };
                    nat_mul(a1712, a1713)?
                };
                nat_mul(a1714, a1715)?
            };
            let a1717 = 1u64;
            nat_add(a1716, a1717)?
        };
        if if nat_eq(a1718, a1719) {
            let a1742 = v8;
            let a1743 = f9({
                let a1740 = {
                    let a1738 = {
                        let a1720 = 1u64;
                        let a1721 = 1u64;
                        nat_add(a1720, a1721)?
                    };
                    let a1739 = {
                        let a1736 = {
                            let a1734 = {
                                let a1722 = 1u64;
                                let a1723 = 1u64;
                                nat_add(a1722, a1723)?
                            };
                            let a1735 = {
                                let a1732 = {
                                    let a1730 = {
                                        let a1724 = 1u64;
                                        let a1725 = 1u64;
                                        nat_add(a1724, a1725)?
                                    };
                                    let a1731 = {
                                        let a1728 = {
                                            let a1726 = 1u64;
                                            let a1727 = 1u64;
                                            nat_add(a1726, a1727)?
                                        };
                                        let a1729 = 1u64;
                                        nat_mul(a1728, a1729)?
                                    };
                                    nat_mul(a1730, a1731)?
                                };
                                let a1733 = 1u64;
                                nat_add(a1732, a1733)?
                            };
                            nat_mul(a1734, a1735)?
                        };
                        let a1737 = 1u64;
                        nat_add(a1736, a1737)?
                    };
                    nat_mul(a1738, a1739)?
                };
                let a1741 = 1u64;
                nat_add(a1740, a1741)?
            })?;
            nat_lt(a1742, a1743)
        } else {
            false
        } {
            0u64
        } else {
            let a1780 = v9;
            let a1781 = {
                let a1778 = {
                    let a1744 = 1u64;
                    let a1745 = 1u64;
                    nat_add(a1744, a1745)?
                };
                let a1779 = {
                    let a1776 = {
                        let a1746 = 1u64;
                        let a1747 = 1u64;
                        nat_add(a1746, a1747)?
                    };
                    let a1777 = {
                        let a1774 = {
                            let a1748 = 1u64;
                            let a1749 = 1u64;
                            nat_add(a1748, a1749)?
                        };
                        let a1775 = {
                            let a1772 = {
                                let a1750 = 1u64;
                                let a1751 = 1u64;
                                nat_add(a1750, a1751)?
                            };
                            let a1773 = {
                                let a1770 = {
                                    let a1752 = 1u64;
                                    let a1753 = 1u64;
                                    nat_add(a1752, a1753)?
                                };
                                let a1771 = {
                                    let a1768 = {
                                        let a1754 = 1u64;
                                        let a1755 = 1u64;
                                        nat_add(a1754, a1755)?
                                    };
                                    let a1769 = {
                                        let a1766 = {
                                            let a1756 = 1u64;
                                            let a1757 = 1u64;
                                            nat_add(a1756, a1757)?
                                        };
                                        let a1767 = {
                                            let a1764 = {
                                                let a1758 = 1u64;
                                                let a1759 = 1u64;
                                                nat_add(a1758, a1759)?
                                            };
                                            let a1765 = {
                                                let a1762 = {
                                                    let a1760 = 1u64;
                                                    let a1761 = 1u64;
                                                    nat_add(a1760, a1761)?
                                                };
                                                let a1763 = 1u64;
                                                nat_mul(a1762, a1763)?
                                            };
                                            nat_mul(a1764, a1765)?
                                        };
                                        nat_mul(a1766, a1767)?
                                    };
                                    nat_mul(a1768, a1769)?
                                };
                                nat_mul(a1770, a1771)?
                            };
                            nat_mul(a1772, a1773)?
                        };
                        nat_mul(a1774, a1775)?
                    };
                    nat_mul(a1776, a1777)?
                };
                nat_mul(a1778, a1779)?
            };
            nat_sub(a1780, a1781)
        }
    };
    let a1782 = v8;
    let a1783 = 0u64;
    if nat_eq(a1782, a1783) {
        f12(v0)
    } else {
        let a1836 = {
            let a1834 = {
                let a1832 = {
                    let a1784 = 1u64;
                    let a1785 = 1u64;
                    nat_add(a1784, a1785)?
                };
                let a1833 = {
                    let a1830 = {
                        let a1828 = {
                            let a1786 = 1u64;
                            let a1787 = 1u64;
                            nat_add(a1786, a1787)?
                        };
                        let a1829 = {
                            let a1826 = {
                                let a1824 = {
                                    let a1788 = 1u64;
                                    let a1789 = 1u64;
                                    nat_add(a1788, a1789)?
                                };
                                let a1825 = {
                                    let a1822 = {
                                        let a1820 = {
                                            let a1790 = 1u64;
                                            let a1791 = 1u64;
                                            nat_add(a1790, a1791)?
                                        };
                                        let a1821 = {
                                            let a1818 = {
                                                let a1816 = {
                                                    let a1792 = 1u64;
                                                    let a1793 = 1u64;
                                                    nat_add(a1792, a1793)?
                                                };
                                                let a1817 = {
                                                    let a1814 = {
                                                        let a1812 = {
                                                            let a1794 = 1u64;
                                                            let a1795 = 1u64;
                                                            nat_add(a1794, a1795)?
                                                        };
                                                        let a1813 = {
                                                            let a1810 = {
                                                                let a1808 = {
                                                                    let a1796 = 1u64;
                                                                    let a1797 = 1u64;
                                                                    nat_add(a1796, a1797)?
                                                                };
                                                                let a1809 = {
                                                                    let a1806 = {
                                                                        let a1804 = {
                                                                            let a1798 = 1u64;
                                                                            let a1799 = 1u64;
                                                                            nat_add(a1798, a1799)?
                                                                        };
                                                                        let a1805 = {
                                                                            let a1802 = {
                                                                                let a1800 = 1u64;
                                                                                let a1801 = 1u64;
                                                                                nat_add(a1800, a1801)?
                                                                            };
                                                                            let a1803 = 1u64;
                                                                            nat_mul(a1802, a1803)?
                                                                        };
                                                                        nat_mul(a1804, a1805)?
                                                                    };
                                                                    let a1807 = 1u64;
                                                                    nat_add(a1806, a1807)?
                                                                };
                                                                nat_mul(a1808, a1809)?
                                                            };
                                                            let a1811 = 1u64;
                                                            nat_add(a1810, a1811)?
                                                        };
                                                        nat_mul(a1812, a1813)?
                                                    };
                                                    let a1815 = 1u64;
                                                    nat_add(a1814, a1815)?
                                                };
                                                nat_mul(a1816, a1817)?
                                            };
                                            let a1819 = 1u64;
                                            nat_add(a1818, a1819)?
                                        };
                                        nat_mul(a1820, a1821)?
                                    };
                                    let a1823 = 1u64;
                                    nat_add(a1822, a1823)?
                                };
                                nat_mul(a1824, a1825)?
                            };
                            let a1827 = 1u64;
                            nat_add(a1826, a1827)?
                        };
                        nat_mul(a1828, a1829)?
                    };
                    let a1831 = 1u64;
                    nat_add(a1830, a1831)?
                };
                nat_mul(a1832, a1833)?
            };
            let a1835 = 1u64;
            nat_add(a1834, a1835)?
        };
        let a1837 = v9;
        if nat_le(a1836, a1837) {
            f10(v0)
        } else {
            let a1889 = {
                let a1862 = f12(v0)?;
                let a1863 = {
                    let a1860 = v10;
                    let a1861 = f9({
                        let a1858 = {
                            let a1856 = {
                                let a1838 = 1u64;
                                let a1839 = 1u64;
                                nat_add(a1838, a1839)?
                            };
                            let a1857 = {
                                let a1854 = {
                                    let a1852 = {
                                        let a1840 = 1u64;
                                        let a1841 = 1u64;
                                        nat_add(a1840, a1841)?
                                    };
                                    let a1853 = {
                                        let a1850 = {
                                            let a1848 = {
                                                let a1842 = 1u64;
                                                let a1843 = 1u64;
                                                nat_add(a1842, a1843)?
                                            };
                                            let a1849 = {
                                                let a1846 = {
                                                    let a1844 = 1u64;
                                                    let a1845 = 1u64;
                                                    nat_add(a1844, a1845)?
                                                };
                                                let a1847 = 1u64;
                                                nat_mul(a1846, a1847)?
                                            };
                                            nat_mul(a1848, a1849)?
                                        };
                                        let a1851 = 1u64;
                                        nat_add(a1850, a1851)?
                                    };
                                    nat_mul(a1852, a1853)?
                                };
                                let a1855 = 1u64;
                                nat_add(a1854, a1855)?
                            };
                            nat_mul(a1856, a1857)?
                        };
                        let a1859 = 1u64;
                        nat_add(a1858, a1859)?
                    })?;
                    nat_mul(a1860, a1861)?
                };
                nat_add(a1862, a1863)?
            };
            let a1890 = {
                let a1886 = v8;
                let a1887 = f9({
                    let a1884 = {
                        let a1882 = {
                            let a1864 = 1u64;
                            let a1865 = 1u64;
                            nat_add(a1864, a1865)?
                        };
                        let a1883 = {
                            let a1880 = {
                                let a1878 = {
                                    let a1866 = 1u64;
                                    let a1867 = 1u64;
                                    nat_add(a1866, a1867)?
                                };
                                let a1879 = {
                                    let a1876 = {
                                        let a1874 = {
                                            let a1868 = 1u64;
                                            let a1869 = 1u64;
                                            nat_add(a1868, a1869)?
                                        };
                                        let a1875 = {
                                            let a1872 = {
                                                let a1870 = 1u64;
                                                let a1871 = 1u64;
                                                nat_add(a1870, a1871)?
                                            };
                                            let a1873 = 1u64;
                                            nat_mul(a1872, a1873)?
                                        };
                                        nat_mul(a1874, a1875)?
                                    };
                                    let a1877 = 1u64;
                                    nat_add(a1876, a1877)?
                                };
                                nat_mul(a1878, a1879)?
                            };
                            let a1881 = 1u64;
                            nat_add(a1880, a1881)?
                        };
                        nat_mul(a1882, a1883)?
                    };
                    let a1885 = 1u64;
                    nat_add(a1884, a1885)?
                })?;
                let a1888 = 0u64;
                nat_rem(a1886, a1887, a1888)
            };
            nat_add(a1889, a1890)
        }
    }
}

pub fn f36(v0: Fn2, p1895: u64, p1896: Adt1) -> R<(Adt1, bool)> {
    let m1894 = (p1895, p1896);
    {
        let mut m1894 = m1894;
        loop {
            let step = {
                let (v1, v2) = m1894;
                let m1891 = v1;
                if m1891 == 0 {
                    Err::<(u64, Adt1), R<(Adt1, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1891 - 1;
                    let m1893 = {
                        let c1892: Fn2 = v0.clone();
                        c1892.apply(v2.clone())?
                    };
                    match m1893 {
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
                Ok(next) => { m1894 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f37(v0: Adt1) -> R<Option<Adt1>> {
    f38(v0)
}

pub fn f38(v0: Adt1) -> R<Option<Adt1>> {
    let a1898 = 0u64;
    let a1899 = match v0.clone() {
        Adt1::C0(_, h1897, _, _) => {
            h1897
        }
    };
    if if if nat_lt(a1898, a1899) {
        let a1902 = match v0.clone() {
            Adt1::C0(_, h1900, _, _) => {
                h1900
            }
        };
        let a1903 = match v0.clone() {
            Adt1::C0(_, _, _, h1901) => {
                h1901
            }
        };
        nat_lt(a1902, a1903)
    } else {
        false
    } {
        let a1906 = match v0.clone() {
            Adt1::C0(_, _, h1904, _) => {
                h1904
            }
        };
        let a1907 = match v0.clone() {
            Adt1::C0(h1905, _, _, _) => {
                h1905
            }
        };
        nat_lt(a1906, a1907)
    } else {
        false
    } {
        Ok(Some(Adt1::C0({
            let a1909 = match v0.clone() {
                Adt1::C0(h1908, _, _, _) => {
                    h1908
                }
            };
            let a1910 = 1u64;
            nat_sub(a1909, a1910)
        }, {
            let a1913 = match v0.clone() {
                Adt1::C0(_, h1911, _, _) => {
                    h1911
                }
            };
            let a1914 = match v0.clone() {
                Adt1::C0(_, h1912, _, _) => {
                    h1912
                }
            };
            nat_add(a1913, a1914)?
        }, match v0.clone() {
            Adt1::C0(_, _, h1915, _) => {
                h1915
            }
        }, match v0 {
            Adt1::C0(_, _, _, h1916) => {
                h1916
            }
        })))
    } else {
        Ok(None::<Adt1>)
    }
}

pub fn run(p0: Adt0) -> R<u64> {
    f0(p0)
}
