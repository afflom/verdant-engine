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
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
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
    F22,
}

impl Fn0 {
    pub fn apply(&self, p0: u64) -> R<u64> {
        match self {
            Fn0::F22 => f22(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F32,
}

impl Fn1 {
    pub fn apply(&self, p0: Adt2) -> R<Option<Adt2>> {
        match self {
            Fn1::F32 => f32(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F38,
}

impl Fn2 {
    pub fn apply(&self, p0: Adt1) -> R<Option<Adt1>> {
        match self {
            Fn2::F38 => f38(p0),
        }
    }
}

pub fn f0(v0: u64, v1: u64, v2: u64) -> R<Option<u64>> {
    let m1 = f1(v0, v1, v2)?;
    match m1 {
        None => {
            Ok(None::<u64>)
        }
        Some(v3) => {
            Ok(Some({
                let m2 = v3;
                match m2 {
                    Adt0::C0 => {
                        f2(v1, v2)?
                    }
                    Adt0::C1 => {
                        f2(v1, f3(v2)?)?
                    }
                    Adt0::C2 => {
                        f4(v1, v2)?
                    }
                    Adt0::C3 => {
                        f5(v1, v2)?
                    }
                    Adt0::C4 => {
                        f6(v1)?
                    }
                    Adt0::C5 => {
                        f4(v1, v1)?
                    }
                    Adt0::C6 => {
                        f3(v1)?
                    }
                    Adt0::C7 => {
                        f7(v1)?
                    }
                }
            }))
        }
    }
}

pub fn f1(v0: u64, v1: u64, v2: u64) -> R<Option<Adt0>> {
    let a15 = v0;
    let a16 = {
        let a13 = {
            let a3 = 1u64;
            let a4 = 1u64;
            nat_add(a3, a4)?
        };
        let a14 = {
            let a11 = {
                let a5 = 1u64;
                let a6 = 1u64;
                nat_add(a5, a6)?
            };
            let a12 = {
                let a9 = {
                    let a7 = 1u64;
                    let a8 = 1u64;
                    nat_add(a7, a8)?
                };
                let a10 = 1u64;
                nat_mul(a9, a10)?
            };
            nat_mul(a11, a12)?
        };
        nat_mul(a13, a14)?
    };
    if if if nat_lt(a15, a16) {
        let a37 = v1;
        let a38 = f8({
            let a35 = {
                let a17 = 1u64;
                let a18 = 1u64;
                nat_add(a17, a18)?
            };
            let a36 = {
                let a33 = {
                    let a19 = 1u64;
                    let a20 = 1u64;
                    nat_add(a19, a20)?
                };
                let a34 = {
                    let a31 = {
                        let a21 = 1u64;
                        let a22 = 1u64;
                        nat_add(a21, a22)?
                    };
                    let a32 = {
                        let a29 = {
                            let a23 = 1u64;
                            let a24 = 1u64;
                            nat_add(a23, a24)?
                        };
                        let a30 = {
                            let a27 = {
                                let a25 = 1u64;
                                let a26 = 1u64;
                                nat_add(a25, a26)?
                            };
                            let a28 = 1u64;
                            nat_mul(a27, a28)?
                        };
                        nat_mul(a29, a30)?
                    };
                    nat_mul(a31, a32)?
                };
                nat_mul(a33, a34)?
            };
            nat_mul(a35, a36)?
        })?;
        nat_lt(a37, a38)
    } else {
        false
    } {
        let a59 = v2;
        let a60 = f8({
            let a57 = {
                let a39 = 1u64;
                let a40 = 1u64;
                nat_add(a39, a40)?
            };
            let a58 = {
                let a55 = {
                    let a41 = 1u64;
                    let a42 = 1u64;
                    nat_add(a41, a42)?
                };
                let a56 = {
                    let a53 = {
                        let a43 = 1u64;
                        let a44 = 1u64;
                        nat_add(a43, a44)?
                    };
                    let a54 = {
                        let a51 = {
                            let a45 = 1u64;
                            let a46 = 1u64;
                            nat_add(a45, a46)?
                        };
                        let a52 = {
                            let a49 = {
                                let a47 = 1u64;
                                let a48 = 1u64;
                                nat_add(a47, a48)?
                            };
                            let a50 = 1u64;
                            nat_mul(a49, a50)?
                        };
                        nat_mul(a51, a52)?
                    };
                    nat_mul(a53, a54)?
                };
                nat_mul(a55, a56)?
            };
            nat_mul(a57, a58)?
        })?;
        nat_lt(a59, a60)
    } else {
        false
    } {
        Ok(Some({
            let a61 = v0;
            let a62 = 0u64;
            if nat_eq(a61, a62) {
                Adt0::C0
            } else {
                let a63 = v0;
                let a64 = 1u64;
                if nat_eq(a63, a64) {
                    Adt0::C1
                } else {
                    let a69 = v0;
                    let a70 = {
                        let a67 = {
                            let a65 = 1u64;
                            let a66 = 1u64;
                            nat_add(a65, a66)?
                        };
                        let a68 = 1u64;
                        nat_mul(a67, a68)?
                    };
                    if nat_eq(a69, a70) {
                        Adt0::C2
                    } else {
                        let a77 = v0;
                        let a78 = {
                            let a75 = {
                                let a73 = {
                                    let a71 = 1u64;
                                    let a72 = 1u64;
                                    nat_add(a71, a72)?
                                };
                                let a74 = 1u64;
                                nat_mul(a73, a74)?
                            };
                            let a76 = 1u64;
                            nat_add(a75, a76)?
                        };
                        if nat_eq(a77, a78) {
                            Adt0::C3
                        } else {
                            let a87 = v0;
                            let a88 = {
                                let a85 = {
                                    let a79 = 1u64;
                                    let a80 = 1u64;
                                    nat_add(a79, a80)?
                                };
                                let a86 = {
                                    let a83 = {
                                        let a81 = 1u64;
                                        let a82 = 1u64;
                                        nat_add(a81, a82)?
                                    };
                                    let a84 = 1u64;
                                    nat_mul(a83, a84)?
                                };
                                nat_mul(a85, a86)?
                            };
                            if nat_eq(a87, a88) {
                                Adt0::C4
                            } else {
                                let a99 = v0;
                                let a100 = {
                                    let a97 = {
                                        let a95 = {
                                            let a89 = 1u64;
                                            let a90 = 1u64;
                                            nat_add(a89, a90)?
                                        };
                                        let a96 = {
                                            let a93 = {
                                                let a91 = 1u64;
                                                let a92 = 1u64;
                                                nat_add(a91, a92)?
                                            };
                                            let a94 = 1u64;
                                            nat_mul(a93, a94)?
                                        };
                                        nat_mul(a95, a96)?
                                    };
                                    let a98 = 1u64;
                                    nat_add(a97, a98)?
                                };
                                if nat_eq(a99, a100) {
                                    Adt0::C5
                                } else {
                                    let a111 = v0;
                                    let a112 = {
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
                                    if nat_eq(a111, a112) {
                                        Adt0::C6
                                    } else {
                                        Adt0::C7
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }))
    } else {
        Ok(None::<Adt0>)
    }
}

pub fn f2(v0: u64, v1: u64) -> R<u64> {
    if if f9(v0)? {
        true
    } else {
        f9(v1)?
    } {
        let a199 = {
            let a177 = {
                let a153 = {
                    let a151 = {
                        let a113 = 1u64;
                        let a114 = 1u64;
                        nat_add(a113, a114)?
                    };
                    let a152 = {
                        let a149 = {
                            let a147 = {
                                let a115 = 1u64;
                                let a116 = 1u64;
                                nat_add(a115, a116)?
                            };
                            let a148 = {
                                let a145 = {
                                    let a143 = {
                                        let a117 = 1u64;
                                        let a118 = 1u64;
                                        nat_add(a117, a118)?
                                    };
                                    let a144 = {
                                        let a141 = {
                                            let a139 = {
                                                let a119 = 1u64;
                                                let a120 = 1u64;
                                                nat_add(a119, a120)?
                                            };
                                            let a140 = {
                                                let a137 = {
                                                    let a135 = {
                                                        let a121 = 1u64;
                                                        let a122 = 1u64;
                                                        nat_add(a121, a122)?
                                                    };
                                                    let a136 = {
                                                        let a133 = {
                                                            let a131 = {
                                                                let a123 = 1u64;
                                                                let a124 = 1u64;
                                                                nat_add(a123, a124)?
                                                            };
                                                            let a132 = {
                                                                let a129 = {
                                                                    let a127 = {
                                                                        let a125 = 1u64;
                                                                        let a126 = 1u64;
                                                                        nat_add(a125, a126)?
                                                                    };
                                                                    let a128 = 1u64;
                                                                    nat_mul(a127, a128)?
                                                                };
                                                                let a130 = 1u64;
                                                                nat_add(a129, a130)?
                                                            };
                                                            nat_mul(a131, a132)?
                                                        };
                                                        let a134 = 1u64;
                                                        nat_add(a133, a134)?
                                                    };
                                                    nat_mul(a135, a136)?
                                                };
                                                let a138 = 1u64;
                                                nat_add(a137, a138)?
                                            };
                                            nat_mul(a139, a140)?
                                        };
                                        let a142 = 1u64;
                                        nat_add(a141, a142)?
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
            };
            let a178 = f8({
                let a175 = {
                    let a173 = {
                        let a155 = 1u64;
                        let a156 = 1u64;
                        nat_add(a155, a156)?
                    };
                    let a174 = {
                        let a171 = {
                            let a169 = {
                                let a157 = 1u64;
                                let a158 = 1u64;
                                nat_add(a157, a158)?
                            };
                            let a170 = {
                                let a167 = {
                                    let a165 = {
                                        let a159 = 1u64;
                                        let a160 = 1u64;
                                        nat_add(a159, a160)?
                                    };
                                    let a166 = {
                                        let a163 = {
                                            let a161 = 1u64;
                                            let a162 = 1u64;
                                            nat_add(a161, a162)?
                                        };
                                        let a164 = 1u64;
                                        nat_mul(a163, a164)?
                                    };
                                    nat_mul(a165, a166)?
                                };
                                let a168 = 1u64;
                                nat_add(a167, a168)?
                            };
                            nat_mul(a169, a170)?
                        };
                        let a172 = 1u64;
                        nat_add(a171, a172)?
                    };
                    nat_mul(a173, a174)?
                };
                let a176 = 1u64;
                nat_add(a175, a176)?
            })?;
            nat_mul(a177, a178)?
        };
        let a200 = f8({
            let a197 = {
                let a179 = 1u64;
                let a180 = 1u64;
                nat_add(a179, a180)?
            };
            let a198 = {
                let a195 = {
                    let a193 = {
                        let a181 = 1u64;
                        let a182 = 1u64;
                        nat_add(a181, a182)?
                    };
                    let a194 = {
                        let a191 = {
                            let a189 = {
                                let a183 = 1u64;
                                let a184 = 1u64;
                                nat_add(a183, a184)?
                            };
                            let a190 = {
                                let a187 = {
                                    let a185 = 1u64;
                                    let a186 = 1u64;
                                    nat_add(a185, a186)?
                                };
                                let a188 = 1u64;
                                nat_mul(a187, a188)?
                            };
                            nat_mul(a189, a190)?
                        };
                        let a192 = 1u64;
                        nat_add(a191, a192)?
                    };
                    nat_mul(a193, a194)?
                };
                let a196 = 1u64;
                nat_add(a195, a196)?
            };
            nat_mul(a197, a198)?
        })?;
        nat_add(a199, a200)
    } else {
        if if f10(v0)? {
            f10(v1)?
        } else {
            false
        } {
            let a201 = f11(v0)?;
            let a202 = f11(v1)?;
            if nat_eq(a201, a202) {
                Ok(v0)
            } else {
                let a289 = {
                    let a267 = {
                        let a243 = {
                            let a241 = {
                                let a203 = 1u64;
                                let a204 = 1u64;
                                nat_add(a203, a204)?
                            };
                            let a242 = {
                                let a239 = {
                                    let a237 = {
                                        let a205 = 1u64;
                                        let a206 = 1u64;
                                        nat_add(a205, a206)?
                                    };
                                    let a238 = {
                                        let a235 = {
                                            let a233 = {
                                                let a207 = 1u64;
                                                let a208 = 1u64;
                                                nat_add(a207, a208)?
                                            };
                                            let a234 = {
                                                let a231 = {
                                                    let a229 = {
                                                        let a209 = 1u64;
                                                        let a210 = 1u64;
                                                        nat_add(a209, a210)?
                                                    };
                                                    let a230 = {
                                                        let a227 = {
                                                            let a225 = {
                                                                let a211 = 1u64;
                                                                let a212 = 1u64;
                                                                nat_add(a211, a212)?
                                                            };
                                                            let a226 = {
                                                                let a223 = {
                                                                    let a221 = {
                                                                        let a213 = 1u64;
                                                                        let a214 = 1u64;
                                                                        nat_add(a213, a214)?
                                                                    };
                                                                    let a222 = {
                                                                        let a219 = {
                                                                            let a217 = {
                                                                                let a215 = 1u64;
                                                                                let a216 = 1u64;
                                                                                nat_add(a215, a216)?
                                                                            };
                                                                            let a218 = 1u64;
                                                                            nat_mul(a217, a218)?
                                                                        };
                                                                        let a220 = 1u64;
                                                                        nat_add(a219, a220)?
                                                                    };
                                                                    nat_mul(a221, a222)?
                                                                };
                                                                let a224 = 1u64;
                                                                nat_add(a223, a224)?
                                                            };
                                                            nat_mul(a225, a226)?
                                                        };
                                                        let a228 = 1u64;
                                                        nat_add(a227, a228)?
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
                            };
                            nat_mul(a241, a242)?
                        };
                        let a244 = 1u64;
                        nat_add(a243, a244)?
                    };
                    let a268 = f8({
                        let a265 = {
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
                        };
                        let a266 = 1u64;
                        nat_add(a265, a266)?
                    })?;
                    nat_mul(a267, a268)?
                };
                let a290 = f8({
                    let a287 = {
                        let a269 = 1u64;
                        let a270 = 1u64;
                        nat_add(a269, a270)?
                    };
                    let a288 = {
                        let a285 = {
                            let a283 = {
                                let a271 = 1u64;
                                let a272 = 1u64;
                                nat_add(a271, a272)?
                            };
                            let a284 = {
                                let a281 = {
                                    let a279 = {
                                        let a273 = 1u64;
                                        let a274 = 1u64;
                                        nat_add(a273, a274)?
                                    };
                                    let a280 = {
                                        let a277 = {
                                            let a275 = 1u64;
                                            let a276 = 1u64;
                                            nat_add(a275, a276)?
                                        };
                                        let a278 = 1u64;
                                        nat_mul(a277, a278)?
                                    };
                                    nat_mul(a279, a280)?
                                };
                                let a282 = 1u64;
                                nat_add(a281, a282)?
                            };
                            nat_mul(a283, a284)?
                        };
                        let a286 = 1u64;
                        nat_add(a285, a286)?
                    };
                    nat_mul(a287, a288)?
                })?;
                nat_add(a289, a290)
            }
        } else {
            if f10(v0)? {
                Ok(v0)
            } else {
                if f10(v1)? {
                    Ok(v1)
                } else {
                    let a291 = f7(v1)?;
                    let a292 = f7(v0)?;
                    if nat_le(a291, a292) {
                        f12(v0, v1)
                    } else {
                        f12(v1, v0)
                    }
                }
            }
        }
    }
}

pub fn f3(v0: u64) -> R<u64> {
    let a339 = {
        let a317 = v0;
        let a318 = f8({
            let a315 = {
                let a313 = {
                    let a293 = 1u64;
                    let a294 = 1u64;
                    nat_add(a293, a294)?
                };
                let a314 = {
                    let a311 = {
                        let a309 = {
                            let a295 = 1u64;
                            let a296 = 1u64;
                            nat_add(a295, a296)?
                        };
                        let a310 = {
                            let a307 = {
                                let a305 = {
                                    let a297 = 1u64;
                                    let a298 = 1u64;
                                    nat_add(a297, a298)?
                                };
                                let a306 = {
                                    let a303 = {
                                        let a301 = {
                                            let a299 = 1u64;
                                            let a300 = 1u64;
                                            nat_add(a299, a300)?
                                        };
                                        let a302 = 1u64;
                                        nat_mul(a301, a302)?
                                    };
                                    let a304 = 1u64;
                                    nat_add(a303, a304)?
                                };
                                nat_mul(a305, a306)?
                            };
                            let a308 = 1u64;
                            nat_add(a307, a308)?
                        };
                        nat_mul(a309, a310)?
                    };
                    let a312 = 1u64;
                    nat_add(a311, a312)?
                };
                nat_mul(a313, a314)?
            };
            let a316 = 1u64;
            nat_add(a315, a316)?
        })?;
        nat_add(a317, a318)?
    };
    let a340 = f8({
        let a337 = {
            let a319 = 1u64;
            let a320 = 1u64;
            nat_add(a319, a320)?
        };
        let a338 = {
            let a335 = {
                let a321 = 1u64;
                let a322 = 1u64;
                nat_add(a321, a322)?
            };
            let a336 = {
                let a333 = {
                    let a323 = 1u64;
                    let a324 = 1u64;
                    nat_add(a323, a324)?
                };
                let a334 = {
                    let a331 = {
                        let a325 = 1u64;
                        let a326 = 1u64;
                        nat_add(a325, a326)?
                    };
                    let a332 = {
                        let a329 = {
                            let a327 = 1u64;
                            let a328 = 1u64;
                            nat_add(a327, a328)?
                        };
                        let a330 = 1u64;
                        nat_mul(a329, a330)?
                    };
                    nat_mul(a331, a332)?
                };
                nat_mul(a333, a334)?
            };
            nat_mul(a335, a336)?
        };
        nat_mul(a337, a338)?
    })?;
    let a341 = 0u64;
    Ok(nat_rem(a339, a340, a341))
}

pub fn f4(v0: u64, v1: u64) -> R<u64> {
    if if if if f9(v0)? {
        true
    } else {
        f9(v1)?
    } {
        true
    } else {
        if f10(v0)? {
            f13(v1)?
        } else {
            false
        }
    } {
        true
    } else {
        if f10(v1)? {
            f13(v0)?
        } else {
            false
        }
    } {
        let a428 = {
            let a406 = {
                let a382 = {
                    let a380 = {
                        let a342 = 1u64;
                        let a343 = 1u64;
                        nat_add(a342, a343)?
                    };
                    let a381 = {
                        let a378 = {
                            let a376 = {
                                let a344 = 1u64;
                                let a345 = 1u64;
                                nat_add(a344, a345)?
                            };
                            let a377 = {
                                let a374 = {
                                    let a372 = {
                                        let a346 = 1u64;
                                        let a347 = 1u64;
                                        nat_add(a346, a347)?
                                    };
                                    let a373 = {
                                        let a370 = {
                                            let a368 = {
                                                let a348 = 1u64;
                                                let a349 = 1u64;
                                                nat_add(a348, a349)?
                                            };
                                            let a369 = {
                                                let a366 = {
                                                    let a364 = {
                                                        let a350 = 1u64;
                                                        let a351 = 1u64;
                                                        nat_add(a350, a351)?
                                                    };
                                                    let a365 = {
                                                        let a362 = {
                                                            let a360 = {
                                                                let a352 = 1u64;
                                                                let a353 = 1u64;
                                                                nat_add(a352, a353)?
                                                            };
                                                            let a361 = {
                                                                let a358 = {
                                                                    let a356 = {
                                                                        let a354 = 1u64;
                                                                        let a355 = 1u64;
                                                                        nat_add(a354, a355)?
                                                                    };
                                                                    let a357 = 1u64;
                                                                    nat_mul(a356, a357)?
                                                                };
                                                                let a359 = 1u64;
                                                                nat_add(a358, a359)?
                                                            };
                                                            nat_mul(a360, a361)?
                                                        };
                                                        let a363 = 1u64;
                                                        nat_add(a362, a363)?
                                                    };
                                                    nat_mul(a364, a365)?
                                                };
                                                let a367 = 1u64;
                                                nat_add(a366, a367)?
                                            };
                                            nat_mul(a368, a369)?
                                        };
                                        let a371 = 1u64;
                                        nat_add(a370, a371)?
                                    };
                                    nat_mul(a372, a373)?
                                };
                                let a375 = 1u64;
                                nat_add(a374, a375)?
                            };
                            nat_mul(a376, a377)?
                        };
                        let a379 = 1u64;
                        nat_add(a378, a379)?
                    };
                    nat_mul(a380, a381)?
                };
                let a383 = 1u64;
                nat_add(a382, a383)?
            };
            let a407 = f8({
                let a404 = {
                    let a402 = {
                        let a384 = 1u64;
                        let a385 = 1u64;
                        nat_add(a384, a385)?
                    };
                    let a403 = {
                        let a400 = {
                            let a398 = {
                                let a386 = 1u64;
                                let a387 = 1u64;
                                nat_add(a386, a387)?
                            };
                            let a399 = {
                                let a396 = {
                                    let a394 = {
                                        let a388 = 1u64;
                                        let a389 = 1u64;
                                        nat_add(a388, a389)?
                                    };
                                    let a395 = {
                                        let a392 = {
                                            let a390 = 1u64;
                                            let a391 = 1u64;
                                            nat_add(a390, a391)?
                                        };
                                        let a393 = 1u64;
                                        nat_mul(a392, a393)?
                                    };
                                    nat_mul(a394, a395)?
                                };
                                let a397 = 1u64;
                                nat_add(a396, a397)?
                            };
                            nat_mul(a398, a399)?
                        };
                        let a401 = 1u64;
                        nat_add(a400, a401)?
                    };
                    nat_mul(a402, a403)?
                };
                let a405 = 1u64;
                nat_add(a404, a405)?
            })?;
            nat_mul(a406, a407)?
        };
        let a429 = f8({
            let a426 = {
                let a408 = 1u64;
                let a409 = 1u64;
                nat_add(a408, a409)?
            };
            let a427 = {
                let a424 = {
                    let a422 = {
                        let a410 = 1u64;
                        let a411 = 1u64;
                        nat_add(a410, a411)?
                    };
                    let a423 = {
                        let a420 = {
                            let a418 = {
                                let a412 = 1u64;
                                let a413 = 1u64;
                                nat_add(a412, a413)?
                            };
                            let a419 = {
                                let a416 = {
                                    let a414 = 1u64;
                                    let a415 = 1u64;
                                    nat_add(a414, a415)?
                                };
                                let a417 = 1u64;
                                nat_mul(a416, a417)?
                            };
                            nat_mul(a418, a419)?
                        };
                        let a421 = 1u64;
                        nat_add(a420, a421)?
                    };
                    nat_mul(a422, a423)?
                };
                let a425 = 1u64;
                nat_add(a424, a425)?
            };
            nat_mul(a426, a427)?
        })?;
        nat_add(a428, a429)
    } else {
        if if f10(v0)? {
            true
        } else {
            f10(v1)?
        } {
            f14(f15(v0, v1)?)
        } else {
            if if f13(v0)? {
                true
            } else {
                f13(v1)?
            } {
                f16(f15(v0, v1)?)
            } else {
                f17(v0, v1)
            }
        }
    }
}

pub fn f5(v0: u64, v1: u64) -> R<u64> {
    if if if if f9(v0)? {
        true
    } else {
        f9(v1)?
    } {
        true
    } else {
        if f10(v0)? {
            f10(v1)?
        } else {
            false
        }
    } {
        true
    } else {
        if f13(v0)? {
            f13(v1)?
        } else {
            false
        }
    } {
        let a516 = {
            let a494 = {
                let a470 = {
                    let a468 = {
                        let a430 = 1u64;
                        let a431 = 1u64;
                        nat_add(a430, a431)?
                    };
                    let a469 = {
                        let a466 = {
                            let a464 = {
                                let a432 = 1u64;
                                let a433 = 1u64;
                                nat_add(a432, a433)?
                            };
                            let a465 = {
                                let a462 = {
                                    let a460 = {
                                        let a434 = 1u64;
                                        let a435 = 1u64;
                                        nat_add(a434, a435)?
                                    };
                                    let a461 = {
                                        let a458 = {
                                            let a456 = {
                                                let a436 = 1u64;
                                                let a437 = 1u64;
                                                nat_add(a436, a437)?
                                            };
                                            let a457 = {
                                                let a454 = {
                                                    let a452 = {
                                                        let a438 = 1u64;
                                                        let a439 = 1u64;
                                                        nat_add(a438, a439)?
                                                    };
                                                    let a453 = {
                                                        let a450 = {
                                                            let a448 = {
                                                                let a440 = 1u64;
                                                                let a441 = 1u64;
                                                                nat_add(a440, a441)?
                                                            };
                                                            let a449 = {
                                                                let a446 = {
                                                                    let a444 = {
                                                                        let a442 = 1u64;
                                                                        let a443 = 1u64;
                                                                        nat_add(a442, a443)?
                                                                    };
                                                                    let a445 = 1u64;
                                                                    nat_mul(a444, a445)?
                                                                };
                                                                let a447 = 1u64;
                                                                nat_add(a446, a447)?
                                                            };
                                                            nat_mul(a448, a449)?
                                                        };
                                                        let a451 = 1u64;
                                                        nat_add(a450, a451)?
                                                    };
                                                    nat_mul(a452, a453)?
                                                };
                                                let a455 = 1u64;
                                                nat_add(a454, a455)?
                                            };
                                            nat_mul(a456, a457)?
                                        };
                                        let a459 = 1u64;
                                        nat_add(a458, a459)?
                                    };
                                    nat_mul(a460, a461)?
                                };
                                let a463 = 1u64;
                                nat_add(a462, a463)?
                            };
                            nat_mul(a464, a465)?
                        };
                        let a467 = 1u64;
                        nat_add(a466, a467)?
                    };
                    nat_mul(a468, a469)?
                };
                let a471 = 1u64;
                nat_add(a470, a471)?
            };
            let a495 = f8({
                let a492 = {
                    let a490 = {
                        let a472 = 1u64;
                        let a473 = 1u64;
                        nat_add(a472, a473)?
                    };
                    let a491 = {
                        let a488 = {
                            let a486 = {
                                let a474 = 1u64;
                                let a475 = 1u64;
                                nat_add(a474, a475)?
                            };
                            let a487 = {
                                let a484 = {
                                    let a482 = {
                                        let a476 = 1u64;
                                        let a477 = 1u64;
                                        nat_add(a476, a477)?
                                    };
                                    let a483 = {
                                        let a480 = {
                                            let a478 = 1u64;
                                            let a479 = 1u64;
                                            nat_add(a478, a479)?
                                        };
                                        let a481 = 1u64;
                                        nat_mul(a480, a481)?
                                    };
                                    nat_mul(a482, a483)?
                                };
                                let a485 = 1u64;
                                nat_add(a484, a485)?
                            };
                            nat_mul(a486, a487)?
                        };
                        let a489 = 1u64;
                        nat_add(a488, a489)?
                    };
                    nat_mul(a490, a491)?
                };
                let a493 = 1u64;
                nat_add(a492, a493)?
            })?;
            nat_mul(a494, a495)?
        };
        let a517 = f8({
            let a514 = {
                let a496 = 1u64;
                let a497 = 1u64;
                nat_add(a496, a497)?
            };
            let a515 = {
                let a512 = {
                    let a510 = {
                        let a498 = 1u64;
                        let a499 = 1u64;
                        nat_add(a498, a499)?
                    };
                    let a511 = {
                        let a508 = {
                            let a506 = {
                                let a500 = 1u64;
                                let a501 = 1u64;
                                nat_add(a500, a501)?
                            };
                            let a507 = {
                                let a504 = {
                                    let a502 = 1u64;
                                    let a503 = 1u64;
                                    nat_add(a502, a503)?
                                };
                                let a505 = 1u64;
                                nat_mul(a504, a505)?
                            };
                            nat_mul(a506, a507)?
                        };
                        let a509 = 1u64;
                        nat_add(a508, a509)?
                    };
                    nat_mul(a510, a511)?
                };
                let a513 = 1u64;
                nat_add(a512, a513)?
            };
            nat_mul(a514, a515)?
        })?;
        nat_add(a516, a517)
    } else {
        if if f10(v0)? {
            true
        } else {
            f13(v1)?
        } {
            f14(f15(v0, v1)?)
        } else {
            if if f10(v1)? {
                true
            } else {
                f13(v0)?
            } {
                f16(f15(v0, v1)?)
            } else {
                f18(v0, v1)
            }
        }
    }
}

pub fn f6(v0: u64) -> R<u64> {
    let m518 = f19(v0)?;
    match m518 {
        None => {
            f20(v0)
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f7(v0: u64) -> R<u64> {
    let a543 = v0;
    let a544 = f8({
        let a541 = {
            let a539 = {
                let a519 = 1u64;
                let a520 = 1u64;
                nat_add(a519, a520)?
            };
            let a540 = {
                let a537 = {
                    let a535 = {
                        let a521 = 1u64;
                        let a522 = 1u64;
                        nat_add(a521, a522)?
                    };
                    let a536 = {
                        let a533 = {
                            let a531 = {
                                let a523 = 1u64;
                                let a524 = 1u64;
                                nat_add(a523, a524)?
                            };
                            let a532 = {
                                let a529 = {
                                    let a527 = {
                                        let a525 = 1u64;
                                        let a526 = 1u64;
                                        nat_add(a525, a526)?
                                    };
                                    let a528 = 1u64;
                                    nat_mul(a527, a528)?
                                };
                                let a530 = 1u64;
                                nat_add(a529, a530)?
                            };
                            nat_mul(a531, a532)?
                        };
                        let a534 = 1u64;
                        nat_add(a533, a534)?
                    };
                    nat_mul(a535, a536)?
                };
                let a538 = 1u64;
                nat_add(a537, a538)?
            };
            nat_mul(a539, a540)?
        };
        let a542 = 1u64;
        nat_add(a541, a542)?
    })?;
    let a545 = 0u64;
    Ok(nat_rem(a543, a544, a545))
}

pub fn f8(v0: u64) -> R<u64> {
    f21(Fn0::F22, v0, 1u64)
}

pub fn f9(v0: u64) -> R<bool> {
    let a588 = f23(v0)?;
    let a589 = {
        let a586 = {
            let a584 = {
                let a546 = 1u64;
                let a547 = 1u64;
                nat_add(a546, a547)?
            };
            let a585 = {
                let a582 = {
                    let a580 = {
                        let a548 = 1u64;
                        let a549 = 1u64;
                        nat_add(a548, a549)?
                    };
                    let a581 = {
                        let a578 = {
                            let a576 = {
                                let a550 = 1u64;
                                let a551 = 1u64;
                                nat_add(a550, a551)?
                            };
                            let a577 = {
                                let a574 = {
                                    let a572 = {
                                        let a552 = 1u64;
                                        let a553 = 1u64;
                                        nat_add(a552, a553)?
                                    };
                                    let a573 = {
                                        let a570 = {
                                            let a568 = {
                                                let a554 = 1u64;
                                                let a555 = 1u64;
                                                nat_add(a554, a555)?
                                            };
                                            let a569 = {
                                                let a566 = {
                                                    let a564 = {
                                                        let a556 = 1u64;
                                                        let a557 = 1u64;
                                                        nat_add(a556, a557)?
                                                    };
                                                    let a565 = {
                                                        let a562 = {
                                                            let a560 = {
                                                                let a558 = 1u64;
                                                                let a559 = 1u64;
                                                                nat_add(a558, a559)?
                                                            };
                                                            let a561 = 1u64;
                                                            nat_mul(a560, a561)?
                                                        };
                                                        let a563 = 1u64;
                                                        nat_add(a562, a563)?
                                                    };
                                                    nat_mul(a564, a565)?
                                                };
                                                let a567 = 1u64;
                                                nat_add(a566, a567)?
                                            };
                                            nat_mul(a568, a569)?
                                        };
                                        let a571 = 1u64;
                                        nat_add(a570, a571)?
                                    };
                                    nat_mul(a572, a573)?
                                };
                                let a575 = 1u64;
                                nat_add(a574, a575)?
                            };
                            nat_mul(a576, a577)?
                        };
                        let a579 = 1u64;
                        nat_add(a578, a579)?
                    };
                    nat_mul(a580, a581)?
                };
                let a583 = 1u64;
                nat_add(a582, a583)?
            };
            nat_mul(a584, a585)?
        };
        let a587 = 1u64;
        nat_add(a586, a587)?
    };
    if nat_eq(a588, a589) {
        let a590 = 0u64;
        let a591 = f24(v0)?;
        Ok(nat_lt(a590, a591))
    } else {
        Ok(false)
    }
}

pub fn f10(v0: u64) -> R<bool> {
    let a634 = f23(v0)?;
    let a635 = {
        let a632 = {
            let a630 = {
                let a592 = 1u64;
                let a593 = 1u64;
                nat_add(a592, a593)?
            };
            let a631 = {
                let a628 = {
                    let a626 = {
                        let a594 = 1u64;
                        let a595 = 1u64;
                        nat_add(a594, a595)?
                    };
                    let a627 = {
                        let a624 = {
                            let a622 = {
                                let a596 = 1u64;
                                let a597 = 1u64;
                                nat_add(a596, a597)?
                            };
                            let a623 = {
                                let a620 = {
                                    let a618 = {
                                        let a598 = 1u64;
                                        let a599 = 1u64;
                                        nat_add(a598, a599)?
                                    };
                                    let a619 = {
                                        let a616 = {
                                            let a614 = {
                                                let a600 = 1u64;
                                                let a601 = 1u64;
                                                nat_add(a600, a601)?
                                            };
                                            let a615 = {
                                                let a612 = {
                                                    let a610 = {
                                                        let a602 = 1u64;
                                                        let a603 = 1u64;
                                                        nat_add(a602, a603)?
                                                    };
                                                    let a611 = {
                                                        let a608 = {
                                                            let a606 = {
                                                                let a604 = 1u64;
                                                                let a605 = 1u64;
                                                                nat_add(a604, a605)?
                                                            };
                                                            let a607 = 1u64;
                                                            nat_mul(a606, a607)?
                                                        };
                                                        let a609 = 1u64;
                                                        nat_add(a608, a609)?
                                                    };
                                                    nat_mul(a610, a611)?
                                                };
                                                let a613 = 1u64;
                                                nat_add(a612, a613)?
                                            };
                                            nat_mul(a614, a615)?
                                        };
                                        let a617 = 1u64;
                                        nat_add(a616, a617)?
                                    };
                                    nat_mul(a618, a619)?
                                };
                                let a621 = 1u64;
                                nat_add(a620, a621)?
                            };
                            nat_mul(a622, a623)?
                        };
                        let a625 = 1u64;
                        nat_add(a624, a625)?
                    };
                    nat_mul(a626, a627)?
                };
                let a629 = 1u64;
                nat_add(a628, a629)?
            };
            nat_mul(a630, a631)?
        };
        let a633 = 1u64;
        nat_add(a632, a633)?
    };
    if nat_eq(a634, a635) {
        let a636 = f24(v0)?;
        let a637 = 0u64;
        Ok(nat_eq(a636, a637))
    } else {
        Ok(false)
    }
}

pub fn f11(v0: u64) -> R<u64> {
    let a662 = v0;
    let a663 = f8({
        let a660 = {
            let a658 = {
                let a638 = 1u64;
                let a639 = 1u64;
                nat_add(a638, a639)?
            };
            let a659 = {
                let a656 = {
                    let a654 = {
                        let a640 = 1u64;
                        let a641 = 1u64;
                        nat_add(a640, a641)?
                    };
                    let a655 = {
                        let a652 = {
                            let a650 = {
                                let a642 = 1u64;
                                let a643 = 1u64;
                                nat_add(a642, a643)?
                            };
                            let a651 = {
                                let a648 = {
                                    let a646 = {
                                        let a644 = 1u64;
                                        let a645 = 1u64;
                                        nat_add(a644, a645)?
                                    };
                                    let a647 = 1u64;
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
    })?;
    let a664 = 0u64;
    Ok(nat_quot(a662, a663, a664))
}

pub fn f12(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f25(v0)?;
    let v3: u64 = {
        let a677 = f26(v0)?;
        let a678 = {
            let a675 = {
                let a665 = 1u64;
                let a666 = 1u64;
                nat_add(a665, a666)?
            };
            let a676 = {
                let a673 = {
                    let a667 = 1u64;
                    let a668 = 1u64;
                    nat_add(a667, a668)?
                };
                let a674 = {
                    let a671 = {
                        let a669 = 1u64;
                        let a670 = 1u64;
                        nat_add(a669, a670)?
                    };
                    let a672 = 1u64;
                    nat_mul(a671, a672)?
                };
                nat_mul(a673, a674)?
            };
            nat_mul(a675, a676)?
        };
        nat_mul(a677, a678)?
    };
    let v4: u64 = f27({
        let a691 = f26(v1)?;
        let a692 = {
            let a689 = {
                let a679 = 1u64;
                let a680 = 1u64;
                nat_add(a679, a680)?
            };
            let a690 = {
                let a687 = {
                    let a681 = 1u64;
                    let a682 = 1u64;
                    nat_add(a681, a682)?
                };
                let a688 = {
                    let a685 = {
                        let a683 = 1u64;
                        let a684 = 1u64;
                        nat_add(a683, a684)?
                    };
                    let a686 = 1u64;
                    nat_mul(a685, a686)?
                };
                nat_mul(a687, a688)?
            };
            nat_mul(a689, a690)?
        };
        nat_mul(a691, a692)?
    }, {
        let a693 = v2;
        let a694 = f25(v1)?;
        nat_sub(a693, a694)
    })?;
    let a695 = f11(v0)?;
    let a696 = f11(v1)?;
    if nat_eq(a695, a696) {
        f28(f11(v0)?, v2, {
            let a697 = v3;
            let a698 = v4;
            nat_add(a697, a698)?
        })
    } else {
        let a699 = v3;
        let a700 = v4;
        if nat_eq(a699, a700) {
            Ok(0u64)
        } else {
            f28(f11(v0)?, v2, {
                let a701 = v3;
                let a702 = v4;
                nat_sub(a701, a702)
            })
        }
    }
}

pub fn f13(v0: u64) -> R<bool> {
    let a703 = f7(v0)?;
    let a704 = 0u64;
    Ok(nat_eq(a703, a704))
}

pub fn f14(v0: u64) -> R<u64> {
    let a771 = f16(v0)?;
    let a772 = {
        let a769 = {
            let a745 = {
                let a743 = {
                    let a705 = 1u64;
                    let a706 = 1u64;
                    nat_add(a705, a706)?
                };
                let a744 = {
                    let a741 = {
                        let a739 = {
                            let a707 = 1u64;
                            let a708 = 1u64;
                            nat_add(a707, a708)?
                        };
                        let a740 = {
                            let a737 = {
                                let a735 = {
                                    let a709 = 1u64;
                                    let a710 = 1u64;
                                    nat_add(a709, a710)?
                                };
                                let a736 = {
                                    let a733 = {
                                        let a731 = {
                                            let a711 = 1u64;
                                            let a712 = 1u64;
                                            nat_add(a711, a712)?
                                        };
                                        let a732 = {
                                            let a729 = {
                                                let a727 = {
                                                    let a713 = 1u64;
                                                    let a714 = 1u64;
                                                    nat_add(a713, a714)?
                                                };
                                                let a728 = {
                                                    let a725 = {
                                                        let a723 = {
                                                            let a715 = 1u64;
                                                            let a716 = 1u64;
                                                            nat_add(a715, a716)?
                                                        };
                                                        let a724 = {
                                                            let a721 = {
                                                                let a719 = {
                                                                    let a717 = 1u64;
                                                                    let a718 = 1u64;
                                                                    nat_add(a717, a718)?
                                                                };
                                                                let a720 = 1u64;
                                                                nat_mul(a719, a720)?
                                                            };
                                                            let a722 = 1u64;
                                                            nat_add(a721, a722)?
                                                        };
                                                        nat_mul(a723, a724)?
                                                    };
                                                    let a726 = 1u64;
                                                    nat_add(a725, a726)?
                                                };
                                                nat_mul(a727, a728)?
                                            };
                                            let a730 = 1u64;
                                            nat_add(a729, a730)?
                                        };
                                        nat_mul(a731, a732)?
                                    };
                                    let a734 = 1u64;
                                    nat_add(a733, a734)?
                                };
                                nat_mul(a735, a736)?
                            };
                            let a738 = 1u64;
                            nat_add(a737, a738)?
                        };
                        nat_mul(a739, a740)?
                    };
                    let a742 = 1u64;
                    nat_add(a741, a742)?
                };
                nat_mul(a743, a744)?
            };
            let a746 = 1u64;
            nat_add(a745, a746)?
        };
        let a770 = f8({
            let a767 = {
                let a765 = {
                    let a747 = 1u64;
                    let a748 = 1u64;
                    nat_add(a747, a748)?
                };
                let a766 = {
                    let a763 = {
                        let a761 = {
                            let a749 = 1u64;
                            let a750 = 1u64;
                            nat_add(a749, a750)?
                        };
                        let a762 = {
                            let a759 = {
                                let a757 = {
                                    let a751 = 1u64;
                                    let a752 = 1u64;
                                    nat_add(a751, a752)?
                                };
                                let a758 = {
                                    let a755 = {
                                        let a753 = 1u64;
                                        let a754 = 1u64;
                                        nat_add(a753, a754)?
                                    };
                                    let a756 = 1u64;
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
        })?;
        nat_mul(a769, a770)?
    };
    nat_add(a771, a772)
}

pub fn f15(v0: u64, v1: u64) -> R<u64> {
    let a779 = {
        let a773 = f11(v0)?;
        let a774 = f11(v1)?;
        nat_add(a773, a774)?
    };
    let a780 = {
        let a777 = {
            let a775 = 1u64;
            let a776 = 1u64;
            nat_add(a775, a776)?
        };
        let a778 = 1u64;
        nat_mul(a777, a778)?
    };
    let a781 = 0u64;
    Ok(nat_rem(a779, a780, a781))
}

pub fn f16(v0: u64) -> R<u64> {
    let a806 = v0;
    let a807 = f8({
        let a804 = {
            let a802 = {
                let a782 = 1u64;
                let a783 = 1u64;
                nat_add(a782, a783)?
            };
            let a803 = {
                let a800 = {
                    let a798 = {
                        let a784 = 1u64;
                        let a785 = 1u64;
                        nat_add(a784, a785)?
                    };
                    let a799 = {
                        let a796 = {
                            let a794 = {
                                let a786 = 1u64;
                                let a787 = 1u64;
                                nat_add(a786, a787)?
                            };
                            let a795 = {
                                let a792 = {
                                    let a790 = {
                                        let a788 = 1u64;
                                        let a789 = 1u64;
                                        nat_add(a788, a789)?
                                    };
                                    let a791 = 1u64;
                                    nat_mul(a790, a791)?
                                };
                                let a793 = 1u64;
                                nat_add(a792, a793)?
                            };
                            nat_mul(a794, a795)?
                        };
                        let a797 = 1u64;
                        nat_add(a796, a797)?
                    };
                    nat_mul(a798, a799)?
                };
                let a801 = 1u64;
                nat_add(a800, a801)?
            };
            nat_mul(a802, a803)?
        };
        let a805 = 1u64;
        nat_add(a804, a805)?
    })?;
    nat_mul(a806, a807)
}

pub fn f17(v0: u64, v1: u64) -> R<u64> {
    let v2: Adt1 = f29(v0)?;
    let v3: Adt1 = f29(v1)?;
    f28(f15(v0, v1)?, {
        let a862 = {
            let a810 = match v2.clone() {
                Adt1::C0(h808, _, _, _) => {
                    h808
                }
            };
            let a811 = match v3.clone() {
                Adt1::C0(h809, _, _, _) => {
                    h809
                }
            };
            nat_add(a810, a811)?
        };
        let a863 = {
            let a860 = {
                let a858 = {
                    let a812 = 1u64;
                    let a813 = 1u64;
                    nat_add(a812, a813)?
                };
                let a859 = {
                    let a856 = {
                        let a854 = {
                            let a814 = 1u64;
                            let a815 = 1u64;
                            nat_add(a814, a815)?
                        };
                        let a855 = {
                            let a852 = {
                                let a850 = {
                                    let a816 = 1u64;
                                    let a817 = 1u64;
                                    nat_add(a816, a817)?
                                };
                                let a851 = {
                                    let a848 = {
                                        let a846 = {
                                            let a818 = 1u64;
                                            let a819 = 1u64;
                                            nat_add(a818, a819)?
                                        };
                                        let a847 = {
                                            let a844 = {
                                                let a842 = {
                                                    let a820 = 1u64;
                                                    let a821 = 1u64;
                                                    nat_add(a820, a821)?
                                                };
                                                let a843 = {
                                                    let a840 = {
                                                        let a838 = {
                                                            let a822 = 1u64;
                                                            let a823 = 1u64;
                                                            nat_add(a822, a823)?
                                                        };
                                                        let a839 = {
                                                            let a836 = {
                                                                let a834 = {
                                                                    let a824 = 1u64;
                                                                    let a825 = 1u64;
                                                                    nat_add(a824, a825)?
                                                                };
                                                                let a835 = {
                                                                    let a832 = {
                                                                        let a826 = 1u64;
                                                                        let a827 = 1u64;
                                                                        nat_add(a826, a827)?
                                                                    };
                                                                    let a833 = {
                                                                        let a830 = {
                                                                            let a828 = 1u64;
                                                                            let a829 = 1u64;
                                                                            nat_add(a828, a829)?
                                                                        };
                                                                        let a831 = 1u64;
                                                                        nat_mul(a830, a831)?
                                                                    };
                                                                    nat_mul(a832, a833)?
                                                                };
                                                                nat_mul(a834, a835)?
                                                            };
                                                            let a837 = 1u64;
                                                            nat_add(a836, a837)?
                                                        };
                                                        nat_mul(a838, a839)?
                                                    };
                                                    let a841 = 1u64;
                                                    nat_add(a840, a841)?
                                                };
                                                nat_mul(a842, a843)?
                                            };
                                            let a845 = 1u64;
                                            nat_add(a844, a845)?
                                        };
                                        nat_mul(a846, a847)?
                                    };
                                    let a849 = 1u64;
                                    nat_add(a848, a849)?
                                };
                                nat_mul(a850, a851)?
                            };
                            let a853 = 1u64;
                            nat_add(a852, a853)?
                        };
                        nat_mul(a854, a855)?
                    };
                    let a857 = 1u64;
                    nat_add(a856, a857)?
                };
                nat_mul(a858, a859)?
            };
            let a861 = 1u64;
            nat_add(a860, a861)?
        };
        nat_sub(a862, a863)
    }, f27({
        let a866 = match v2 {
            Adt1::C0(_, h864, _, _) => {
                h864
            }
        };
        let a867 = match v3 {
            Adt1::C0(_, h865, _, _) => {
                h865
            }
        };
        nat_mul(a866, a867)?
    }, {
        let a884 = {
            let a868 = 1u64;
            let a869 = 1u64;
            nat_add(a868, a869)?
        };
        let a885 = {
            let a882 = {
                let a870 = 1u64;
                let a871 = 1u64;
                nat_add(a870, a871)?
            };
            let a883 = {
                let a880 = {
                    let a878 = {
                        let a872 = 1u64;
                        let a873 = 1u64;
                        nat_add(a872, a873)?
                    };
                    let a879 = {
                        let a876 = {
                            let a874 = 1u64;
                            let a875 = 1u64;
                            nat_add(a874, a875)?
                        };
                        let a877 = 1u64;
                        nat_mul(a876, a877)?
                    };
                    nat_mul(a878, a879)?
                };
                let a881 = 1u64;
                nat_add(a880, a881)?
            };
            nat_mul(a882, a883)?
        };
        nat_mul(a884, a885)?
    })?)
}

pub fn f18(v0: u64, v1: u64) -> R<u64> {
    let v2: Adt1 = f29(v0)?;
    let v3: Adt1 = f29(v1)?;
    let v4: bool = {
        let a888 = match v2.clone() {
            Adt1::C0(_, h886, _, _) => {
                h886
            }
        };
        let a889 = match v3.clone() {
            Adt1::C0(_, h887, _, _) => {
                h887
            }
        };
        nat_lt(a888, a889)
    };
    let v5: u64 = {
        let a933 = match v2.clone() {
            Adt1::C0(_, h890, _, _) => {
                h890
            }
        };
        let a934 = f8(if v4 {
            let a911 = {
                let a909 = {
                    let a891 = 1u64;
                    let a892 = 1u64;
                    nat_add(a891, a892)?
                };
                let a910 = {
                    let a907 = {
                        let a905 = {
                            let a893 = 1u64;
                            let a894 = 1u64;
                            nat_add(a893, a894)?
                        };
                        let a906 = {
                            let a903 = {
                                let a895 = 1u64;
                                let a896 = 1u64;
                                nat_add(a895, a896)?
                            };
                            let a904 = {
                                let a901 = {
                                    let a899 = {
                                        let a897 = 1u64;
                                        let a898 = 1u64;
                                        nat_add(a897, a898)?
                                    };
                                    let a900 = 1u64;
                                    nat_mul(a899, a900)?
                                };
                                let a902 = 1u64;
                                nat_add(a901, a902)?
                            };
                            nat_mul(a903, a904)?
                        };
                        nat_mul(a905, a906)?
                    };
                    let a908 = 1u64;
                    nat_add(a907, a908)?
                };
                nat_mul(a909, a910)?
            };
            let a912 = 1u64;
            nat_add(a911, a912)?
        } else {
            let a931 = {
                let a913 = 1u64;
                let a914 = 1u64;
                nat_add(a913, a914)?
            };
            let a932 = {
                let a929 = {
                    let a927 = {
                        let a915 = 1u64;
                        let a916 = 1u64;
                        nat_add(a915, a916)?
                    };
                    let a928 = {
                        let a925 = {
                            let a917 = 1u64;
                            let a918 = 1u64;
                            nat_add(a917, a918)?
                        };
                        let a926 = {
                            let a923 = {
                                let a921 = {
                                    let a919 = 1u64;
                                    let a920 = 1u64;
                                    nat_add(a919, a920)?
                                };
                                let a922 = 1u64;
                                nat_mul(a921, a922)?
                            };
                            let a924 = 1u64;
                            nat_add(a923, a924)?
                        };
                        nat_mul(a925, a926)?
                    };
                    nat_mul(a927, a928)?
                };
                let a930 = 1u64;
                nat_add(a929, a930)?
            };
            nat_mul(a931, a932)?
        })?;
        nat_mul(a933, a934)?
    };
    let v6: u64 = {
        let a991 = {
            let a989 = {
                let a986 = match v2 {
                    Adt1::C0(h935, _, _, _) => {
                        h935
                    }
                };
                let a987 = {
                    let a984 = {
                        let a982 = {
                            let a936 = 1u64;
                            let a937 = 1u64;
                            nat_add(a936, a937)?
                        };
                        let a983 = {
                            let a980 = {
                                let a978 = {
                                    let a938 = 1u64;
                                    let a939 = 1u64;
                                    nat_add(a938, a939)?
                                };
                                let a979 = {
                                    let a976 = {
                                        let a974 = {
                                            let a940 = 1u64;
                                            let a941 = 1u64;
                                            nat_add(a940, a941)?
                                        };
                                        let a975 = {
                                            let a972 = {
                                                let a970 = {
                                                    let a942 = 1u64;
                                                    let a943 = 1u64;
                                                    nat_add(a942, a943)?
                                                };
                                                let a971 = {
                                                    let a968 = {
                                                        let a966 = {
                                                            let a944 = 1u64;
                                                            let a945 = 1u64;
                                                            nat_add(a944, a945)?
                                                        };
                                                        let a967 = {
                                                            let a964 = {
                                                                let a962 = {
                                                                    let a946 = 1u64;
                                                                    let a947 = 1u64;
                                                                    nat_add(a946, a947)?
                                                                };
                                                                let a963 = {
                                                                    let a960 = {
                                                                        let a958 = {
                                                                            let a948 = 1u64;
                                                                            let a949 = 1u64;
                                                                            nat_add(a948, a949)?
                                                                        };
                                                                        let a959 = {
                                                                            let a956 = {
                                                                                let a950 = 1u64;
                                                                                let a951 = 1u64;
                                                                                nat_add(a950, a951)?
                                                                            };
                                                                            let a957 = {
                                                                                let a954 = {
                                                                                    let a952 = 1u64;
                                                                                    let a953 = 1u64;
                                                                                    nat_add(a952, a953)?
                                                                                };
                                                                                let a955 = 1u64;
                                                                                nat_mul(a954, a955)?
                                                                            };
                                                                            nat_mul(a956, a957)?
                                                                        };
                                                                        nat_mul(a958, a959)?
                                                                    };
                                                                    let a961 = 1u64;
                                                                    nat_add(a960, a961)?
                                                                };
                                                                nat_mul(a962, a963)?
                                                            };
                                                            let a965 = 1u64;
                                                            nat_add(a964, a965)?
                                                        };
                                                        nat_mul(a966, a967)?
                                                    };
                                                    let a969 = 1u64;
                                                    nat_add(a968, a969)?
                                                };
                                                nat_mul(a970, a971)?
                                            };
                                            let a973 = 1u64;
                                            nat_add(a972, a973)?
                                        };
                                        nat_mul(a974, a975)?
                                    };
                                    let a977 = 1u64;
                                    nat_add(a976, a977)?
                                };
                                nat_mul(a978, a979)?
                            };
                            let a981 = 1u64;
                            nat_add(a980, a981)?
                        };
                        nat_mul(a982, a983)?
                    };
                    let a985 = 1u64;
                    nat_add(a984, a985)?
                };
                nat_add(a986, a987)?
            };
            let a990 = match v3.clone() {
                Adt1::C0(h988, _, _, _) => {
                    h988
                }
            };
            nat_sub(a989, a990)
        };
        let a992 = if v4 {
            1u64
        } else {
            0u64
        };
        nat_sub(a991, a992)
    };
    let v7: u64 = {
        let a994 = v5;
        let a995 = match v3.clone() {
            Adt1::C0(_, h993, _, _) => {
                h993
            }
        };
        let a996 = 0u64;
        nat_quot(a994, a995, a996)
    };
    let v8: bool = {
        let a1001 = 0u64;
        let a1002 = {
            let a998 = v5;
            let a999 = match v3 {
                Adt1::C0(_, h997, _, _) => {
                    h997
                }
            };
            let a1000 = 0u64;
            nat_rem(a998, a999, a1000)
        };
        nat_lt(a1001, a1002)
    };
    f28(f15(v0, v1)?, v6, f30(v7, v8)?)
}

pub fn f19(v0: u64) -> R<Option<u64>> {
    if if f9(v0)? {
        true
    } else {
        let a1003 = f11(v0)?;
        let a1004 = 1u64;
        if nat_eq(a1003, a1004) {
            let a1005 = 0u64;
            let a1006 = f7(v0)?;
            nat_lt(a1005, a1006)
        } else {
            false
        }
    } {
        Ok(Some({
            let a1093 = {
                let a1071 = {
                    let a1047 = {
                        let a1045 = {
                            let a1007 = 1u64;
                            let a1008 = 1u64;
                            nat_add(a1007, a1008)?
                        };
                        let a1046 = {
                            let a1043 = {
                                let a1041 = {
                                    let a1009 = 1u64;
                                    let a1010 = 1u64;
                                    nat_add(a1009, a1010)?
                                };
                                let a1042 = {
                                    let a1039 = {
                                        let a1037 = {
                                            let a1011 = 1u64;
                                            let a1012 = 1u64;
                                            nat_add(a1011, a1012)?
                                        };
                                        let a1038 = {
                                            let a1035 = {
                                                let a1033 = {
                                                    let a1013 = 1u64;
                                                    let a1014 = 1u64;
                                                    nat_add(a1013, a1014)?
                                                };
                                                let a1034 = {
                                                    let a1031 = {
                                                        let a1029 = {
                                                            let a1015 = 1u64;
                                                            let a1016 = 1u64;
                                                            nat_add(a1015, a1016)?
                                                        };
                                                        let a1030 = {
                                                            let a1027 = {
                                                                let a1025 = {
                                                                    let a1017 = 1u64;
                                                                    let a1018 = 1u64;
                                                                    nat_add(a1017, a1018)?
                                                                };
                                                                let a1026 = {
                                                                    let a1023 = {
                                                                        let a1021 = {
                                                                            let a1019 = 1u64;
                                                                            let a1020 = 1u64;
                                                                            nat_add(a1019, a1020)?
                                                                        };
                                                                        let a1022 = 1u64;
                                                                        nat_mul(a1021, a1022)?
                                                                    };
                                                                    let a1024 = 1u64;
                                                                    nat_add(a1023, a1024)?
                                                                };
                                                                nat_mul(a1025, a1026)?
                                                            };
                                                            let a1028 = 1u64;
                                                            nat_add(a1027, a1028)?
                                                        };
                                                        nat_mul(a1029, a1030)?
                                                    };
                                                    let a1032 = 1u64;
                                                    nat_add(a1031, a1032)?
                                                };
                                                nat_mul(a1033, a1034)?
                                            };
                                            let a1036 = 1u64;
                                            nat_add(a1035, a1036)?
                                        };
                                        nat_mul(a1037, a1038)?
                                    };
                                    let a1040 = 1u64;
                                    nat_add(a1039, a1040)?
                                };
                                nat_mul(a1041, a1042)?
                            };
                            let a1044 = 1u64;
                            nat_add(a1043, a1044)?
                        };
                        nat_mul(a1045, a1046)?
                    };
                    let a1048 = 1u64;
                    nat_add(a1047, a1048)?
                };
                let a1072 = f8({
                    let a1069 = {
                        let a1067 = {
                            let a1049 = 1u64;
                            let a1050 = 1u64;
                            nat_add(a1049, a1050)?
                        };
                        let a1068 = {
                            let a1065 = {
                                let a1063 = {
                                    let a1051 = 1u64;
                                    let a1052 = 1u64;
                                    nat_add(a1051, a1052)?
                                };
                                let a1064 = {
                                    let a1061 = {
                                        let a1059 = {
                                            let a1053 = 1u64;
                                            let a1054 = 1u64;
                                            nat_add(a1053, a1054)?
                                        };
                                        let a1060 = {
                                            let a1057 = {
                                                let a1055 = 1u64;
                                                let a1056 = 1u64;
                                                nat_add(a1055, a1056)?
                                            };
                                            let a1058 = 1u64;
                                            nat_mul(a1057, a1058)?
                                        };
                                        nat_mul(a1059, a1060)?
                                    };
                                    let a1062 = 1u64;
                                    nat_add(a1061, a1062)?
                                };
                                nat_mul(a1063, a1064)?
                            };
                            let a1066 = 1u64;
                            nat_add(a1065, a1066)?
                        };
                        nat_mul(a1067, a1068)?
                    };
                    let a1070 = 1u64;
                    nat_add(a1069, a1070)?
                })?;
                nat_mul(a1071, a1072)?
            };
            let a1094 = f8({
                let a1091 = {
                    let a1073 = 1u64;
                    let a1074 = 1u64;
                    nat_add(a1073, a1074)?
                };
                let a1092 = {
                    let a1089 = {
                        let a1087 = {
                            let a1075 = 1u64;
                            let a1076 = 1u64;
                            nat_add(a1075, a1076)?
                        };
                        let a1088 = {
                            let a1085 = {
                                let a1083 = {
                                    let a1077 = 1u64;
                                    let a1078 = 1u64;
                                    nat_add(a1077, a1078)?
                                };
                                let a1084 = {
                                    let a1081 = {
                                        let a1079 = 1u64;
                                        let a1080 = 1u64;
                                        nat_add(a1079, a1080)?
                                    };
                                    let a1082 = 1u64;
                                    nat_mul(a1081, a1082)?
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
            })?;
            nat_add(a1093, a1094)?
        }))
    } else {
        if if f13(v0)? {
            true
        } else {
            f10(v0)?
        } {
            Ok(Some(v0))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f20(v0: u64) -> R<u64> {
    let v1: Adt1 = f29(v0)?;
    let v2: u64 = {
        let a1103 = {
            let a1100 = match v1.clone() {
                Adt1::C0(h1095, _, _, _) => {
                    h1095
                }
            };
            let a1101 = {
                let a1098 = {
                    let a1096 = 1u64;
                    let a1097 = 1u64;
                    nat_add(a1096, a1097)?
                };
                let a1099 = 1u64;
                nat_mul(a1098, a1099)?
            };
            let a1102 = 0u64;
            nat_rem(a1100, a1101, a1102)
        };
        let a1104 = 0u64;
        if nat_eq(a1103, a1104) {
            let a1110 = match v1.clone() {
                Adt1::C0(_, h1105, _, _) => {
                    h1105
                }
            };
            let a1111 = {
                let a1108 = {
                    let a1106 = 1u64;
                    let a1107 = 1u64;
                    nat_add(a1106, a1107)?
                };
                let a1109 = 1u64;
                nat_mul(a1108, a1109)?
            };
            nat_mul(a1110, a1111)?
        } else {
            match v1.clone() {
                Adt1::C0(_, h1112, _, _) => {
                    h1112
                }
            }
        }
    };
    let v3: u64 = {
        let a1135 = v2;
        let a1136 = f8({
            let a1133 = {
                let a1131 = {
                    let a1113 = 1u64;
                    let a1114 = 1u64;
                    nat_add(a1113, a1114)?
                };
                let a1132 = {
                    let a1129 = {
                        let a1115 = 1u64;
                        let a1116 = 1u64;
                        nat_add(a1115, a1116)?
                    };
                    let a1130 = {
                        let a1127 = {
                            let a1125 = {
                                let a1117 = 1u64;
                                let a1118 = 1u64;
                                nat_add(a1117, a1118)?
                            };
                            let a1126 = {
                                let a1123 = {
                                    let a1121 = {
                                        let a1119 = 1u64;
                                        let a1120 = 1u64;
                                        nat_add(a1119, a1120)?
                                    };
                                    let a1122 = 1u64;
                                    nat_mul(a1121, a1122)?
                                };
                                let a1124 = 1u64;
                                nat_add(a1123, a1124)?
                            };
                            nat_mul(a1125, a1126)?
                        };
                        let a1128 = 1u64;
                        nat_add(a1127, a1128)?
                    };
                    nat_mul(a1129, a1130)?
                };
                nat_mul(a1131, a1132)?
            };
            let a1134 = 1u64;
            nat_add(a1133, a1134)?
        })?;
        nat_mul(a1135, a1136)?
    };
    let v4: Adt2 = {
        let (h1137, _) = f31(Fn1::F32, {
            let a1156 = {
                let a1138 = 1u64;
                let a1139 = 1u64;
                nat_add(a1138, a1139)?
            };
            let a1157 = {
                let a1154 = {
                    let a1140 = 1u64;
                    let a1141 = 1u64;
                    nat_add(a1140, a1141)?
                };
                let a1155 = {
                    let a1152 = {
                        let a1142 = 1u64;
                        let a1143 = 1u64;
                        nat_add(a1142, a1143)?
                    };
                    let a1153 = {
                        let a1150 = {
                            let a1144 = 1u64;
                            let a1145 = 1u64;
                            nat_add(a1144, a1145)?
                        };
                        let a1151 = {
                            let a1148 = {
                                let a1146 = 1u64;
                                let a1147 = 1u64;
                                nat_add(a1146, a1147)?
                            };
                            let a1149 = 1u64;
                            nat_mul(a1148, a1149)?
                        };
                        nat_mul(a1150, a1151)?
                    };
                    nat_mul(a1152, a1153)?
                };
                nat_mul(a1154, a1155)?
            };
            nat_mul(a1156, a1157)?
        }, Adt2::C0(v3, f8({
            let a1178 = {
                let a1176 = {
                    let a1158 = 1u64;
                    let a1159 = 1u64;
                    nat_add(a1158, a1159)?
                };
                let a1177 = {
                    let a1174 = {
                        let a1172 = {
                            let a1160 = 1u64;
                            let a1161 = 1u64;
                            nat_add(a1160, a1161)?
                        };
                        let a1173 = {
                            let a1170 = {
                                let a1162 = 1u64;
                                let a1163 = 1u64;
                                nat_add(a1162, a1163)?
                            };
                            let a1171 = {
                                let a1168 = {
                                    let a1166 = {
                                        let a1164 = 1u64;
                                        let a1165 = 1u64;
                                        nat_add(a1164, a1165)?
                                    };
                                    let a1167 = 1u64;
                                    nat_mul(a1166, a1167)?
                                };
                                let a1169 = 1u64;
                                nat_add(a1168, a1169)?
                            };
                            nat_mul(a1170, a1171)?
                        };
                        nat_mul(a1172, a1173)?
                    };
                    let a1175 = 1u64;
                    nat_add(a1174, a1175)?
                };
                nat_mul(a1176, a1177)?
            };
            let a1179 = 1u64;
            nat_add(a1178, a1179)?
        })?))?;
        h1137
    };
    let v5: bool = {
        let a1184 = {
            let a1182 = match v4.clone() {
                Adt2::C0(_, h1180) => {
                    h1180
                }
            };
            let a1183 = match v4.clone() {
                Adt2::C0(_, h1181) => {
                    h1181
                }
            };
            nat_mul(a1182, a1183)?
        };
        let a1185 = v3;
        nat_lt(a1184, a1185)
    };
    f33(0u64, {
        let a1243 = {
            let a1237 = match v1 {
                Adt1::C0(h1186, _, _, _) => {
                    h1186
                }
            };
            let a1238 = {
                let a1235 = {
                    let a1233 = {
                        let a1187 = 1u64;
                        let a1188 = 1u64;
                        nat_add(a1187, a1188)?
                    };
                    let a1234 = {
                        let a1231 = {
                            let a1229 = {
                                let a1189 = 1u64;
                                let a1190 = 1u64;
                                nat_add(a1189, a1190)?
                            };
                            let a1230 = {
                                let a1227 = {
                                    let a1225 = {
                                        let a1191 = 1u64;
                                        let a1192 = 1u64;
                                        nat_add(a1191, a1192)?
                                    };
                                    let a1226 = {
                                        let a1223 = {
                                            let a1221 = {
                                                let a1193 = 1u64;
                                                let a1194 = 1u64;
                                                nat_add(a1193, a1194)?
                                            };
                                            let a1222 = {
                                                let a1219 = {
                                                    let a1217 = {
                                                        let a1195 = 1u64;
                                                        let a1196 = 1u64;
                                                        nat_add(a1195, a1196)?
                                                    };
                                                    let a1218 = {
                                                        let a1215 = {
                                                            let a1213 = {
                                                                let a1197 = 1u64;
                                                                let a1198 = 1u64;
                                                                nat_add(a1197, a1198)?
                                                            };
                                                            let a1214 = {
                                                                let a1211 = {
                                                                    let a1209 = {
                                                                        let a1199 = 1u64;
                                                                        let a1200 = 1u64;
                                                                        nat_add(a1199, a1200)?
                                                                    };
                                                                    let a1210 = {
                                                                        let a1207 = {
                                                                            let a1201 = 1u64;
                                                                            let a1202 = 1u64;
                                                                            nat_add(a1201, a1202)?
                                                                        };
                                                                        let a1208 = {
                                                                            let a1205 = {
                                                                                let a1203 = 1u64;
                                                                                let a1204 = 1u64;
                                                                                nat_add(a1203, a1204)?
                                                                            };
                                                                            let a1206 = 1u64;
                                                                            nat_mul(a1205, a1206)?
                                                                        };
                                                                        nat_mul(a1207, a1208)?
                                                                    };
                                                                    nat_mul(a1209, a1210)?
                                                                };
                                                                let a1212 = 1u64;
                                                                nat_add(a1211, a1212)?
                                                            };
                                                            nat_mul(a1213, a1214)?
                                                        };
                                                        let a1216 = 1u64;
                                                        nat_add(a1215, a1216)?
                                                    };
                                                    nat_mul(a1217, a1218)?
                                                };
                                                let a1220 = 1u64;
                                                nat_add(a1219, a1220)?
                                            };
                                            nat_mul(a1221, a1222)?
                                        };
                                        let a1224 = 1u64;
                                        nat_add(a1223, a1224)?
                                    };
                                    nat_mul(a1225, a1226)?
                                };
                                let a1228 = 1u64;
                                nat_add(a1227, a1228)?
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
            };
            nat_add(a1237, a1238)?
        };
        let a1244 = {
            let a1241 = {
                let a1239 = 1u64;
                let a1240 = 1u64;
                nat_add(a1239, a1240)?
            };
            let a1242 = 1u64;
            nat_mul(a1241, a1242)?
        };
        let a1245 = 0u64;
        nat_quot(a1243, a1244, a1245)
    }, f30(match v4 {
        Adt2::C0(_, h1246) => {
            h1246
        }
    }, v5)?)
}

pub fn f21(v0: Fn0, p1250: u64, p1251: u64) -> R<u64> {
    let m1249 = (p1250, p1251);
    {
        let mut m1249 = m1249;
        loop {
            let step = {
                let (v1, v2) = m1249;
                let m1247 = v1;
                if m1247 == 0 {
                    Err::<(u64, u64), R<u64>>(Ok(v2))
                } else {
                    let v3 = m1247 - 1;
                    Ok::<(u64, u64), R<u64>>((v3, {
                        let c1248: Fn0 = v0.clone();
                        c1248.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1249 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f22(v0: u64) -> R<u64> {
    let a1252 = v0;
    let a1253 = v0;
    nat_add(a1252, a1253)
}

pub fn f23(v0: u64) -> R<u64> {
    let a1276 = f7(v0)?;
    let a1277 = f8({
        let a1274 = {
            let a1272 = {
                let a1254 = 1u64;
                let a1255 = 1u64;
                nat_add(a1254, a1255)?
            };
            let a1273 = {
                let a1270 = {
                    let a1268 = {
                        let a1256 = 1u64;
                        let a1257 = 1u64;
                        nat_add(a1256, a1257)?
                    };
                    let a1269 = {
                        let a1266 = {
                            let a1264 = {
                                let a1258 = 1u64;
                                let a1259 = 1u64;
                                nat_add(a1258, a1259)?
                            };
                            let a1265 = {
                                let a1262 = {
                                    let a1260 = 1u64;
                                    let a1261 = 1u64;
                                    nat_add(a1260, a1261)?
                                };
                                let a1263 = 1u64;
                                nat_mul(a1262, a1263)?
                            };
                            nat_mul(a1264, a1265)?
                        };
                        let a1267 = 1u64;
                        nat_add(a1266, a1267)?
                    };
                    nat_mul(a1268, a1269)?
                };
                let a1271 = 1u64;
                nat_add(a1270, a1271)?
            };
            nat_mul(a1272, a1273)?
        };
        let a1275 = 1u64;
        nat_add(a1274, a1275)?
    })?;
    let a1278 = 0u64;
    Ok(nat_quot(a1276, a1277, a1278))
}

pub fn f24(v0: u64) -> R<u64> {
    let a1301 = v0;
    let a1302 = f8({
        let a1299 = {
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
                        let a1292 = 1u64;
                        nat_add(a1291, a1292)?
                    };
                    nat_mul(a1293, a1294)?
                };
                let a1296 = 1u64;
                nat_add(a1295, a1296)?
            };
            nat_mul(a1297, a1298)?
        };
        let a1300 = 1u64;
        nat_add(a1299, a1300)?
    })?;
    let a1303 = 0u64;
    Ok(nat_rem(a1301, a1302, a1303))
}

pub fn f25(v0: u64) -> R<u64> {
    let a1342 = {
        let a1304 = f23(v0)?;
        let a1305 = 0u64;
        if nat_eq(a1304, a1305) {
            1u64
        } else {
            f23(v0)?
        }
    };
    let a1343 = {
        let a1340 = {
            let a1306 = 1u64;
            let a1307 = 1u64;
            nat_add(a1306, a1307)?
        };
        let a1341 = {
            let a1338 = {
                let a1308 = 1u64;
                let a1309 = 1u64;
                nat_add(a1308, a1309)?
            };
            let a1339 = {
                let a1336 = {
                    let a1310 = 1u64;
                    let a1311 = 1u64;
                    nat_add(a1310, a1311)?
                };
                let a1337 = {
                    let a1334 = {
                        let a1312 = 1u64;
                        let a1313 = 1u64;
                        nat_add(a1312, a1313)?
                    };
                    let a1335 = {
                        let a1332 = {
                            let a1314 = 1u64;
                            let a1315 = 1u64;
                            nat_add(a1314, a1315)?
                        };
                        let a1333 = {
                            let a1330 = {
                                let a1316 = 1u64;
                                let a1317 = 1u64;
                                nat_add(a1316, a1317)?
                            };
                            let a1331 = {
                                let a1328 = {
                                    let a1318 = 1u64;
                                    let a1319 = 1u64;
                                    nat_add(a1318, a1319)?
                                };
                                let a1329 = {
                                    let a1326 = {
                                        let a1320 = 1u64;
                                        let a1321 = 1u64;
                                        nat_add(a1320, a1321)?
                                    };
                                    let a1327 = {
                                        let a1324 = {
                                            let a1322 = 1u64;
                                            let a1323 = 1u64;
                                            nat_add(a1322, a1323)?
                                        };
                                        let a1325 = 1u64;
                                        nat_mul(a1324, a1325)?
                                    };
                                    nat_mul(a1326, a1327)?
                                };
                                nat_mul(a1328, a1329)?
                            };
                            nat_mul(a1330, a1331)?
                        };
                        nat_mul(a1332, a1333)?
                    };
                    nat_mul(a1334, a1335)?
                };
                nat_mul(a1336, a1337)?
            };
            nat_mul(a1338, a1339)?
        };
        nat_mul(a1340, a1341)?
    };
    nat_add(a1342, a1343)
}

pub fn f26(v0: u64) -> R<u64> {
    let a1368 = f24(v0)?;
    let a1369 = {
        let a1344 = f23(v0)?;
        let a1345 = 0u64;
        if nat_eq(a1344, a1345) {
            0u64
        } else {
            f8({
                let a1366 = {
                    let a1364 = {
                        let a1346 = 1u64;
                        let a1347 = 1u64;
                        nat_add(a1346, a1347)?
                    };
                    let a1365 = {
                        let a1362 = {
                            let a1360 = {
                                let a1348 = 1u64;
                                let a1349 = 1u64;
                                nat_add(a1348, a1349)?
                            };
                            let a1361 = {
                                let a1358 = {
                                    let a1356 = {
                                        let a1350 = 1u64;
                                        let a1351 = 1u64;
                                        nat_add(a1350, a1351)?
                                    };
                                    let a1357 = {
                                        let a1354 = {
                                            let a1352 = 1u64;
                                            let a1353 = 1u64;
                                            nat_add(a1352, a1353)?
                                        };
                                        let a1355 = 1u64;
                                        nat_mul(a1354, a1355)?
                                    };
                                    nat_mul(a1356, a1357)?
                                };
                                let a1359 = 1u64;
                                nat_add(a1358, a1359)?
                            };
                            nat_mul(a1360, a1361)?
                        };
                        let a1363 = 1u64;
                        nat_add(a1362, a1363)?
                    };
                    nat_mul(a1364, a1365)?
                };
                let a1367 = 1u64;
                nat_add(a1366, a1367)?
            })?
        }
    };
    nat_add(a1368, a1369)
}

pub fn f27(v0: u64, v1: u64) -> R<u64> {
    let a1370 = v1;
    let a1371 = 0u64;
    if nat_eq(a1370, a1371) {
        Ok(v0)
    } else {
        let a1396 = {
            let a1394 = {
                let a1372 = 1u64;
                let a1373 = 1u64;
                nat_add(a1372, a1373)?
            };
            let a1395 = {
                let a1392 = {
                    let a1374 = 1u64;
                    let a1375 = 1u64;
                    nat_add(a1374, a1375)?
                };
                let a1393 = {
                    let a1390 = {
                        let a1376 = 1u64;
                        let a1377 = 1u64;
                        nat_add(a1376, a1377)?
                    };
                    let a1391 = {
                        let a1388 = {
                            let a1378 = 1u64;
                            let a1379 = 1u64;
                            nat_add(a1378, a1379)?
                        };
                        let a1389 = {
                            let a1386 = {
                                let a1380 = 1u64;
                                let a1381 = 1u64;
                                nat_add(a1380, a1381)?
                            };
                            let a1387 = {
                                let a1384 = {
                                    let a1382 = 1u64;
                                    let a1383 = 1u64;
                                    nat_add(a1382, a1383)?
                                };
                                let a1385 = 1u64;
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
            nat_mul(a1394, a1395)?
        };
        let a1397 = v1;
        if nat_le(a1396, a1397) {
            let a1398 = v0;
            let a1399 = 0u64;
            if nat_eq(a1398, a1399) {
                Ok(0u64)
            } else {
                Ok(1u64)
            }
        } else {
            let v2: u64 = f8(v1)?;
            f30({
                let a1400 = v0;
                let a1401 = v2;
                let a1402 = 0u64;
                nat_quot(a1400, a1401, a1402)
            }, {
                let a1406 = 0u64;
                let a1407 = {
                    let a1403 = v0;
                    let a1404 = v2;
                    let a1405 = 0u64;
                    nat_rem(a1403, a1404, a1405)
                };
                nat_lt(a1406, a1407)
            })
        }
    }
}

pub fn f28(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1430 = f8({
        let a1428 = {
            let a1426 = {
                let a1408 = 1u64;
                let a1409 = 1u64;
                nat_add(a1408, a1409)?
            };
            let a1427 = {
                let a1424 = {
                    let a1422 = {
                        let a1410 = 1u64;
                        let a1411 = 1u64;
                        nat_add(a1410, a1411)?
                    };
                    let a1423 = {
                        let a1420 = {
                            let a1412 = 1u64;
                            let a1413 = 1u64;
                            nat_add(a1412, a1413)?
                        };
                        let a1421 = {
                            let a1418 = {
                                let a1416 = {
                                    let a1414 = 1u64;
                                    let a1415 = 1u64;
                                    nat_add(a1414, a1415)?
                                };
                                let a1417 = 1u64;
                                nat_mul(a1416, a1417)?
                            };
                            let a1419 = 1u64;
                            nat_add(a1418, a1419)?
                        };
                        nat_mul(a1420, a1421)?
                    };
                    nat_mul(a1422, a1423)?
                };
                let a1425 = 1u64;
                nat_add(a1424, a1425)?
            };
            nat_mul(a1426, a1427)?
        };
        let a1429 = 1u64;
        nat_add(a1428, a1429)?
    })?;
    let a1431 = v2;
    if nat_le(a1430, a1431) {
        f33(v0, {
            let a1432 = v1;
            let a1433 = 1u64;
            nat_add(a1432, a1433)?
        }, f27(v2, 1u64)?)
    } else {
        let v3: Adt1 = f34(v1, v2, {
            let a1470 = {
                let a1468 = {
                    let a1434 = 1u64;
                    let a1435 = 1u64;
                    nat_add(a1434, a1435)?
                };
                let a1469 = {
                    let a1466 = {
                        let a1436 = 1u64;
                        let a1437 = 1u64;
                        nat_add(a1436, a1437)?
                    };
                    let a1467 = {
                        let a1464 = {
                            let a1438 = 1u64;
                            let a1439 = 1u64;
                            nat_add(a1438, a1439)?
                        };
                        let a1465 = {
                            let a1462 = {
                                let a1440 = 1u64;
                                let a1441 = 1u64;
                                nat_add(a1440, a1441)?
                            };
                            let a1463 = {
                                let a1460 = {
                                    let a1442 = 1u64;
                                    let a1443 = 1u64;
                                    nat_add(a1442, a1443)?
                                };
                                let a1461 = {
                                    let a1458 = {
                                        let a1444 = 1u64;
                                        let a1445 = 1u64;
                                        nat_add(a1444, a1445)?
                                    };
                                    let a1459 = {
                                        let a1456 = {
                                            let a1446 = 1u64;
                                            let a1447 = 1u64;
                                            nat_add(a1446, a1447)?
                                        };
                                        let a1457 = {
                                            let a1454 = {
                                                let a1448 = 1u64;
                                                let a1449 = 1u64;
                                                nat_add(a1448, a1449)?
                                            };
                                            let a1455 = {
                                                let a1452 = {
                                                    let a1450 = 1u64;
                                                    let a1451 = 1u64;
                                                    nat_add(a1450, a1451)?
                                                };
                                                let a1453 = 1u64;
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
                            nat_mul(a1462, a1463)?
                        };
                        nat_mul(a1464, a1465)?
                    };
                    nat_mul(a1466, a1467)?
                };
                nat_mul(a1468, a1469)?
            };
            let a1471 = 1u64;
            nat_add(a1470, a1471)?
        }, f8({
            let a1490 = {
                let a1472 = 1u64;
                let a1473 = 1u64;
                nat_add(a1472, a1473)?
            };
            let a1491 = {
                let a1488 = {
                    let a1486 = {
                        let a1474 = 1u64;
                        let a1475 = 1u64;
                        nat_add(a1474, a1475)?
                    };
                    let a1487 = {
                        let a1484 = {
                            let a1476 = 1u64;
                            let a1477 = 1u64;
                            nat_add(a1476, a1477)?
                        };
                        let a1485 = {
                            let a1482 = {
                                let a1480 = {
                                    let a1478 = 1u64;
                                    let a1479 = 1u64;
                                    nat_add(a1478, a1479)?
                                };
                                let a1481 = 1u64;
                                nat_mul(a1480, a1481)?
                            };
                            let a1483 = 1u64;
                            nat_add(a1482, a1483)?
                        };
                        nat_mul(a1484, a1485)?
                    };
                    nat_mul(a1486, a1487)?
                };
                let a1489 = 1u64;
                nat_add(a1488, a1489)?
            };
            nat_mul(a1490, a1491)?
        })?)?;
        f33(v0, match v3.clone() {
            Adt1::C0(h1492, _, _, _) => {
                h1492
            }
        }, match v3 {
            Adt1::C0(_, h1493, _, _) => {
                h1493
            }
        })
    }
}

pub fn f29(v0: u64) -> R<Adt1> {
    f34(f25(v0)?, f26(v0)?, 0u64, f8({
        let a1514 = {
            let a1512 = {
                let a1494 = 1u64;
                let a1495 = 1u64;
                nat_add(a1494, a1495)?
            };
            let a1513 = {
                let a1510 = {
                    let a1508 = {
                        let a1496 = 1u64;
                        let a1497 = 1u64;
                        nat_add(a1496, a1497)?
                    };
                    let a1509 = {
                        let a1506 = {
                            let a1504 = {
                                let a1498 = 1u64;
                                let a1499 = 1u64;
                                nat_add(a1498, a1499)?
                            };
                            let a1505 = {
                                let a1502 = {
                                    let a1500 = 1u64;
                                    let a1501 = 1u64;
                                    nat_add(a1500, a1501)?
                                };
                                let a1503 = 1u64;
                                nat_mul(a1502, a1503)?
                            };
                            nat_mul(a1504, a1505)?
                        };
                        let a1507 = 1u64;
                        nat_add(a1506, a1507)?
                    };
                    nat_mul(a1508, a1509)?
                };
                let a1511 = 1u64;
                nat_add(a1510, a1511)?
            };
            nat_mul(a1512, a1513)?
        };
        let a1515 = 1u64;
        nat_add(a1514, a1515)?
    })?)
}

pub fn f30(v0: u64, v1: bool) -> R<u64> {
    if if v1 {
        let a1523 = {
            let a1520 = v0;
            let a1521 = {
                let a1518 = {
                    let a1516 = 1u64;
                    let a1517 = 1u64;
                    nat_add(a1516, a1517)?
                };
                let a1519 = 1u64;
                nat_mul(a1518, a1519)?
            };
            let a1522 = 0u64;
            nat_rem(a1520, a1521, a1522)
        };
        let a1524 = 0u64;
        nat_eq(a1523, a1524)
    } else {
        false
    } {
        let a1525 = v0;
        let a1526 = 1u64;
        nat_add(a1525, a1526)
    } else {
        Ok(v0)
    }
}

pub fn f31(v0: Fn1, p1531: u64, p1532: Adt2) -> R<(Adt2, bool)> {
    let m1530 = (p1531, p1532);
    {
        let mut m1530 = m1530;
        loop {
            let step = {
                let (v1, v2) = m1530;
                let m1527 = v1;
                if m1527 == 0 {
                    Err::<(u64, Adt2), R<(Adt2, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1527 - 1;
                    let m1529 = {
                        let c1528: Fn1 = v0.clone();
                        c1528.apply(v2.clone())?
                    };
                    match m1529 {
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
                Ok(next) => { m1530 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f32(v0: Adt2) -> R<Option<Adt2>> {
    f35(v0)
}

pub fn f33(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1571 = v1;
    let a1572 = {
        let a1569 = {
            let a1567 = {
                let a1533 = 1u64;
                let a1534 = 1u64;
                nat_add(a1533, a1534)?
            };
            let a1568 = {
                let a1565 = {
                    let a1535 = 1u64;
                    let a1536 = 1u64;
                    nat_add(a1535, a1536)?
                };
                let a1566 = {
                    let a1563 = {
                        let a1537 = 1u64;
                        let a1538 = 1u64;
                        nat_add(a1537, a1538)?
                    };
                    let a1564 = {
                        let a1561 = {
                            let a1539 = 1u64;
                            let a1540 = 1u64;
                            nat_add(a1539, a1540)?
                        };
                        let a1562 = {
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
                        };
                        nat_mul(a1561, a1562)?
                    };
                    nat_mul(a1563, a1564)?
                };
                nat_mul(a1565, a1566)?
            };
            nat_mul(a1567, a1568)?
        };
        let a1570 = 1u64;
        nat_add(a1569, a1570)?
    };
    if nat_lt(a1571, a1572) {
        f36(v0, {
            let a1609 = {
                let a1607 = {
                    let a1573 = 1u64;
                    let a1574 = 1u64;
                    nat_add(a1573, a1574)?
                };
                let a1608 = {
                    let a1605 = {
                        let a1575 = 1u64;
                        let a1576 = 1u64;
                        nat_add(a1575, a1576)?
                    };
                    let a1606 = {
                        let a1603 = {
                            let a1577 = 1u64;
                            let a1578 = 1u64;
                            nat_add(a1577, a1578)?
                        };
                        let a1604 = {
                            let a1601 = {
                                let a1579 = 1u64;
                                let a1580 = 1u64;
                                nat_add(a1579, a1580)?
                            };
                            let a1602 = {
                                let a1599 = {
                                    let a1581 = 1u64;
                                    let a1582 = 1u64;
                                    nat_add(a1581, a1582)?
                                };
                                let a1600 = {
                                    let a1597 = {
                                        let a1583 = 1u64;
                                        let a1584 = 1u64;
                                        nat_add(a1583, a1584)?
                                    };
                                    let a1598 = {
                                        let a1595 = {
                                            let a1585 = 1u64;
                                            let a1586 = 1u64;
                                            nat_add(a1585, a1586)?
                                        };
                                        let a1596 = {
                                            let a1593 = {
                                                let a1587 = 1u64;
                                                let a1588 = 1u64;
                                                nat_add(a1587, a1588)?
                                            };
                                            let a1594 = {
                                                let a1591 = {
                                                    let a1589 = 1u64;
                                                    let a1590 = 1u64;
                                                    nat_add(a1589, a1590)?
                                                };
                                                let a1592 = 1u64;
                                                nat_mul(a1591, a1592)?
                                            };
                                            nat_mul(a1593, a1594)?
                                        };
                                        nat_mul(a1595, a1596)?
                                    };
                                    nat_mul(a1597, a1598)?
                                };
                                nat_mul(a1599, a1600)?
                            };
                            nat_mul(a1601, a1602)?
                        };
                        nat_mul(a1603, a1604)?
                    };
                    nat_mul(a1605, a1606)?
                };
                nat_mul(a1607, a1608)?
            };
            let a1610 = 1u64;
            nat_add(a1609, a1610)?
        }, f27(v2, {
            let a1649 = {
                let a1647 = {
                    let a1645 = {
                        let a1611 = 1u64;
                        let a1612 = 1u64;
                        nat_add(a1611, a1612)?
                    };
                    let a1646 = {
                        let a1643 = {
                            let a1613 = 1u64;
                            let a1614 = 1u64;
                            nat_add(a1613, a1614)?
                        };
                        let a1644 = {
                            let a1641 = {
                                let a1615 = 1u64;
                                let a1616 = 1u64;
                                nat_add(a1615, a1616)?
                            };
                            let a1642 = {
                                let a1639 = {
                                    let a1617 = 1u64;
                                    let a1618 = 1u64;
                                    nat_add(a1617, a1618)?
                                };
                                let a1640 = {
                                    let a1637 = {
                                        let a1619 = 1u64;
                                        let a1620 = 1u64;
                                        nat_add(a1619, a1620)?
                                    };
                                    let a1638 = {
                                        let a1635 = {
                                            let a1621 = 1u64;
                                            let a1622 = 1u64;
                                            nat_add(a1621, a1622)?
                                        };
                                        let a1636 = {
                                            let a1633 = {
                                                let a1623 = 1u64;
                                                let a1624 = 1u64;
                                                nat_add(a1623, a1624)?
                                            };
                                            let a1634 = {
                                                let a1631 = {
                                                    let a1625 = 1u64;
                                                    let a1626 = 1u64;
                                                    nat_add(a1625, a1626)?
                                                };
                                                let a1632 = {
                                                    let a1629 = {
                                                        let a1627 = 1u64;
                                                        let a1628 = 1u64;
                                                        nat_add(a1627, a1628)?
                                                    };
                                                    let a1630 = 1u64;
                                                    nat_mul(a1629, a1630)?
                                                };
                                                nat_mul(a1631, a1632)?
                                            };
                                            nat_mul(a1633, a1634)?
                                        };
                                        nat_mul(a1635, a1636)?
                                    };
                                    nat_mul(a1637, a1638)?
                                };
                                nat_mul(a1639, a1640)?
                            };
                            nat_mul(a1641, a1642)?
                        };
                        nat_mul(a1643, a1644)?
                    };
                    nat_mul(a1645, a1646)?
                };
                let a1648 = 1u64;
                nat_add(a1647, a1648)?
            };
            let a1650 = v1;
            nat_sub(a1649, a1650)
        })?)
    } else {
        f36(v0, v1, v2)
    }
}

pub fn f34(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt1> {
    let (h1651, _) = f37(Fn2::F38, {
        let a1670 = {
            let a1652 = 1u64;
            let a1653 = 1u64;
            nat_add(a1652, a1653)?
        };
        let a1671 = {
            let a1668 = {
                let a1654 = 1u64;
                let a1655 = 1u64;
                nat_add(a1654, a1655)?
            };
            let a1669 = {
                let a1666 = {
                    let a1656 = 1u64;
                    let a1657 = 1u64;
                    nat_add(a1656, a1657)?
                };
                let a1667 = {
                    let a1664 = {
                        let a1658 = 1u64;
                        let a1659 = 1u64;
                        nat_add(a1658, a1659)?
                    };
                    let a1665 = {
                        let a1662 = {
                            let a1660 = 1u64;
                            let a1661 = 1u64;
                            nat_add(a1660, a1661)?
                        };
                        let a1663 = 1u64;
                        nat_mul(a1662, a1663)?
                    };
                    nat_mul(a1664, a1665)?
                };
                nat_mul(a1666, a1667)?
            };
            nat_mul(a1668, a1669)?
        };
        nat_mul(a1670, a1671)?
    }, Adt1::C0(v0, v1, v2, v3))?;
    Ok(h1651)
}

pub fn f35(v0: Adt2) -> R<Option<Adt2>> {
    let v1: u64 = {
        let a1684 = {
            let a1678 = match v0.clone() {
                Adt2::C0(_, h1672) => {
                    h1672
                }
            };
            let a1679 = {
                let a1675 = match v0.clone() {
                    Adt2::C0(h1673, _) => {
                        h1673
                    }
                };
                let a1676 = match v0.clone() {
                    Adt2::C0(_, h1674) => {
                        h1674
                    }
                };
                let a1677 = 0u64;
                nat_quot(a1675, a1676, a1677)
            };
            nat_add(a1678, a1679)?
        };
        let a1685 = {
            let a1682 = {
                let a1680 = 1u64;
                let a1681 = 1u64;
                nat_add(a1680, a1681)?
            };
            let a1683 = 1u64;
            nat_mul(a1682, a1683)?
        };
        let a1686 = 0u64;
        nat_quot(a1684, a1685, a1686)
    };
    let a1688 = v1;
    let a1689 = match v0.clone() {
        Adt2::C0(_, h1687) => {
            h1687
        }
    };
    if nat_lt(a1688, a1689) {
        Ok(Some(Adt2::C0(match v0 {
            Adt2::C0(h1690, _) => {
                h1690
            }
        }, v1)))
    } else {
        Ok(None::<Adt2>)
    }
}

pub fn f36(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let v3: u64 = {
        let a1703 = v2;
        let a1704 = {
            let a1701 = {
                let a1691 = 1u64;
                let a1692 = 1u64;
                nat_add(a1691, a1692)?
            };
            let a1702 = {
                let a1699 = {
                    let a1693 = 1u64;
                    let a1694 = 1u64;
                    nat_add(a1693, a1694)?
                };
                let a1700 = {
                    let a1697 = {
                        let a1695 = 1u64;
                        let a1696 = 1u64;
                        nat_add(a1695, a1696)?
                    };
                    let a1698 = 1u64;
                    nat_mul(a1697, a1698)?
                };
                nat_mul(a1699, a1700)?
            };
            nat_mul(a1701, a1702)?
        };
        let a1705 = 0u64;
        nat_rem(a1703, a1704, a1705)
    };
    let v4: u64 = {
        let a1718 = v2;
        let a1719 = {
            let a1716 = {
                let a1706 = 1u64;
                let a1707 = 1u64;
                nat_add(a1706, a1707)?
            };
            let a1717 = {
                let a1714 = {
                    let a1708 = 1u64;
                    let a1709 = 1u64;
                    nat_add(a1708, a1709)?
                };
                let a1715 = {
                    let a1712 = {
                        let a1710 = 1u64;
                        let a1711 = 1u64;
                        nat_add(a1710, a1711)?
                    };
                    let a1713 = 1u64;
                    nat_mul(a1712, a1713)?
                };
                nat_mul(a1714, a1715)?
            };
            nat_mul(a1716, a1717)?
        };
        let a1720 = 0u64;
        nat_quot(a1718, a1719, a1720)
    };
    let v5: bool = {
        let a1729 = {
            let a1727 = {
                let a1721 = 1u64;
                let a1722 = 1u64;
                nat_add(a1721, a1722)?
            };
            let a1728 = {
                let a1725 = {
                    let a1723 = 1u64;
                    let a1724 = 1u64;
                    nat_add(a1723, a1724)?
                };
                let a1726 = 1u64;
                nat_mul(a1725, a1726)?
            };
            nat_mul(a1727, a1728)?
        };
        let a1730 = v3;
        if nat_lt(a1729, a1730) {
            true
        } else {
            let a1739 = v3;
            let a1740 = {
                let a1737 = {
                    let a1731 = 1u64;
                    let a1732 = 1u64;
                    nat_add(a1731, a1732)?
                };
                let a1738 = {
                    let a1735 = {
                        let a1733 = 1u64;
                        let a1734 = 1u64;
                        nat_add(a1733, a1734)?
                    };
                    let a1736 = 1u64;
                    nat_mul(a1735, a1736)?
                };
                nat_mul(a1737, a1738)?
            };
            if nat_eq(a1739, a1740) {
                let a1748 = {
                    let a1745 = v4;
                    let a1746 = {
                        let a1743 = {
                            let a1741 = 1u64;
                            let a1742 = 1u64;
                            nat_add(a1741, a1742)?
                        };
                        let a1744 = 1u64;
                        nat_mul(a1743, a1744)?
                    };
                    let a1747 = 0u64;
                    nat_rem(a1745, a1746, a1747)
                };
                let a1749 = 1u64;
                nat_eq(a1748, a1749)
            } else {
                false
            }
        }
    };
    let v6: u64 = {
        let a1750 = v4;
        let a1751 = if v5 {
            1u64
        } else {
            0u64
        };
        nat_add(a1750, a1751)?
    };
    let v7: bool = {
        let a1780 = {
            let a1778 = {
                let a1754 = {
                    let a1752 = 1u64;
                    let a1753 = 1u64;
                    nat_add(a1752, a1753)?
                };
                let a1755 = 1u64;
                nat_mul(a1754, a1755)?
            };
            let a1779 = f8({
                let a1776 = {
                    let a1774 = {
                        let a1756 = 1u64;
                        let a1757 = 1u64;
                        nat_add(a1756, a1757)?
                    };
                    let a1775 = {
                        let a1772 = {
                            let a1770 = {
                                let a1758 = 1u64;
                                let a1759 = 1u64;
                                nat_add(a1758, a1759)?
                            };
                            let a1771 = {
                                let a1768 = {
                                    let a1766 = {
                                        let a1760 = 1u64;
                                        let a1761 = 1u64;
                                        nat_add(a1760, a1761)?
                                    };
                                    let a1767 = {
                                        let a1764 = {
                                            let a1762 = 1u64;
                                            let a1763 = 1u64;
                                            nat_add(a1762, a1763)?
                                        };
                                        let a1765 = 1u64;
                                        nat_mul(a1764, a1765)?
                                    };
                                    nat_mul(a1766, a1767)?
                                };
                                let a1769 = 1u64;
                                nat_add(a1768, a1769)?
                            };
                            nat_mul(a1770, a1771)?
                        };
                        let a1773 = 1u64;
                        nat_add(a1772, a1773)?
                    };
                    nat_mul(a1774, a1775)?
                };
                let a1777 = 1u64;
                nat_add(a1776, a1777)?
            })?;
            nat_mul(a1778, a1779)?
        };
        let a1781 = v6;
        nat_le(a1780, a1781)
    };
    let v8: u64 = if v7 {
        let a1786 = v6;
        let a1787 = {
            let a1784 = {
                let a1782 = 1u64;
                let a1783 = 1u64;
                nat_add(a1782, a1783)?
            };
            let a1785 = 1u64;
            nat_mul(a1784, a1785)?
        };
        let a1788 = 0u64;
        nat_quot(a1786, a1787, a1788)
    } else {
        v6
    };
    let v9: u64 = if v7 {
        let a1789 = v1;
        let a1790 = 1u64;
        nat_add(a1789, a1790)?
    } else {
        v1
    };
    let v10: u64 = {
        let a1829 = v9;
        let a1830 = {
            let a1827 = {
                let a1825 = {
                    let a1791 = 1u64;
                    let a1792 = 1u64;
                    nat_add(a1791, a1792)?
                };
                let a1826 = {
                    let a1823 = {
                        let a1793 = 1u64;
                        let a1794 = 1u64;
                        nat_add(a1793, a1794)?
                    };
                    let a1824 = {
                        let a1821 = {
                            let a1795 = 1u64;
                            let a1796 = 1u64;
                            nat_add(a1795, a1796)?
                        };
                        let a1822 = {
                            let a1819 = {
                                let a1797 = 1u64;
                                let a1798 = 1u64;
                                nat_add(a1797, a1798)?
                            };
                            let a1820 = {
                                let a1817 = {
                                    let a1799 = 1u64;
                                    let a1800 = 1u64;
                                    nat_add(a1799, a1800)?
                                };
                                let a1818 = {
                                    let a1815 = {
                                        let a1801 = 1u64;
                                        let a1802 = 1u64;
                                        nat_add(a1801, a1802)?
                                    };
                                    let a1816 = {
                                        let a1813 = {
                                            let a1803 = 1u64;
                                            let a1804 = 1u64;
                                            nat_add(a1803, a1804)?
                                        };
                                        let a1814 = {
                                            let a1811 = {
                                                let a1805 = 1u64;
                                                let a1806 = 1u64;
                                                nat_add(a1805, a1806)?
                                            };
                                            let a1812 = {
                                                let a1809 = {
                                                    let a1807 = 1u64;
                                                    let a1808 = 1u64;
                                                    nat_add(a1807, a1808)?
                                                };
                                                let a1810 = 1u64;
                                                nat_mul(a1809, a1810)?
                                            };
                                            nat_mul(a1811, a1812)?
                                        };
                                        nat_mul(a1813, a1814)?
                                    };
                                    nat_mul(a1815, a1816)?
                                };
                                nat_mul(a1817, a1818)?
                            };
                            nat_mul(a1819, a1820)?
                        };
                        nat_mul(a1821, a1822)?
                    };
                    nat_mul(a1823, a1824)?
                };
                nat_mul(a1825, a1826)?
            };
            let a1828 = 1u64;
            nat_add(a1827, a1828)?
        };
        if if nat_eq(a1829, a1830) {
            let a1853 = v8;
            let a1854 = f8({
                let a1851 = {
                    let a1849 = {
                        let a1831 = 1u64;
                        let a1832 = 1u64;
                        nat_add(a1831, a1832)?
                    };
                    let a1850 = {
                        let a1847 = {
                            let a1845 = {
                                let a1833 = 1u64;
                                let a1834 = 1u64;
                                nat_add(a1833, a1834)?
                            };
                            let a1846 = {
                                let a1843 = {
                                    let a1841 = {
                                        let a1835 = 1u64;
                                        let a1836 = 1u64;
                                        nat_add(a1835, a1836)?
                                    };
                                    let a1842 = {
                                        let a1839 = {
                                            let a1837 = 1u64;
                                            let a1838 = 1u64;
                                            nat_add(a1837, a1838)?
                                        };
                                        let a1840 = 1u64;
                                        nat_mul(a1839, a1840)?
                                    };
                                    nat_mul(a1841, a1842)?
                                };
                                let a1844 = 1u64;
                                nat_add(a1843, a1844)?
                            };
                            nat_mul(a1845, a1846)?
                        };
                        let a1848 = 1u64;
                        nat_add(a1847, a1848)?
                    };
                    nat_mul(a1849, a1850)?
                };
                let a1852 = 1u64;
                nat_add(a1851, a1852)?
            })?;
            nat_lt(a1853, a1854)
        } else {
            false
        } {
            0u64
        } else {
            let a1891 = v9;
            let a1892 = {
                let a1889 = {
                    let a1855 = 1u64;
                    let a1856 = 1u64;
                    nat_add(a1855, a1856)?
                };
                let a1890 = {
                    let a1887 = {
                        let a1857 = 1u64;
                        let a1858 = 1u64;
                        nat_add(a1857, a1858)?
                    };
                    let a1888 = {
                        let a1885 = {
                            let a1859 = 1u64;
                            let a1860 = 1u64;
                            nat_add(a1859, a1860)?
                        };
                        let a1886 = {
                            let a1883 = {
                                let a1861 = 1u64;
                                let a1862 = 1u64;
                                nat_add(a1861, a1862)?
                            };
                            let a1884 = {
                                let a1881 = {
                                    let a1863 = 1u64;
                                    let a1864 = 1u64;
                                    nat_add(a1863, a1864)?
                                };
                                let a1882 = {
                                    let a1879 = {
                                        let a1865 = 1u64;
                                        let a1866 = 1u64;
                                        nat_add(a1865, a1866)?
                                    };
                                    let a1880 = {
                                        let a1877 = {
                                            let a1867 = 1u64;
                                            let a1868 = 1u64;
                                            nat_add(a1867, a1868)?
                                        };
                                        let a1878 = {
                                            let a1875 = {
                                                let a1869 = 1u64;
                                                let a1870 = 1u64;
                                                nat_add(a1869, a1870)?
                                            };
                                            let a1876 = {
                                                let a1873 = {
                                                    let a1871 = 1u64;
                                                    let a1872 = 1u64;
                                                    nat_add(a1871, a1872)?
                                                };
                                                let a1874 = 1u64;
                                                nat_mul(a1873, a1874)?
                                            };
                                            nat_mul(a1875, a1876)?
                                        };
                                        nat_mul(a1877, a1878)?
                                    };
                                    nat_mul(a1879, a1880)?
                                };
                                nat_mul(a1881, a1882)?
                            };
                            nat_mul(a1883, a1884)?
                        };
                        nat_mul(a1885, a1886)?
                    };
                    nat_mul(a1887, a1888)?
                };
                nat_mul(a1889, a1890)?
            };
            nat_sub(a1891, a1892)
        }
    };
    let a1893 = v8;
    let a1894 = 0u64;
    if nat_eq(a1893, a1894) {
        f16(v0)
    } else {
        let a1947 = {
            let a1945 = {
                let a1943 = {
                    let a1895 = 1u64;
                    let a1896 = 1u64;
                    nat_add(a1895, a1896)?
                };
                let a1944 = {
                    let a1941 = {
                        let a1939 = {
                            let a1897 = 1u64;
                            let a1898 = 1u64;
                            nat_add(a1897, a1898)?
                        };
                        let a1940 = {
                            let a1937 = {
                                let a1935 = {
                                    let a1899 = 1u64;
                                    let a1900 = 1u64;
                                    nat_add(a1899, a1900)?
                                };
                                let a1936 = {
                                    let a1933 = {
                                        let a1931 = {
                                            let a1901 = 1u64;
                                            let a1902 = 1u64;
                                            nat_add(a1901, a1902)?
                                        };
                                        let a1932 = {
                                            let a1929 = {
                                                let a1927 = {
                                                    let a1903 = 1u64;
                                                    let a1904 = 1u64;
                                                    nat_add(a1903, a1904)?
                                                };
                                                let a1928 = {
                                                    let a1925 = {
                                                        let a1923 = {
                                                            let a1905 = 1u64;
                                                            let a1906 = 1u64;
                                                            nat_add(a1905, a1906)?
                                                        };
                                                        let a1924 = {
                                                            let a1921 = {
                                                                let a1919 = {
                                                                    let a1907 = 1u64;
                                                                    let a1908 = 1u64;
                                                                    nat_add(a1907, a1908)?
                                                                };
                                                                let a1920 = {
                                                                    let a1917 = {
                                                                        let a1915 = {
                                                                            let a1909 = 1u64;
                                                                            let a1910 = 1u64;
                                                                            nat_add(a1909, a1910)?
                                                                        };
                                                                        let a1916 = {
                                                                            let a1913 = {
                                                                                let a1911 = 1u64;
                                                                                let a1912 = 1u64;
                                                                                nat_add(a1911, a1912)?
                                                                            };
                                                                            let a1914 = 1u64;
                                                                            nat_mul(a1913, a1914)?
                                                                        };
                                                                        nat_mul(a1915, a1916)?
                                                                    };
                                                                    let a1918 = 1u64;
                                                                    nat_add(a1917, a1918)?
                                                                };
                                                                nat_mul(a1919, a1920)?
                                                            };
                                                            let a1922 = 1u64;
                                                            nat_add(a1921, a1922)?
                                                        };
                                                        nat_mul(a1923, a1924)?
                                                    };
                                                    let a1926 = 1u64;
                                                    nat_add(a1925, a1926)?
                                                };
                                                nat_mul(a1927, a1928)?
                                            };
                                            let a1930 = 1u64;
                                            nat_add(a1929, a1930)?
                                        };
                                        nat_mul(a1931, a1932)?
                                    };
                                    let a1934 = 1u64;
                                    nat_add(a1933, a1934)?
                                };
                                nat_mul(a1935, a1936)?
                            };
                            let a1938 = 1u64;
                            nat_add(a1937, a1938)?
                        };
                        nat_mul(a1939, a1940)?
                    };
                    let a1942 = 1u64;
                    nat_add(a1941, a1942)?
                };
                nat_mul(a1943, a1944)?
            };
            let a1946 = 1u64;
            nat_add(a1945, a1946)?
        };
        let a1948 = v9;
        if nat_le(a1947, a1948) {
            f14(v0)
        } else {
            let a2000 = {
                let a1973 = f16(v0)?;
                let a1974 = {
                    let a1971 = v10;
                    let a1972 = f8({
                        let a1969 = {
                            let a1967 = {
                                let a1949 = 1u64;
                                let a1950 = 1u64;
                                nat_add(a1949, a1950)?
                            };
                            let a1968 = {
                                let a1965 = {
                                    let a1963 = {
                                        let a1951 = 1u64;
                                        let a1952 = 1u64;
                                        nat_add(a1951, a1952)?
                                    };
                                    let a1964 = {
                                        let a1961 = {
                                            let a1959 = {
                                                let a1953 = 1u64;
                                                let a1954 = 1u64;
                                                nat_add(a1953, a1954)?
                                            };
                                            let a1960 = {
                                                let a1957 = {
                                                    let a1955 = 1u64;
                                                    let a1956 = 1u64;
                                                    nat_add(a1955, a1956)?
                                                };
                                                let a1958 = 1u64;
                                                nat_mul(a1957, a1958)?
                                            };
                                            nat_mul(a1959, a1960)?
                                        };
                                        let a1962 = 1u64;
                                        nat_add(a1961, a1962)?
                                    };
                                    nat_mul(a1963, a1964)?
                                };
                                let a1966 = 1u64;
                                nat_add(a1965, a1966)?
                            };
                            nat_mul(a1967, a1968)?
                        };
                        let a1970 = 1u64;
                        nat_add(a1969, a1970)?
                    })?;
                    nat_mul(a1971, a1972)?
                };
                nat_add(a1973, a1974)?
            };
            let a2001 = {
                let a1997 = v8;
                let a1998 = f8({
                    let a1995 = {
                        let a1993 = {
                            let a1975 = 1u64;
                            let a1976 = 1u64;
                            nat_add(a1975, a1976)?
                        };
                        let a1994 = {
                            let a1991 = {
                                let a1989 = {
                                    let a1977 = 1u64;
                                    let a1978 = 1u64;
                                    nat_add(a1977, a1978)?
                                };
                                let a1990 = {
                                    let a1987 = {
                                        let a1985 = {
                                            let a1979 = 1u64;
                                            let a1980 = 1u64;
                                            nat_add(a1979, a1980)?
                                        };
                                        let a1986 = {
                                            let a1983 = {
                                                let a1981 = 1u64;
                                                let a1982 = 1u64;
                                                nat_add(a1981, a1982)?
                                            };
                                            let a1984 = 1u64;
                                            nat_mul(a1983, a1984)?
                                        };
                                        nat_mul(a1985, a1986)?
                                    };
                                    let a1988 = 1u64;
                                    nat_add(a1987, a1988)?
                                };
                                nat_mul(a1989, a1990)?
                            };
                            let a1992 = 1u64;
                            nat_add(a1991, a1992)?
                        };
                        nat_mul(a1993, a1994)?
                    };
                    let a1996 = 1u64;
                    nat_add(a1995, a1996)?
                })?;
                let a1999 = 0u64;
                nat_rem(a1997, a1998, a1999)
            };
            nat_add(a2000, a2001)
        }
    }
}

pub fn f37(v0: Fn2, p2006: u64, p2007: Adt1) -> R<(Adt1, bool)> {
    let m2005 = (p2006, p2007);
    {
        let mut m2005 = m2005;
        loop {
            let step = {
                let (v1, v2) = m2005;
                let m2002 = v1;
                if m2002 == 0 {
                    Err::<(u64, Adt1), R<(Adt1, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2002 - 1;
                    let m2004 = {
                        let c2003: Fn2 = v0.clone();
                        c2003.apply(v2.clone())?
                    };
                    match m2004 {
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
                Ok(next) => { m2005 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f38(v0: Adt1) -> R<Option<Adt1>> {
    f39(v0)
}

pub fn f39(v0: Adt1) -> R<Option<Adt1>> {
    let a2009 = 0u64;
    let a2010 = match v0.clone() {
        Adt1::C0(_, h2008, _, _) => {
            h2008
        }
    };
    if if if nat_lt(a2009, a2010) {
        let a2013 = match v0.clone() {
            Adt1::C0(_, h2011, _, _) => {
                h2011
            }
        };
        let a2014 = match v0.clone() {
            Adt1::C0(_, _, _, h2012) => {
                h2012
            }
        };
        nat_lt(a2013, a2014)
    } else {
        false
    } {
        let a2017 = match v0.clone() {
            Adt1::C0(_, _, h2015, _) => {
                h2015
            }
        };
        let a2018 = match v0.clone() {
            Adt1::C0(h2016, _, _, _) => {
                h2016
            }
        };
        nat_lt(a2017, a2018)
    } else {
        false
    } {
        Ok(Some(Adt1::C0({
            let a2020 = match v0.clone() {
                Adt1::C0(h2019, _, _, _) => {
                    h2019
                }
            };
            let a2021 = 1u64;
            nat_sub(a2020, a2021)
        }, {
            let a2024 = match v0.clone() {
                Adt1::C0(_, h2022, _, _) => {
                    h2022
                }
            };
            let a2025 = match v0.clone() {
                Adt1::C0(_, h2023, _, _) => {
                    h2023
                }
            };
            nat_add(a2024, a2025)?
        }, match v0.clone() {
            Adt1::C0(_, _, h2026, _) => {
                h2026
            }
        }, match v0 {
            Adt1::C0(_, _, _, h2027) => {
                h2027
            }
        })))
    } else {
        Ok(None::<Adt1>)
    }
}

pub fn run(p0: u64, p1: u64, p2: u64) -> R<Option<u64>> {
    f0(p0, p1, p2)
}
