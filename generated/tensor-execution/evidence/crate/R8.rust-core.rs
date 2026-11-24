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
    C0(u64, u64),
    C1(u64, u64),
    C2(u64, u64),
    C3(u64, u64),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Fn0 {
    F18,
}

impl Fn0 {
    pub fn apply(&self, p0: u64) -> R<u64> {
        match self {
            Fn0::F18 => f18(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F20,
}

impl Fn1 {
    pub fn apply(&self, p0: Adt1) -> R<Option<Adt1>> {
        match self {
            Fn1::F20 => f20(p0),
        }
    }
}

pub fn f0(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Option<Adt0>> {
    let a1 = v0;
    let a2 = 13u64;
    if if if nat_lt(a1, a2) {
        f1(v1)
    } else {
        false
    } {
        let a3 = v1;
        let a4 = v2;
        if nat_eq(a3, a4) {
            true
        } else {
            let a5 = v2;
            let a6 = 0u64;
            nat_eq(a5, a6)
        }
    } else {
        false
    } {
        let m7 = f2(v1, 0u64, v3)?;
        match m7 {
            None => {
                Ok(None::<Adt0>)
            }
            Some(v4) => {
                Ok(Some(f3(v0, v4)))
            }
        }
    } else {
        Ok(None::<Adt0>)
    }
}

pub fn f1(v0: u64) -> bool {
    let a8 = v0;
    let a9 = 0u64;
    if if nat_eq(a8, a9) {
        true
    } else {
        let a10 = v0;
        let a11 = 1u64;
        nat_eq(a10, a11)
    } {
        true
    } else {
        let a12 = v0;
        let a13 = 30u64;
        nat_eq(a12, a13)
    }
}

pub fn f2(v0: u64, v1: u64, v2: u64) -> R<Option<u64>> {
    if if if f1(v0) {
        f1(v1)
    } else {
        false
    } {
        let a16 = v2;
        let a17 = {
            let a14 = v0;
            let a15 = 0u64;
            if nat_eq(a14, a15) {
                4294967296u64
            } else {
                65536u64
            }
        };
        nat_lt(a16, a17)
    } else {
        false
    } {
        Ok(Some({
            let a18 = v0;
            let a19 = v1;
            if nat_eq(a18, a19) {
                v2
            } else {
                let v3: u64 = f4(v0, v2)?;
                let a20 = v1;
                let a21 = 0u64;
                if nat_eq(a20, a21) {
                    v3
                } else {
                    let a22 = v1;
                    let a23 = 1u64;
                    if nat_eq(a22, a23) {
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

pub fn f3(v0: u64, v1: u64) -> Adt0 {
    let a24 = v0;
    let a25 = 0u64;
    if if if if nat_eq(a24, a25) {
        true
    } else {
        let a26 = v0;
        let a27 = 2u64;
        nat_eq(a26, a27)
    } {
        true
    } else {
        let a28 = v0;
        let a29 = 5u64;
        nat_eq(a28, a29)
    } {
        true
    } else {
        let a30 = v0;
        let a31 = 6u64;
        nat_eq(a30, a31)
    } {
        Adt0::C0({
            let a32 = v0;
            let a33 = 0u64;
            if nat_eq(a32, a33) {
                7u64
            } else {
                let a34 = v0;
                let a35 = 2u64;
                if nat_eq(a34, a35) {
                    6u64
                } else {
                    let a36 = v0;
                    let a37 = 5u64;
                    if nat_eq(a36, a37) {
                        5u64
                    } else {
                        4u64
                    }
                }
            }
        }, v1)
    } else {
        let a38 = v0;
        let a39 = 1u64;
        if if if nat_eq(a38, a39) {
            true
        } else {
            let a40 = v0;
            let a41 = 3u64;
            nat_eq(a40, a41)
        } {
            true
        } else {
            let a42 = v0;
            let a43 = 4u64;
            nat_eq(a42, a43)
        } {
            Adt0::C1({
                let a44 = v0;
                let a45 = 1u64;
                if nat_eq(a44, a45) {
                    0u64
                } else {
                    let a46 = v0;
                    let a47 = 3u64;
                    if nat_eq(a46, a47) {
                        1u64
                    } else {
                        2u64
                    }
                }
            }, v1)
        } else {
            let a48 = v0;
            let a49 = 7u64;
            if if nat_eq(a48, a49) {
                true
            } else {
                let a50 = v0;
                let a51 = 8u64;
                nat_eq(a50, a51)
            } {
                Adt0::C2({
                    let a52 = v0;
                    let a53 = 7u64;
                    nat_sub(a52, a53)
                }, v1)
            } else {
                Adt0::C3({
                    let a54 = v0;
                    let a55 = 9u64;
                    nat_sub(a54, a55)
                }, v1)
            }
        }
    }
}

pub fn f4(v0: u64, v1: u64) -> R<u64> {
    let a56 = v0;
    let a57 = 0u64;
    if nat_eq(a56, a57) {
        Ok(v1)
    } else {
        let a58 = v0;
        let a59 = 1u64;
        if nat_eq(a58, a59) {
            f7(v1)
        } else {
            let a60 = v1;
            let a61 = 65536u64;
            nat_mul(a60, a61)
        }
    }
}

pub fn f5(v0: u64) -> R<u64> {
    let v1: u64 = {
        let a62 = f8(v0)?;
        let a63 = 32768u64;
        nat_mul(a62, a63)?
    };
    let v2: u64 = f9(v0)?;
    let a82 = v1;
    let a83 = {
        let a64 = v2;
        let a65 = 255u64;
        if nat_eq(a64, a65) {
            let a66 = f10(v0)?;
            let a67 = 0u64;
            if nat_eq(a66, a67) {
                31744u64
            } else {
                32256u64
            }
        } else {
            let a68 = 143u64;
            let a69 = v2;
            if nat_le(a68, a69) {
                31744u64
            } else {
                let a70 = v2;
                let a71 = 102u64;
                if nat_lt(a70, a71) {
                    0u64
                } else {
                    let a72 = 113u64;
                    let a73 = v2;
                    if nat_le(a72, a73) {
                        let a78 = {
                            let a76 = {
                                let a74 = v2;
                                let a75 = 113u64;
                                nat_sub(a74, a75)
                            };
                            let a77 = 1024u64;
                            nat_mul(a76, a77)?
                        };
                        let a79 = f11(f12(v0)?, 8192u64)?;
                        nat_add(a78, a79)?
                    } else {
                        f11(f12(v0)?, f13({
                            let a80 = 126u64;
                            let a81 = v2;
                            nat_sub(a80, a81)
                        })?)?
                    }
                }
            }
        }
    };
    nat_add(a82, a83)
}

pub fn f6(v0: u64) -> R<u64> {
    if f14(v0)? {
        let v1: u64 = {
            let a84 = v0;
            let a85 = 65536u64;
            let a86 = 0u64;
            nat_quot(a84, a85, a86)
        };
        let a95 = v1;
        let a96 = {
            let a93 = {
                let a90 = {
                    let a87 = v1;
                    let a88 = 64u64;
                    let a89 = 0u64;
                    nat_quot(a87, a88, a89)
                };
                let a91 = 2u64;
                let a92 = 0u64;
                nat_rem(a90, a91, a92)
            };
            let a94 = 0u64;
            if nat_eq(a93, a94) {
                64u64
            } else {
                0u64
            }
        };
        nat_add(a95, a96)
    } else {
        let a107 = {
            let a105 = {
                let a97 = v0;
                let a98 = 32767u64;
                nat_add(a97, a98)?
            };
            let a106 = {
                let a102 = {
                    let a99 = v0;
                    let a100 = 65536u64;
                    let a101 = 0u64;
                    nat_quot(a99, a100, a101)
                };
                let a103 = 2u64;
                let a104 = 0u64;
                nat_rem(a102, a103, a104)
            };
            nat_add(a105, a106)?
        };
        let a108 = 65536u64;
        let a109 = 0u64;
        Ok(nat_quot(a107, a108, a109))
    }
}

pub fn f7(v0: u64) -> R<u64> {
    let v1: u64 = {
        let a113 = {
            let a110 = v0;
            let a111 = 32768u64;
            let a112 = 0u64;
            nat_quot(a110, a111, a112)
        };
        let a114 = 2147483648u64;
        nat_mul(a113, a114)?
    };
    let v2: u64 = {
        let a118 = {
            let a115 = v0;
            let a116 = 32768u64;
            let a117 = 0u64;
            nat_rem(a115, a116, a117)
        };
        let a119 = 1024u64;
        let a120 = 0u64;
        nat_quot(a118, a119, a120)
    };
    let v3: u64 = {
        let a121 = v0;
        let a122 = 1024u64;
        let a123 = 0u64;
        nat_rem(a121, a122, a123)
    };
    let a158 = v1;
    let a159 = {
        let a124 = v2;
        let a125 = 31u64;
        if nat_eq(a124, a125) {
            let a134 = 2139095040u64;
            let a135 = {
                let a132 = {
                    let a126 = 0u64;
                    let a127 = v3;
                    if if nat_lt(a126, a127) {
                        let a128 = v3;
                        let a129 = 512u64;
                        nat_lt(a128, a129)
                    } else {
                        false
                    } {
                        let a130 = v3;
                        let a131 = 512u64;
                        nat_add(a130, a131)?
                    } else {
                        v3
                    }
                };
                let a133 = 8192u64;
                nat_mul(a132, a133)?
            };
            nat_add(a134, a135)?
        } else {
            let a136 = v2;
            let a137 = 0u64;
            if nat_eq(a136, a137) {
                let a138 = v3;
                let a139 = 0u64;
                if nat_eq(a138, a139) {
                    0u64
                } else {
                    let v4: Adt1 = f15(113u64, v3, 0u64, 1024u64)?;
                    let a148 = {
                        let a141 = match v4.clone() {
                            Adt1::C0(h140, _, _, _) => {
                                h140
                            }
                        };
                        let a142 = 8388608u64;
                        nat_mul(a141, a142)?
                    };
                    let a149 = {
                        let a146 = {
                            let a144 = match v4 {
                                Adt1::C0(_, h143, _, _) => {
                                    h143
                                }
                            };
                            let a145 = 1024u64;
                            nat_sub(a144, a145)
                        };
                        let a147 = 8192u64;
                        nat_mul(a146, a147)?
                    };
                    nat_add(a148, a149)?
                }
            } else {
                let a156 = {
                    let a152 = {
                        let a150 = v2;
                        let a151 = 112u64;
                        nat_add(a150, a151)?
                    };
                    let a153 = 8388608u64;
                    nat_mul(a152, a153)?
                };
                let a157 = {
                    let a154 = v3;
                    let a155 = 8192u64;
                    nat_mul(a154, a155)?
                };
                nat_add(a156, a157)?
            }
        }
    };
    nat_add(a158, a159)
}

pub fn f8(v0: u64) -> R<u64> {
    let a184 = v0;
    let a185 = f13({
        let a182 = {
            let a180 = {
                let a160 = 1u64;
                let a161 = 1u64;
                nat_add(a160, a161)?
            };
            let a181 = {
                let a178 = {
                    let a176 = {
                        let a162 = 1u64;
                        let a163 = 1u64;
                        nat_add(a162, a163)?
                    };
                    let a177 = {
                        let a174 = {
                            let a172 = {
                                let a164 = 1u64;
                                let a165 = 1u64;
                                nat_add(a164, a165)?
                            };
                            let a173 = {
                                let a170 = {
                                    let a168 = {
                                        let a166 = 1u64;
                                        let a167 = 1u64;
                                        nat_add(a166, a167)?
                                    };
                                    let a169 = 1u64;
                                    nat_mul(a168, a169)?
                                };
                                let a171 = 1u64;
                                nat_add(a170, a171)?
                            };
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
    })?;
    let a186 = 0u64;
    Ok(nat_quot(a184, a185, a186))
}

pub fn f9(v0: u64) -> R<u64> {
    let a209 = f16(v0)?;
    let a210 = f13({
        let a207 = {
            let a205 = {
                let a187 = 1u64;
                let a188 = 1u64;
                nat_add(a187, a188)?
            };
            let a206 = {
                let a203 = {
                    let a201 = {
                        let a189 = 1u64;
                        let a190 = 1u64;
                        nat_add(a189, a190)?
                    };
                    let a202 = {
                        let a199 = {
                            let a197 = {
                                let a191 = 1u64;
                                let a192 = 1u64;
                                nat_add(a191, a192)?
                            };
                            let a198 = {
                                let a195 = {
                                    let a193 = 1u64;
                                    let a194 = 1u64;
                                    nat_add(a193, a194)?
                                };
                                let a196 = 1u64;
                                nat_mul(a195, a196)?
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
    })?;
    let a211 = 0u64;
    Ok(nat_quot(a209, a210, a211))
}

pub fn f10(v0: u64) -> R<u64> {
    let a234 = v0;
    let a235 = f13({
        let a232 = {
            let a230 = {
                let a212 = 1u64;
                let a213 = 1u64;
                nat_add(a212, a213)?
            };
            let a231 = {
                let a228 = {
                    let a226 = {
                        let a214 = 1u64;
                        let a215 = 1u64;
                        nat_add(a214, a215)?
                    };
                    let a227 = {
                        let a224 = {
                            let a222 = {
                                let a216 = 1u64;
                                let a217 = 1u64;
                                nat_add(a216, a217)?
                            };
                            let a223 = {
                                let a220 = {
                                    let a218 = 1u64;
                                    let a219 = 1u64;
                                    nat_add(a218, a219)?
                                };
                                let a221 = 1u64;
                                nat_mul(a220, a221)?
                            };
                            nat_mul(a222, a223)?
                        };
                        let a225 = 1u64;
                        nat_add(a224, a225)?
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
    })?;
    let a236 = 0u64;
    Ok(nat_rem(a234, a235, a236))
}

pub fn f11(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = {
        let a237 = v0;
        let a238 = v1;
        let a239 = 0u64;
        nat_quot(a237, a238, a239)
    };
    let v3: u64 = {
        let a243 = {
            let a240 = v0;
            let a241 = v1;
            let a242 = 0u64;
            nat_rem(a240, a241, a242)
        };
        let a244 = 2u64;
        nat_mul(a243, a244)?
    };
    let a254 = v2;
    let a255 = {
        let a245 = v1;
        let a246 = v3;
        if if nat_lt(a245, a246) {
            true
        } else {
            let a247 = v3;
            let a248 = v1;
            if nat_eq(a247, a248) {
                let a252 = {
                    let a249 = v2;
                    let a250 = 2u64;
                    let a251 = 0u64;
                    nat_rem(a249, a250, a251)
                };
                let a253 = 1u64;
                nat_eq(a252, a253)
            } else {
                false
            }
        } {
            1u64
        } else {
            0u64
        }
    };
    nat_add(a254, a255)
}

pub fn f12(v0: u64) -> R<u64> {
    let a280 = f10(v0)?;
    let a281 = {
        let a256 = f9(v0)?;
        let a257 = 0u64;
        if nat_eq(a256, a257) {
            0u64
        } else {
            f13({
                let a278 = {
                    let a276 = {
                        let a258 = 1u64;
                        let a259 = 1u64;
                        nat_add(a258, a259)?
                    };
                    let a277 = {
                        let a274 = {
                            let a272 = {
                                let a260 = 1u64;
                                let a261 = 1u64;
                                nat_add(a260, a261)?
                            };
                            let a273 = {
                                let a270 = {
                                    let a268 = {
                                        let a262 = 1u64;
                                        let a263 = 1u64;
                                        nat_add(a262, a263)?
                                    };
                                    let a269 = {
                                        let a266 = {
                                            let a264 = 1u64;
                                            let a265 = 1u64;
                                            nat_add(a264, a265)?
                                        };
                                        let a267 = 1u64;
                                        nat_mul(a266, a267)?
                                    };
                                    nat_mul(a268, a269)?
                                };
                                let a271 = 1u64;
                                nat_add(a270, a271)?
                            };
                            nat_mul(a272, a273)?
                        };
                        let a275 = 1u64;
                        nat_add(a274, a275)?
                    };
                    nat_mul(a276, a277)?
                };
                let a279 = 1u64;
                nat_add(a278, a279)?
            })?
        }
    };
    nat_add(a280, a281)
}

pub fn f13(v0: u64) -> R<u64> {
    f17(Fn0::F18, v0, 1u64)
}

pub fn f14(v0: u64) -> R<bool> {
    let a324 = f9(v0)?;
    let a325 = {
        let a322 = {
            let a320 = {
                let a282 = 1u64;
                let a283 = 1u64;
                nat_add(a282, a283)?
            };
            let a321 = {
                let a318 = {
                    let a316 = {
                        let a284 = 1u64;
                        let a285 = 1u64;
                        nat_add(a284, a285)?
                    };
                    let a317 = {
                        let a314 = {
                            let a312 = {
                                let a286 = 1u64;
                                let a287 = 1u64;
                                nat_add(a286, a287)?
                            };
                            let a313 = {
                                let a310 = {
                                    let a308 = {
                                        let a288 = 1u64;
                                        let a289 = 1u64;
                                        nat_add(a288, a289)?
                                    };
                                    let a309 = {
                                        let a306 = {
                                            let a304 = {
                                                let a290 = 1u64;
                                                let a291 = 1u64;
                                                nat_add(a290, a291)?
                                            };
                                            let a305 = {
                                                let a302 = {
                                                    let a300 = {
                                                        let a292 = 1u64;
                                                        let a293 = 1u64;
                                                        nat_add(a292, a293)?
                                                    };
                                                    let a301 = {
                                                        let a298 = {
                                                            let a296 = {
                                                                let a294 = 1u64;
                                                                let a295 = 1u64;
                                                                nat_add(a294, a295)?
                                                            };
                                                            let a297 = 1u64;
                                                            nat_mul(a296, a297)?
                                                        };
                                                        let a299 = 1u64;
                                                        nat_add(a298, a299)?
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
                            };
                            nat_mul(a312, a313)?
                        };
                        let a315 = 1u64;
                        nat_add(a314, a315)?
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
    if nat_eq(a324, a325) {
        let a326 = 0u64;
        let a327 = f10(v0)?;
        Ok(nat_lt(a326, a327))
    } else {
        Ok(false)
    }
}

pub fn f15(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt1> {
    let (h328, _) = f19(Fn1::F20, {
        let a347 = {
            let a329 = 1u64;
            let a330 = 1u64;
            nat_add(a329, a330)?
        };
        let a348 = {
            let a345 = {
                let a331 = 1u64;
                let a332 = 1u64;
                nat_add(a331, a332)?
            };
            let a346 = {
                let a343 = {
                    let a333 = 1u64;
                    let a334 = 1u64;
                    nat_add(a333, a334)?
                };
                let a344 = {
                    let a341 = {
                        let a335 = 1u64;
                        let a336 = 1u64;
                        nat_add(a335, a336)?
                    };
                    let a342 = {
                        let a339 = {
                            let a337 = 1u64;
                            let a338 = 1u64;
                            nat_add(a337, a338)?
                        };
                        let a340 = 1u64;
                        nat_mul(a339, a340)?
                    };
                    nat_mul(a341, a342)?
                };
                nat_mul(a343, a344)?
            };
            nat_mul(a345, a346)?
        };
        nat_mul(a347, a348)?
    }, Adt1::C0(v0, v1, v2, v3))?;
    Ok(h328)
}

pub fn f16(v0: u64) -> R<u64> {
    let a373 = v0;
    let a374 = f13({
        let a371 = {
            let a369 = {
                let a349 = 1u64;
                let a350 = 1u64;
                nat_add(a349, a350)?
            };
            let a370 = {
                let a367 = {
                    let a365 = {
                        let a351 = 1u64;
                        let a352 = 1u64;
                        nat_add(a351, a352)?
                    };
                    let a366 = {
                        let a363 = {
                            let a361 = {
                                let a353 = 1u64;
                                let a354 = 1u64;
                                nat_add(a353, a354)?
                            };
                            let a362 = {
                                let a359 = {
                                    let a357 = {
                                        let a355 = 1u64;
                                        let a356 = 1u64;
                                        nat_add(a355, a356)?
                                    };
                                    let a358 = 1u64;
                                    nat_mul(a357, a358)?
                                };
                                let a360 = 1u64;
                                nat_add(a359, a360)?
                            };
                            nat_mul(a361, a362)?
                        };
                        let a364 = 1u64;
                        nat_add(a363, a364)?
                    };
                    nat_mul(a365, a366)?
                };
                let a368 = 1u64;
                nat_add(a367, a368)?
            };
            nat_mul(a369, a370)?
        };
        let a372 = 1u64;
        nat_add(a371, a372)?
    })?;
    let a375 = 0u64;
    Ok(nat_rem(a373, a374, a375))
}

pub fn f17(v0: Fn0, p379: u64, p380: u64) -> R<u64> {
    let m378 = (p379, p380);
    {
        let mut m378 = m378;
        loop {
            let step = {
                let (v1, v2) = m378;
                let m376 = v1;
                if m376 == 0 {
                    Err::<(u64, u64), R<u64>>(Ok(v2))
                } else {
                    let v3 = m376 - 1;
                    Ok::<(u64, u64), R<u64>>((v3, {
                        let c377: Fn0 = v0.clone();
                        c377.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m378 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f18(v0: u64) -> R<u64> {
    let a381 = v0;
    let a382 = v0;
    nat_add(a381, a382)
}

pub fn f19(v0: Fn1, p387: u64, p388: Adt1) -> R<(Adt1, bool)> {
    let m386 = (p387, p388);
    {
        let mut m386 = m386;
        loop {
            let step = {
                let (v1, v2) = m386;
                let m383 = v1;
                if m383 == 0 {
                    Err::<(u64, Adt1), R<(Adt1, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m383 - 1;
                    let m385 = {
                        let c384: Fn1 = v0.clone();
                        c384.apply(v2.clone())?
                    };
                    match m385 {
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
                Ok(next) => { m386 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f20(v0: Adt1) -> R<Option<Adt1>> {
    f21(v0)
}

pub fn f21(v0: Adt1) -> R<Option<Adt1>> {
    let a390 = 0u64;
    let a391 = match v0.clone() {
        Adt1::C0(_, h389, _, _) => {
            h389
        }
    };
    if if if nat_lt(a390, a391) {
        let a394 = match v0.clone() {
            Adt1::C0(_, h392, _, _) => {
                h392
            }
        };
        let a395 = match v0.clone() {
            Adt1::C0(_, _, _, h393) => {
                h393
            }
        };
        nat_lt(a394, a395)
    } else {
        false
    } {
        let a398 = match v0.clone() {
            Adt1::C0(_, _, h396, _) => {
                h396
            }
        };
        let a399 = match v0.clone() {
            Adt1::C0(h397, _, _, _) => {
                h397
            }
        };
        nat_lt(a398, a399)
    } else {
        false
    } {
        Ok(Some(Adt1::C0({
            let a401 = match v0.clone() {
                Adt1::C0(h400, _, _, _) => {
                    h400
                }
            };
            let a402 = 1u64;
            nat_sub(a401, a402)
        }, {
            let a405 = match v0.clone() {
                Adt1::C0(_, h403, _, _) => {
                    h403
                }
            };
            let a406 = match v0.clone() {
                Adt1::C0(_, h404, _, _) => {
                    h404
                }
            };
            nat_add(a405, a406)?
        }, match v0.clone() {
            Adt1::C0(_, _, h407, _) => {
                h407
            }
        }, match v0 {
            Adt1::C0(_, _, _, h408) => {
                h408
            }
        })))
    } else {
        Ok(None::<Adt1>)
    }
}
