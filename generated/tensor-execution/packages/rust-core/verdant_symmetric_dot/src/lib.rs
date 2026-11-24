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
pub enum Adt1 {
    C0(u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Fn0 {
    F6,
    F7,
}

impl Fn0 {
    pub fn apply(&self, p0: Adt0) -> R<Option<Adt0>> {
        match self {
            Fn0::F6 => f6(p0),
            Fn0::F7 => f7(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F27,
    F41,
}

impl Fn1 {
    pub fn apply(&self, p0: u64) -> R<u64> {
        match self {
            Fn1::F27 => f27(p0),
            Fn1::F41 => f41(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F32,
}

impl Fn2 {
    pub fn apply(&self, p0: Adt1) -> R<Option<Adt1>> {
        match self {
            Fn2::F32 => f32(p0),
        }
    }
}

pub fn f0(v0: u64, v1: bool, v2: u64, v3: u64, v4: u64) -> R<Option<u64>> {
    let a1 = v0;
    let a2 = 4294967296u64;
    if if if if nat_lt(a1, a2) {
        let a3 = v2;
        let a4 = 524288u64;
        nat_le(a3, a4)
    } else {
        false
    } {
        let a5 = v3;
        let a6 = 65536u64;
        nat_lt(a5, a6)
    } else {
        false
    } {
        let a7 = v4;
        let a8 = 65536u64;
        nat_lt(a7, a8)
    } else {
        false
    } {
        let m9 = f1(v2)?;
        match m9 {
            None => {
                Ok(None::<u64>)
            }
            Some(v5) => {
                let v6: u64 = if if v1 {
                    let a10 = 0u64;
                    let a11 = v2;
                    nat_lt(a10, a11)
                } else {
                    false
                } {
                    let a12 = 2147483648u64;
                    let a13 = v5;
                    nat_add(a12, a13)?
                } else {
                    v5
                };
                let v7: u64 = f2(f3(v3)?, f3(v4)?)?;
                Ok(Some(f4(v0, f2(v6, v7)?)?))
            }
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f1(v0: u64) -> R<Option<u64>> {
    let a14 = v0;
    let a15 = 18446744073709551615u64;
    if nat_le(a14, a15) {
        let v1: Adt0 = {
            let (h16, _) = f5(Fn0::F6, 64u64, Adt0::C0(153u64, v0, 1u64, 67108864u64))?;
            h16
        };
        let v2: Adt0 = {
            let (h17, _) = f5(Fn0::F7, 32u64, v1)?;
            h17
        };
        Ok(Some(f8(match v2.clone() {
            Adt0::C0(h18, _, _, _) => {
                h18
            }
        }, f9(match v2 {
            Adt0::C0(_, h19, _, _) => {
                h19
            }
        })?)?))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f2(v0: u64, v1: u64) -> R<u64> {
    if if if if f10(v0)? {
        true
    } else {
        f10(v1)?
    } {
        true
    } else {
        if f11(v0)? {
            f12(v1)?
        } else {
            false
        }
    } {
        true
    } else {
        if f11(v1)? {
            f12(v0)?
        } else {
            false
        }
    } {
        let a106 = {
            let a84 = {
                let a60 = {
                    let a58 = {
                        let a20 = 1u64;
                        let a21 = 1u64;
                        nat_add(a20, a21)?
                    };
                    let a59 = {
                        let a56 = {
                            let a54 = {
                                let a22 = 1u64;
                                let a23 = 1u64;
                                nat_add(a22, a23)?
                            };
                            let a55 = {
                                let a52 = {
                                    let a50 = {
                                        let a24 = 1u64;
                                        let a25 = 1u64;
                                        nat_add(a24, a25)?
                                    };
                                    let a51 = {
                                        let a48 = {
                                            let a46 = {
                                                let a26 = 1u64;
                                                let a27 = 1u64;
                                                nat_add(a26, a27)?
                                            };
                                            let a47 = {
                                                let a44 = {
                                                    let a42 = {
                                                        let a28 = 1u64;
                                                        let a29 = 1u64;
                                                        nat_add(a28, a29)?
                                                    };
                                                    let a43 = {
                                                        let a40 = {
                                                            let a38 = {
                                                                let a30 = 1u64;
                                                                let a31 = 1u64;
                                                                nat_add(a30, a31)?
                                                            };
                                                            let a39 = {
                                                                let a36 = {
                                                                    let a34 = {
                                                                        let a32 = 1u64;
                                                                        let a33 = 1u64;
                                                                        nat_add(a32, a33)?
                                                                    };
                                                                    let a35 = 1u64;
                                                                    nat_mul(a34, a35)?
                                                                };
                                                                let a37 = 1u64;
                                                                nat_add(a36, a37)?
                                                            };
                                                            nat_mul(a38, a39)?
                                                        };
                                                        let a41 = 1u64;
                                                        nat_add(a40, a41)?
                                                    };
                                                    nat_mul(a42, a43)?
                                                };
                                                let a45 = 1u64;
                                                nat_add(a44, a45)?
                                            };
                                            nat_mul(a46, a47)?
                                        };
                                        let a49 = 1u64;
                                        nat_add(a48, a49)?
                                    };
                                    nat_mul(a50, a51)?
                                };
                                let a53 = 1u64;
                                nat_add(a52, a53)?
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
            let a85 = f13({
                let a82 = {
                    let a80 = {
                        let a62 = 1u64;
                        let a63 = 1u64;
                        nat_add(a62, a63)?
                    };
                    let a81 = {
                        let a78 = {
                            let a76 = {
                                let a64 = 1u64;
                                let a65 = 1u64;
                                nat_add(a64, a65)?
                            };
                            let a77 = {
                                let a74 = {
                                    let a72 = {
                                        let a66 = 1u64;
                                        let a67 = 1u64;
                                        nat_add(a66, a67)?
                                    };
                                    let a73 = {
                                        let a70 = {
                                            let a68 = 1u64;
                                            let a69 = 1u64;
                                            nat_add(a68, a69)?
                                        };
                                        let a71 = 1u64;
                                        nat_mul(a70, a71)?
                                    };
                                    nat_mul(a72, a73)?
                                };
                                let a75 = 1u64;
                                nat_add(a74, a75)?
                            };
                            nat_mul(a76, a77)?
                        };
                        let a79 = 1u64;
                        nat_add(a78, a79)?
                    };
                    nat_mul(a80, a81)?
                };
                let a83 = 1u64;
                nat_add(a82, a83)?
            })?;
            nat_mul(a84, a85)?
        };
        let a107 = f13({
            let a104 = {
                let a86 = 1u64;
                let a87 = 1u64;
                nat_add(a86, a87)?
            };
            let a105 = {
                let a102 = {
                    let a100 = {
                        let a88 = 1u64;
                        let a89 = 1u64;
                        nat_add(a88, a89)?
                    };
                    let a101 = {
                        let a98 = {
                            let a96 = {
                                let a90 = 1u64;
                                let a91 = 1u64;
                                nat_add(a90, a91)?
                            };
                            let a97 = {
                                let a94 = {
                                    let a92 = 1u64;
                                    let a93 = 1u64;
                                    nat_add(a92, a93)?
                                };
                                let a95 = 1u64;
                                nat_mul(a94, a95)?
                            };
                            nat_mul(a96, a97)?
                        };
                        let a99 = 1u64;
                        nat_add(a98, a99)?
                    };
                    nat_mul(a100, a101)?
                };
                let a103 = 1u64;
                nat_add(a102, a103)?
            };
            nat_mul(a104, a105)?
        })?;
        nat_add(a106, a107)
    } else {
        if if f11(v0)? {
            true
        } else {
            f11(v1)?
        } {
            f14(f15(v0, v1)?)
        } else {
            if if f12(v0)? {
                true
            } else {
                f12(v1)?
            } {
                f16(f15(v0, v1)?)
            } else {
                f17(v0, v1)
            }
        }
    }
}

pub fn f3(v0: u64) -> R<u64> {
    let v1: u64 = {
        let a111 = {
            let a108 = v0;
            let a109 = 32768u64;
            let a110 = 0u64;
            nat_quot(a108, a109, a110)
        };
        let a112 = 2147483648u64;
        nat_mul(a111, a112)?
    };
    let v2: u64 = {
        let a116 = {
            let a113 = v0;
            let a114 = 32768u64;
            let a115 = 0u64;
            nat_rem(a113, a114, a115)
        };
        let a117 = 1024u64;
        let a118 = 0u64;
        nat_quot(a116, a117, a118)
    };
    let v3: u64 = {
        let a119 = v0;
        let a120 = 1024u64;
        let a121 = 0u64;
        nat_rem(a119, a120, a121)
    };
    let a156 = v1;
    let a157 = {
        let a122 = v2;
        let a123 = 31u64;
        if nat_eq(a122, a123) {
            let a132 = 2139095040u64;
            let a133 = {
                let a130 = {
                    let a124 = 0u64;
                    let a125 = v3;
                    if if nat_lt(a124, a125) {
                        let a126 = v3;
                        let a127 = 512u64;
                        nat_lt(a126, a127)
                    } else {
                        false
                    } {
                        let a128 = v3;
                        let a129 = 512u64;
                        nat_add(a128, a129)?
                    } else {
                        v3
                    }
                };
                let a131 = 8192u64;
                nat_mul(a130, a131)?
            };
            nat_add(a132, a133)?
        } else {
            let a134 = v2;
            let a135 = 0u64;
            if nat_eq(a134, a135) {
                let a136 = v3;
                let a137 = 0u64;
                if nat_eq(a136, a137) {
                    0u64
                } else {
                    let v4: Adt1 = f18(113u64, v3, 0u64, 1024u64)?;
                    let a146 = {
                        let a139 = match v4.clone() {
                            Adt1::C0(h138, _, _, _) => {
                                h138
                            }
                        };
                        let a140 = 8388608u64;
                        nat_mul(a139, a140)?
                    };
                    let a147 = {
                        let a144 = {
                            let a142 = match v4 {
                                Adt1::C0(_, h141, _, _) => {
                                    h141
                                }
                            };
                            let a143 = 1024u64;
                            nat_sub(a142, a143)
                        };
                        let a145 = 8192u64;
                        nat_mul(a144, a145)?
                    };
                    nat_add(a146, a147)?
                }
            } else {
                let a154 = {
                    let a150 = {
                        let a148 = v2;
                        let a149 = 112u64;
                        nat_add(a148, a149)?
                    };
                    let a151 = 8388608u64;
                    nat_mul(a150, a151)?
                };
                let a155 = {
                    let a152 = v3;
                    let a153 = 8192u64;
                    nat_mul(a152, a153)?
                };
                nat_add(a154, a155)?
            }
        }
    };
    nat_add(a156, a157)
}

pub fn f4(v0: u64, v1: u64) -> R<u64> {
    if if f10(v0)? {
        true
    } else {
        f10(v1)?
    } {
        let a244 = {
            let a222 = {
                let a198 = {
                    let a196 = {
                        let a158 = 1u64;
                        let a159 = 1u64;
                        nat_add(a158, a159)?
                    };
                    let a197 = {
                        let a194 = {
                            let a192 = {
                                let a160 = 1u64;
                                let a161 = 1u64;
                                nat_add(a160, a161)?
                            };
                            let a193 = {
                                let a190 = {
                                    let a188 = {
                                        let a162 = 1u64;
                                        let a163 = 1u64;
                                        nat_add(a162, a163)?
                                    };
                                    let a189 = {
                                        let a186 = {
                                            let a184 = {
                                                let a164 = 1u64;
                                                let a165 = 1u64;
                                                nat_add(a164, a165)?
                                            };
                                            let a185 = {
                                                let a182 = {
                                                    let a180 = {
                                                        let a166 = 1u64;
                                                        let a167 = 1u64;
                                                        nat_add(a166, a167)?
                                                    };
                                                    let a181 = {
                                                        let a178 = {
                                                            let a176 = {
                                                                let a168 = 1u64;
                                                                let a169 = 1u64;
                                                                nat_add(a168, a169)?
                                                            };
                                                            let a177 = {
                                                                let a174 = {
                                                                    let a172 = {
                                                                        let a170 = 1u64;
                                                                        let a171 = 1u64;
                                                                        nat_add(a170, a171)?
                                                                    };
                                                                    let a173 = 1u64;
                                                                    nat_mul(a172, a173)?
                                                                };
                                                                let a175 = 1u64;
                                                                nat_add(a174, a175)?
                                                            };
                                                            nat_mul(a176, a177)?
                                                        };
                                                        let a179 = 1u64;
                                                        nat_add(a178, a179)?
                                                    };
                                                    nat_mul(a180, a181)?
                                                };
                                                let a183 = 1u64;
                                                nat_add(a182, a183)?
                                            };
                                            nat_mul(a184, a185)?
                                        };
                                        let a187 = 1u64;
                                        nat_add(a186, a187)?
                                    };
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
            let a223 = f13({
                let a220 = {
                    let a218 = {
                        let a200 = 1u64;
                        let a201 = 1u64;
                        nat_add(a200, a201)?
                    };
                    let a219 = {
                        let a216 = {
                            let a214 = {
                                let a202 = 1u64;
                                let a203 = 1u64;
                                nat_add(a202, a203)?
                            };
                            let a215 = {
                                let a212 = {
                                    let a210 = {
                                        let a204 = 1u64;
                                        let a205 = 1u64;
                                        nat_add(a204, a205)?
                                    };
                                    let a211 = {
                                        let a208 = {
                                            let a206 = 1u64;
                                            let a207 = 1u64;
                                            nat_add(a206, a207)?
                                        };
                                        let a209 = 1u64;
                                        nat_mul(a208, a209)?
                                    };
                                    nat_mul(a210, a211)?
                                };
                                let a213 = 1u64;
                                nat_add(a212, a213)?
                            };
                            nat_mul(a214, a215)?
                        };
                        let a217 = 1u64;
                        nat_add(a216, a217)?
                    };
                    nat_mul(a218, a219)?
                };
                let a221 = 1u64;
                nat_add(a220, a221)?
            })?;
            nat_mul(a222, a223)?
        };
        let a245 = f13({
            let a242 = {
                let a224 = 1u64;
                let a225 = 1u64;
                nat_add(a224, a225)?
            };
            let a243 = {
                let a240 = {
                    let a238 = {
                        let a226 = 1u64;
                        let a227 = 1u64;
                        nat_add(a226, a227)?
                    };
                    let a239 = {
                        let a236 = {
                            let a234 = {
                                let a228 = 1u64;
                                let a229 = 1u64;
                                nat_add(a228, a229)?
                            };
                            let a235 = {
                                let a232 = {
                                    let a230 = 1u64;
                                    let a231 = 1u64;
                                    nat_add(a230, a231)?
                                };
                                let a233 = 1u64;
                                nat_mul(a232, a233)?
                            };
                            nat_mul(a234, a235)?
                        };
                        let a237 = 1u64;
                        nat_add(a236, a237)?
                    };
                    nat_mul(a238, a239)?
                };
                let a241 = 1u64;
                nat_add(a240, a241)?
            };
            nat_mul(a242, a243)?
        })?;
        nat_add(a244, a245)
    } else {
        if if f11(v0)? {
            f11(v1)?
        } else {
            false
        } {
            let a246 = f19(v0)?;
            let a247 = f19(v1)?;
            if nat_eq(a246, a247) {
                Ok(v0)
            } else {
                let a334 = {
                    let a312 = {
                        let a288 = {
                            let a286 = {
                                let a248 = 1u64;
                                let a249 = 1u64;
                                nat_add(a248, a249)?
                            };
                            let a287 = {
                                let a284 = {
                                    let a282 = {
                                        let a250 = 1u64;
                                        let a251 = 1u64;
                                        nat_add(a250, a251)?
                                    };
                                    let a283 = {
                                        let a280 = {
                                            let a278 = {
                                                let a252 = 1u64;
                                                let a253 = 1u64;
                                                nat_add(a252, a253)?
                                            };
                                            let a279 = {
                                                let a276 = {
                                                    let a274 = {
                                                        let a254 = 1u64;
                                                        let a255 = 1u64;
                                                        nat_add(a254, a255)?
                                                    };
                                                    let a275 = {
                                                        let a272 = {
                                                            let a270 = {
                                                                let a256 = 1u64;
                                                                let a257 = 1u64;
                                                                nat_add(a256, a257)?
                                                            };
                                                            let a271 = {
                                                                let a268 = {
                                                                    let a266 = {
                                                                        let a258 = 1u64;
                                                                        let a259 = 1u64;
                                                                        nat_add(a258, a259)?
                                                                    };
                                                                    let a267 = {
                                                                        let a264 = {
                                                                            let a262 = {
                                                                                let a260 = 1u64;
                                                                                let a261 = 1u64;
                                                                                nat_add(a260, a261)?
                                                                            };
                                                                            let a263 = 1u64;
                                                                            nat_mul(a262, a263)?
                                                                        };
                                                                        let a265 = 1u64;
                                                                        nat_add(a264, a265)?
                                                                    };
                                                                    nat_mul(a266, a267)?
                                                                };
                                                                let a269 = 1u64;
                                                                nat_add(a268, a269)?
                                                            };
                                                            nat_mul(a270, a271)?
                                                        };
                                                        let a273 = 1u64;
                                                        nat_add(a272, a273)?
                                                    };
                                                    nat_mul(a274, a275)?
                                                };
                                                let a277 = 1u64;
                                                nat_add(a276, a277)?
                                            };
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
                    let a313 = f13({
                        let a310 = {
                            let a308 = {
                                let a290 = 1u64;
                                let a291 = 1u64;
                                nat_add(a290, a291)?
                            };
                            let a309 = {
                                let a306 = {
                                    let a304 = {
                                        let a292 = 1u64;
                                        let a293 = 1u64;
                                        nat_add(a292, a293)?
                                    };
                                    let a305 = {
                                        let a302 = {
                                            let a300 = {
                                                let a294 = 1u64;
                                                let a295 = 1u64;
                                                nat_add(a294, a295)?
                                            };
                                            let a301 = {
                                                let a298 = {
                                                    let a296 = 1u64;
                                                    let a297 = 1u64;
                                                    nat_add(a296, a297)?
                                                };
                                                let a299 = 1u64;
                                                nat_mul(a298, a299)?
                                            };
                                            nat_mul(a300, a301)?
                                        };
                                        let a303 = 1u64;
                                        nat_add(a302, a303)?
                                    };
                                    nat_mul(a304, a305)?
                                };
                                let a307 = 1u64;
                                nat_add(a306, a307)?
                            };
                            nat_mul(a308, a309)?
                        };
                        let a311 = 1u64;
                        nat_add(a310, a311)?
                    })?;
                    nat_mul(a312, a313)?
                };
                let a335 = f13({
                    let a332 = {
                        let a314 = 1u64;
                        let a315 = 1u64;
                        nat_add(a314, a315)?
                    };
                    let a333 = {
                        let a330 = {
                            let a328 = {
                                let a316 = 1u64;
                                let a317 = 1u64;
                                nat_add(a316, a317)?
                            };
                            let a329 = {
                                let a326 = {
                                    let a324 = {
                                        let a318 = 1u64;
                                        let a319 = 1u64;
                                        nat_add(a318, a319)?
                                    };
                                    let a325 = {
                                        let a322 = {
                                            let a320 = 1u64;
                                            let a321 = 1u64;
                                            nat_add(a320, a321)?
                                        };
                                        let a323 = 1u64;
                                        nat_mul(a322, a323)?
                                    };
                                    nat_mul(a324, a325)?
                                };
                                let a327 = 1u64;
                                nat_add(a326, a327)?
                            };
                            nat_mul(a328, a329)?
                        };
                        let a331 = 1u64;
                        nat_add(a330, a331)?
                    };
                    nat_mul(a332, a333)?
                })?;
                nat_add(a334, a335)
            }
        } else {
            if f11(v0)? {
                Ok(v0)
            } else {
                if f11(v1)? {
                    Ok(v1)
                } else {
                    let a336 = f20(v1)?;
                    let a337 = f20(v0)?;
                    if nat_le(a336, a337) {
                        f21(v0, v1)
                    } else {
                        f21(v1, v0)
                    }
                }
            }
        }
    }
}

pub fn f5(v0: Fn0, p342: u64, p343: Adt0) -> R<(Adt0, bool)> {
    let m341 = (p342, p343);
    {
        let mut m341 = m341;
        loop {
            let step = {
                let (v1, v2) = m341;
                let m338 = v1;
                if m338 == 0 {
                    Err::<(u64, Adt0), R<(Adt0, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m338 - 1;
                    let m340 = {
                        let c339: Fn0 = v0.clone();
                        c339.apply(v2.clone())?
                    };
                    match m340 {
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
                Ok(next) => { m341 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f6(v0: Adt0) -> R<Option<Adt0>> {
    f22(v0)
}

pub fn f7(v0: Adt0) -> R<Option<Adt0>> {
    f23(v0)
}

pub fn f8(v0: u64, v1: u64) -> R<u64> {
    let a344 = v1;
    let a345 = 0u64;
    if nat_eq(a344, a345) {
        Ok(0u64)
    } else {
        let a346 = 16777216u64;
        let a347 = v1;
        if nat_le(a346, a347) {
            let a353 = {
                let a348 = v0;
                let a349 = 8388608u64;
                nat_mul(a348, a349)?
            };
            let a354 = {
                let a350 = v1;
                let a351 = 2u64;
                let a352 = 0u64;
                nat_quot(a350, a351, a352)
            };
            nat_add(a353, a354)
        } else {
            let a359 = {
                let a357 = {
                    let a355 = v0;
                    let a356 = 1u64;
                    nat_sub(a355, a356)
                };
                let a358 = 8388608u64;
                nat_mul(a357, a358)?
            };
            let a360 = v1;
            nat_add(a359, a360)
        }
    }
}

pub fn f9(v0: u64) -> R<u64> {
    let v1: u64 = {
        let a361 = v0;
        let a362 = 8u64;
        let a363 = 0u64;
        nat_quot(a361, a362, a363)
    };
    let v2: u64 = {
        let a364 = v0;
        let a365 = 8u64;
        let a366 = 0u64;
        nat_rem(a364, a365, a366)
    };
    let a376 = v1;
    let a377 = {
        let a367 = 4u64;
        let a368 = v2;
        if if nat_lt(a367, a368) {
            true
        } else {
            let a369 = v2;
            let a370 = 4u64;
            if nat_eq(a369, a370) {
                let a374 = {
                    let a371 = v1;
                    let a372 = 2u64;
                    let a373 = 0u64;
                    nat_rem(a371, a372, a373)
                };
                let a375 = 1u64;
                nat_eq(a374, a375)
            } else {
                false
            }
        } {
            1u64
        } else {
            0u64
        }
    };
    nat_add(a376, a377)
}

pub fn f10(v0: u64) -> R<bool> {
    let a420 = f24(v0)?;
    let a421 = {
        let a418 = {
            let a416 = {
                let a378 = 1u64;
                let a379 = 1u64;
                nat_add(a378, a379)?
            };
            let a417 = {
                let a414 = {
                    let a412 = {
                        let a380 = 1u64;
                        let a381 = 1u64;
                        nat_add(a380, a381)?
                    };
                    let a413 = {
                        let a410 = {
                            let a408 = {
                                let a382 = 1u64;
                                let a383 = 1u64;
                                nat_add(a382, a383)?
                            };
                            let a409 = {
                                let a406 = {
                                    let a404 = {
                                        let a384 = 1u64;
                                        let a385 = 1u64;
                                        nat_add(a384, a385)?
                                    };
                                    let a405 = {
                                        let a402 = {
                                            let a400 = {
                                                let a386 = 1u64;
                                                let a387 = 1u64;
                                                nat_add(a386, a387)?
                                            };
                                            let a401 = {
                                                let a398 = {
                                                    let a396 = {
                                                        let a388 = 1u64;
                                                        let a389 = 1u64;
                                                        nat_add(a388, a389)?
                                                    };
                                                    let a397 = {
                                                        let a394 = {
                                                            let a392 = {
                                                                let a390 = 1u64;
                                                                let a391 = 1u64;
                                                                nat_add(a390, a391)?
                                                            };
                                                            let a393 = 1u64;
                                                            nat_mul(a392, a393)?
                                                        };
                                                        let a395 = 1u64;
                                                        nat_add(a394, a395)?
                                                    };
                                                    nat_mul(a396, a397)?
                                                };
                                                let a399 = 1u64;
                                                nat_add(a398, a399)?
                                            };
                                            nat_mul(a400, a401)?
                                        };
                                        let a403 = 1u64;
                                        nat_add(a402, a403)?
                                    };
                                    nat_mul(a404, a405)?
                                };
                                let a407 = 1u64;
                                nat_add(a406, a407)?
                            };
                            nat_mul(a408, a409)?
                        };
                        let a411 = 1u64;
                        nat_add(a410, a411)?
                    };
                    nat_mul(a412, a413)?
                };
                let a415 = 1u64;
                nat_add(a414, a415)?
            };
            nat_mul(a416, a417)?
        };
        let a419 = 1u64;
        nat_add(a418, a419)?
    };
    if nat_eq(a420, a421) {
        let a422 = 0u64;
        let a423 = f25(v0)?;
        Ok(nat_lt(a422, a423))
    } else {
        Ok(false)
    }
}

pub fn f11(v0: u64) -> R<bool> {
    let a466 = f24(v0)?;
    let a467 = {
        let a464 = {
            let a462 = {
                let a424 = 1u64;
                let a425 = 1u64;
                nat_add(a424, a425)?
            };
            let a463 = {
                let a460 = {
                    let a458 = {
                        let a426 = 1u64;
                        let a427 = 1u64;
                        nat_add(a426, a427)?
                    };
                    let a459 = {
                        let a456 = {
                            let a454 = {
                                let a428 = 1u64;
                                let a429 = 1u64;
                                nat_add(a428, a429)?
                            };
                            let a455 = {
                                let a452 = {
                                    let a450 = {
                                        let a430 = 1u64;
                                        let a431 = 1u64;
                                        nat_add(a430, a431)?
                                    };
                                    let a451 = {
                                        let a448 = {
                                            let a446 = {
                                                let a432 = 1u64;
                                                let a433 = 1u64;
                                                nat_add(a432, a433)?
                                            };
                                            let a447 = {
                                                let a444 = {
                                                    let a442 = {
                                                        let a434 = 1u64;
                                                        let a435 = 1u64;
                                                        nat_add(a434, a435)?
                                                    };
                                                    let a443 = {
                                                        let a440 = {
                                                            let a438 = {
                                                                let a436 = 1u64;
                                                                let a437 = 1u64;
                                                                nat_add(a436, a437)?
                                                            };
                                                            let a439 = 1u64;
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
                                    nat_mul(a450, a451)?
                                };
                                let a453 = 1u64;
                                nat_add(a452, a453)?
                            };
                            nat_mul(a454, a455)?
                        };
                        let a457 = 1u64;
                        nat_add(a456, a457)?
                    };
                    nat_mul(a458, a459)?
                };
                let a461 = 1u64;
                nat_add(a460, a461)?
            };
            nat_mul(a462, a463)?
        };
        let a465 = 1u64;
        nat_add(a464, a465)?
    };
    if nat_eq(a466, a467) {
        let a468 = f25(v0)?;
        let a469 = 0u64;
        Ok(nat_eq(a468, a469))
    } else {
        Ok(false)
    }
}

pub fn f12(v0: u64) -> R<bool> {
    let a470 = f20(v0)?;
    let a471 = 0u64;
    Ok(nat_eq(a470, a471))
}

pub fn f13(v0: u64) -> R<u64> {
    f26(Fn1::F27, v0, 1u64)
}

pub fn f14(v0: u64) -> R<u64> {
    let a538 = f16(v0)?;
    let a539 = {
        let a536 = {
            let a512 = {
                let a510 = {
                    let a472 = 1u64;
                    let a473 = 1u64;
                    nat_add(a472, a473)?
                };
                let a511 = {
                    let a508 = {
                        let a506 = {
                            let a474 = 1u64;
                            let a475 = 1u64;
                            nat_add(a474, a475)?
                        };
                        let a507 = {
                            let a504 = {
                                let a502 = {
                                    let a476 = 1u64;
                                    let a477 = 1u64;
                                    nat_add(a476, a477)?
                                };
                                let a503 = {
                                    let a500 = {
                                        let a498 = {
                                            let a478 = 1u64;
                                            let a479 = 1u64;
                                            nat_add(a478, a479)?
                                        };
                                        let a499 = {
                                            let a496 = {
                                                let a494 = {
                                                    let a480 = 1u64;
                                                    let a481 = 1u64;
                                                    nat_add(a480, a481)?
                                                };
                                                let a495 = {
                                                    let a492 = {
                                                        let a490 = {
                                                            let a482 = 1u64;
                                                            let a483 = 1u64;
                                                            nat_add(a482, a483)?
                                                        };
                                                        let a491 = {
                                                            let a488 = {
                                                                let a486 = {
                                                                    let a484 = 1u64;
                                                                    let a485 = 1u64;
                                                                    nat_add(a484, a485)?
                                                                };
                                                                let a487 = 1u64;
                                                                nat_mul(a486, a487)?
                                                            };
                                                            let a489 = 1u64;
                                                            nat_add(a488, a489)?
                                                        };
                                                        nat_mul(a490, a491)?
                                                    };
                                                    let a493 = 1u64;
                                                    nat_add(a492, a493)?
                                                };
                                                nat_mul(a494, a495)?
                                            };
                                            let a497 = 1u64;
                                            nat_add(a496, a497)?
                                        };
                                        nat_mul(a498, a499)?
                                    };
                                    let a501 = 1u64;
                                    nat_add(a500, a501)?
                                };
                                nat_mul(a502, a503)?
                            };
                            let a505 = 1u64;
                            nat_add(a504, a505)?
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
        let a537 = f13({
            let a534 = {
                let a532 = {
                    let a514 = 1u64;
                    let a515 = 1u64;
                    nat_add(a514, a515)?
                };
                let a533 = {
                    let a530 = {
                        let a528 = {
                            let a516 = 1u64;
                            let a517 = 1u64;
                            nat_add(a516, a517)?
                        };
                        let a529 = {
                            let a526 = {
                                let a524 = {
                                    let a518 = 1u64;
                                    let a519 = 1u64;
                                    nat_add(a518, a519)?
                                };
                                let a525 = {
                                    let a522 = {
                                        let a520 = 1u64;
                                        let a521 = 1u64;
                                        nat_add(a520, a521)?
                                    };
                                    let a523 = 1u64;
                                    nat_mul(a522, a523)?
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
        })?;
        nat_mul(a536, a537)?
    };
    nat_add(a538, a539)
}

pub fn f15(v0: u64, v1: u64) -> R<u64> {
    let a546 = {
        let a540 = f19(v0)?;
        let a541 = f19(v1)?;
        nat_add(a540, a541)?
    };
    let a547 = {
        let a544 = {
            let a542 = 1u64;
            let a543 = 1u64;
            nat_add(a542, a543)?
        };
        let a545 = 1u64;
        nat_mul(a544, a545)?
    };
    let a548 = 0u64;
    Ok(nat_rem(a546, a547, a548))
}

pub fn f16(v0: u64) -> R<u64> {
    let a573 = v0;
    let a574 = f13({
        let a571 = {
            let a569 = {
                let a549 = 1u64;
                let a550 = 1u64;
                nat_add(a549, a550)?
            };
            let a570 = {
                let a567 = {
                    let a565 = {
                        let a551 = 1u64;
                        let a552 = 1u64;
                        nat_add(a551, a552)?
                    };
                    let a566 = {
                        let a563 = {
                            let a561 = {
                                let a553 = 1u64;
                                let a554 = 1u64;
                                nat_add(a553, a554)?
                            };
                            let a562 = {
                                let a559 = {
                                    let a557 = {
                                        let a555 = 1u64;
                                        let a556 = 1u64;
                                        nat_add(a555, a556)?
                                    };
                                    let a558 = 1u64;
                                    nat_mul(a557, a558)?
                                };
                                let a560 = 1u64;
                                nat_add(a559, a560)?
                            };
                            nat_mul(a561, a562)?
                        };
                        let a564 = 1u64;
                        nat_add(a563, a564)?
                    };
                    nat_mul(a565, a566)?
                };
                let a568 = 1u64;
                nat_add(a567, a568)?
            };
            nat_mul(a569, a570)?
        };
        let a572 = 1u64;
        nat_add(a571, a572)?
    })?;
    nat_mul(a573, a574)
}

pub fn f17(v0: u64, v1: u64) -> R<u64> {
    let v2: Adt1 = f28(v0)?;
    let v3: Adt1 = f28(v1)?;
    f29(f15(v0, v1)?, {
        let a629 = {
            let a577 = match v2.clone() {
                Adt1::C0(h575, _, _, _) => {
                    h575
                }
            };
            let a578 = match v3.clone() {
                Adt1::C0(h576, _, _, _) => {
                    h576
                }
            };
            nat_add(a577, a578)?
        };
        let a630 = {
            let a627 = {
                let a625 = {
                    let a579 = 1u64;
                    let a580 = 1u64;
                    nat_add(a579, a580)?
                };
                let a626 = {
                    let a623 = {
                        let a621 = {
                            let a581 = 1u64;
                            let a582 = 1u64;
                            nat_add(a581, a582)?
                        };
                        let a622 = {
                            let a619 = {
                                let a617 = {
                                    let a583 = 1u64;
                                    let a584 = 1u64;
                                    nat_add(a583, a584)?
                                };
                                let a618 = {
                                    let a615 = {
                                        let a613 = {
                                            let a585 = 1u64;
                                            let a586 = 1u64;
                                            nat_add(a585, a586)?
                                        };
                                        let a614 = {
                                            let a611 = {
                                                let a609 = {
                                                    let a587 = 1u64;
                                                    let a588 = 1u64;
                                                    nat_add(a587, a588)?
                                                };
                                                let a610 = {
                                                    let a607 = {
                                                        let a605 = {
                                                            let a589 = 1u64;
                                                            let a590 = 1u64;
                                                            nat_add(a589, a590)?
                                                        };
                                                        let a606 = {
                                                            let a603 = {
                                                                let a601 = {
                                                                    let a591 = 1u64;
                                                                    let a592 = 1u64;
                                                                    nat_add(a591, a592)?
                                                                };
                                                                let a602 = {
                                                                    let a599 = {
                                                                        let a593 = 1u64;
                                                                        let a594 = 1u64;
                                                                        nat_add(a593, a594)?
                                                                    };
                                                                    let a600 = {
                                                                        let a597 = {
                                                                            let a595 = 1u64;
                                                                            let a596 = 1u64;
                                                                            nat_add(a595, a596)?
                                                                        };
                                                                        let a598 = 1u64;
                                                                        nat_mul(a597, a598)?
                                                                    };
                                                                    nat_mul(a599, a600)?
                                                                };
                                                                nat_mul(a601, a602)?
                                                            };
                                                            let a604 = 1u64;
                                                            nat_add(a603, a604)?
                                                        };
                                                        nat_mul(a605, a606)?
                                                    };
                                                    let a608 = 1u64;
                                                    nat_add(a607, a608)?
                                                };
                                                nat_mul(a609, a610)?
                                            };
                                            let a612 = 1u64;
                                            nat_add(a611, a612)?
                                        };
                                        nat_mul(a613, a614)?
                                    };
                                    let a616 = 1u64;
                                    nat_add(a615, a616)?
                                };
                                nat_mul(a617, a618)?
                            };
                            let a620 = 1u64;
                            nat_add(a619, a620)?
                        };
                        nat_mul(a621, a622)?
                    };
                    let a624 = 1u64;
                    nat_add(a623, a624)?
                };
                nat_mul(a625, a626)?
            };
            let a628 = 1u64;
            nat_add(a627, a628)?
        };
        nat_sub(a629, a630)
    }, f30({
        let a633 = match v2 {
            Adt1::C0(_, h631, _, _) => {
                h631
            }
        };
        let a634 = match v3 {
            Adt1::C0(_, h632, _, _) => {
                h632
            }
        };
        nat_mul(a633, a634)?
    }, {
        let a651 = {
            let a635 = 1u64;
            let a636 = 1u64;
            nat_add(a635, a636)?
        };
        let a652 = {
            let a649 = {
                let a637 = 1u64;
                let a638 = 1u64;
                nat_add(a637, a638)?
            };
            let a650 = {
                let a647 = {
                    let a645 = {
                        let a639 = 1u64;
                        let a640 = 1u64;
                        nat_add(a639, a640)?
                    };
                    let a646 = {
                        let a643 = {
                            let a641 = 1u64;
                            let a642 = 1u64;
                            nat_add(a641, a642)?
                        };
                        let a644 = 1u64;
                        nat_mul(a643, a644)?
                    };
                    nat_mul(a645, a646)?
                };
                let a648 = 1u64;
                nat_add(a647, a648)?
            };
            nat_mul(a649, a650)?
        };
        nat_mul(a651, a652)?
    })?)
}

pub fn f18(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt1> {
    let (h653, _) = f31(Fn2::F32, {
        let a672 = {
            let a654 = 1u64;
            let a655 = 1u64;
            nat_add(a654, a655)?
        };
        let a673 = {
            let a670 = {
                let a656 = 1u64;
                let a657 = 1u64;
                nat_add(a656, a657)?
            };
            let a671 = {
                let a668 = {
                    let a658 = 1u64;
                    let a659 = 1u64;
                    nat_add(a658, a659)?
                };
                let a669 = {
                    let a666 = {
                        let a660 = 1u64;
                        let a661 = 1u64;
                        nat_add(a660, a661)?
                    };
                    let a667 = {
                        let a664 = {
                            let a662 = 1u64;
                            let a663 = 1u64;
                            nat_add(a662, a663)?
                        };
                        let a665 = 1u64;
                        nat_mul(a664, a665)?
                    };
                    nat_mul(a666, a667)?
                };
                nat_mul(a668, a669)?
            };
            nat_mul(a670, a671)?
        };
        nat_mul(a672, a673)?
    }, Adt1::C0(v0, v1, v2, v3))?;
    Ok(h653)
}

pub fn f19(v0: u64) -> R<u64> {
    let a698 = v0;
    let a699 = f13({
        let a696 = {
            let a694 = {
                let a674 = 1u64;
                let a675 = 1u64;
                nat_add(a674, a675)?
            };
            let a695 = {
                let a692 = {
                    let a690 = {
                        let a676 = 1u64;
                        let a677 = 1u64;
                        nat_add(a676, a677)?
                    };
                    let a691 = {
                        let a688 = {
                            let a686 = {
                                let a678 = 1u64;
                                let a679 = 1u64;
                                nat_add(a678, a679)?
                            };
                            let a687 = {
                                let a684 = {
                                    let a682 = {
                                        let a680 = 1u64;
                                        let a681 = 1u64;
                                        nat_add(a680, a681)?
                                    };
                                    let a683 = 1u64;
                                    nat_mul(a682, a683)?
                                };
                                let a685 = 1u64;
                                nat_add(a684, a685)?
                            };
                            nat_mul(a686, a687)?
                        };
                        let a689 = 1u64;
                        nat_add(a688, a689)?
                    };
                    nat_mul(a690, a691)?
                };
                let a693 = 1u64;
                nat_add(a692, a693)?
            };
            nat_mul(a694, a695)?
        };
        let a697 = 1u64;
        nat_add(a696, a697)?
    })?;
    let a700 = 0u64;
    Ok(nat_quot(a698, a699, a700))
}

pub fn f20(v0: u64) -> R<u64> {
    let a725 = v0;
    let a726 = f13({
        let a723 = {
            let a721 = {
                let a701 = 1u64;
                let a702 = 1u64;
                nat_add(a701, a702)?
            };
            let a722 = {
                let a719 = {
                    let a717 = {
                        let a703 = 1u64;
                        let a704 = 1u64;
                        nat_add(a703, a704)?
                    };
                    let a718 = {
                        let a715 = {
                            let a713 = {
                                let a705 = 1u64;
                                let a706 = 1u64;
                                nat_add(a705, a706)?
                            };
                            let a714 = {
                                let a711 = {
                                    let a709 = {
                                        let a707 = 1u64;
                                        let a708 = 1u64;
                                        nat_add(a707, a708)?
                                    };
                                    let a710 = 1u64;
                                    nat_mul(a709, a710)?
                                };
                                let a712 = 1u64;
                                nat_add(a711, a712)?
                            };
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
    })?;
    let a727 = 0u64;
    Ok(nat_rem(a725, a726, a727))
}

pub fn f21(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f33(v0)?;
    let v3: u64 = {
        let a740 = f34(v0)?;
        let a741 = {
            let a738 = {
                let a728 = 1u64;
                let a729 = 1u64;
                nat_add(a728, a729)?
            };
            let a739 = {
                let a736 = {
                    let a730 = 1u64;
                    let a731 = 1u64;
                    nat_add(a730, a731)?
                };
                let a737 = {
                    let a734 = {
                        let a732 = 1u64;
                        let a733 = 1u64;
                        nat_add(a732, a733)?
                    };
                    let a735 = 1u64;
                    nat_mul(a734, a735)?
                };
                nat_mul(a736, a737)?
            };
            nat_mul(a738, a739)?
        };
        nat_mul(a740, a741)?
    };
    let v4: u64 = f30({
        let a754 = f34(v1)?;
        let a755 = {
            let a752 = {
                let a742 = 1u64;
                let a743 = 1u64;
                nat_add(a742, a743)?
            };
            let a753 = {
                let a750 = {
                    let a744 = 1u64;
                    let a745 = 1u64;
                    nat_add(a744, a745)?
                };
                let a751 = {
                    let a748 = {
                        let a746 = 1u64;
                        let a747 = 1u64;
                        nat_add(a746, a747)?
                    };
                    let a749 = 1u64;
                    nat_mul(a748, a749)?
                };
                nat_mul(a750, a751)?
            };
            nat_mul(a752, a753)?
        };
        nat_mul(a754, a755)?
    }, {
        let a756 = v2;
        let a757 = f33(v1)?;
        nat_sub(a756, a757)
    })?;
    let a758 = f19(v0)?;
    let a759 = f19(v1)?;
    if nat_eq(a758, a759) {
        f29(f19(v0)?, v2, {
            let a760 = v3;
            let a761 = v4;
            nat_add(a760, a761)?
        })
    } else {
        let a762 = v3;
        let a763 = v4;
        if nat_eq(a762, a763) {
            Ok(0u64)
        } else {
            f29(f19(v0)?, v2, {
                let a764 = v3;
                let a765 = v4;
                nat_sub(a764, a765)
            })
        }
    }
}

pub fn f22(v0: Adt0) -> R<Option<Adt0>> {
    let a767 = 134217728u64;
    let a768 = match v0.clone() {
        Adt0::C0(_, h766, _, _) => {
            h766
        }
    };
    if nat_le(a767, a768) {
        Ok(Some(Adt0::C0({
            let a770 = match v0.clone() {
                Adt0::C0(h769, _, _, _) => {
                    h769
                }
            };
            let a771 = 1u64;
            nat_add(a770, a771)?
        }, f35(match v0.clone() {
            Adt0::C0(_, h772, _, _) => {
                h772
            }
        }, 1u64)?, match v0.clone() {
            Adt0::C0(_, _, h773, _) => {
                h773
            }
        }, match v0 {
            Adt0::C0(_, _, _, h774) => {
                h774
            }
        })))
    } else {
        Ok(None::<Adt0>)
    }
}

pub fn f23(v0: Adt0) -> R<Option<Adt0>> {
    let a776 = 0u64;
    let a777 = match v0.clone() {
        Adt0::C0(_, h775, _, _) => {
            h775
        }
    };
    if if if nat_lt(a776, a777) {
        let a780 = match v0.clone() {
            Adt0::C0(_, h778, _, _) => {
                h778
            }
        };
        let a781 = match v0.clone() {
            Adt0::C0(_, _, _, h779) => {
                h779
            }
        };
        nat_lt(a780, a781)
    } else {
        false
    } {
        let a784 = match v0.clone() {
            Adt0::C0(_, _, h782, _) => {
                h782
            }
        };
        let a785 = match v0.clone() {
            Adt0::C0(h783, _, _, _) => {
                h783
            }
        };
        nat_lt(a784, a785)
    } else {
        false
    } {
        Ok(Some(Adt0::C0({
            let a787 = match v0.clone() {
                Adt0::C0(h786, _, _, _) => {
                    h786
                }
            };
            let a788 = 1u64;
            nat_sub(a787, a788)
        }, {
            let a791 = match v0.clone() {
                Adt0::C0(_, h789, _, _) => {
                    h789
                }
            };
            let a792 = match v0.clone() {
                Adt0::C0(_, h790, _, _) => {
                    h790
                }
            };
            nat_add(a791, a792)?
        }, match v0.clone() {
            Adt0::C0(_, _, h793, _) => {
                h793
            }
        }, match v0 {
            Adt0::C0(_, _, _, h794) => {
                h794
            }
        })))
    } else {
        Ok(None::<Adt0>)
    }
}

pub fn f24(v0: u64) -> R<u64> {
    let a817 = f20(v0)?;
    let a818 = f13({
        let a815 = {
            let a813 = {
                let a795 = 1u64;
                let a796 = 1u64;
                nat_add(a795, a796)?
            };
            let a814 = {
                let a811 = {
                    let a809 = {
                        let a797 = 1u64;
                        let a798 = 1u64;
                        nat_add(a797, a798)?
                    };
                    let a810 = {
                        let a807 = {
                            let a805 = {
                                let a799 = 1u64;
                                let a800 = 1u64;
                                nat_add(a799, a800)?
                            };
                            let a806 = {
                                let a803 = {
                                    let a801 = 1u64;
                                    let a802 = 1u64;
                                    nat_add(a801, a802)?
                                };
                                let a804 = 1u64;
                                nat_mul(a803, a804)?
                            };
                            nat_mul(a805, a806)?
                        };
                        let a808 = 1u64;
                        nat_add(a807, a808)?
                    };
                    nat_mul(a809, a810)?
                };
                let a812 = 1u64;
                nat_add(a811, a812)?
            };
            nat_mul(a813, a814)?
        };
        let a816 = 1u64;
        nat_add(a815, a816)?
    })?;
    let a819 = 0u64;
    Ok(nat_quot(a817, a818, a819))
}

pub fn f25(v0: u64) -> R<u64> {
    let a842 = v0;
    let a843 = f13({
        let a840 = {
            let a838 = {
                let a820 = 1u64;
                let a821 = 1u64;
                nat_add(a820, a821)?
            };
            let a839 = {
                let a836 = {
                    let a834 = {
                        let a822 = 1u64;
                        let a823 = 1u64;
                        nat_add(a822, a823)?
                    };
                    let a835 = {
                        let a832 = {
                            let a830 = {
                                let a824 = 1u64;
                                let a825 = 1u64;
                                nat_add(a824, a825)?
                            };
                            let a831 = {
                                let a828 = {
                                    let a826 = 1u64;
                                    let a827 = 1u64;
                                    nat_add(a826, a827)?
                                };
                                let a829 = 1u64;
                                nat_mul(a828, a829)?
                            };
                            nat_mul(a830, a831)?
                        };
                        let a833 = 1u64;
                        nat_add(a832, a833)?
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
    })?;
    let a844 = 0u64;
    Ok(nat_rem(a842, a843, a844))
}

pub fn f26(v0: Fn1, p848: u64, p849: u64) -> R<u64> {
    let m847 = (p848, p849);
    {
        let mut m847 = m847;
        loop {
            let step = {
                let (v1, v2) = m847;
                let m845 = v1;
                if m845 == 0 {
                    Err::<(u64, u64), R<u64>>(Ok(v2))
                } else {
                    let v3 = m845 - 1;
                    Ok::<(u64, u64), R<u64>>((v3, {
                        let c846: Fn1 = v0.clone();
                        c846.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m847 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f27(v0: u64) -> R<u64> {
    let a850 = v0;
    let a851 = v0;
    nat_add(a850, a851)
}

pub fn f28(v0: u64) -> R<Adt1> {
    f18(f33(v0)?, f34(v0)?, 0u64, f13({
        let a872 = {
            let a870 = {
                let a852 = 1u64;
                let a853 = 1u64;
                nat_add(a852, a853)?
            };
            let a871 = {
                let a868 = {
                    let a866 = {
                        let a854 = 1u64;
                        let a855 = 1u64;
                        nat_add(a854, a855)?
                    };
                    let a867 = {
                        let a864 = {
                            let a862 = {
                                let a856 = 1u64;
                                let a857 = 1u64;
                                nat_add(a856, a857)?
                            };
                            let a863 = {
                                let a860 = {
                                    let a858 = 1u64;
                                    let a859 = 1u64;
                                    nat_add(a858, a859)?
                                };
                                let a861 = 1u64;
                                nat_mul(a860, a861)?
                            };
                            nat_mul(a862, a863)?
                        };
                        let a865 = 1u64;
                        nat_add(a864, a865)?
                    };
                    nat_mul(a866, a867)?
                };
                let a869 = 1u64;
                nat_add(a868, a869)?
            };
            nat_mul(a870, a871)?
        };
        let a873 = 1u64;
        nat_add(a872, a873)?
    })?)
}

pub fn f29(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a896 = f13({
        let a894 = {
            let a892 = {
                let a874 = 1u64;
                let a875 = 1u64;
                nat_add(a874, a875)?
            };
            let a893 = {
                let a890 = {
                    let a888 = {
                        let a876 = 1u64;
                        let a877 = 1u64;
                        nat_add(a876, a877)?
                    };
                    let a889 = {
                        let a886 = {
                            let a878 = 1u64;
                            let a879 = 1u64;
                            nat_add(a878, a879)?
                        };
                        let a887 = {
                            let a884 = {
                                let a882 = {
                                    let a880 = 1u64;
                                    let a881 = 1u64;
                                    nat_add(a880, a881)?
                                };
                                let a883 = 1u64;
                                nat_mul(a882, a883)?
                            };
                            let a885 = 1u64;
                            nat_add(a884, a885)?
                        };
                        nat_mul(a886, a887)?
                    };
                    nat_mul(a888, a889)?
                };
                let a891 = 1u64;
                nat_add(a890, a891)?
            };
            nat_mul(a892, a893)?
        };
        let a895 = 1u64;
        nat_add(a894, a895)?
    })?;
    let a897 = v2;
    if nat_le(a896, a897) {
        f36(v0, {
            let a898 = v1;
            let a899 = 1u64;
            nat_add(a898, a899)?
        }, f30(v2, 1u64)?)
    } else {
        let v3: Adt1 = f18(v1, v2, {
            let a936 = {
                let a934 = {
                    let a900 = 1u64;
                    let a901 = 1u64;
                    nat_add(a900, a901)?
                };
                let a935 = {
                    let a932 = {
                        let a902 = 1u64;
                        let a903 = 1u64;
                        nat_add(a902, a903)?
                    };
                    let a933 = {
                        let a930 = {
                            let a904 = 1u64;
                            let a905 = 1u64;
                            nat_add(a904, a905)?
                        };
                        let a931 = {
                            let a928 = {
                                let a906 = 1u64;
                                let a907 = 1u64;
                                nat_add(a906, a907)?
                            };
                            let a929 = {
                                let a926 = {
                                    let a908 = 1u64;
                                    let a909 = 1u64;
                                    nat_add(a908, a909)?
                                };
                                let a927 = {
                                    let a924 = {
                                        let a910 = 1u64;
                                        let a911 = 1u64;
                                        nat_add(a910, a911)?
                                    };
                                    let a925 = {
                                        let a922 = {
                                            let a912 = 1u64;
                                            let a913 = 1u64;
                                            nat_add(a912, a913)?
                                        };
                                        let a923 = {
                                            let a920 = {
                                                let a914 = 1u64;
                                                let a915 = 1u64;
                                                nat_add(a914, a915)?
                                            };
                                            let a921 = {
                                                let a918 = {
                                                    let a916 = 1u64;
                                                    let a917 = 1u64;
                                                    nat_add(a916, a917)?
                                                };
                                                let a919 = 1u64;
                                                nat_mul(a918, a919)?
                                            };
                                            nat_mul(a920, a921)?
                                        };
                                        nat_mul(a922, a923)?
                                    };
                                    nat_mul(a924, a925)?
                                };
                                nat_mul(a926, a927)?
                            };
                            nat_mul(a928, a929)?
                        };
                        nat_mul(a930, a931)?
                    };
                    nat_mul(a932, a933)?
                };
                nat_mul(a934, a935)?
            };
            let a937 = 1u64;
            nat_add(a936, a937)?
        }, f13({
            let a956 = {
                let a938 = 1u64;
                let a939 = 1u64;
                nat_add(a938, a939)?
            };
            let a957 = {
                let a954 = {
                    let a952 = {
                        let a940 = 1u64;
                        let a941 = 1u64;
                        nat_add(a940, a941)?
                    };
                    let a953 = {
                        let a950 = {
                            let a942 = 1u64;
                            let a943 = 1u64;
                            nat_add(a942, a943)?
                        };
                        let a951 = {
                            let a948 = {
                                let a946 = {
                                    let a944 = 1u64;
                                    let a945 = 1u64;
                                    nat_add(a944, a945)?
                                };
                                let a947 = 1u64;
                                nat_mul(a946, a947)?
                            };
                            let a949 = 1u64;
                            nat_add(a948, a949)?
                        };
                        nat_mul(a950, a951)?
                    };
                    nat_mul(a952, a953)?
                };
                let a955 = 1u64;
                nat_add(a954, a955)?
            };
            nat_mul(a956, a957)?
        })?)?;
        f36(v0, match v3.clone() {
            Adt1::C0(h958, _, _, _) => {
                h958
            }
        }, match v3 {
            Adt1::C0(_, h959, _, _) => {
                h959
            }
        })
    }
}

pub fn f30(v0: u64, v1: u64) -> R<u64> {
    let a960 = v1;
    let a961 = 0u64;
    if nat_eq(a960, a961) {
        Ok(v0)
    } else {
        let a986 = {
            let a984 = {
                let a962 = 1u64;
                let a963 = 1u64;
                nat_add(a962, a963)?
            };
            let a985 = {
                let a982 = {
                    let a964 = 1u64;
                    let a965 = 1u64;
                    nat_add(a964, a965)?
                };
                let a983 = {
                    let a980 = {
                        let a966 = 1u64;
                        let a967 = 1u64;
                        nat_add(a966, a967)?
                    };
                    let a981 = {
                        let a978 = {
                            let a968 = 1u64;
                            let a969 = 1u64;
                            nat_add(a968, a969)?
                        };
                        let a979 = {
                            let a976 = {
                                let a970 = 1u64;
                                let a971 = 1u64;
                                nat_add(a970, a971)?
                            };
                            let a977 = {
                                let a974 = {
                                    let a972 = 1u64;
                                    let a973 = 1u64;
                                    nat_add(a972, a973)?
                                };
                                let a975 = 1u64;
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
        let a987 = v1;
        if nat_le(a986, a987) {
            let a988 = v0;
            let a989 = 0u64;
            if nat_eq(a988, a989) {
                Ok(0u64)
            } else {
                Ok(1u64)
            }
        } else {
            let v2: u64 = f13(v1)?;
            f37({
                let a990 = v0;
                let a991 = v2;
                let a992 = 0u64;
                nat_quot(a990, a991, a992)
            }, {
                let a996 = 0u64;
                let a997 = {
                    let a993 = v0;
                    let a994 = v2;
                    let a995 = 0u64;
                    nat_rem(a993, a994, a995)
                };
                nat_lt(a996, a997)
            })
        }
    }
}

pub fn f31(v0: Fn2, p1002: u64, p1003: Adt1) -> R<(Adt1, bool)> {
    let m1001 = (p1002, p1003);
    {
        let mut m1001 = m1001;
        loop {
            let step = {
                let (v1, v2) = m1001;
                let m998 = v1;
                if m998 == 0 {
                    Err::<(u64, Adt1), R<(Adt1, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m998 - 1;
                    let m1000 = {
                        let c999: Fn2 = v0.clone();
                        c999.apply(v2.clone())?
                    };
                    match m1000 {
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
                Ok(next) => { m1001 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f32(v0: Adt1) -> R<Option<Adt1>> {
    f38(v0)
}

pub fn f33(v0: u64) -> R<u64> {
    let a1042 = {
        let a1004 = f24(v0)?;
        let a1005 = 0u64;
        if nat_eq(a1004, a1005) {
            1u64
        } else {
            f24(v0)?
        }
    };
    let a1043 = {
        let a1040 = {
            let a1006 = 1u64;
            let a1007 = 1u64;
            nat_add(a1006, a1007)?
        };
        let a1041 = {
            let a1038 = {
                let a1008 = 1u64;
                let a1009 = 1u64;
                nat_add(a1008, a1009)?
            };
            let a1039 = {
                let a1036 = {
                    let a1010 = 1u64;
                    let a1011 = 1u64;
                    nat_add(a1010, a1011)?
                };
                let a1037 = {
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
                nat_mul(a1036, a1037)?
            };
            nat_mul(a1038, a1039)?
        };
        nat_mul(a1040, a1041)?
    };
    nat_add(a1042, a1043)
}

pub fn f34(v0: u64) -> R<u64> {
    let a1068 = f25(v0)?;
    let a1069 = {
        let a1044 = f24(v0)?;
        let a1045 = 0u64;
        if nat_eq(a1044, a1045) {
            0u64
        } else {
            f13({
                let a1066 = {
                    let a1064 = {
                        let a1046 = 1u64;
                        let a1047 = 1u64;
                        nat_add(a1046, a1047)?
                    };
                    let a1065 = {
                        let a1062 = {
                            let a1060 = {
                                let a1048 = 1u64;
                                let a1049 = 1u64;
                                nat_add(a1048, a1049)?
                            };
                            let a1061 = {
                                let a1058 = {
                                    let a1056 = {
                                        let a1050 = 1u64;
                                        let a1051 = 1u64;
                                        nat_add(a1050, a1051)?
                                    };
                                    let a1057 = {
                                        let a1054 = {
                                            let a1052 = 1u64;
                                            let a1053 = 1u64;
                                            nat_add(a1052, a1053)?
                                        };
                                        let a1055 = 1u64;
                                        nat_mul(a1054, a1055)?
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
            })?
        }
    };
    nat_add(a1068, a1069)
}

pub fn f35(v0: u64, v1: u64) -> R<u64> {
    let a1070 = v1;
    let a1071 = 0u64;
    if nat_eq(a1070, a1071) {
        Ok(v0)
    } else {
        let a1072 = 64u64;
        let a1073 = v1;
        if nat_le(a1072, a1073) {
            let a1074 = v0;
            let a1075 = 0u64;
            if nat_eq(a1074, a1075) {
                Ok(0u64)
            } else {
                Ok(1u64)
            }
        } else {
            let v2: u64 = f39(v1)?;
            let v3: u64 = {
                let a1076 = v0;
                let a1077 = v2;
                let a1078 = 0u64;
                nat_quot(a1076, a1077, a1078)
            };
            let a1089 = v3;
            let a1090 = {
                let a1082 = 0u64;
                let a1083 = {
                    let a1079 = v0;
                    let a1080 = v2;
                    let a1081 = 0u64;
                    nat_rem(a1079, a1080, a1081)
                };
                if if nat_lt(a1082, a1083) {
                    let a1087 = {
                        let a1084 = v3;
                        let a1085 = 2u64;
                        let a1086 = 0u64;
                        nat_rem(a1084, a1085, a1086)
                    };
                    let a1088 = 0u64;
                    nat_eq(a1087, a1088)
                } else {
                    false
                } {
                    1u64
                } else {
                    0u64
                }
            };
            nat_add(a1089, a1090)
        }
    }
}

pub fn f36(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1129 = v1;
    let a1130 = {
        let a1127 = {
            let a1125 = {
                let a1091 = 1u64;
                let a1092 = 1u64;
                nat_add(a1091, a1092)?
            };
            let a1126 = {
                let a1123 = {
                    let a1093 = 1u64;
                    let a1094 = 1u64;
                    nat_add(a1093, a1094)?
                };
                let a1124 = {
                    let a1121 = {
                        let a1095 = 1u64;
                        let a1096 = 1u64;
                        nat_add(a1095, a1096)?
                    };
                    let a1122 = {
                        let a1119 = {
                            let a1097 = 1u64;
                            let a1098 = 1u64;
                            nat_add(a1097, a1098)?
                        };
                        let a1120 = {
                            let a1117 = {
                                let a1099 = 1u64;
                                let a1100 = 1u64;
                                nat_add(a1099, a1100)?
                            };
                            let a1118 = {
                                let a1115 = {
                                    let a1101 = 1u64;
                                    let a1102 = 1u64;
                                    nat_add(a1101, a1102)?
                                };
                                let a1116 = {
                                    let a1113 = {
                                        let a1103 = 1u64;
                                        let a1104 = 1u64;
                                        nat_add(a1103, a1104)?
                                    };
                                    let a1114 = {
                                        let a1111 = {
                                            let a1105 = 1u64;
                                            let a1106 = 1u64;
                                            nat_add(a1105, a1106)?
                                        };
                                        let a1112 = {
                                            let a1109 = {
                                                let a1107 = 1u64;
                                                let a1108 = 1u64;
                                                nat_add(a1107, a1108)?
                                            };
                                            let a1110 = 1u64;
                                            nat_mul(a1109, a1110)?
                                        };
                                        nat_mul(a1111, a1112)?
                                    };
                                    nat_mul(a1113, a1114)?
                                };
                                nat_mul(a1115, a1116)?
                            };
                            nat_mul(a1117, a1118)?
                        };
                        nat_mul(a1119, a1120)?
                    };
                    nat_mul(a1121, a1122)?
                };
                nat_mul(a1123, a1124)?
            };
            nat_mul(a1125, a1126)?
        };
        let a1128 = 1u64;
        nat_add(a1127, a1128)?
    };
    if nat_lt(a1129, a1130) {
        f40(v0, {
            let a1167 = {
                let a1165 = {
                    let a1131 = 1u64;
                    let a1132 = 1u64;
                    nat_add(a1131, a1132)?
                };
                let a1166 = {
                    let a1163 = {
                        let a1133 = 1u64;
                        let a1134 = 1u64;
                        nat_add(a1133, a1134)?
                    };
                    let a1164 = {
                        let a1161 = {
                            let a1135 = 1u64;
                            let a1136 = 1u64;
                            nat_add(a1135, a1136)?
                        };
                        let a1162 = {
                            let a1159 = {
                                let a1137 = 1u64;
                                let a1138 = 1u64;
                                nat_add(a1137, a1138)?
                            };
                            let a1160 = {
                                let a1157 = {
                                    let a1139 = 1u64;
                                    let a1140 = 1u64;
                                    nat_add(a1139, a1140)?
                                };
                                let a1158 = {
                                    let a1155 = {
                                        let a1141 = 1u64;
                                        let a1142 = 1u64;
                                        nat_add(a1141, a1142)?
                                    };
                                    let a1156 = {
                                        let a1153 = {
                                            let a1143 = 1u64;
                                            let a1144 = 1u64;
                                            nat_add(a1143, a1144)?
                                        };
                                        let a1154 = {
                                            let a1151 = {
                                                let a1145 = 1u64;
                                                let a1146 = 1u64;
                                                nat_add(a1145, a1146)?
                                            };
                                            let a1152 = {
                                                let a1149 = {
                                                    let a1147 = 1u64;
                                                    let a1148 = 1u64;
                                                    nat_add(a1147, a1148)?
                                                };
                                                let a1150 = 1u64;
                                                nat_mul(a1149, a1150)?
                                            };
                                            nat_mul(a1151, a1152)?
                                        };
                                        nat_mul(a1153, a1154)?
                                    };
                                    nat_mul(a1155, a1156)?
                                };
                                nat_mul(a1157, a1158)?
                            };
                            nat_mul(a1159, a1160)?
                        };
                        nat_mul(a1161, a1162)?
                    };
                    nat_mul(a1163, a1164)?
                };
                nat_mul(a1165, a1166)?
            };
            let a1168 = 1u64;
            nat_add(a1167, a1168)?
        }, f30(v2, {
            let a1207 = {
                let a1205 = {
                    let a1203 = {
                        let a1169 = 1u64;
                        let a1170 = 1u64;
                        nat_add(a1169, a1170)?
                    };
                    let a1204 = {
                        let a1201 = {
                            let a1171 = 1u64;
                            let a1172 = 1u64;
                            nat_add(a1171, a1172)?
                        };
                        let a1202 = {
                            let a1199 = {
                                let a1173 = 1u64;
                                let a1174 = 1u64;
                                nat_add(a1173, a1174)?
                            };
                            let a1200 = {
                                let a1197 = {
                                    let a1175 = 1u64;
                                    let a1176 = 1u64;
                                    nat_add(a1175, a1176)?
                                };
                                let a1198 = {
                                    let a1195 = {
                                        let a1177 = 1u64;
                                        let a1178 = 1u64;
                                        nat_add(a1177, a1178)?
                                    };
                                    let a1196 = {
                                        let a1193 = {
                                            let a1179 = 1u64;
                                            let a1180 = 1u64;
                                            nat_add(a1179, a1180)?
                                        };
                                        let a1194 = {
                                            let a1191 = {
                                                let a1181 = 1u64;
                                                let a1182 = 1u64;
                                                nat_add(a1181, a1182)?
                                            };
                                            let a1192 = {
                                                let a1189 = {
                                                    let a1183 = 1u64;
                                                    let a1184 = 1u64;
                                                    nat_add(a1183, a1184)?
                                                };
                                                let a1190 = {
                                                    let a1187 = {
                                                        let a1185 = 1u64;
                                                        let a1186 = 1u64;
                                                        nat_add(a1185, a1186)?
                                                    };
                                                    let a1188 = 1u64;
                                                    nat_mul(a1187, a1188)?
                                                };
                                                nat_mul(a1189, a1190)?
                                            };
                                            nat_mul(a1191, a1192)?
                                        };
                                        nat_mul(a1193, a1194)?
                                    };
                                    nat_mul(a1195, a1196)?
                                };
                                nat_mul(a1197, a1198)?
                            };
                            nat_mul(a1199, a1200)?
                        };
                        nat_mul(a1201, a1202)?
                    };
                    nat_mul(a1203, a1204)?
                };
                let a1206 = 1u64;
                nat_add(a1205, a1206)?
            };
            let a1208 = v1;
            nat_sub(a1207, a1208)
        })?)
    } else {
        f40(v0, v1, v2)
    }
}

pub fn f37(v0: u64, v1: bool) -> R<u64> {
    if if v1 {
        let a1216 = {
            let a1213 = v0;
            let a1214 = {
                let a1211 = {
                    let a1209 = 1u64;
                    let a1210 = 1u64;
                    nat_add(a1209, a1210)?
                };
                let a1212 = 1u64;
                nat_mul(a1211, a1212)?
            };
            let a1215 = 0u64;
            nat_rem(a1213, a1214, a1215)
        };
        let a1217 = 0u64;
        nat_eq(a1216, a1217)
    } else {
        false
    } {
        let a1218 = v0;
        let a1219 = 1u64;
        nat_add(a1218, a1219)
    } else {
        Ok(v0)
    }
}

pub fn f38(v0: Adt1) -> R<Option<Adt1>> {
    let a1221 = 0u64;
    let a1222 = match v0.clone() {
        Adt1::C0(_, h1220, _, _) => {
            h1220
        }
    };
    if if if nat_lt(a1221, a1222) {
        let a1225 = match v0.clone() {
            Adt1::C0(_, h1223, _, _) => {
                h1223
            }
        };
        let a1226 = match v0.clone() {
            Adt1::C0(_, _, _, h1224) => {
                h1224
            }
        };
        nat_lt(a1225, a1226)
    } else {
        false
    } {
        let a1229 = match v0.clone() {
            Adt1::C0(_, _, h1227, _) => {
                h1227
            }
        };
        let a1230 = match v0.clone() {
            Adt1::C0(h1228, _, _, _) => {
                h1228
            }
        };
        nat_lt(a1229, a1230)
    } else {
        false
    } {
        Ok(Some(Adt1::C0({
            let a1232 = match v0.clone() {
                Adt1::C0(h1231, _, _, _) => {
                    h1231
                }
            };
            let a1233 = 1u64;
            nat_sub(a1232, a1233)
        }, {
            let a1236 = match v0.clone() {
                Adt1::C0(_, h1234, _, _) => {
                    h1234
                }
            };
            let a1237 = match v0.clone() {
                Adt1::C0(_, h1235, _, _) => {
                    h1235
                }
            };
            nat_add(a1236, a1237)?
        }, match v0.clone() {
            Adt1::C0(_, _, h1238, _) => {
                h1238
            }
        }, match v0 {
            Adt1::C0(_, _, _, h1239) => {
                h1239
            }
        })))
    } else {
        Ok(None::<Adt1>)
    }
}

pub fn f39(v0: u64) -> R<u64> {
    f26(Fn1::F41, v0, 1u64)
}

pub fn f40(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let v3: u64 = {
        let a1252 = v2;
        let a1253 = {
            let a1250 = {
                let a1240 = 1u64;
                let a1241 = 1u64;
                nat_add(a1240, a1241)?
            };
            let a1251 = {
                let a1248 = {
                    let a1242 = 1u64;
                    let a1243 = 1u64;
                    nat_add(a1242, a1243)?
                };
                let a1249 = {
                    let a1246 = {
                        let a1244 = 1u64;
                        let a1245 = 1u64;
                        nat_add(a1244, a1245)?
                    };
                    let a1247 = 1u64;
                    nat_mul(a1246, a1247)?
                };
                nat_mul(a1248, a1249)?
            };
            nat_mul(a1250, a1251)?
        };
        let a1254 = 0u64;
        nat_rem(a1252, a1253, a1254)
    };
    let v4: u64 = {
        let a1267 = v2;
        let a1268 = {
            let a1265 = {
                let a1255 = 1u64;
                let a1256 = 1u64;
                nat_add(a1255, a1256)?
            };
            let a1266 = {
                let a1263 = {
                    let a1257 = 1u64;
                    let a1258 = 1u64;
                    nat_add(a1257, a1258)?
                };
                let a1264 = {
                    let a1261 = {
                        let a1259 = 1u64;
                        let a1260 = 1u64;
                        nat_add(a1259, a1260)?
                    };
                    let a1262 = 1u64;
                    nat_mul(a1261, a1262)?
                };
                nat_mul(a1263, a1264)?
            };
            nat_mul(a1265, a1266)?
        };
        let a1269 = 0u64;
        nat_quot(a1267, a1268, a1269)
    };
    let v5: bool = {
        let a1278 = {
            let a1276 = {
                let a1270 = 1u64;
                let a1271 = 1u64;
                nat_add(a1270, a1271)?
            };
            let a1277 = {
                let a1274 = {
                    let a1272 = 1u64;
                    let a1273 = 1u64;
                    nat_add(a1272, a1273)?
                };
                let a1275 = 1u64;
                nat_mul(a1274, a1275)?
            };
            nat_mul(a1276, a1277)?
        };
        let a1279 = v3;
        if nat_lt(a1278, a1279) {
            true
        } else {
            let a1288 = v3;
            let a1289 = {
                let a1286 = {
                    let a1280 = 1u64;
                    let a1281 = 1u64;
                    nat_add(a1280, a1281)?
                };
                let a1287 = {
                    let a1284 = {
                        let a1282 = 1u64;
                        let a1283 = 1u64;
                        nat_add(a1282, a1283)?
                    };
                    let a1285 = 1u64;
                    nat_mul(a1284, a1285)?
                };
                nat_mul(a1286, a1287)?
            };
            if nat_eq(a1288, a1289) {
                let a1297 = {
                    let a1294 = v4;
                    let a1295 = {
                        let a1292 = {
                            let a1290 = 1u64;
                            let a1291 = 1u64;
                            nat_add(a1290, a1291)?
                        };
                        let a1293 = 1u64;
                        nat_mul(a1292, a1293)?
                    };
                    let a1296 = 0u64;
                    nat_rem(a1294, a1295, a1296)
                };
                let a1298 = 1u64;
                nat_eq(a1297, a1298)
            } else {
                false
            }
        }
    };
    let v6: u64 = {
        let a1299 = v4;
        let a1300 = if v5 {
            1u64
        } else {
            0u64
        };
        nat_add(a1299, a1300)?
    };
    let v7: bool = {
        let a1329 = {
            let a1327 = {
                let a1303 = {
                    let a1301 = 1u64;
                    let a1302 = 1u64;
                    nat_add(a1301, a1302)?
                };
                let a1304 = 1u64;
                nat_mul(a1303, a1304)?
            };
            let a1328 = f13({
                let a1325 = {
                    let a1323 = {
                        let a1305 = 1u64;
                        let a1306 = 1u64;
                        nat_add(a1305, a1306)?
                    };
                    let a1324 = {
                        let a1321 = {
                            let a1319 = {
                                let a1307 = 1u64;
                                let a1308 = 1u64;
                                nat_add(a1307, a1308)?
                            };
                            let a1320 = {
                                let a1317 = {
                                    let a1315 = {
                                        let a1309 = 1u64;
                                        let a1310 = 1u64;
                                        nat_add(a1309, a1310)?
                                    };
                                    let a1316 = {
                                        let a1313 = {
                                            let a1311 = 1u64;
                                            let a1312 = 1u64;
                                            nat_add(a1311, a1312)?
                                        };
                                        let a1314 = 1u64;
                                        nat_mul(a1313, a1314)?
                                    };
                                    nat_mul(a1315, a1316)?
                                };
                                let a1318 = 1u64;
                                nat_add(a1317, a1318)?
                            };
                            nat_mul(a1319, a1320)?
                        };
                        let a1322 = 1u64;
                        nat_add(a1321, a1322)?
                    };
                    nat_mul(a1323, a1324)?
                };
                let a1326 = 1u64;
                nat_add(a1325, a1326)?
            })?;
            nat_mul(a1327, a1328)?
        };
        let a1330 = v6;
        nat_le(a1329, a1330)
    };
    let v8: u64 = if v7 {
        let a1335 = v6;
        let a1336 = {
            let a1333 = {
                let a1331 = 1u64;
                let a1332 = 1u64;
                nat_add(a1331, a1332)?
            };
            let a1334 = 1u64;
            nat_mul(a1333, a1334)?
        };
        let a1337 = 0u64;
        nat_quot(a1335, a1336, a1337)
    } else {
        v6
    };
    let v9: u64 = if v7 {
        let a1338 = v1;
        let a1339 = 1u64;
        nat_add(a1338, a1339)?
    } else {
        v1
    };
    let v10: u64 = {
        let a1378 = v9;
        let a1379 = {
            let a1376 = {
                let a1374 = {
                    let a1340 = 1u64;
                    let a1341 = 1u64;
                    nat_add(a1340, a1341)?
                };
                let a1375 = {
                    let a1372 = {
                        let a1342 = 1u64;
                        let a1343 = 1u64;
                        nat_add(a1342, a1343)?
                    };
                    let a1373 = {
                        let a1370 = {
                            let a1344 = 1u64;
                            let a1345 = 1u64;
                            nat_add(a1344, a1345)?
                        };
                        let a1371 = {
                            let a1368 = {
                                let a1346 = 1u64;
                                let a1347 = 1u64;
                                nat_add(a1346, a1347)?
                            };
                            let a1369 = {
                                let a1366 = {
                                    let a1348 = 1u64;
                                    let a1349 = 1u64;
                                    nat_add(a1348, a1349)?
                                };
                                let a1367 = {
                                    let a1364 = {
                                        let a1350 = 1u64;
                                        let a1351 = 1u64;
                                        nat_add(a1350, a1351)?
                                    };
                                    let a1365 = {
                                        let a1362 = {
                                            let a1352 = 1u64;
                                            let a1353 = 1u64;
                                            nat_add(a1352, a1353)?
                                        };
                                        let a1363 = {
                                            let a1360 = {
                                                let a1354 = 1u64;
                                                let a1355 = 1u64;
                                                nat_add(a1354, a1355)?
                                            };
                                            let a1361 = {
                                                let a1358 = {
                                                    let a1356 = 1u64;
                                                    let a1357 = 1u64;
                                                    nat_add(a1356, a1357)?
                                                };
                                                let a1359 = 1u64;
                                                nat_mul(a1358, a1359)?
                                            };
                                            nat_mul(a1360, a1361)?
                                        };
                                        nat_mul(a1362, a1363)?
                                    };
                                    nat_mul(a1364, a1365)?
                                };
                                nat_mul(a1366, a1367)?
                            };
                            nat_mul(a1368, a1369)?
                        };
                        nat_mul(a1370, a1371)?
                    };
                    nat_mul(a1372, a1373)?
                };
                nat_mul(a1374, a1375)?
            };
            let a1377 = 1u64;
            nat_add(a1376, a1377)?
        };
        if if nat_eq(a1378, a1379) {
            let a1402 = v8;
            let a1403 = f13({
                let a1400 = {
                    let a1398 = {
                        let a1380 = 1u64;
                        let a1381 = 1u64;
                        nat_add(a1380, a1381)?
                    };
                    let a1399 = {
                        let a1396 = {
                            let a1394 = {
                                let a1382 = 1u64;
                                let a1383 = 1u64;
                                nat_add(a1382, a1383)?
                            };
                            let a1395 = {
                                let a1392 = {
                                    let a1390 = {
                                        let a1384 = 1u64;
                                        let a1385 = 1u64;
                                        nat_add(a1384, a1385)?
                                    };
                                    let a1391 = {
                                        let a1388 = {
                                            let a1386 = 1u64;
                                            let a1387 = 1u64;
                                            nat_add(a1386, a1387)?
                                        };
                                        let a1389 = 1u64;
                                        nat_mul(a1388, a1389)?
                                    };
                                    nat_mul(a1390, a1391)?
                                };
                                let a1393 = 1u64;
                                nat_add(a1392, a1393)?
                            };
                            nat_mul(a1394, a1395)?
                        };
                        let a1397 = 1u64;
                        nat_add(a1396, a1397)?
                    };
                    nat_mul(a1398, a1399)?
                };
                let a1401 = 1u64;
                nat_add(a1400, a1401)?
            })?;
            nat_lt(a1402, a1403)
        } else {
            false
        } {
            0u64
        } else {
            let a1440 = v9;
            let a1441 = {
                let a1438 = {
                    let a1404 = 1u64;
                    let a1405 = 1u64;
                    nat_add(a1404, a1405)?
                };
                let a1439 = {
                    let a1436 = {
                        let a1406 = 1u64;
                        let a1407 = 1u64;
                        nat_add(a1406, a1407)?
                    };
                    let a1437 = {
                        let a1434 = {
                            let a1408 = 1u64;
                            let a1409 = 1u64;
                            nat_add(a1408, a1409)?
                        };
                        let a1435 = {
                            let a1432 = {
                                let a1410 = 1u64;
                                let a1411 = 1u64;
                                nat_add(a1410, a1411)?
                            };
                            let a1433 = {
                                let a1430 = {
                                    let a1412 = 1u64;
                                    let a1413 = 1u64;
                                    nat_add(a1412, a1413)?
                                };
                                let a1431 = {
                                    let a1428 = {
                                        let a1414 = 1u64;
                                        let a1415 = 1u64;
                                        nat_add(a1414, a1415)?
                                    };
                                    let a1429 = {
                                        let a1426 = {
                                            let a1416 = 1u64;
                                            let a1417 = 1u64;
                                            nat_add(a1416, a1417)?
                                        };
                                        let a1427 = {
                                            let a1424 = {
                                                let a1418 = 1u64;
                                                let a1419 = 1u64;
                                                nat_add(a1418, a1419)?
                                            };
                                            let a1425 = {
                                                let a1422 = {
                                                    let a1420 = 1u64;
                                                    let a1421 = 1u64;
                                                    nat_add(a1420, a1421)?
                                                };
                                                let a1423 = 1u64;
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
                            nat_mul(a1432, a1433)?
                        };
                        nat_mul(a1434, a1435)?
                    };
                    nat_mul(a1436, a1437)?
                };
                nat_mul(a1438, a1439)?
            };
            nat_sub(a1440, a1441)
        }
    };
    let a1442 = v8;
    let a1443 = 0u64;
    if nat_eq(a1442, a1443) {
        f16(v0)
    } else {
        let a1496 = {
            let a1494 = {
                let a1492 = {
                    let a1444 = 1u64;
                    let a1445 = 1u64;
                    nat_add(a1444, a1445)?
                };
                let a1493 = {
                    let a1490 = {
                        let a1488 = {
                            let a1446 = 1u64;
                            let a1447 = 1u64;
                            nat_add(a1446, a1447)?
                        };
                        let a1489 = {
                            let a1486 = {
                                let a1484 = {
                                    let a1448 = 1u64;
                                    let a1449 = 1u64;
                                    nat_add(a1448, a1449)?
                                };
                                let a1485 = {
                                    let a1482 = {
                                        let a1480 = {
                                            let a1450 = 1u64;
                                            let a1451 = 1u64;
                                            nat_add(a1450, a1451)?
                                        };
                                        let a1481 = {
                                            let a1478 = {
                                                let a1476 = {
                                                    let a1452 = 1u64;
                                                    let a1453 = 1u64;
                                                    nat_add(a1452, a1453)?
                                                };
                                                let a1477 = {
                                                    let a1474 = {
                                                        let a1472 = {
                                                            let a1454 = 1u64;
                                                            let a1455 = 1u64;
                                                            nat_add(a1454, a1455)?
                                                        };
                                                        let a1473 = {
                                                            let a1470 = {
                                                                let a1468 = {
                                                                    let a1456 = 1u64;
                                                                    let a1457 = 1u64;
                                                                    nat_add(a1456, a1457)?
                                                                };
                                                                let a1469 = {
                                                                    let a1466 = {
                                                                        let a1464 = {
                                                                            let a1458 = 1u64;
                                                                            let a1459 = 1u64;
                                                                            nat_add(a1458, a1459)?
                                                                        };
                                                                        let a1465 = {
                                                                            let a1462 = {
                                                                                let a1460 = 1u64;
                                                                                let a1461 = 1u64;
                                                                                nat_add(a1460, a1461)?
                                                                            };
                                                                            let a1463 = 1u64;
                                                                            nat_mul(a1462, a1463)?
                                                                        };
                                                                        nat_mul(a1464, a1465)?
                                                                    };
                                                                    let a1467 = 1u64;
                                                                    nat_add(a1466, a1467)?
                                                                };
                                                                nat_mul(a1468, a1469)?
                                                            };
                                                            let a1471 = 1u64;
                                                            nat_add(a1470, a1471)?
                                                        };
                                                        nat_mul(a1472, a1473)?
                                                    };
                                                    let a1475 = 1u64;
                                                    nat_add(a1474, a1475)?
                                                };
                                                nat_mul(a1476, a1477)?
                                            };
                                            let a1479 = 1u64;
                                            nat_add(a1478, a1479)?
                                        };
                                        nat_mul(a1480, a1481)?
                                    };
                                    let a1483 = 1u64;
                                    nat_add(a1482, a1483)?
                                };
                                nat_mul(a1484, a1485)?
                            };
                            let a1487 = 1u64;
                            nat_add(a1486, a1487)?
                        };
                        nat_mul(a1488, a1489)?
                    };
                    let a1491 = 1u64;
                    nat_add(a1490, a1491)?
                };
                nat_mul(a1492, a1493)?
            };
            let a1495 = 1u64;
            nat_add(a1494, a1495)?
        };
        let a1497 = v9;
        if nat_le(a1496, a1497) {
            f14(v0)
        } else {
            let a1549 = {
                let a1522 = f16(v0)?;
                let a1523 = {
                    let a1520 = v10;
                    let a1521 = f13({
                        let a1518 = {
                            let a1516 = {
                                let a1498 = 1u64;
                                let a1499 = 1u64;
                                nat_add(a1498, a1499)?
                            };
                            let a1517 = {
                                let a1514 = {
                                    let a1512 = {
                                        let a1500 = 1u64;
                                        let a1501 = 1u64;
                                        nat_add(a1500, a1501)?
                                    };
                                    let a1513 = {
                                        let a1510 = {
                                            let a1508 = {
                                                let a1502 = 1u64;
                                                let a1503 = 1u64;
                                                nat_add(a1502, a1503)?
                                            };
                                            let a1509 = {
                                                let a1506 = {
                                                    let a1504 = 1u64;
                                                    let a1505 = 1u64;
                                                    nat_add(a1504, a1505)?
                                                };
                                                let a1507 = 1u64;
                                                nat_mul(a1506, a1507)?
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
                            };
                            nat_mul(a1516, a1517)?
                        };
                        let a1519 = 1u64;
                        nat_add(a1518, a1519)?
                    })?;
                    nat_mul(a1520, a1521)?
                };
                nat_add(a1522, a1523)?
            };
            let a1550 = {
                let a1546 = v8;
                let a1547 = f13({
                    let a1544 = {
                        let a1542 = {
                            let a1524 = 1u64;
                            let a1525 = 1u64;
                            nat_add(a1524, a1525)?
                        };
                        let a1543 = {
                            let a1540 = {
                                let a1538 = {
                                    let a1526 = 1u64;
                                    let a1527 = 1u64;
                                    nat_add(a1526, a1527)?
                                };
                                let a1539 = {
                                    let a1536 = {
                                        let a1534 = {
                                            let a1528 = 1u64;
                                            let a1529 = 1u64;
                                            nat_add(a1528, a1529)?
                                        };
                                        let a1535 = {
                                            let a1532 = {
                                                let a1530 = 1u64;
                                                let a1531 = 1u64;
                                                nat_add(a1530, a1531)?
                                            };
                                            let a1533 = 1u64;
                                            nat_mul(a1532, a1533)?
                                        };
                                        nat_mul(a1534, a1535)?
                                    };
                                    let a1537 = 1u64;
                                    nat_add(a1536, a1537)?
                                };
                                nat_mul(a1538, a1539)?
                            };
                            let a1541 = 1u64;
                            nat_add(a1540, a1541)?
                        };
                        nat_mul(a1542, a1543)?
                    };
                    let a1545 = 1u64;
                    nat_add(a1544, a1545)?
                })?;
                let a1548 = 0u64;
                nat_rem(a1546, a1547, a1548)
            };
            nat_add(a1549, a1550)
        }
    }
}

pub fn f41(v0: u64) -> R<u64> {
    let a1551 = v0;
    let a1552 = v0;
    nat_add(a1551, a1552)
}

pub fn run(p0: u64, p1: bool, p2: u64, p3: u64, p4: u64) -> R<Option<u64>> {
    f0(p0, p1, p2, p3, p4)
}
