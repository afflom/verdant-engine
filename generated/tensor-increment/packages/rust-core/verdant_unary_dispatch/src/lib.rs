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
    C0(Adt1),
    C1(u64),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(u64, u64),
    C1(u64, u64),
    C2(u64, u64),
    C3(u64, u64),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Fn0 {
    F19,
}

impl Fn0 {
    pub fn apply(&self, p0: u64) -> R<u64> {
        match self {
            Fn0::F19 => f19(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F21,
}

impl Fn1 {
    pub fn apply(&self, p0: Adt2) -> R<Option<Adt2>> {
        match self {
            Fn1::F21 => f21(p0),
        }
    }
}

pub fn f0(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Option<Adt0>> {
    let a1 = v0;
    let a2 = 13u64;
    if nat_eq(a1, a2) {
        if if f1(v1) {
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
                    Ok(Some(Adt0::C1(v4)))
                }
            }
        } else {
            Ok(None::<Adt0>)
        }
    } else {
        let m8 = f3(v0, v1, v2, v3)?;
        match m8 {
            None => {
                Ok(None::<Adt0>)
            }
            Some(v5) => {
                Ok(Some(Adt0::C0(v5)))
            }
        }
    }
}

pub fn f1(v0: u64) -> bool {
    let a9 = v0;
    let a10 = 0u64;
    if if nat_eq(a9, a10) {
        true
    } else {
        let a11 = v0;
        let a12 = 1u64;
        nat_eq(a11, a12)
    } {
        true
    } else {
        let a13 = v0;
        let a14 = 30u64;
        nat_eq(a13, a14)
    }
}

pub fn f2(v0: u64, v1: u64, v2: u64) -> R<Option<u64>> {
    if if if f1(v0) {
        f1(v1)
    } else {
        false
    } {
        let a17 = v2;
        let a18 = {
            let a15 = v0;
            let a16 = 0u64;
            if nat_eq(a15, a16) {
                4294967296u64
            } else {
                65536u64
            }
        };
        nat_lt(a17, a18)
    } else {
        false
    } {
        Ok(Some({
            let a19 = v0;
            let a20 = v1;
            if nat_eq(a19, a20) {
                v2
            } else {
                let v3: u64 = f4(v0, v2)?;
                let a21 = v1;
                let a22 = 0u64;
                if nat_eq(a21, a22) {
                    v3
                } else {
                    let a23 = v1;
                    let a24 = 1u64;
                    if nat_eq(a23, a24) {
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

pub fn f3(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Option<Adt1>> {
    let a25 = v0;
    let a26 = 13u64;
    if if if nat_lt(a25, a26) {
        f1(v1)
    } else {
        false
    } {
        let a27 = v1;
        let a28 = v2;
        if nat_eq(a27, a28) {
            true
        } else {
            let a29 = v2;
            let a30 = 0u64;
            nat_eq(a29, a30)
        }
    } else {
        false
    } {
        let m31 = f2(v1, 0u64, v3)?;
        match m31 {
            None => {
                Ok(None::<Adt1>)
            }
            Some(v4) => {
                Ok(Some(f7(v0, v4)))
            }
        }
    } else {
        Ok(None::<Adt1>)
    }
}

pub fn f4(v0: u64, v1: u64) -> R<u64> {
    let a32 = v0;
    let a33 = 0u64;
    if nat_eq(a32, a33) {
        Ok(v1)
    } else {
        let a34 = v0;
        let a35 = 1u64;
        if nat_eq(a34, a35) {
            f8(v1)
        } else {
            let a36 = v1;
            let a37 = 65536u64;
            nat_mul(a36, a37)
        }
    }
}

pub fn f5(v0: u64) -> R<u64> {
    let v1: u64 = {
        let a38 = f9(v0)?;
        let a39 = 32768u64;
        nat_mul(a38, a39)?
    };
    let v2: u64 = f10(v0)?;
    let a58 = v1;
    let a59 = {
        let a40 = v2;
        let a41 = 255u64;
        if nat_eq(a40, a41) {
            let a42 = f11(v0)?;
            let a43 = 0u64;
            if nat_eq(a42, a43) {
                31744u64
            } else {
                32256u64
            }
        } else {
            let a44 = 143u64;
            let a45 = v2;
            if nat_le(a44, a45) {
                31744u64
            } else {
                let a46 = v2;
                let a47 = 102u64;
                if nat_lt(a46, a47) {
                    0u64
                } else {
                    let a48 = 113u64;
                    let a49 = v2;
                    if nat_le(a48, a49) {
                        let a54 = {
                            let a52 = {
                                let a50 = v2;
                                let a51 = 113u64;
                                nat_sub(a50, a51)
                            };
                            let a53 = 1024u64;
                            nat_mul(a52, a53)?
                        };
                        let a55 = f12(f13(v0)?, 8192u64)?;
                        nat_add(a54, a55)?
                    } else {
                        f12(f13(v0)?, f14({
                            let a56 = 126u64;
                            let a57 = v2;
                            nat_sub(a56, a57)
                        })?)?
                    }
                }
            }
        }
    };
    nat_add(a58, a59)
}

pub fn f6(v0: u64) -> R<u64> {
    if f15(v0)? {
        let v1: u64 = {
            let a60 = v0;
            let a61 = 65536u64;
            let a62 = 0u64;
            nat_quot(a60, a61, a62)
        };
        let a71 = v1;
        let a72 = {
            let a69 = {
                let a66 = {
                    let a63 = v1;
                    let a64 = 64u64;
                    let a65 = 0u64;
                    nat_quot(a63, a64, a65)
                };
                let a67 = 2u64;
                let a68 = 0u64;
                nat_rem(a66, a67, a68)
            };
            let a70 = 0u64;
            if nat_eq(a69, a70) {
                64u64
            } else {
                0u64
            }
        };
        nat_add(a71, a72)
    } else {
        let a83 = {
            let a81 = {
                let a73 = v0;
                let a74 = 32767u64;
                nat_add(a73, a74)?
            };
            let a82 = {
                let a78 = {
                    let a75 = v0;
                    let a76 = 65536u64;
                    let a77 = 0u64;
                    nat_quot(a75, a76, a77)
                };
                let a79 = 2u64;
                let a80 = 0u64;
                nat_rem(a78, a79, a80)
            };
            nat_add(a81, a82)?
        };
        let a84 = 65536u64;
        let a85 = 0u64;
        Ok(nat_quot(a83, a84, a85))
    }
}

pub fn f7(v0: u64, v1: u64) -> Adt1 {
    let a86 = v0;
    let a87 = 0u64;
    if if if if nat_eq(a86, a87) {
        true
    } else {
        let a88 = v0;
        let a89 = 2u64;
        nat_eq(a88, a89)
    } {
        true
    } else {
        let a90 = v0;
        let a91 = 5u64;
        nat_eq(a90, a91)
    } {
        true
    } else {
        let a92 = v0;
        let a93 = 6u64;
        nat_eq(a92, a93)
    } {
        Adt1::C0({
            let a94 = v0;
            let a95 = 0u64;
            if nat_eq(a94, a95) {
                7u64
            } else {
                let a96 = v0;
                let a97 = 2u64;
                if nat_eq(a96, a97) {
                    6u64
                } else {
                    let a98 = v0;
                    let a99 = 5u64;
                    if nat_eq(a98, a99) {
                        5u64
                    } else {
                        4u64
                    }
                }
            }
        }, v1)
    } else {
        let a100 = v0;
        let a101 = 1u64;
        if if if nat_eq(a100, a101) {
            true
        } else {
            let a102 = v0;
            let a103 = 3u64;
            nat_eq(a102, a103)
        } {
            true
        } else {
            let a104 = v0;
            let a105 = 4u64;
            nat_eq(a104, a105)
        } {
            Adt1::C1({
                let a106 = v0;
                let a107 = 1u64;
                if nat_eq(a106, a107) {
                    0u64
                } else {
                    let a108 = v0;
                    let a109 = 3u64;
                    if nat_eq(a108, a109) {
                        1u64
                    } else {
                        2u64
                    }
                }
            }, v1)
        } else {
            let a110 = v0;
            let a111 = 7u64;
            if if nat_eq(a110, a111) {
                true
            } else {
                let a112 = v0;
                let a113 = 8u64;
                nat_eq(a112, a113)
            } {
                Adt1::C2({
                    let a114 = v0;
                    let a115 = 7u64;
                    nat_sub(a114, a115)
                }, v1)
            } else {
                Adt1::C3({
                    let a116 = v0;
                    let a117 = 9u64;
                    nat_sub(a116, a117)
                }, v1)
            }
        }
    }
}

pub fn f8(v0: u64) -> R<u64> {
    let v1: u64 = {
        let a121 = {
            let a118 = v0;
            let a119 = 32768u64;
            let a120 = 0u64;
            nat_quot(a118, a119, a120)
        };
        let a122 = 2147483648u64;
        nat_mul(a121, a122)?
    };
    let v2: u64 = {
        let a126 = {
            let a123 = v0;
            let a124 = 32768u64;
            let a125 = 0u64;
            nat_rem(a123, a124, a125)
        };
        let a127 = 1024u64;
        let a128 = 0u64;
        nat_quot(a126, a127, a128)
    };
    let v3: u64 = {
        let a129 = v0;
        let a130 = 1024u64;
        let a131 = 0u64;
        nat_rem(a129, a130, a131)
    };
    let a166 = v1;
    let a167 = {
        let a132 = v2;
        let a133 = 31u64;
        if nat_eq(a132, a133) {
            let a142 = 2139095040u64;
            let a143 = {
                let a140 = {
                    let a134 = 0u64;
                    let a135 = v3;
                    if if nat_lt(a134, a135) {
                        let a136 = v3;
                        let a137 = 512u64;
                        nat_lt(a136, a137)
                    } else {
                        false
                    } {
                        let a138 = v3;
                        let a139 = 512u64;
                        nat_add(a138, a139)?
                    } else {
                        v3
                    }
                };
                let a141 = 8192u64;
                nat_mul(a140, a141)?
            };
            nat_add(a142, a143)?
        } else {
            let a144 = v2;
            let a145 = 0u64;
            if nat_eq(a144, a145) {
                let a146 = v3;
                let a147 = 0u64;
                if nat_eq(a146, a147) {
                    0u64
                } else {
                    let v4: Adt2 = f16(113u64, v3, 0u64, 1024u64)?;
                    let a156 = {
                        let a149 = match v4.clone() {
                            Adt2::C0(h148, _, _, _) => {
                                h148
                            }
                        };
                        let a150 = 8388608u64;
                        nat_mul(a149, a150)?
                    };
                    let a157 = {
                        let a154 = {
                            let a152 = match v4 {
                                Adt2::C0(_, h151, _, _) => {
                                    h151
                                }
                            };
                            let a153 = 1024u64;
                            nat_sub(a152, a153)
                        };
                        let a155 = 8192u64;
                        nat_mul(a154, a155)?
                    };
                    nat_add(a156, a157)?
                }
            } else {
                let a164 = {
                    let a160 = {
                        let a158 = v2;
                        let a159 = 112u64;
                        nat_add(a158, a159)?
                    };
                    let a161 = 8388608u64;
                    nat_mul(a160, a161)?
                };
                let a165 = {
                    let a162 = v3;
                    let a163 = 8192u64;
                    nat_mul(a162, a163)?
                };
                nat_add(a164, a165)?
            }
        }
    };
    nat_add(a166, a167)
}

pub fn f9(v0: u64) -> R<u64> {
    let a192 = v0;
    let a193 = f14({
        let a190 = {
            let a188 = {
                let a168 = 1u64;
                let a169 = 1u64;
                nat_add(a168, a169)?
            };
            let a189 = {
                let a186 = {
                    let a184 = {
                        let a170 = 1u64;
                        let a171 = 1u64;
                        nat_add(a170, a171)?
                    };
                    let a185 = {
                        let a182 = {
                            let a180 = {
                                let a172 = 1u64;
                                let a173 = 1u64;
                                nat_add(a172, a173)?
                            };
                            let a181 = {
                                let a178 = {
                                    let a176 = {
                                        let a174 = 1u64;
                                        let a175 = 1u64;
                                        nat_add(a174, a175)?
                                    };
                                    let a177 = 1u64;
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
    })?;
    let a194 = 0u64;
    Ok(nat_quot(a192, a193, a194))
}

pub fn f10(v0: u64) -> R<u64> {
    let a217 = f17(v0)?;
    let a218 = f14({
        let a215 = {
            let a213 = {
                let a195 = 1u64;
                let a196 = 1u64;
                nat_add(a195, a196)?
            };
            let a214 = {
                let a211 = {
                    let a209 = {
                        let a197 = 1u64;
                        let a198 = 1u64;
                        nat_add(a197, a198)?
                    };
                    let a210 = {
                        let a207 = {
                            let a205 = {
                                let a199 = 1u64;
                                let a200 = 1u64;
                                nat_add(a199, a200)?
                            };
                            let a206 = {
                                let a203 = {
                                    let a201 = 1u64;
                                    let a202 = 1u64;
                                    nat_add(a201, a202)?
                                };
                                let a204 = 1u64;
                                nat_mul(a203, a204)?
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
    })?;
    let a219 = 0u64;
    Ok(nat_quot(a217, a218, a219))
}

pub fn f11(v0: u64) -> R<u64> {
    let a242 = v0;
    let a243 = f14({
        let a240 = {
            let a238 = {
                let a220 = 1u64;
                let a221 = 1u64;
                nat_add(a220, a221)?
            };
            let a239 = {
                let a236 = {
                    let a234 = {
                        let a222 = 1u64;
                        let a223 = 1u64;
                        nat_add(a222, a223)?
                    };
                    let a235 = {
                        let a232 = {
                            let a230 = {
                                let a224 = 1u64;
                                let a225 = 1u64;
                                nat_add(a224, a225)?
                            };
                            let a231 = {
                                let a228 = {
                                    let a226 = 1u64;
                                    let a227 = 1u64;
                                    nat_add(a226, a227)?
                                };
                                let a229 = 1u64;
                                nat_mul(a228, a229)?
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
            };
            nat_mul(a238, a239)?
        };
        let a241 = 1u64;
        nat_add(a240, a241)?
    })?;
    let a244 = 0u64;
    Ok(nat_rem(a242, a243, a244))
}

pub fn f12(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = {
        let a245 = v0;
        let a246 = v1;
        let a247 = 0u64;
        nat_quot(a245, a246, a247)
    };
    let v3: u64 = {
        let a251 = {
            let a248 = v0;
            let a249 = v1;
            let a250 = 0u64;
            nat_rem(a248, a249, a250)
        };
        let a252 = 2u64;
        nat_mul(a251, a252)?
    };
    let a262 = v2;
    let a263 = {
        let a253 = v1;
        let a254 = v3;
        if if nat_lt(a253, a254) {
            true
        } else {
            let a255 = v3;
            let a256 = v1;
            if nat_eq(a255, a256) {
                let a260 = {
                    let a257 = v2;
                    let a258 = 2u64;
                    let a259 = 0u64;
                    nat_rem(a257, a258, a259)
                };
                let a261 = 1u64;
                nat_eq(a260, a261)
            } else {
                false
            }
        } {
            1u64
        } else {
            0u64
        }
    };
    nat_add(a262, a263)
}

pub fn f13(v0: u64) -> R<u64> {
    let a288 = f11(v0)?;
    let a289 = {
        let a264 = f10(v0)?;
        let a265 = 0u64;
        if nat_eq(a264, a265) {
            0u64
        } else {
            f14({
                let a286 = {
                    let a284 = {
                        let a266 = 1u64;
                        let a267 = 1u64;
                        nat_add(a266, a267)?
                    };
                    let a285 = {
                        let a282 = {
                            let a280 = {
                                let a268 = 1u64;
                                let a269 = 1u64;
                                nat_add(a268, a269)?
                            };
                            let a281 = {
                                let a278 = {
                                    let a276 = {
                                        let a270 = 1u64;
                                        let a271 = 1u64;
                                        nat_add(a270, a271)?
                                    };
                                    let a277 = {
                                        let a274 = {
                                            let a272 = 1u64;
                                            let a273 = 1u64;
                                            nat_add(a272, a273)?
                                        };
                                        let a275 = 1u64;
                                        nat_mul(a274, a275)?
                                    };
                                    nat_mul(a276, a277)?
                                };
                                let a279 = 1u64;
                                nat_add(a278, a279)?
                            };
                            nat_mul(a280, a281)?
                        };
                        let a283 = 1u64;
                        nat_add(a282, a283)?
                    };
                    nat_mul(a284, a285)?
                };
                let a287 = 1u64;
                nat_add(a286, a287)?
            })?
        }
    };
    nat_add(a288, a289)
}

pub fn f14(v0: u64) -> R<u64> {
    f18(Fn0::F19, v0, 1u64)
}

pub fn f15(v0: u64) -> R<bool> {
    let a332 = f10(v0)?;
    let a333 = {
        let a330 = {
            let a328 = {
                let a290 = 1u64;
                let a291 = 1u64;
                nat_add(a290, a291)?
            };
            let a329 = {
                let a326 = {
                    let a324 = {
                        let a292 = 1u64;
                        let a293 = 1u64;
                        nat_add(a292, a293)?
                    };
                    let a325 = {
                        let a322 = {
                            let a320 = {
                                let a294 = 1u64;
                                let a295 = 1u64;
                                nat_add(a294, a295)?
                            };
                            let a321 = {
                                let a318 = {
                                    let a316 = {
                                        let a296 = 1u64;
                                        let a297 = 1u64;
                                        nat_add(a296, a297)?
                                    };
                                    let a317 = {
                                        let a314 = {
                                            let a312 = {
                                                let a298 = 1u64;
                                                let a299 = 1u64;
                                                nat_add(a298, a299)?
                                            };
                                            let a313 = {
                                                let a310 = {
                                                    let a308 = {
                                                        let a300 = 1u64;
                                                        let a301 = 1u64;
                                                        nat_add(a300, a301)?
                                                    };
                                                    let a309 = {
                                                        let a306 = {
                                                            let a304 = {
                                                                let a302 = 1u64;
                                                                let a303 = 1u64;
                                                                nat_add(a302, a303)?
                                                            };
                                                            let a305 = 1u64;
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
    if nat_eq(a332, a333) {
        let a334 = 0u64;
        let a335 = f11(v0)?;
        Ok(nat_lt(a334, a335))
    } else {
        Ok(false)
    }
}

pub fn f16(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt2> {
    let (h336, _) = f20(Fn1::F21, {
        let a355 = {
            let a337 = 1u64;
            let a338 = 1u64;
            nat_add(a337, a338)?
        };
        let a356 = {
            let a353 = {
                let a339 = 1u64;
                let a340 = 1u64;
                nat_add(a339, a340)?
            };
            let a354 = {
                let a351 = {
                    let a341 = 1u64;
                    let a342 = 1u64;
                    nat_add(a341, a342)?
                };
                let a352 = {
                    let a349 = {
                        let a343 = 1u64;
                        let a344 = 1u64;
                        nat_add(a343, a344)?
                    };
                    let a350 = {
                        let a347 = {
                            let a345 = 1u64;
                            let a346 = 1u64;
                            nat_add(a345, a346)?
                        };
                        let a348 = 1u64;
                        nat_mul(a347, a348)?
                    };
                    nat_mul(a349, a350)?
                };
                nat_mul(a351, a352)?
            };
            nat_mul(a353, a354)?
        };
        nat_mul(a355, a356)?
    }, Adt2::C0(v0, v1, v2, v3))?;
    Ok(h336)
}

pub fn f17(v0: u64) -> R<u64> {
    let a381 = v0;
    let a382 = f14({
        let a379 = {
            let a377 = {
                let a357 = 1u64;
                let a358 = 1u64;
                nat_add(a357, a358)?
            };
            let a378 = {
                let a375 = {
                    let a373 = {
                        let a359 = 1u64;
                        let a360 = 1u64;
                        nat_add(a359, a360)?
                    };
                    let a374 = {
                        let a371 = {
                            let a369 = {
                                let a361 = 1u64;
                                let a362 = 1u64;
                                nat_add(a361, a362)?
                            };
                            let a370 = {
                                let a367 = {
                                    let a365 = {
                                        let a363 = 1u64;
                                        let a364 = 1u64;
                                        nat_add(a363, a364)?
                                    };
                                    let a366 = 1u64;
                                    nat_mul(a365, a366)?
                                };
                                let a368 = 1u64;
                                nat_add(a367, a368)?
                            };
                            nat_mul(a369, a370)?
                        };
                        let a372 = 1u64;
                        nat_add(a371, a372)?
                    };
                    nat_mul(a373, a374)?
                };
                let a376 = 1u64;
                nat_add(a375, a376)?
            };
            nat_mul(a377, a378)?
        };
        let a380 = 1u64;
        nat_add(a379, a380)?
    })?;
    let a383 = 0u64;
    Ok(nat_rem(a381, a382, a383))
}

pub fn f18(v0: Fn0, p387: u64, p388: u64) -> R<u64> {
    let m386 = (p387, p388);
    {
        let mut m386 = m386;
        loop {
            let step = {
                let (v1, v2) = m386;
                let m384 = v1;
                if m384 == 0 {
                    Err::<(u64, u64), R<u64>>(Ok(v2))
                } else {
                    let v3 = m384 - 1;
                    Ok::<(u64, u64), R<u64>>((v3, {
                        let c385: Fn0 = v0.clone();
                        c385.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m386 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f19(v0: u64) -> R<u64> {
    let a389 = v0;
    let a390 = v0;
    nat_add(a389, a390)
}

pub fn f20(v0: Fn1, p395: u64, p396: Adt2) -> R<(Adt2, bool)> {
    let m394 = (p395, p396);
    {
        let mut m394 = m394;
        loop {
            let step = {
                let (v1, v2) = m394;
                let m391 = v1;
                if m391 == 0 {
                    Err::<(u64, Adt2), R<(Adt2, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m391 - 1;
                    let m393 = {
                        let c392: Fn1 = v0.clone();
                        c392.apply(v2.clone())?
                    };
                    match m393 {
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
                Ok(next) => { m394 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f21(v0: Adt2) -> R<Option<Adt2>> {
    f22(v0)
}

pub fn f22(v0: Adt2) -> R<Option<Adt2>> {
    let a398 = 0u64;
    let a399 = match v0.clone() {
        Adt2::C0(_, h397, _, _) => {
            h397
        }
    };
    if if if nat_lt(a398, a399) {
        let a402 = match v0.clone() {
            Adt2::C0(_, h400, _, _) => {
                h400
            }
        };
        let a403 = match v0.clone() {
            Adt2::C0(_, _, _, h401) => {
                h401
            }
        };
        nat_lt(a402, a403)
    } else {
        false
    } {
        let a406 = match v0.clone() {
            Adt2::C0(_, _, h404, _) => {
                h404
            }
        };
        let a407 = match v0.clone() {
            Adt2::C0(h405, _, _, _) => {
                h405
            }
        };
        nat_lt(a406, a407)
    } else {
        false
    } {
        Ok(Some(Adt2::C0({
            let a409 = match v0.clone() {
                Adt2::C0(h408, _, _, _) => {
                    h408
                }
            };
            let a410 = 1u64;
            nat_sub(a409, a410)
        }, {
            let a413 = match v0.clone() {
                Adt2::C0(_, h411, _, _) => {
                    h411
                }
            };
            let a414 = match v0.clone() {
                Adt2::C0(_, h412, _, _) => {
                    h412
                }
            };
            nat_add(a413, a414)?
        }, match v0.clone() {
            Adt2::C0(_, _, h415, _) => {
                h415
            }
        }, match v0 {
            Adt2::C0(_, _, _, h416) => {
                h416
            }
        })))
    } else {
        Ok(None::<Adt2>)
    }
}

pub fn run(p0: u64, p1: u64, p2: u64, p3: u64) -> R<Option<Adt0>> {
    f0(p0, p1, p2, p3)
}
