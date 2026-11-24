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
    C1(u64),
    C2(u64, u64),
    C3(u64, u64, u64, u64),
    C4(u64, u64, u64),
    C5(u64, u64, u64, u64),
    C6(u64, u64, u64, u64, u64, u64),
    C7(u64, u64, u64),
    C8(u64, u64, u64, u64, u64, u64),
    C9(u64, u64),
    C10(u64, u64, u64, u64, u64, u64),
    C11(u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(u64, u64),
}

#[derive(Clone)]
pub enum Fn0 {
    F25,
}

impl Fn0 {
    pub fn apply(&self, p0: u64) -> R<u64> {
        match self {
            Fn0::F25 => f25(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F32,
}

impl Fn1 {
    pub fn apply(&self, p0: Adt3) -> R<Option<Adt3>> {
        match self {
            Fn1::F32 => f32(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F39,
}

impl Fn2 {
    pub fn apply(&self, p0: Adt2) -> R<Option<Adt2>> {
        match self {
            Fn2::F39 => f39(p0),
        }
    }
}

pub fn f0(v0: Adt0, v1: u64) -> R<Adt1> {
    let m1 = v0;
    match m1 {
        Adt0::C0(v2, v3) => {
            Ok(Adt1::C0(f1(v3, f2(v2, v2)?)?, 0u64, 0u64))
        }
        Adt0::C1(v4) => {
            Ok(Adt1::C0(f3(f4(v4, 1107296256u64)?)?, 0u64, 0u64))
        }
        Adt0::C2(v5, v6) => {
            let v7: u64 = f5(v5)?;
            Ok(Adt1::C0({
                let a2 = v1;
                let a3 = 2u64;
                if nat_eq(a2, a3) {
                    v7
                } else {
                    f1(v6, v7)?
                }
            }, 0u64, 0u64))
        }
        Adt0::C3(v8, v9, v10, v11) => {
            Ok(Adt1::C0(f1(v10, v9)?, f1(v11, f2(v9, v8)?)?, 0u64))
        }
        Adt0::C4(v12, v13, v14) => {
            let v15: u64 = f4(f6(v14)?, f1(v12, f7(v13)?)?)?;
            Ok(Adt1::C0(v15, f4(1065353216u64, v15)?, 0u64))
        }
        Adt0::C5(v16, v17, v18, v19) => {
            let v20: u64 = f1({
                let a4 = v1;
                let a5 = 4u64;
                if nat_eq(a4, a5) {
                    3212836864u64
                } else {
                    3204448256u64
                }
            }, f2(1036831949u64, f6(v18)?)?)?;
            Ok(Adt1::C0(f4(f1(v20, f6(v19)?)?, f1(v16, f7(v17)?)?)?, 0u64, 0u64))
        }
        Adt0::C6(v21, v22, v23, v24, v25, v26) => {
            let v27: u64 = f6(v23)?;
            let v28: u64 = f2(v22, v27)?;
            Ok(Adt1::C0(f1(v24, v28)?, f1(v25, f2(v28, v27)?)?, f1(v26, f2(v28, v21)?)?))
        }
        Adt0::C7(v29, v30, v31) => {
            Ok(Adt1::C0(f1(f2(v29, v31)?, f7(f2(v30, v30)?)?)?, 0u64, 0u64))
        }
        Adt0::C8(v32, v33, v34, v35, v36, v37) => {
            Ok(Adt1::C0(f4(f1(f2(v32, v36)?, f7(f2(v33, v34)?)?)?, v37)?, f4(f1(f2(v35, v33)?, f7(f2(v34, v36)?)?)?, v37)?, 0u64))
        }
        Adt0::C9(v38, v39) => {
            Ok(Adt1::C0(f4(v38, v39)?, 0u64, 0u64))
        }
        Adt0::C10(v40, v41, v42, v43, v44, v45) => {
            let v46: u64 = f1(f1(f2(v43, f6(v42)?)?, v44)?, f7(v40)?)?;
            let v47: u64 = {
                let a6 = v1;
                let a7 = 2u64;
                if nat_eq(a6, a7) {
                    f5(v46)?
                } else {
                    f2(v46, v46)?
                }
            };
            Ok(Adt1::C0(f1(v45, f2(v41, v47)?)?, 0u64, 0u64))
        }
        Adt0::C11(v48, v49, v50, v51) => {
            Ok(Adt1::C0(f8(f4(f1(v48, v50)?, v49)?, v51, 1262485504u64)?, 0u64, 0u64))
        }
    }
}

pub fn f1(v0: u64, v1: u64) -> R<u64> {
    if if f9(v0)? {
        true
    } else {
        f9(v1)?
    } {
        let a94 = {
            let a72 = {
                let a48 = {
                    let a46 = {
                        let a8 = 1u64;
                        let a9 = 1u64;
                        nat_add(a8, a9)?
                    };
                    let a47 = {
                        let a44 = {
                            let a42 = {
                                let a10 = 1u64;
                                let a11 = 1u64;
                                nat_add(a10, a11)?
                            };
                            let a43 = {
                                let a40 = {
                                    let a38 = {
                                        let a12 = 1u64;
                                        let a13 = 1u64;
                                        nat_add(a12, a13)?
                                    };
                                    let a39 = {
                                        let a36 = {
                                            let a34 = {
                                                let a14 = 1u64;
                                                let a15 = 1u64;
                                                nat_add(a14, a15)?
                                            };
                                            let a35 = {
                                                let a32 = {
                                                    let a30 = {
                                                        let a16 = 1u64;
                                                        let a17 = 1u64;
                                                        nat_add(a16, a17)?
                                                    };
                                                    let a31 = {
                                                        let a28 = {
                                                            let a26 = {
                                                                let a18 = 1u64;
                                                                let a19 = 1u64;
                                                                nat_add(a18, a19)?
                                                            };
                                                            let a27 = {
                                                                let a24 = {
                                                                    let a22 = {
                                                                        let a20 = 1u64;
                                                                        let a21 = 1u64;
                                                                        nat_add(a20, a21)?
                                                                    };
                                                                    let a23 = 1u64;
                                                                    nat_mul(a22, a23)?
                                                                };
                                                                let a25 = 1u64;
                                                                nat_add(a24, a25)?
                                                            };
                                                            nat_mul(a26, a27)?
                                                        };
                                                        let a29 = 1u64;
                                                        nat_add(a28, a29)?
                                                    };
                                                    nat_mul(a30, a31)?
                                                };
                                                let a33 = 1u64;
                                                nat_add(a32, a33)?
                                            };
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
            let a73 = f10({
                let a70 = {
                    let a68 = {
                        let a50 = 1u64;
                        let a51 = 1u64;
                        nat_add(a50, a51)?
                    };
                    let a69 = {
                        let a66 = {
                            let a64 = {
                                let a52 = 1u64;
                                let a53 = 1u64;
                                nat_add(a52, a53)?
                            };
                            let a65 = {
                                let a62 = {
                                    let a60 = {
                                        let a54 = 1u64;
                                        let a55 = 1u64;
                                        nat_add(a54, a55)?
                                    };
                                    let a61 = {
                                        let a58 = {
                                            let a56 = 1u64;
                                            let a57 = 1u64;
                                            nat_add(a56, a57)?
                                        };
                                        let a59 = 1u64;
                                        nat_mul(a58, a59)?
                                    };
                                    nat_mul(a60, a61)?
                                };
                                let a63 = 1u64;
                                nat_add(a62, a63)?
                            };
                            nat_mul(a64, a65)?
                        };
                        let a67 = 1u64;
                        nat_add(a66, a67)?
                    };
                    nat_mul(a68, a69)?
                };
                let a71 = 1u64;
                nat_add(a70, a71)?
            })?;
            nat_mul(a72, a73)?
        };
        let a95 = f10({
            let a92 = {
                let a74 = 1u64;
                let a75 = 1u64;
                nat_add(a74, a75)?
            };
            let a93 = {
                let a90 = {
                    let a88 = {
                        let a76 = 1u64;
                        let a77 = 1u64;
                        nat_add(a76, a77)?
                    };
                    let a89 = {
                        let a86 = {
                            let a84 = {
                                let a78 = 1u64;
                                let a79 = 1u64;
                                nat_add(a78, a79)?
                            };
                            let a85 = {
                                let a82 = {
                                    let a80 = 1u64;
                                    let a81 = 1u64;
                                    nat_add(a80, a81)?
                                };
                                let a83 = 1u64;
                                nat_mul(a82, a83)?
                            };
                            nat_mul(a84, a85)?
                        };
                        let a87 = 1u64;
                        nat_add(a86, a87)?
                    };
                    nat_mul(a88, a89)?
                };
                let a91 = 1u64;
                nat_add(a90, a91)?
            };
            nat_mul(a92, a93)?
        })?;
        nat_add(a94, a95)
    } else {
        if if f11(v0)? {
            f11(v1)?
        } else {
            false
        } {
            let a96 = f12(v0)?;
            let a97 = f12(v1)?;
            if nat_eq(a96, a97) {
                Ok(v0)
            } else {
                let a184 = {
                    let a162 = {
                        let a138 = {
                            let a136 = {
                                let a98 = 1u64;
                                let a99 = 1u64;
                                nat_add(a98, a99)?
                            };
                            let a137 = {
                                let a134 = {
                                    let a132 = {
                                        let a100 = 1u64;
                                        let a101 = 1u64;
                                        nat_add(a100, a101)?
                                    };
                                    let a133 = {
                                        let a130 = {
                                            let a128 = {
                                                let a102 = 1u64;
                                                let a103 = 1u64;
                                                nat_add(a102, a103)?
                                            };
                                            let a129 = {
                                                let a126 = {
                                                    let a124 = {
                                                        let a104 = 1u64;
                                                        let a105 = 1u64;
                                                        nat_add(a104, a105)?
                                                    };
                                                    let a125 = {
                                                        let a122 = {
                                                            let a120 = {
                                                                let a106 = 1u64;
                                                                let a107 = 1u64;
                                                                nat_add(a106, a107)?
                                                            };
                                                            let a121 = {
                                                                let a118 = {
                                                                    let a116 = {
                                                                        let a108 = 1u64;
                                                                        let a109 = 1u64;
                                                                        nat_add(a108, a109)?
                                                                    };
                                                                    let a117 = {
                                                                        let a114 = {
                                                                            let a112 = {
                                                                                let a110 = 1u64;
                                                                                let a111 = 1u64;
                                                                                nat_add(a110, a111)?
                                                                            };
                                                                            let a113 = 1u64;
                                                                            nat_mul(a112, a113)?
                                                                        };
                                                                        let a115 = 1u64;
                                                                        nat_add(a114, a115)?
                                                                    };
                                                                    nat_mul(a116, a117)?
                                                                };
                                                                let a119 = 1u64;
                                                                nat_add(a118, a119)?
                                                            };
                                                            nat_mul(a120, a121)?
                                                        };
                                                        let a123 = 1u64;
                                                        nat_add(a122, a123)?
                                                    };
                                                    nat_mul(a124, a125)?
                                                };
                                                let a127 = 1u64;
                                                nat_add(a126, a127)?
                                            };
                                            nat_mul(a128, a129)?
                                        };
                                        let a131 = 1u64;
                                        nat_add(a130, a131)?
                                    };
                                    nat_mul(a132, a133)?
                                };
                                let a135 = 1u64;
                                nat_add(a134, a135)?
                            };
                            nat_mul(a136, a137)?
                        };
                        let a139 = 1u64;
                        nat_add(a138, a139)?
                    };
                    let a163 = f10({
                        let a160 = {
                            let a158 = {
                                let a140 = 1u64;
                                let a141 = 1u64;
                                nat_add(a140, a141)?
                            };
                            let a159 = {
                                let a156 = {
                                    let a154 = {
                                        let a142 = 1u64;
                                        let a143 = 1u64;
                                        nat_add(a142, a143)?
                                    };
                                    let a155 = {
                                        let a152 = {
                                            let a150 = {
                                                let a144 = 1u64;
                                                let a145 = 1u64;
                                                nat_add(a144, a145)?
                                            };
                                            let a151 = {
                                                let a148 = {
                                                    let a146 = 1u64;
                                                    let a147 = 1u64;
                                                    nat_add(a146, a147)?
                                                };
                                                let a149 = 1u64;
                                                nat_mul(a148, a149)?
                                            };
                                            nat_mul(a150, a151)?
                                        };
                                        let a153 = 1u64;
                                        nat_add(a152, a153)?
                                    };
                                    nat_mul(a154, a155)?
                                };
                                let a157 = 1u64;
                                nat_add(a156, a157)?
                            };
                            nat_mul(a158, a159)?
                        };
                        let a161 = 1u64;
                        nat_add(a160, a161)?
                    })?;
                    nat_mul(a162, a163)?
                };
                let a185 = f10({
                    let a182 = {
                        let a164 = 1u64;
                        let a165 = 1u64;
                        nat_add(a164, a165)?
                    };
                    let a183 = {
                        let a180 = {
                            let a178 = {
                                let a166 = 1u64;
                                let a167 = 1u64;
                                nat_add(a166, a167)?
                            };
                            let a179 = {
                                let a176 = {
                                    let a174 = {
                                        let a168 = 1u64;
                                        let a169 = 1u64;
                                        nat_add(a168, a169)?
                                    };
                                    let a175 = {
                                        let a172 = {
                                            let a170 = 1u64;
                                            let a171 = 1u64;
                                            nat_add(a170, a171)?
                                        };
                                        let a173 = 1u64;
                                        nat_mul(a172, a173)?
                                    };
                                    nat_mul(a174, a175)?
                                };
                                let a177 = 1u64;
                                nat_add(a176, a177)?
                            };
                            nat_mul(a178, a179)?
                        };
                        let a181 = 1u64;
                        nat_add(a180, a181)?
                    };
                    nat_mul(a182, a183)?
                })?;
                nat_add(a184, a185)
            }
        } else {
            if f11(v0)? {
                Ok(v0)
            } else {
                if f11(v1)? {
                    Ok(v1)
                } else {
                    let a186 = f5(v1)?;
                    let a187 = f5(v0)?;
                    if nat_le(a186, a187) {
                        f13(v0, v1)
                    } else {
                        f13(v1, v0)
                    }
                }
            }
        }
    }
}

pub fn f2(v0: u64, v1: u64) -> R<u64> {
    if if if if f9(v0)? {
        true
    } else {
        f9(v1)?
    } {
        true
    } else {
        if f11(v0)? {
            f14(v1)?
        } else {
            false
        }
    } {
        true
    } else {
        if f11(v1)? {
            f14(v0)?
        } else {
            false
        }
    } {
        let a274 = {
            let a252 = {
                let a228 = {
                    let a226 = {
                        let a188 = 1u64;
                        let a189 = 1u64;
                        nat_add(a188, a189)?
                    };
                    let a227 = {
                        let a224 = {
                            let a222 = {
                                let a190 = 1u64;
                                let a191 = 1u64;
                                nat_add(a190, a191)?
                            };
                            let a223 = {
                                let a220 = {
                                    let a218 = {
                                        let a192 = 1u64;
                                        let a193 = 1u64;
                                        nat_add(a192, a193)?
                                    };
                                    let a219 = {
                                        let a216 = {
                                            let a214 = {
                                                let a194 = 1u64;
                                                let a195 = 1u64;
                                                nat_add(a194, a195)?
                                            };
                                            let a215 = {
                                                let a212 = {
                                                    let a210 = {
                                                        let a196 = 1u64;
                                                        let a197 = 1u64;
                                                        nat_add(a196, a197)?
                                                    };
                                                    let a211 = {
                                                        let a208 = {
                                                            let a206 = {
                                                                let a198 = 1u64;
                                                                let a199 = 1u64;
                                                                nat_add(a198, a199)?
                                                            };
                                                            let a207 = {
                                                                let a204 = {
                                                                    let a202 = {
                                                                        let a200 = 1u64;
                                                                        let a201 = 1u64;
                                                                        nat_add(a200, a201)?
                                                                    };
                                                                    let a203 = 1u64;
                                                                    nat_mul(a202, a203)?
                                                                };
                                                                let a205 = 1u64;
                                                                nat_add(a204, a205)?
                                                            };
                                                            nat_mul(a206, a207)?
                                                        };
                                                        let a209 = 1u64;
                                                        nat_add(a208, a209)?
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
            let a253 = f10({
                let a250 = {
                    let a248 = {
                        let a230 = 1u64;
                        let a231 = 1u64;
                        nat_add(a230, a231)?
                    };
                    let a249 = {
                        let a246 = {
                            let a244 = {
                                let a232 = 1u64;
                                let a233 = 1u64;
                                nat_add(a232, a233)?
                            };
                            let a245 = {
                                let a242 = {
                                    let a240 = {
                                        let a234 = 1u64;
                                        let a235 = 1u64;
                                        nat_add(a234, a235)?
                                    };
                                    let a241 = {
                                        let a238 = {
                                            let a236 = 1u64;
                                            let a237 = 1u64;
                                            nat_add(a236, a237)?
                                        };
                                        let a239 = 1u64;
                                        nat_mul(a238, a239)?
                                    };
                                    nat_mul(a240, a241)?
                                };
                                let a243 = 1u64;
                                nat_add(a242, a243)?
                            };
                            nat_mul(a244, a245)?
                        };
                        let a247 = 1u64;
                        nat_add(a246, a247)?
                    };
                    nat_mul(a248, a249)?
                };
                let a251 = 1u64;
                nat_add(a250, a251)?
            })?;
            nat_mul(a252, a253)?
        };
        let a275 = f10({
            let a272 = {
                let a254 = 1u64;
                let a255 = 1u64;
                nat_add(a254, a255)?
            };
            let a273 = {
                let a270 = {
                    let a268 = {
                        let a256 = 1u64;
                        let a257 = 1u64;
                        nat_add(a256, a257)?
                    };
                    let a269 = {
                        let a266 = {
                            let a264 = {
                                let a258 = 1u64;
                                let a259 = 1u64;
                                nat_add(a258, a259)?
                            };
                            let a265 = {
                                let a262 = {
                                    let a260 = 1u64;
                                    let a261 = 1u64;
                                    nat_add(a260, a261)?
                                };
                                let a263 = 1u64;
                                nat_mul(a262, a263)?
                            };
                            nat_mul(a264, a265)?
                        };
                        let a267 = 1u64;
                        nat_add(a266, a267)?
                    };
                    nat_mul(a268, a269)?
                };
                let a271 = 1u64;
                nat_add(a270, a271)?
            };
            nat_mul(a272, a273)?
        })?;
        nat_add(a274, a275)
    } else {
        if if f11(v0)? {
            true
        } else {
            f11(v1)?
        } {
            f15(f16(v0, v1)?)
        } else {
            if if f14(v0)? {
                true
            } else {
                f14(v1)?
            } {
                f17(f16(v0, v1)?)
            } else {
                f18(v0, v1)
            }
        }
    }
}

pub fn f3(v0: u64) -> R<u64> {
    let m276 = f19(v0)?;
    match m276 {
        None => {
            f20(v0)
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f4(v0: u64, v1: u64) -> R<u64> {
    if if if if f9(v0)? {
        true
    } else {
        f9(v1)?
    } {
        true
    } else {
        if f11(v0)? {
            f11(v1)?
        } else {
            false
        }
    } {
        true
    } else {
        if f14(v0)? {
            f14(v1)?
        } else {
            false
        }
    } {
        let a363 = {
            let a341 = {
                let a317 = {
                    let a315 = {
                        let a277 = 1u64;
                        let a278 = 1u64;
                        nat_add(a277, a278)?
                    };
                    let a316 = {
                        let a313 = {
                            let a311 = {
                                let a279 = 1u64;
                                let a280 = 1u64;
                                nat_add(a279, a280)?
                            };
                            let a312 = {
                                let a309 = {
                                    let a307 = {
                                        let a281 = 1u64;
                                        let a282 = 1u64;
                                        nat_add(a281, a282)?
                                    };
                                    let a308 = {
                                        let a305 = {
                                            let a303 = {
                                                let a283 = 1u64;
                                                let a284 = 1u64;
                                                nat_add(a283, a284)?
                                            };
                                            let a304 = {
                                                let a301 = {
                                                    let a299 = {
                                                        let a285 = 1u64;
                                                        let a286 = 1u64;
                                                        nat_add(a285, a286)?
                                                    };
                                                    let a300 = {
                                                        let a297 = {
                                                            let a295 = {
                                                                let a287 = 1u64;
                                                                let a288 = 1u64;
                                                                nat_add(a287, a288)?
                                                            };
                                                            let a296 = {
                                                                let a293 = {
                                                                    let a291 = {
                                                                        let a289 = 1u64;
                                                                        let a290 = 1u64;
                                                                        nat_add(a289, a290)?
                                                                    };
                                                                    let a292 = 1u64;
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
                            nat_mul(a311, a312)?
                        };
                        let a314 = 1u64;
                        nat_add(a313, a314)?
                    };
                    nat_mul(a315, a316)?
                };
                let a318 = 1u64;
                nat_add(a317, a318)?
            };
            let a342 = f10({
                let a339 = {
                    let a337 = {
                        let a319 = 1u64;
                        let a320 = 1u64;
                        nat_add(a319, a320)?
                    };
                    let a338 = {
                        let a335 = {
                            let a333 = {
                                let a321 = 1u64;
                                let a322 = 1u64;
                                nat_add(a321, a322)?
                            };
                            let a334 = {
                                let a331 = {
                                    let a329 = {
                                        let a323 = 1u64;
                                        let a324 = 1u64;
                                        nat_add(a323, a324)?
                                    };
                                    let a330 = {
                                        let a327 = {
                                            let a325 = 1u64;
                                            let a326 = 1u64;
                                            nat_add(a325, a326)?
                                        };
                                        let a328 = 1u64;
                                        nat_mul(a327, a328)?
                                    };
                                    nat_mul(a329, a330)?
                                };
                                let a332 = 1u64;
                                nat_add(a331, a332)?
                            };
                            nat_mul(a333, a334)?
                        };
                        let a336 = 1u64;
                        nat_add(a335, a336)?
                    };
                    nat_mul(a337, a338)?
                };
                let a340 = 1u64;
                nat_add(a339, a340)?
            })?;
            nat_mul(a341, a342)?
        };
        let a364 = f10({
            let a361 = {
                let a343 = 1u64;
                let a344 = 1u64;
                nat_add(a343, a344)?
            };
            let a362 = {
                let a359 = {
                    let a357 = {
                        let a345 = 1u64;
                        let a346 = 1u64;
                        nat_add(a345, a346)?
                    };
                    let a358 = {
                        let a355 = {
                            let a353 = {
                                let a347 = 1u64;
                                let a348 = 1u64;
                                nat_add(a347, a348)?
                            };
                            let a354 = {
                                let a351 = {
                                    let a349 = 1u64;
                                    let a350 = 1u64;
                                    nat_add(a349, a350)?
                                };
                                let a352 = 1u64;
                                nat_mul(a351, a352)?
                            };
                            nat_mul(a353, a354)?
                        };
                        let a356 = 1u64;
                        nat_add(a355, a356)?
                    };
                    nat_mul(a357, a358)?
                };
                let a360 = 1u64;
                nat_add(a359, a360)?
            };
            nat_mul(a361, a362)?
        })?;
        nat_add(a363, a364)
    } else {
        if if f11(v0)? {
            true
        } else {
            f14(v1)?
        } {
            f15(f16(v0, v1)?)
        } else {
            if if f11(v1)? {
                true
            } else {
                f14(v0)?
            } {
                f17(f16(v0, v1)?)
            } else {
                f21(v0, v1)
            }
        }
    }
}

pub fn f5(v0: u64) -> R<u64> {
    let a389 = v0;
    let a390 = f10({
        let a387 = {
            let a385 = {
                let a365 = 1u64;
                let a366 = 1u64;
                nat_add(a365, a366)?
            };
            let a386 = {
                let a383 = {
                    let a381 = {
                        let a367 = 1u64;
                        let a368 = 1u64;
                        nat_add(a367, a368)?
                    };
                    let a382 = {
                        let a379 = {
                            let a377 = {
                                let a369 = 1u64;
                                let a370 = 1u64;
                                nat_add(a369, a370)?
                            };
                            let a378 = {
                                let a375 = {
                                    let a373 = {
                                        let a371 = 1u64;
                                        let a372 = 1u64;
                                        nat_add(a371, a372)?
                                    };
                                    let a374 = 1u64;
                                    nat_mul(a373, a374)?
                                };
                                let a376 = 1u64;
                                nat_add(a375, a376)?
                            };
                            nat_mul(a377, a378)?
                        };
                        let a380 = 1u64;
                        nat_add(a379, a380)?
                    };
                    nat_mul(a381, a382)?
                };
                let a384 = 1u64;
                nat_add(a383, a384)?
            };
            nat_mul(a385, a386)?
        };
        let a388 = 1u64;
        nat_add(a387, a388)?
    })?;
    let a391 = 0u64;
    Ok(nat_rem(a389, a390, a391))
}

pub fn f6(v0: u64) -> R<u64> {
    let a392 = v0;
    let a393 = 0u64;
    if nat_eq(a392, a393) {
        Ok(0u64)
    } else {
        let a394 = v0;
        let a395 = 2u64;
        if nat_lt(a394, a395) {
            let a400 = 1065353216u64;
            let a401 = {
                let a398 = {
                    let a396 = v0;
                    let a397 = 1u64;
                    nat_sub(a396, a397)
                };
                let a399 = 8388608u64;
                nat_mul(a398, a399)?
            };
            nat_add(a400, a401)
        } else {
            let a402 = v0;
            let a403 = 4u64;
            if nat_lt(a402, a403) {
                let a408 = 1073741824u64;
                let a409 = {
                    let a406 = {
                        let a404 = v0;
                        let a405 = 2u64;
                        nat_sub(a404, a405)
                    };
                    let a407 = 4194304u64;
                    nat_mul(a406, a407)?
                };
                nat_add(a408, a409)
            } else {
                let a410 = v0;
                let a411 = 8u64;
                if nat_lt(a410, a411) {
                    let a416 = 1082130432u64;
                    let a417 = {
                        let a414 = {
                            let a412 = v0;
                            let a413 = 4u64;
                            nat_sub(a412, a413)
                        };
                        let a415 = 2097152u64;
                        nat_mul(a414, a415)?
                    };
                    nat_add(a416, a417)
                } else {
                    let a418 = v0;
                    let a419 = 16u64;
                    if nat_lt(a418, a419) {
                        let a424 = 1090519040u64;
                        let a425 = {
                            let a422 = {
                                let a420 = v0;
                                let a421 = 8u64;
                                nat_sub(a420, a421)
                            };
                            let a423 = 1048576u64;
                            nat_mul(a422, a423)?
                        };
                        nat_add(a424, a425)
                    } else {
                        let a426 = v0;
                        let a427 = 32u64;
                        if nat_lt(a426, a427) {
                            let a432 = 1098907648u64;
                            let a433 = {
                                let a430 = {
                                    let a428 = v0;
                                    let a429 = 16u64;
                                    nat_sub(a428, a429)
                                };
                                let a431 = 524288u64;
                                nat_mul(a430, a431)?
                            };
                            nat_add(a432, a433)
                        } else {
                            let a434 = v0;
                            let a435 = 64u64;
                            if nat_lt(a434, a435) {
                                let a440 = 1107296256u64;
                                let a441 = {
                                    let a438 = {
                                        let a436 = v0;
                                        let a437 = 32u64;
                                        nat_sub(a436, a437)
                                    };
                                    let a439 = 262144u64;
                                    nat_mul(a438, a439)?
                                };
                                nat_add(a440, a441)
                            } else {
                                let a442 = v0;
                                let a443 = 128u64;
                                if nat_lt(a442, a443) {
                                    let a448 = 1115684864u64;
                                    let a449 = {
                                        let a446 = {
                                            let a444 = v0;
                                            let a445 = 64u64;
                                            nat_sub(a444, a445)
                                        };
                                        let a447 = 131072u64;
                                        nat_mul(a446, a447)?
                                    };
                                    nat_add(a448, a449)
                                } else {
                                    let a450 = v0;
                                    let a451 = 256u64;
                                    if nat_lt(a450, a451) {
                                        let a456 = 1124073472u64;
                                        let a457 = {
                                            let a454 = {
                                                let a452 = v0;
                                                let a453 = 128u64;
                                                nat_sub(a452, a453)
                                            };
                                            let a455 = 65536u64;
                                            nat_mul(a454, a455)?
                                        };
                                        nat_add(a456, a457)
                                    } else {
                                        Ok(1132462080u64)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f7(v0: u64) -> R<u64> {
    let a504 = {
        let a482 = v0;
        let a483 = f10({
            let a480 = {
                let a478 = {
                    let a458 = 1u64;
                    let a459 = 1u64;
                    nat_add(a458, a459)?
                };
                let a479 = {
                    let a476 = {
                        let a474 = {
                            let a460 = 1u64;
                            let a461 = 1u64;
                            nat_add(a460, a461)?
                        };
                        let a475 = {
                            let a472 = {
                                let a470 = {
                                    let a462 = 1u64;
                                    let a463 = 1u64;
                                    nat_add(a462, a463)?
                                };
                                let a471 = {
                                    let a468 = {
                                        let a466 = {
                                            let a464 = 1u64;
                                            let a465 = 1u64;
                                            nat_add(a464, a465)?
                                        };
                                        let a467 = 1u64;
                                        nat_mul(a466, a467)?
                                    };
                                    let a469 = 1u64;
                                    nat_add(a468, a469)?
                                };
                                nat_mul(a470, a471)?
                            };
                            let a473 = 1u64;
                            nat_add(a472, a473)?
                        };
                        nat_mul(a474, a475)?
                    };
                    let a477 = 1u64;
                    nat_add(a476, a477)?
                };
                nat_mul(a478, a479)?
            };
            let a481 = 1u64;
            nat_add(a480, a481)?
        })?;
        nat_add(a482, a483)?
    };
    let a505 = f10({
        let a502 = {
            let a484 = 1u64;
            let a485 = 1u64;
            nat_add(a484, a485)?
        };
        let a503 = {
            let a500 = {
                let a486 = 1u64;
                let a487 = 1u64;
                nat_add(a486, a487)?
            };
            let a501 = {
                let a498 = {
                    let a488 = 1u64;
                    let a489 = 1u64;
                    nat_add(a488, a489)?
                };
                let a499 = {
                    let a496 = {
                        let a490 = 1u64;
                        let a491 = 1u64;
                        nat_add(a490, a491)?
                    };
                    let a497 = {
                        let a494 = {
                            let a492 = 1u64;
                            let a493 = 1u64;
                            nat_add(a492, a493)?
                        };
                        let a495 = 1u64;
                        nat_mul(a494, a495)?
                    };
                    nat_mul(a496, a497)?
                };
                nat_mul(a498, a499)?
            };
            nat_mul(a500, a501)?
        };
        nat_mul(a502, a503)?
    })?;
    let a506 = 0u64;
    Ok(nat_rem(a504, a505, a506))
}

pub fn f8(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let v3: u64 = f1(v0, v2)?;
    let v4: u64 = {
        let a507 = v3;
        let a508 = v2;
        nat_sub(a507, a508)
    };
    let a509 = v4;
    let a510 = v1;
    if nat_lt(a509, a510) {
        Ok(v4)
    } else {
        Ok(v1)
    }
}

pub fn f9(v0: u64) -> R<bool> {
    let a553 = f22(v0)?;
    let a554 = {
        let a551 = {
            let a549 = {
                let a511 = 1u64;
                let a512 = 1u64;
                nat_add(a511, a512)?
            };
            let a550 = {
                let a547 = {
                    let a545 = {
                        let a513 = 1u64;
                        let a514 = 1u64;
                        nat_add(a513, a514)?
                    };
                    let a546 = {
                        let a543 = {
                            let a541 = {
                                let a515 = 1u64;
                                let a516 = 1u64;
                                nat_add(a515, a516)?
                            };
                            let a542 = {
                                let a539 = {
                                    let a537 = {
                                        let a517 = 1u64;
                                        let a518 = 1u64;
                                        nat_add(a517, a518)?
                                    };
                                    let a538 = {
                                        let a535 = {
                                            let a533 = {
                                                let a519 = 1u64;
                                                let a520 = 1u64;
                                                nat_add(a519, a520)?
                                            };
                                            let a534 = {
                                                let a531 = {
                                                    let a529 = {
                                                        let a521 = 1u64;
                                                        let a522 = 1u64;
                                                        nat_add(a521, a522)?
                                                    };
                                                    let a530 = {
                                                        let a527 = {
                                                            let a525 = {
                                                                let a523 = 1u64;
                                                                let a524 = 1u64;
                                                                nat_add(a523, a524)?
                                                            };
                                                            let a526 = 1u64;
                                                            nat_mul(a525, a526)?
                                                        };
                                                        let a528 = 1u64;
                                                        nat_add(a527, a528)?
                                                    };
                                                    nat_mul(a529, a530)?
                                                };
                                                let a532 = 1u64;
                                                nat_add(a531, a532)?
                                            };
                                            nat_mul(a533, a534)?
                                        };
                                        let a536 = 1u64;
                                        nat_add(a535, a536)?
                                    };
                                    nat_mul(a537, a538)?
                                };
                                let a540 = 1u64;
                                nat_add(a539, a540)?
                            };
                            nat_mul(a541, a542)?
                        };
                        let a544 = 1u64;
                        nat_add(a543, a544)?
                    };
                    nat_mul(a545, a546)?
                };
                let a548 = 1u64;
                nat_add(a547, a548)?
            };
            nat_mul(a549, a550)?
        };
        let a552 = 1u64;
        nat_add(a551, a552)?
    };
    if nat_eq(a553, a554) {
        let a555 = 0u64;
        let a556 = f23(v0)?;
        Ok(nat_lt(a555, a556))
    } else {
        Ok(false)
    }
}

pub fn f10(v0: u64) -> R<u64> {
    f24(Fn0::F25, v0, 1u64)
}

pub fn f11(v0: u64) -> R<bool> {
    let a599 = f22(v0)?;
    let a600 = {
        let a597 = {
            let a595 = {
                let a557 = 1u64;
                let a558 = 1u64;
                nat_add(a557, a558)?
            };
            let a596 = {
                let a593 = {
                    let a591 = {
                        let a559 = 1u64;
                        let a560 = 1u64;
                        nat_add(a559, a560)?
                    };
                    let a592 = {
                        let a589 = {
                            let a587 = {
                                let a561 = 1u64;
                                let a562 = 1u64;
                                nat_add(a561, a562)?
                            };
                            let a588 = {
                                let a585 = {
                                    let a583 = {
                                        let a563 = 1u64;
                                        let a564 = 1u64;
                                        nat_add(a563, a564)?
                                    };
                                    let a584 = {
                                        let a581 = {
                                            let a579 = {
                                                let a565 = 1u64;
                                                let a566 = 1u64;
                                                nat_add(a565, a566)?
                                            };
                                            let a580 = {
                                                let a577 = {
                                                    let a575 = {
                                                        let a567 = 1u64;
                                                        let a568 = 1u64;
                                                        nat_add(a567, a568)?
                                                    };
                                                    let a576 = {
                                                        let a573 = {
                                                            let a571 = {
                                                                let a569 = 1u64;
                                                                let a570 = 1u64;
                                                                nat_add(a569, a570)?
                                                            };
                                                            let a572 = 1u64;
                                                            nat_mul(a571, a572)?
                                                        };
                                                        let a574 = 1u64;
                                                        nat_add(a573, a574)?
                                                    };
                                                    nat_mul(a575, a576)?
                                                };
                                                let a578 = 1u64;
                                                nat_add(a577, a578)?
                                            };
                                            nat_mul(a579, a580)?
                                        };
                                        let a582 = 1u64;
                                        nat_add(a581, a582)?
                                    };
                                    nat_mul(a583, a584)?
                                };
                                let a586 = 1u64;
                                nat_add(a585, a586)?
                            };
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
    if nat_eq(a599, a600) {
        let a601 = f23(v0)?;
        let a602 = 0u64;
        Ok(nat_eq(a601, a602))
    } else {
        Ok(false)
    }
}

pub fn f12(v0: u64) -> R<u64> {
    let a627 = v0;
    let a628 = f10({
        let a625 = {
            let a623 = {
                let a603 = 1u64;
                let a604 = 1u64;
                nat_add(a603, a604)?
            };
            let a624 = {
                let a621 = {
                    let a619 = {
                        let a605 = 1u64;
                        let a606 = 1u64;
                        nat_add(a605, a606)?
                    };
                    let a620 = {
                        let a617 = {
                            let a615 = {
                                let a607 = 1u64;
                                let a608 = 1u64;
                                nat_add(a607, a608)?
                            };
                            let a616 = {
                                let a613 = {
                                    let a611 = {
                                        let a609 = 1u64;
                                        let a610 = 1u64;
                                        nat_add(a609, a610)?
                                    };
                                    let a612 = 1u64;
                                    nat_mul(a611, a612)?
                                };
                                let a614 = 1u64;
                                nat_add(a613, a614)?
                            };
                            nat_mul(a615, a616)?
                        };
                        let a618 = 1u64;
                        nat_add(a617, a618)?
                    };
                    nat_mul(a619, a620)?
                };
                let a622 = 1u64;
                nat_add(a621, a622)?
            };
            nat_mul(a623, a624)?
        };
        let a626 = 1u64;
        nat_add(a625, a626)?
    })?;
    let a629 = 0u64;
    Ok(nat_quot(a627, a628, a629))
}

pub fn f13(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f26(v0)?;
    let v3: u64 = {
        let a642 = f27(v0)?;
        let a643 = {
            let a640 = {
                let a630 = 1u64;
                let a631 = 1u64;
                nat_add(a630, a631)?
            };
            let a641 = {
                let a638 = {
                    let a632 = 1u64;
                    let a633 = 1u64;
                    nat_add(a632, a633)?
                };
                let a639 = {
                    let a636 = {
                        let a634 = 1u64;
                        let a635 = 1u64;
                        nat_add(a634, a635)?
                    };
                    let a637 = 1u64;
                    nat_mul(a636, a637)?
                };
                nat_mul(a638, a639)?
            };
            nat_mul(a640, a641)?
        };
        nat_mul(a642, a643)?
    };
    let v4: u64 = f28({
        let a656 = f27(v1)?;
        let a657 = {
            let a654 = {
                let a644 = 1u64;
                let a645 = 1u64;
                nat_add(a644, a645)?
            };
            let a655 = {
                let a652 = {
                    let a646 = 1u64;
                    let a647 = 1u64;
                    nat_add(a646, a647)?
                };
                let a653 = {
                    let a650 = {
                        let a648 = 1u64;
                        let a649 = 1u64;
                        nat_add(a648, a649)?
                    };
                    let a651 = 1u64;
                    nat_mul(a650, a651)?
                };
                nat_mul(a652, a653)?
            };
            nat_mul(a654, a655)?
        };
        nat_mul(a656, a657)?
    }, {
        let a658 = v2;
        let a659 = f26(v1)?;
        nat_sub(a658, a659)
    })?;
    let a660 = f12(v0)?;
    let a661 = f12(v1)?;
    if nat_eq(a660, a661) {
        f29(f12(v0)?, v2, {
            let a662 = v3;
            let a663 = v4;
            nat_add(a662, a663)?
        })
    } else {
        let a664 = v3;
        let a665 = v4;
        if nat_eq(a664, a665) {
            Ok(0u64)
        } else {
            f29(f12(v0)?, v2, {
                let a666 = v3;
                let a667 = v4;
                nat_sub(a666, a667)
            })
        }
    }
}

pub fn f14(v0: u64) -> R<bool> {
    let a668 = f5(v0)?;
    let a669 = 0u64;
    Ok(nat_eq(a668, a669))
}

pub fn f15(v0: u64) -> R<u64> {
    let a736 = f17(v0)?;
    let a737 = {
        let a734 = {
            let a710 = {
                let a708 = {
                    let a670 = 1u64;
                    let a671 = 1u64;
                    nat_add(a670, a671)?
                };
                let a709 = {
                    let a706 = {
                        let a704 = {
                            let a672 = 1u64;
                            let a673 = 1u64;
                            nat_add(a672, a673)?
                        };
                        let a705 = {
                            let a702 = {
                                let a700 = {
                                    let a674 = 1u64;
                                    let a675 = 1u64;
                                    nat_add(a674, a675)?
                                };
                                let a701 = {
                                    let a698 = {
                                        let a696 = {
                                            let a676 = 1u64;
                                            let a677 = 1u64;
                                            nat_add(a676, a677)?
                                        };
                                        let a697 = {
                                            let a694 = {
                                                let a692 = {
                                                    let a678 = 1u64;
                                                    let a679 = 1u64;
                                                    nat_add(a678, a679)?
                                                };
                                                let a693 = {
                                                    let a690 = {
                                                        let a688 = {
                                                            let a680 = 1u64;
                                                            let a681 = 1u64;
                                                            nat_add(a680, a681)?
                                                        };
                                                        let a689 = {
                                                            let a686 = {
                                                                let a684 = {
                                                                    let a682 = 1u64;
                                                                    let a683 = 1u64;
                                                                    nat_add(a682, a683)?
                                                                };
                                                                let a685 = 1u64;
                                                                nat_mul(a684, a685)?
                                                            };
                                                            let a687 = 1u64;
                                                            nat_add(a686, a687)?
                                                        };
                                                        nat_mul(a688, a689)?
                                                    };
                                                    let a691 = 1u64;
                                                    nat_add(a690, a691)?
                                                };
                                                nat_mul(a692, a693)?
                                            };
                                            let a695 = 1u64;
                                            nat_add(a694, a695)?
                                        };
                                        nat_mul(a696, a697)?
                                    };
                                    let a699 = 1u64;
                                    nat_add(a698, a699)?
                                };
                                nat_mul(a700, a701)?
                            };
                            let a703 = 1u64;
                            nat_add(a702, a703)?
                        };
                        nat_mul(a704, a705)?
                    };
                    let a707 = 1u64;
                    nat_add(a706, a707)?
                };
                nat_mul(a708, a709)?
            };
            let a711 = 1u64;
            nat_add(a710, a711)?
        };
        let a735 = f10({
            let a732 = {
                let a730 = {
                    let a712 = 1u64;
                    let a713 = 1u64;
                    nat_add(a712, a713)?
                };
                let a731 = {
                    let a728 = {
                        let a726 = {
                            let a714 = 1u64;
                            let a715 = 1u64;
                            nat_add(a714, a715)?
                        };
                        let a727 = {
                            let a724 = {
                                let a722 = {
                                    let a716 = 1u64;
                                    let a717 = 1u64;
                                    nat_add(a716, a717)?
                                };
                                let a723 = {
                                    let a720 = {
                                        let a718 = 1u64;
                                        let a719 = 1u64;
                                        nat_add(a718, a719)?
                                    };
                                    let a721 = 1u64;
                                    nat_mul(a720, a721)?
                                };
                                nat_mul(a722, a723)?
                            };
                            let a725 = 1u64;
                            nat_add(a724, a725)?
                        };
                        nat_mul(a726, a727)?
                    };
                    let a729 = 1u64;
                    nat_add(a728, a729)?
                };
                nat_mul(a730, a731)?
            };
            let a733 = 1u64;
            nat_add(a732, a733)?
        })?;
        nat_mul(a734, a735)?
    };
    nat_add(a736, a737)
}

pub fn f16(v0: u64, v1: u64) -> R<u64> {
    let a744 = {
        let a738 = f12(v0)?;
        let a739 = f12(v1)?;
        nat_add(a738, a739)?
    };
    let a745 = {
        let a742 = {
            let a740 = 1u64;
            let a741 = 1u64;
            nat_add(a740, a741)?
        };
        let a743 = 1u64;
        nat_mul(a742, a743)?
    };
    let a746 = 0u64;
    Ok(nat_rem(a744, a745, a746))
}

pub fn f17(v0: u64) -> R<u64> {
    let a771 = v0;
    let a772 = f10({
        let a769 = {
            let a767 = {
                let a747 = 1u64;
                let a748 = 1u64;
                nat_add(a747, a748)?
            };
            let a768 = {
                let a765 = {
                    let a763 = {
                        let a749 = 1u64;
                        let a750 = 1u64;
                        nat_add(a749, a750)?
                    };
                    let a764 = {
                        let a761 = {
                            let a759 = {
                                let a751 = 1u64;
                                let a752 = 1u64;
                                nat_add(a751, a752)?
                            };
                            let a760 = {
                                let a757 = {
                                    let a755 = {
                                        let a753 = 1u64;
                                        let a754 = 1u64;
                                        nat_add(a753, a754)?
                                    };
                                    let a756 = 1u64;
                                    nat_mul(a755, a756)?
                                };
                                let a758 = 1u64;
                                nat_add(a757, a758)?
                            };
                            nat_mul(a759, a760)?
                        };
                        let a762 = 1u64;
                        nat_add(a761, a762)?
                    };
                    nat_mul(a763, a764)?
                };
                let a766 = 1u64;
                nat_add(a765, a766)?
            };
            nat_mul(a767, a768)?
        };
        let a770 = 1u64;
        nat_add(a769, a770)?
    })?;
    nat_mul(a771, a772)
}

pub fn f18(v0: u64, v1: u64) -> R<u64> {
    let v2: Adt2 = f30(v0)?;
    let v3: Adt2 = f30(v1)?;
    f29(f16(v0, v1)?, {
        let a827 = {
            let a775 = match v2.clone() {
                Adt2::C0(h773, _, _, _) => {
                    h773
                }
            };
            let a776 = match v3.clone() {
                Adt2::C0(h774, _, _, _) => {
                    h774
                }
            };
            nat_add(a775, a776)?
        };
        let a828 = {
            let a825 = {
                let a823 = {
                    let a777 = 1u64;
                    let a778 = 1u64;
                    nat_add(a777, a778)?
                };
                let a824 = {
                    let a821 = {
                        let a819 = {
                            let a779 = 1u64;
                            let a780 = 1u64;
                            nat_add(a779, a780)?
                        };
                        let a820 = {
                            let a817 = {
                                let a815 = {
                                    let a781 = 1u64;
                                    let a782 = 1u64;
                                    nat_add(a781, a782)?
                                };
                                let a816 = {
                                    let a813 = {
                                        let a811 = {
                                            let a783 = 1u64;
                                            let a784 = 1u64;
                                            nat_add(a783, a784)?
                                        };
                                        let a812 = {
                                            let a809 = {
                                                let a807 = {
                                                    let a785 = 1u64;
                                                    let a786 = 1u64;
                                                    nat_add(a785, a786)?
                                                };
                                                let a808 = {
                                                    let a805 = {
                                                        let a803 = {
                                                            let a787 = 1u64;
                                                            let a788 = 1u64;
                                                            nat_add(a787, a788)?
                                                        };
                                                        let a804 = {
                                                            let a801 = {
                                                                let a799 = {
                                                                    let a789 = 1u64;
                                                                    let a790 = 1u64;
                                                                    nat_add(a789, a790)?
                                                                };
                                                                let a800 = {
                                                                    let a797 = {
                                                                        let a791 = 1u64;
                                                                        let a792 = 1u64;
                                                                        nat_add(a791, a792)?
                                                                    };
                                                                    let a798 = {
                                                                        let a795 = {
                                                                            let a793 = 1u64;
                                                                            let a794 = 1u64;
                                                                            nat_add(a793, a794)?
                                                                        };
                                                                        let a796 = 1u64;
                                                                        nat_mul(a795, a796)?
                                                                    };
                                                                    nat_mul(a797, a798)?
                                                                };
                                                                nat_mul(a799, a800)?
                                                            };
                                                            let a802 = 1u64;
                                                            nat_add(a801, a802)?
                                                        };
                                                        nat_mul(a803, a804)?
                                                    };
                                                    let a806 = 1u64;
                                                    nat_add(a805, a806)?
                                                };
                                                nat_mul(a807, a808)?
                                            };
                                            let a810 = 1u64;
                                            nat_add(a809, a810)?
                                        };
                                        nat_mul(a811, a812)?
                                    };
                                    let a814 = 1u64;
                                    nat_add(a813, a814)?
                                };
                                nat_mul(a815, a816)?
                            };
                            let a818 = 1u64;
                            nat_add(a817, a818)?
                        };
                        nat_mul(a819, a820)?
                    };
                    let a822 = 1u64;
                    nat_add(a821, a822)?
                };
                nat_mul(a823, a824)?
            };
            let a826 = 1u64;
            nat_add(a825, a826)?
        };
        nat_sub(a827, a828)
    }, f28({
        let a831 = match v2 {
            Adt2::C0(_, h829, _, _) => {
                h829
            }
        };
        let a832 = match v3 {
            Adt2::C0(_, h830, _, _) => {
                h830
            }
        };
        nat_mul(a831, a832)?
    }, {
        let a849 = {
            let a833 = 1u64;
            let a834 = 1u64;
            nat_add(a833, a834)?
        };
        let a850 = {
            let a847 = {
                let a835 = 1u64;
                let a836 = 1u64;
                nat_add(a835, a836)?
            };
            let a848 = {
                let a845 = {
                    let a843 = {
                        let a837 = 1u64;
                        let a838 = 1u64;
                        nat_add(a837, a838)?
                    };
                    let a844 = {
                        let a841 = {
                            let a839 = 1u64;
                            let a840 = 1u64;
                            nat_add(a839, a840)?
                        };
                        let a842 = 1u64;
                        nat_mul(a841, a842)?
                    };
                    nat_mul(a843, a844)?
                };
                let a846 = 1u64;
                nat_add(a845, a846)?
            };
            nat_mul(a847, a848)?
        };
        nat_mul(a849, a850)?
    })?)
}

pub fn f19(v0: u64) -> R<Option<u64>> {
    if if f9(v0)? {
        true
    } else {
        let a851 = f12(v0)?;
        let a852 = 1u64;
        if nat_eq(a851, a852) {
            let a853 = 0u64;
            let a854 = f5(v0)?;
            nat_lt(a853, a854)
        } else {
            false
        }
    } {
        Ok(Some({
            let a941 = {
                let a919 = {
                    let a895 = {
                        let a893 = {
                            let a855 = 1u64;
                            let a856 = 1u64;
                            nat_add(a855, a856)?
                        };
                        let a894 = {
                            let a891 = {
                                let a889 = {
                                    let a857 = 1u64;
                                    let a858 = 1u64;
                                    nat_add(a857, a858)?
                                };
                                let a890 = {
                                    let a887 = {
                                        let a885 = {
                                            let a859 = 1u64;
                                            let a860 = 1u64;
                                            nat_add(a859, a860)?
                                        };
                                        let a886 = {
                                            let a883 = {
                                                let a881 = {
                                                    let a861 = 1u64;
                                                    let a862 = 1u64;
                                                    nat_add(a861, a862)?
                                                };
                                                let a882 = {
                                                    let a879 = {
                                                        let a877 = {
                                                            let a863 = 1u64;
                                                            let a864 = 1u64;
                                                            nat_add(a863, a864)?
                                                        };
                                                        let a878 = {
                                                            let a875 = {
                                                                let a873 = {
                                                                    let a865 = 1u64;
                                                                    let a866 = 1u64;
                                                                    nat_add(a865, a866)?
                                                                };
                                                                let a874 = {
                                                                    let a871 = {
                                                                        let a869 = {
                                                                            let a867 = 1u64;
                                                                            let a868 = 1u64;
                                                                            nat_add(a867, a868)?
                                                                        };
                                                                        let a870 = 1u64;
                                                                        nat_mul(a869, a870)?
                                                                    };
                                                                    let a872 = 1u64;
                                                                    nat_add(a871, a872)?
                                                                };
                                                                nat_mul(a873, a874)?
                                                            };
                                                            let a876 = 1u64;
                                                            nat_add(a875, a876)?
                                                        };
                                                        nat_mul(a877, a878)?
                                                    };
                                                    let a880 = 1u64;
                                                    nat_add(a879, a880)?
                                                };
                                                nat_mul(a881, a882)?
                                            };
                                            let a884 = 1u64;
                                            nat_add(a883, a884)?
                                        };
                                        nat_mul(a885, a886)?
                                    };
                                    let a888 = 1u64;
                                    nat_add(a887, a888)?
                                };
                                nat_mul(a889, a890)?
                            };
                            let a892 = 1u64;
                            nat_add(a891, a892)?
                        };
                        nat_mul(a893, a894)?
                    };
                    let a896 = 1u64;
                    nat_add(a895, a896)?
                };
                let a920 = f10({
                    let a917 = {
                        let a915 = {
                            let a897 = 1u64;
                            let a898 = 1u64;
                            nat_add(a897, a898)?
                        };
                        let a916 = {
                            let a913 = {
                                let a911 = {
                                    let a899 = 1u64;
                                    let a900 = 1u64;
                                    nat_add(a899, a900)?
                                };
                                let a912 = {
                                    let a909 = {
                                        let a907 = {
                                            let a901 = 1u64;
                                            let a902 = 1u64;
                                            nat_add(a901, a902)?
                                        };
                                        let a908 = {
                                            let a905 = {
                                                let a903 = 1u64;
                                                let a904 = 1u64;
                                                nat_add(a903, a904)?
                                            };
                                            let a906 = 1u64;
                                            nat_mul(a905, a906)?
                                        };
                                        nat_mul(a907, a908)?
                                    };
                                    let a910 = 1u64;
                                    nat_add(a909, a910)?
                                };
                                nat_mul(a911, a912)?
                            };
                            let a914 = 1u64;
                            nat_add(a913, a914)?
                        };
                        nat_mul(a915, a916)?
                    };
                    let a918 = 1u64;
                    nat_add(a917, a918)?
                })?;
                nat_mul(a919, a920)?
            };
            let a942 = f10({
                let a939 = {
                    let a921 = 1u64;
                    let a922 = 1u64;
                    nat_add(a921, a922)?
                };
                let a940 = {
                    let a937 = {
                        let a935 = {
                            let a923 = 1u64;
                            let a924 = 1u64;
                            nat_add(a923, a924)?
                        };
                        let a936 = {
                            let a933 = {
                                let a931 = {
                                    let a925 = 1u64;
                                    let a926 = 1u64;
                                    nat_add(a925, a926)?
                                };
                                let a932 = {
                                    let a929 = {
                                        let a927 = 1u64;
                                        let a928 = 1u64;
                                        nat_add(a927, a928)?
                                    };
                                    let a930 = 1u64;
                                    nat_mul(a929, a930)?
                                };
                                nat_mul(a931, a932)?
                            };
                            let a934 = 1u64;
                            nat_add(a933, a934)?
                        };
                        nat_mul(a935, a936)?
                    };
                    let a938 = 1u64;
                    nat_add(a937, a938)?
                };
                nat_mul(a939, a940)?
            })?;
            nat_add(a941, a942)?
        }))
    } else {
        if if f14(v0)? {
            true
        } else {
            f11(v0)?
        } {
            Ok(Some(v0))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f20(v0: u64) -> R<u64> {
    let v1: Adt2 = f30(v0)?;
    let v2: u64 = {
        let a951 = {
            let a948 = match v1.clone() {
                Adt2::C0(h943, _, _, _) => {
                    h943
                }
            };
            let a949 = {
                let a946 = {
                    let a944 = 1u64;
                    let a945 = 1u64;
                    nat_add(a944, a945)?
                };
                let a947 = 1u64;
                nat_mul(a946, a947)?
            };
            let a950 = 0u64;
            nat_rem(a948, a949, a950)
        };
        let a952 = 0u64;
        if nat_eq(a951, a952) {
            let a958 = match v1.clone() {
                Adt2::C0(_, h953, _, _) => {
                    h953
                }
            };
            let a959 = {
                let a956 = {
                    let a954 = 1u64;
                    let a955 = 1u64;
                    nat_add(a954, a955)?
                };
                let a957 = 1u64;
                nat_mul(a956, a957)?
            };
            nat_mul(a958, a959)?
        } else {
            match v1.clone() {
                Adt2::C0(_, h960, _, _) => {
                    h960
                }
            }
        }
    };
    let v3: u64 = {
        let a983 = v2;
        let a984 = f10({
            let a981 = {
                let a979 = {
                    let a961 = 1u64;
                    let a962 = 1u64;
                    nat_add(a961, a962)?
                };
                let a980 = {
                    let a977 = {
                        let a963 = 1u64;
                        let a964 = 1u64;
                        nat_add(a963, a964)?
                    };
                    let a978 = {
                        let a975 = {
                            let a973 = {
                                let a965 = 1u64;
                                let a966 = 1u64;
                                nat_add(a965, a966)?
                            };
                            let a974 = {
                                let a971 = {
                                    let a969 = {
                                        let a967 = 1u64;
                                        let a968 = 1u64;
                                        nat_add(a967, a968)?
                                    };
                                    let a970 = 1u64;
                                    nat_mul(a969, a970)?
                                };
                                let a972 = 1u64;
                                nat_add(a971, a972)?
                            };
                            nat_mul(a973, a974)?
                        };
                        let a976 = 1u64;
                        nat_add(a975, a976)?
                    };
                    nat_mul(a977, a978)?
                };
                nat_mul(a979, a980)?
            };
            let a982 = 1u64;
            nat_add(a981, a982)?
        })?;
        nat_mul(a983, a984)?
    };
    let v4: Adt3 = {
        let (h985, _) = f31(Fn1::F32, {
            let a1004 = {
                let a986 = 1u64;
                let a987 = 1u64;
                nat_add(a986, a987)?
            };
            let a1005 = {
                let a1002 = {
                    let a988 = 1u64;
                    let a989 = 1u64;
                    nat_add(a988, a989)?
                };
                let a1003 = {
                    let a1000 = {
                        let a990 = 1u64;
                        let a991 = 1u64;
                        nat_add(a990, a991)?
                    };
                    let a1001 = {
                        let a998 = {
                            let a992 = 1u64;
                            let a993 = 1u64;
                            nat_add(a992, a993)?
                        };
                        let a999 = {
                            let a996 = {
                                let a994 = 1u64;
                                let a995 = 1u64;
                                nat_add(a994, a995)?
                            };
                            let a997 = 1u64;
                            nat_mul(a996, a997)?
                        };
                        nat_mul(a998, a999)?
                    };
                    nat_mul(a1000, a1001)?
                };
                nat_mul(a1002, a1003)?
            };
            nat_mul(a1004, a1005)?
        }, Adt3::C0(v3, f10({
            let a1026 = {
                let a1024 = {
                    let a1006 = 1u64;
                    let a1007 = 1u64;
                    nat_add(a1006, a1007)?
                };
                let a1025 = {
                    let a1022 = {
                        let a1020 = {
                            let a1008 = 1u64;
                            let a1009 = 1u64;
                            nat_add(a1008, a1009)?
                        };
                        let a1021 = {
                            let a1018 = {
                                let a1010 = 1u64;
                                let a1011 = 1u64;
                                nat_add(a1010, a1011)?
                            };
                            let a1019 = {
                                let a1016 = {
                                    let a1014 = {
                                        let a1012 = 1u64;
                                        let a1013 = 1u64;
                                        nat_add(a1012, a1013)?
                                    };
                                    let a1015 = 1u64;
                                    nat_mul(a1014, a1015)?
                                };
                                let a1017 = 1u64;
                                nat_add(a1016, a1017)?
                            };
                            nat_mul(a1018, a1019)?
                        };
                        nat_mul(a1020, a1021)?
                    };
                    let a1023 = 1u64;
                    nat_add(a1022, a1023)?
                };
                nat_mul(a1024, a1025)?
            };
            let a1027 = 1u64;
            nat_add(a1026, a1027)?
        })?))?;
        h985
    };
    let v5: bool = {
        let a1032 = {
            let a1030 = match v4.clone() {
                Adt3::C0(_, h1028) => {
                    h1028
                }
            };
            let a1031 = match v4.clone() {
                Adt3::C0(_, h1029) => {
                    h1029
                }
            };
            nat_mul(a1030, a1031)?
        };
        let a1033 = v3;
        nat_lt(a1032, a1033)
    };
    f33(0u64, {
        let a1091 = {
            let a1085 = match v1 {
                Adt2::C0(h1034, _, _, _) => {
                    h1034
                }
            };
            let a1086 = {
                let a1083 = {
                    let a1081 = {
                        let a1035 = 1u64;
                        let a1036 = 1u64;
                        nat_add(a1035, a1036)?
                    };
                    let a1082 = {
                        let a1079 = {
                            let a1077 = {
                                let a1037 = 1u64;
                                let a1038 = 1u64;
                                nat_add(a1037, a1038)?
                            };
                            let a1078 = {
                                let a1075 = {
                                    let a1073 = {
                                        let a1039 = 1u64;
                                        let a1040 = 1u64;
                                        nat_add(a1039, a1040)?
                                    };
                                    let a1074 = {
                                        let a1071 = {
                                            let a1069 = {
                                                let a1041 = 1u64;
                                                let a1042 = 1u64;
                                                nat_add(a1041, a1042)?
                                            };
                                            let a1070 = {
                                                let a1067 = {
                                                    let a1065 = {
                                                        let a1043 = 1u64;
                                                        let a1044 = 1u64;
                                                        nat_add(a1043, a1044)?
                                                    };
                                                    let a1066 = {
                                                        let a1063 = {
                                                            let a1061 = {
                                                                let a1045 = 1u64;
                                                                let a1046 = 1u64;
                                                                nat_add(a1045, a1046)?
                                                            };
                                                            let a1062 = {
                                                                let a1059 = {
                                                                    let a1057 = {
                                                                        let a1047 = 1u64;
                                                                        let a1048 = 1u64;
                                                                        nat_add(a1047, a1048)?
                                                                    };
                                                                    let a1058 = {
                                                                        let a1055 = {
                                                                            let a1049 = 1u64;
                                                                            let a1050 = 1u64;
                                                                            nat_add(a1049, a1050)?
                                                                        };
                                                                        let a1056 = {
                                                                            let a1053 = {
                                                                                let a1051 = 1u64;
                                                                                let a1052 = 1u64;
                                                                                nat_add(a1051, a1052)?
                                                                            };
                                                                            let a1054 = 1u64;
                                                                            nat_mul(a1053, a1054)?
                                                                        };
                                                                        nat_mul(a1055, a1056)?
                                                                    };
                                                                    nat_mul(a1057, a1058)?
                                                                };
                                                                let a1060 = 1u64;
                                                                nat_add(a1059, a1060)?
                                                            };
                                                            nat_mul(a1061, a1062)?
                                                        };
                                                        let a1064 = 1u64;
                                                        nat_add(a1063, a1064)?
                                                    };
                                                    nat_mul(a1065, a1066)?
                                                };
                                                let a1068 = 1u64;
                                                nat_add(a1067, a1068)?
                                            };
                                            nat_mul(a1069, a1070)?
                                        };
                                        let a1072 = 1u64;
                                        nat_add(a1071, a1072)?
                                    };
                                    nat_mul(a1073, a1074)?
                                };
                                let a1076 = 1u64;
                                nat_add(a1075, a1076)?
                            };
                            nat_mul(a1077, a1078)?
                        };
                        let a1080 = 1u64;
                        nat_add(a1079, a1080)?
                    };
                    nat_mul(a1081, a1082)?
                };
                let a1084 = 1u64;
                nat_add(a1083, a1084)?
            };
            nat_add(a1085, a1086)?
        };
        let a1092 = {
            let a1089 = {
                let a1087 = 1u64;
                let a1088 = 1u64;
                nat_add(a1087, a1088)?
            };
            let a1090 = 1u64;
            nat_mul(a1089, a1090)?
        };
        let a1093 = 0u64;
        nat_quot(a1091, a1092, a1093)
    }, f34(match v4 {
        Adt3::C0(_, h1094) => {
            h1094
        }
    }, v5)?)
}

pub fn f21(v0: u64, v1: u64) -> R<u64> {
    let v2: Adt2 = f30(v0)?;
    let v3: Adt2 = f30(v1)?;
    let v4: bool = {
        let a1097 = match v2.clone() {
            Adt2::C0(_, h1095, _, _) => {
                h1095
            }
        };
        let a1098 = match v3.clone() {
            Adt2::C0(_, h1096, _, _) => {
                h1096
            }
        };
        nat_lt(a1097, a1098)
    };
    let v5: u64 = {
        let a1142 = match v2.clone() {
            Adt2::C0(_, h1099, _, _) => {
                h1099
            }
        };
        let a1143 = f10(if v4 {
            let a1120 = {
                let a1118 = {
                    let a1100 = 1u64;
                    let a1101 = 1u64;
                    nat_add(a1100, a1101)?
                };
                let a1119 = {
                    let a1116 = {
                        let a1114 = {
                            let a1102 = 1u64;
                            let a1103 = 1u64;
                            nat_add(a1102, a1103)?
                        };
                        let a1115 = {
                            let a1112 = {
                                let a1104 = 1u64;
                                let a1105 = 1u64;
                                nat_add(a1104, a1105)?
                            };
                            let a1113 = {
                                let a1110 = {
                                    let a1108 = {
                                        let a1106 = 1u64;
                                        let a1107 = 1u64;
                                        nat_add(a1106, a1107)?
                                    };
                                    let a1109 = 1u64;
                                    nat_mul(a1108, a1109)?
                                };
                                let a1111 = 1u64;
                                nat_add(a1110, a1111)?
                            };
                            nat_mul(a1112, a1113)?
                        };
                        nat_mul(a1114, a1115)?
                    };
                    let a1117 = 1u64;
                    nat_add(a1116, a1117)?
                };
                nat_mul(a1118, a1119)?
            };
            let a1121 = 1u64;
            nat_add(a1120, a1121)?
        } else {
            let a1140 = {
                let a1122 = 1u64;
                let a1123 = 1u64;
                nat_add(a1122, a1123)?
            };
            let a1141 = {
                let a1138 = {
                    let a1136 = {
                        let a1124 = 1u64;
                        let a1125 = 1u64;
                        nat_add(a1124, a1125)?
                    };
                    let a1137 = {
                        let a1134 = {
                            let a1126 = 1u64;
                            let a1127 = 1u64;
                            nat_add(a1126, a1127)?
                        };
                        let a1135 = {
                            let a1132 = {
                                let a1130 = {
                                    let a1128 = 1u64;
                                    let a1129 = 1u64;
                                    nat_add(a1128, a1129)?
                                };
                                let a1131 = 1u64;
                                nat_mul(a1130, a1131)?
                            };
                            let a1133 = 1u64;
                            nat_add(a1132, a1133)?
                        };
                        nat_mul(a1134, a1135)?
                    };
                    nat_mul(a1136, a1137)?
                };
                let a1139 = 1u64;
                nat_add(a1138, a1139)?
            };
            nat_mul(a1140, a1141)?
        })?;
        nat_mul(a1142, a1143)?
    };
    let v6: u64 = {
        let a1200 = {
            let a1198 = {
                let a1195 = match v2 {
                    Adt2::C0(h1144, _, _, _) => {
                        h1144
                    }
                };
                let a1196 = {
                    let a1193 = {
                        let a1191 = {
                            let a1145 = 1u64;
                            let a1146 = 1u64;
                            nat_add(a1145, a1146)?
                        };
                        let a1192 = {
                            let a1189 = {
                                let a1187 = {
                                    let a1147 = 1u64;
                                    let a1148 = 1u64;
                                    nat_add(a1147, a1148)?
                                };
                                let a1188 = {
                                    let a1185 = {
                                        let a1183 = {
                                            let a1149 = 1u64;
                                            let a1150 = 1u64;
                                            nat_add(a1149, a1150)?
                                        };
                                        let a1184 = {
                                            let a1181 = {
                                                let a1179 = {
                                                    let a1151 = 1u64;
                                                    let a1152 = 1u64;
                                                    nat_add(a1151, a1152)?
                                                };
                                                let a1180 = {
                                                    let a1177 = {
                                                        let a1175 = {
                                                            let a1153 = 1u64;
                                                            let a1154 = 1u64;
                                                            nat_add(a1153, a1154)?
                                                        };
                                                        let a1176 = {
                                                            let a1173 = {
                                                                let a1171 = {
                                                                    let a1155 = 1u64;
                                                                    let a1156 = 1u64;
                                                                    nat_add(a1155, a1156)?
                                                                };
                                                                let a1172 = {
                                                                    let a1169 = {
                                                                        let a1167 = {
                                                                            let a1157 = 1u64;
                                                                            let a1158 = 1u64;
                                                                            nat_add(a1157, a1158)?
                                                                        };
                                                                        let a1168 = {
                                                                            let a1165 = {
                                                                                let a1159 = 1u64;
                                                                                let a1160 = 1u64;
                                                                                nat_add(a1159, a1160)?
                                                                            };
                                                                            let a1166 = {
                                                                                let a1163 = {
                                                                                    let a1161 = 1u64;
                                                                                    let a1162 = 1u64;
                                                                                    nat_add(a1161, a1162)?
                                                                                };
                                                                                let a1164 = 1u64;
                                                                                nat_mul(a1163, a1164)?
                                                                            };
                                                                            nat_mul(a1165, a1166)?
                                                                        };
                                                                        nat_mul(a1167, a1168)?
                                                                    };
                                                                    let a1170 = 1u64;
                                                                    nat_add(a1169, a1170)?
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
                                        };
                                        nat_mul(a1183, a1184)?
                                    };
                                    let a1186 = 1u64;
                                    nat_add(a1185, a1186)?
                                };
                                nat_mul(a1187, a1188)?
                            };
                            let a1190 = 1u64;
                            nat_add(a1189, a1190)?
                        };
                        nat_mul(a1191, a1192)?
                    };
                    let a1194 = 1u64;
                    nat_add(a1193, a1194)?
                };
                nat_add(a1195, a1196)?
            };
            let a1199 = match v3.clone() {
                Adt2::C0(h1197, _, _, _) => {
                    h1197
                }
            };
            nat_sub(a1198, a1199)
        };
        let a1201 = if v4 {
            1u64
        } else {
            0u64
        };
        nat_sub(a1200, a1201)
    };
    let v7: u64 = {
        let a1203 = v5;
        let a1204 = match v3.clone() {
            Adt2::C0(_, h1202, _, _) => {
                h1202
            }
        };
        let a1205 = 0u64;
        nat_quot(a1203, a1204, a1205)
    };
    let v8: bool = {
        let a1210 = 0u64;
        let a1211 = {
            let a1207 = v5;
            let a1208 = match v3 {
                Adt2::C0(_, h1206, _, _) => {
                    h1206
                }
            };
            let a1209 = 0u64;
            nat_rem(a1207, a1208, a1209)
        };
        nat_lt(a1210, a1211)
    };
    f29(f16(v0, v1)?, v6, f34(v7, v8)?)
}

pub fn f22(v0: u64) -> R<u64> {
    let a1234 = f5(v0)?;
    let a1235 = f10({
        let a1232 = {
            let a1230 = {
                let a1212 = 1u64;
                let a1213 = 1u64;
                nat_add(a1212, a1213)?
            };
            let a1231 = {
                let a1228 = {
                    let a1226 = {
                        let a1214 = 1u64;
                        let a1215 = 1u64;
                        nat_add(a1214, a1215)?
                    };
                    let a1227 = {
                        let a1224 = {
                            let a1222 = {
                                let a1216 = 1u64;
                                let a1217 = 1u64;
                                nat_add(a1216, a1217)?
                            };
                            let a1223 = {
                                let a1220 = {
                                    let a1218 = 1u64;
                                    let a1219 = 1u64;
                                    nat_add(a1218, a1219)?
                                };
                                let a1221 = 1u64;
                                nat_mul(a1220, a1221)?
                            };
                            nat_mul(a1222, a1223)?
                        };
                        let a1225 = 1u64;
                        nat_add(a1224, a1225)?
                    };
                    nat_mul(a1226, a1227)?
                };
                let a1229 = 1u64;
                nat_add(a1228, a1229)?
            };
            nat_mul(a1230, a1231)?
        };
        let a1233 = 1u64;
        nat_add(a1232, a1233)?
    })?;
    let a1236 = 0u64;
    Ok(nat_quot(a1234, a1235, a1236))
}

pub fn f23(v0: u64) -> R<u64> {
    let a1259 = v0;
    let a1260 = f10({
        let a1257 = {
            let a1255 = {
                let a1237 = 1u64;
                let a1238 = 1u64;
                nat_add(a1237, a1238)?
            };
            let a1256 = {
                let a1253 = {
                    let a1251 = {
                        let a1239 = 1u64;
                        let a1240 = 1u64;
                        nat_add(a1239, a1240)?
                    };
                    let a1252 = {
                        let a1249 = {
                            let a1247 = {
                                let a1241 = 1u64;
                                let a1242 = 1u64;
                                nat_add(a1241, a1242)?
                            };
                            let a1248 = {
                                let a1245 = {
                                    let a1243 = 1u64;
                                    let a1244 = 1u64;
                                    nat_add(a1243, a1244)?
                                };
                                let a1246 = 1u64;
                                nat_mul(a1245, a1246)?
                            };
                            nat_mul(a1247, a1248)?
                        };
                        let a1250 = 1u64;
                        nat_add(a1249, a1250)?
                    };
                    nat_mul(a1251, a1252)?
                };
                let a1254 = 1u64;
                nat_add(a1253, a1254)?
            };
            nat_mul(a1255, a1256)?
        };
        let a1258 = 1u64;
        nat_add(a1257, a1258)?
    })?;
    let a1261 = 0u64;
    Ok(nat_rem(a1259, a1260, a1261))
}

pub fn f24(v0: Fn0, p1265: u64, p1266: u64) -> R<u64> {
    let m1264 = (p1265, p1266);
    {
        let mut m1264 = m1264;
        loop {
            let step = {
                let (v1, v2) = m1264;
                let m1262 = v1;
                if m1262 == 0 {
                    Err::<(u64, u64), R<u64>>(Ok(v2))
                } else {
                    let v3 = m1262 - 1;
                    Ok::<(u64, u64), R<u64>>((v3, {
                        let c1263: Fn0 = v0.clone();
                        c1263.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1264 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f25(v0: u64) -> R<u64> {
    let a1267 = v0;
    let a1268 = v0;
    nat_add(a1267, a1268)
}

pub fn f26(v0: u64) -> R<u64> {
    let a1307 = {
        let a1269 = f22(v0)?;
        let a1270 = 0u64;
        if nat_eq(a1269, a1270) {
            1u64
        } else {
            f22(v0)?
        }
    };
    let a1308 = {
        let a1305 = {
            let a1271 = 1u64;
            let a1272 = 1u64;
            nat_add(a1271, a1272)?
        };
        let a1306 = {
            let a1303 = {
                let a1273 = 1u64;
                let a1274 = 1u64;
                nat_add(a1273, a1274)?
            };
            let a1304 = {
                let a1301 = {
                    let a1275 = 1u64;
                    let a1276 = 1u64;
                    nat_add(a1275, a1276)?
                };
                let a1302 = {
                    let a1299 = {
                        let a1277 = 1u64;
                        let a1278 = 1u64;
                        nat_add(a1277, a1278)?
                    };
                    let a1300 = {
                        let a1297 = {
                            let a1279 = 1u64;
                            let a1280 = 1u64;
                            nat_add(a1279, a1280)?
                        };
                        let a1298 = {
                            let a1295 = {
                                let a1281 = 1u64;
                                let a1282 = 1u64;
                                nat_add(a1281, a1282)?
                            };
                            let a1296 = {
                                let a1293 = {
                                    let a1283 = 1u64;
                                    let a1284 = 1u64;
                                    nat_add(a1283, a1284)?
                                };
                                let a1294 = {
                                    let a1291 = {
                                        let a1285 = 1u64;
                                        let a1286 = 1u64;
                                        nat_add(a1285, a1286)?
                                    };
                                    let a1292 = {
                                        let a1289 = {
                                            let a1287 = 1u64;
                                            let a1288 = 1u64;
                                            nat_add(a1287, a1288)?
                                        };
                                        let a1290 = 1u64;
                                        nat_mul(a1289, a1290)?
                                    };
                                    nat_mul(a1291, a1292)?
                                };
                                nat_mul(a1293, a1294)?
                            };
                            nat_mul(a1295, a1296)?
                        };
                        nat_mul(a1297, a1298)?
                    };
                    nat_mul(a1299, a1300)?
                };
                nat_mul(a1301, a1302)?
            };
            nat_mul(a1303, a1304)?
        };
        nat_mul(a1305, a1306)?
    };
    nat_add(a1307, a1308)
}

pub fn f27(v0: u64) -> R<u64> {
    let a1333 = f23(v0)?;
    let a1334 = {
        let a1309 = f22(v0)?;
        let a1310 = 0u64;
        if nat_eq(a1309, a1310) {
            0u64
        } else {
            f10({
                let a1331 = {
                    let a1329 = {
                        let a1311 = 1u64;
                        let a1312 = 1u64;
                        nat_add(a1311, a1312)?
                    };
                    let a1330 = {
                        let a1327 = {
                            let a1325 = {
                                let a1313 = 1u64;
                                let a1314 = 1u64;
                                nat_add(a1313, a1314)?
                            };
                            let a1326 = {
                                let a1323 = {
                                    let a1321 = {
                                        let a1315 = 1u64;
                                        let a1316 = 1u64;
                                        nat_add(a1315, a1316)?
                                    };
                                    let a1322 = {
                                        let a1319 = {
                                            let a1317 = 1u64;
                                            let a1318 = 1u64;
                                            nat_add(a1317, a1318)?
                                        };
                                        let a1320 = 1u64;
                                        nat_mul(a1319, a1320)?
                                    };
                                    nat_mul(a1321, a1322)?
                                };
                                let a1324 = 1u64;
                                nat_add(a1323, a1324)?
                            };
                            nat_mul(a1325, a1326)?
                        };
                        let a1328 = 1u64;
                        nat_add(a1327, a1328)?
                    };
                    nat_mul(a1329, a1330)?
                };
                let a1332 = 1u64;
                nat_add(a1331, a1332)?
            })?
        }
    };
    nat_add(a1333, a1334)
}

pub fn f28(v0: u64, v1: u64) -> R<u64> {
    let a1335 = v1;
    let a1336 = 0u64;
    if nat_eq(a1335, a1336) {
        Ok(v0)
    } else {
        let a1361 = {
            let a1359 = {
                let a1337 = 1u64;
                let a1338 = 1u64;
                nat_add(a1337, a1338)?
            };
            let a1360 = {
                let a1357 = {
                    let a1339 = 1u64;
                    let a1340 = 1u64;
                    nat_add(a1339, a1340)?
                };
                let a1358 = {
                    let a1355 = {
                        let a1341 = 1u64;
                        let a1342 = 1u64;
                        nat_add(a1341, a1342)?
                    };
                    let a1356 = {
                        let a1353 = {
                            let a1343 = 1u64;
                            let a1344 = 1u64;
                            nat_add(a1343, a1344)?
                        };
                        let a1354 = {
                            let a1351 = {
                                let a1345 = 1u64;
                                let a1346 = 1u64;
                                nat_add(a1345, a1346)?
                            };
                            let a1352 = {
                                let a1349 = {
                                    let a1347 = 1u64;
                                    let a1348 = 1u64;
                                    nat_add(a1347, a1348)?
                                };
                                let a1350 = 1u64;
                                nat_mul(a1349, a1350)?
                            };
                            nat_mul(a1351, a1352)?
                        };
                        nat_mul(a1353, a1354)?
                    };
                    nat_mul(a1355, a1356)?
                };
                nat_mul(a1357, a1358)?
            };
            nat_mul(a1359, a1360)?
        };
        let a1362 = v1;
        if nat_le(a1361, a1362) {
            let a1363 = v0;
            let a1364 = 0u64;
            if nat_eq(a1363, a1364) {
                Ok(0u64)
            } else {
                Ok(1u64)
            }
        } else {
            let v2: u64 = f10(v1)?;
            f34({
                let a1365 = v0;
                let a1366 = v2;
                let a1367 = 0u64;
                nat_quot(a1365, a1366, a1367)
            }, {
                let a1371 = 0u64;
                let a1372 = {
                    let a1368 = v0;
                    let a1369 = v2;
                    let a1370 = 0u64;
                    nat_rem(a1368, a1369, a1370)
                };
                nat_lt(a1371, a1372)
            })
        }
    }
}

pub fn f29(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1395 = f10({
        let a1393 = {
            let a1391 = {
                let a1373 = 1u64;
                let a1374 = 1u64;
                nat_add(a1373, a1374)?
            };
            let a1392 = {
                let a1389 = {
                    let a1387 = {
                        let a1375 = 1u64;
                        let a1376 = 1u64;
                        nat_add(a1375, a1376)?
                    };
                    let a1388 = {
                        let a1385 = {
                            let a1377 = 1u64;
                            let a1378 = 1u64;
                            nat_add(a1377, a1378)?
                        };
                        let a1386 = {
                            let a1383 = {
                                let a1381 = {
                                    let a1379 = 1u64;
                                    let a1380 = 1u64;
                                    nat_add(a1379, a1380)?
                                };
                                let a1382 = 1u64;
                                nat_mul(a1381, a1382)?
                            };
                            let a1384 = 1u64;
                            nat_add(a1383, a1384)?
                        };
                        nat_mul(a1385, a1386)?
                    };
                    nat_mul(a1387, a1388)?
                };
                let a1390 = 1u64;
                nat_add(a1389, a1390)?
            };
            nat_mul(a1391, a1392)?
        };
        let a1394 = 1u64;
        nat_add(a1393, a1394)?
    })?;
    let a1396 = v2;
    if nat_le(a1395, a1396) {
        f33(v0, {
            let a1397 = v1;
            let a1398 = 1u64;
            nat_add(a1397, a1398)?
        }, f28(v2, 1u64)?)
    } else {
        let v3: Adt2 = f35(v1, v2, {
            let a1435 = {
                let a1433 = {
                    let a1399 = 1u64;
                    let a1400 = 1u64;
                    nat_add(a1399, a1400)?
                };
                let a1434 = {
                    let a1431 = {
                        let a1401 = 1u64;
                        let a1402 = 1u64;
                        nat_add(a1401, a1402)?
                    };
                    let a1432 = {
                        let a1429 = {
                            let a1403 = 1u64;
                            let a1404 = 1u64;
                            nat_add(a1403, a1404)?
                        };
                        let a1430 = {
                            let a1427 = {
                                let a1405 = 1u64;
                                let a1406 = 1u64;
                                nat_add(a1405, a1406)?
                            };
                            let a1428 = {
                                let a1425 = {
                                    let a1407 = 1u64;
                                    let a1408 = 1u64;
                                    nat_add(a1407, a1408)?
                                };
                                let a1426 = {
                                    let a1423 = {
                                        let a1409 = 1u64;
                                        let a1410 = 1u64;
                                        nat_add(a1409, a1410)?
                                    };
                                    let a1424 = {
                                        let a1421 = {
                                            let a1411 = 1u64;
                                            let a1412 = 1u64;
                                            nat_add(a1411, a1412)?
                                        };
                                        let a1422 = {
                                            let a1419 = {
                                                let a1413 = 1u64;
                                                let a1414 = 1u64;
                                                nat_add(a1413, a1414)?
                                            };
                                            let a1420 = {
                                                let a1417 = {
                                                    let a1415 = 1u64;
                                                    let a1416 = 1u64;
                                                    nat_add(a1415, a1416)?
                                                };
                                                let a1418 = 1u64;
                                                nat_mul(a1417, a1418)?
                                            };
                                            nat_mul(a1419, a1420)?
                                        };
                                        nat_mul(a1421, a1422)?
                                    };
                                    nat_mul(a1423, a1424)?
                                };
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
            let a1436 = 1u64;
            nat_add(a1435, a1436)?
        }, f10({
            let a1455 = {
                let a1437 = 1u64;
                let a1438 = 1u64;
                nat_add(a1437, a1438)?
            };
            let a1456 = {
                let a1453 = {
                    let a1451 = {
                        let a1439 = 1u64;
                        let a1440 = 1u64;
                        nat_add(a1439, a1440)?
                    };
                    let a1452 = {
                        let a1449 = {
                            let a1441 = 1u64;
                            let a1442 = 1u64;
                            nat_add(a1441, a1442)?
                        };
                        let a1450 = {
                            let a1447 = {
                                let a1445 = {
                                    let a1443 = 1u64;
                                    let a1444 = 1u64;
                                    nat_add(a1443, a1444)?
                                };
                                let a1446 = 1u64;
                                nat_mul(a1445, a1446)?
                            };
                            let a1448 = 1u64;
                            nat_add(a1447, a1448)?
                        };
                        nat_mul(a1449, a1450)?
                    };
                    nat_mul(a1451, a1452)?
                };
                let a1454 = 1u64;
                nat_add(a1453, a1454)?
            };
            nat_mul(a1455, a1456)?
        })?)?;
        f33(v0, match v3.clone() {
            Adt2::C0(h1457, _, _, _) => {
                h1457
            }
        }, match v3 {
            Adt2::C0(_, h1458, _, _) => {
                h1458
            }
        })
    }
}

pub fn f30(v0: u64) -> R<Adt2> {
    f35(f26(v0)?, f27(v0)?, 0u64, f10({
        let a1479 = {
            let a1477 = {
                let a1459 = 1u64;
                let a1460 = 1u64;
                nat_add(a1459, a1460)?
            };
            let a1478 = {
                let a1475 = {
                    let a1473 = {
                        let a1461 = 1u64;
                        let a1462 = 1u64;
                        nat_add(a1461, a1462)?
                    };
                    let a1474 = {
                        let a1471 = {
                            let a1469 = {
                                let a1463 = 1u64;
                                let a1464 = 1u64;
                                nat_add(a1463, a1464)?
                            };
                            let a1470 = {
                                let a1467 = {
                                    let a1465 = 1u64;
                                    let a1466 = 1u64;
                                    nat_add(a1465, a1466)?
                                };
                                let a1468 = 1u64;
                                nat_mul(a1467, a1468)?
                            };
                            nat_mul(a1469, a1470)?
                        };
                        let a1472 = 1u64;
                        nat_add(a1471, a1472)?
                    };
                    nat_mul(a1473, a1474)?
                };
                let a1476 = 1u64;
                nat_add(a1475, a1476)?
            };
            nat_mul(a1477, a1478)?
        };
        let a1480 = 1u64;
        nat_add(a1479, a1480)?
    })?)
}

pub fn f31(v0: Fn1, p1485: u64, p1486: Adt3) -> R<(Adt3, bool)> {
    let m1484 = (p1485, p1486);
    {
        let mut m1484 = m1484;
        loop {
            let step = {
                let (v1, v2) = m1484;
                let m1481 = v1;
                if m1481 == 0 {
                    Err::<(u64, Adt3), R<(Adt3, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1481 - 1;
                    let m1483 = {
                        let c1482: Fn1 = v0.clone();
                        c1482.apply(v2.clone())?
                    };
                    match m1483 {
                        None => {
                            Err::<(u64, Adt3), R<(Adt3, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt3), R<(Adt3, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1484 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f32(v0: Adt3) -> R<Option<Adt3>> {
    f36(v0)
}

pub fn f33(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1525 = v1;
    let a1526 = {
        let a1523 = {
            let a1521 = {
                let a1487 = 1u64;
                let a1488 = 1u64;
                nat_add(a1487, a1488)?
            };
            let a1522 = {
                let a1519 = {
                    let a1489 = 1u64;
                    let a1490 = 1u64;
                    nat_add(a1489, a1490)?
                };
                let a1520 = {
                    let a1517 = {
                        let a1491 = 1u64;
                        let a1492 = 1u64;
                        nat_add(a1491, a1492)?
                    };
                    let a1518 = {
                        let a1515 = {
                            let a1493 = 1u64;
                            let a1494 = 1u64;
                            nat_add(a1493, a1494)?
                        };
                        let a1516 = {
                            let a1513 = {
                                let a1495 = 1u64;
                                let a1496 = 1u64;
                                nat_add(a1495, a1496)?
                            };
                            let a1514 = {
                                let a1511 = {
                                    let a1497 = 1u64;
                                    let a1498 = 1u64;
                                    nat_add(a1497, a1498)?
                                };
                                let a1512 = {
                                    let a1509 = {
                                        let a1499 = 1u64;
                                        let a1500 = 1u64;
                                        nat_add(a1499, a1500)?
                                    };
                                    let a1510 = {
                                        let a1507 = {
                                            let a1501 = 1u64;
                                            let a1502 = 1u64;
                                            nat_add(a1501, a1502)?
                                        };
                                        let a1508 = {
                                            let a1505 = {
                                                let a1503 = 1u64;
                                                let a1504 = 1u64;
                                                nat_add(a1503, a1504)?
                                            };
                                            let a1506 = 1u64;
                                            nat_mul(a1505, a1506)?
                                        };
                                        nat_mul(a1507, a1508)?
                                    };
                                    nat_mul(a1509, a1510)?
                                };
                                nat_mul(a1511, a1512)?
                            };
                            nat_mul(a1513, a1514)?
                        };
                        nat_mul(a1515, a1516)?
                    };
                    nat_mul(a1517, a1518)?
                };
                nat_mul(a1519, a1520)?
            };
            nat_mul(a1521, a1522)?
        };
        let a1524 = 1u64;
        nat_add(a1523, a1524)?
    };
    if nat_lt(a1525, a1526) {
        f37(v0, {
            let a1563 = {
                let a1561 = {
                    let a1527 = 1u64;
                    let a1528 = 1u64;
                    nat_add(a1527, a1528)?
                };
                let a1562 = {
                    let a1559 = {
                        let a1529 = 1u64;
                        let a1530 = 1u64;
                        nat_add(a1529, a1530)?
                    };
                    let a1560 = {
                        let a1557 = {
                            let a1531 = 1u64;
                            let a1532 = 1u64;
                            nat_add(a1531, a1532)?
                        };
                        let a1558 = {
                            let a1555 = {
                                let a1533 = 1u64;
                                let a1534 = 1u64;
                                nat_add(a1533, a1534)?
                            };
                            let a1556 = {
                                let a1553 = {
                                    let a1535 = 1u64;
                                    let a1536 = 1u64;
                                    nat_add(a1535, a1536)?
                                };
                                let a1554 = {
                                    let a1551 = {
                                        let a1537 = 1u64;
                                        let a1538 = 1u64;
                                        nat_add(a1537, a1538)?
                                    };
                                    let a1552 = {
                                        let a1549 = {
                                            let a1539 = 1u64;
                                            let a1540 = 1u64;
                                            nat_add(a1539, a1540)?
                                        };
                                        let a1550 = {
                                            let a1547 = {
                                                let a1541 = 1u64;
                                                let a1542 = 1u64;
                                                nat_add(a1541, a1542)?
                                            };
                                            let a1548 = {
                                                let a1545 = {
                                                    let a1543 = 1u64;
                                                    let a1544 = 1u64;
                                                    nat_add(a1543, a1544)?
                                                };
                                                let a1546 = 1u64;
                                                nat_mul(a1545, a1546)?
                                            };
                                            nat_mul(a1547, a1548)?
                                        };
                                        nat_mul(a1549, a1550)?
                                    };
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
            let a1564 = 1u64;
            nat_add(a1563, a1564)?
        }, f28(v2, {
            let a1603 = {
                let a1601 = {
                    let a1599 = {
                        let a1565 = 1u64;
                        let a1566 = 1u64;
                        nat_add(a1565, a1566)?
                    };
                    let a1600 = {
                        let a1597 = {
                            let a1567 = 1u64;
                            let a1568 = 1u64;
                            nat_add(a1567, a1568)?
                        };
                        let a1598 = {
                            let a1595 = {
                                let a1569 = 1u64;
                                let a1570 = 1u64;
                                nat_add(a1569, a1570)?
                            };
                            let a1596 = {
                                let a1593 = {
                                    let a1571 = 1u64;
                                    let a1572 = 1u64;
                                    nat_add(a1571, a1572)?
                                };
                                let a1594 = {
                                    let a1591 = {
                                        let a1573 = 1u64;
                                        let a1574 = 1u64;
                                        nat_add(a1573, a1574)?
                                    };
                                    let a1592 = {
                                        let a1589 = {
                                            let a1575 = 1u64;
                                            let a1576 = 1u64;
                                            nat_add(a1575, a1576)?
                                        };
                                        let a1590 = {
                                            let a1587 = {
                                                let a1577 = 1u64;
                                                let a1578 = 1u64;
                                                nat_add(a1577, a1578)?
                                            };
                                            let a1588 = {
                                                let a1585 = {
                                                    let a1579 = 1u64;
                                                    let a1580 = 1u64;
                                                    nat_add(a1579, a1580)?
                                                };
                                                let a1586 = {
                                                    let a1583 = {
                                                        let a1581 = 1u64;
                                                        let a1582 = 1u64;
                                                        nat_add(a1581, a1582)?
                                                    };
                                                    let a1584 = 1u64;
                                                    nat_mul(a1583, a1584)?
                                                };
                                                nat_mul(a1585, a1586)?
                                            };
                                            nat_mul(a1587, a1588)?
                                        };
                                        nat_mul(a1589, a1590)?
                                    };
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
                let a1602 = 1u64;
                nat_add(a1601, a1602)?
            };
            let a1604 = v1;
            nat_sub(a1603, a1604)
        })?)
    } else {
        f37(v0, v1, v2)
    }
}

pub fn f34(v0: u64, v1: bool) -> R<u64> {
    if if v1 {
        let a1612 = {
            let a1609 = v0;
            let a1610 = {
                let a1607 = {
                    let a1605 = 1u64;
                    let a1606 = 1u64;
                    nat_add(a1605, a1606)?
                };
                let a1608 = 1u64;
                nat_mul(a1607, a1608)?
            };
            let a1611 = 0u64;
            nat_rem(a1609, a1610, a1611)
        };
        let a1613 = 0u64;
        nat_eq(a1612, a1613)
    } else {
        false
    } {
        let a1614 = v0;
        let a1615 = 1u64;
        nat_add(a1614, a1615)
    } else {
        Ok(v0)
    }
}

pub fn f35(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt2> {
    let (h1616, _) = f38(Fn2::F39, {
        let a1635 = {
            let a1617 = 1u64;
            let a1618 = 1u64;
            nat_add(a1617, a1618)?
        };
        let a1636 = {
            let a1633 = {
                let a1619 = 1u64;
                let a1620 = 1u64;
                nat_add(a1619, a1620)?
            };
            let a1634 = {
                let a1631 = {
                    let a1621 = 1u64;
                    let a1622 = 1u64;
                    nat_add(a1621, a1622)?
                };
                let a1632 = {
                    let a1629 = {
                        let a1623 = 1u64;
                        let a1624 = 1u64;
                        nat_add(a1623, a1624)?
                    };
                    let a1630 = {
                        let a1627 = {
                            let a1625 = 1u64;
                            let a1626 = 1u64;
                            nat_add(a1625, a1626)?
                        };
                        let a1628 = 1u64;
                        nat_mul(a1627, a1628)?
                    };
                    nat_mul(a1629, a1630)?
                };
                nat_mul(a1631, a1632)?
            };
            nat_mul(a1633, a1634)?
        };
        nat_mul(a1635, a1636)?
    }, Adt2::C0(v0, v1, v2, v3))?;
    Ok(h1616)
}

pub fn f36(v0: Adt3) -> R<Option<Adt3>> {
    let v1: u64 = {
        let a1649 = {
            let a1643 = match v0.clone() {
                Adt3::C0(_, h1637) => {
                    h1637
                }
            };
            let a1644 = {
                let a1640 = match v0.clone() {
                    Adt3::C0(h1638, _) => {
                        h1638
                    }
                };
                let a1641 = match v0.clone() {
                    Adt3::C0(_, h1639) => {
                        h1639
                    }
                };
                let a1642 = 0u64;
                nat_quot(a1640, a1641, a1642)
            };
            nat_add(a1643, a1644)?
        };
        let a1650 = {
            let a1647 = {
                let a1645 = 1u64;
                let a1646 = 1u64;
                nat_add(a1645, a1646)?
            };
            let a1648 = 1u64;
            nat_mul(a1647, a1648)?
        };
        let a1651 = 0u64;
        nat_quot(a1649, a1650, a1651)
    };
    let a1653 = v1;
    let a1654 = match v0.clone() {
        Adt3::C0(_, h1652) => {
            h1652
        }
    };
    if nat_lt(a1653, a1654) {
        Ok(Some(Adt3::C0(match v0 {
            Adt3::C0(h1655, _) => {
                h1655
            }
        }, v1)))
    } else {
        Ok(None::<Adt3>)
    }
}

pub fn f37(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let v3: u64 = {
        let a1668 = v2;
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
        let a1670 = 0u64;
        nat_rem(a1668, a1669, a1670)
    };
    let v4: u64 = {
        let a1683 = v2;
        let a1684 = {
            let a1681 = {
                let a1671 = 1u64;
                let a1672 = 1u64;
                nat_add(a1671, a1672)?
            };
            let a1682 = {
                let a1679 = {
                    let a1673 = 1u64;
                    let a1674 = 1u64;
                    nat_add(a1673, a1674)?
                };
                let a1680 = {
                    let a1677 = {
                        let a1675 = 1u64;
                        let a1676 = 1u64;
                        nat_add(a1675, a1676)?
                    };
                    let a1678 = 1u64;
                    nat_mul(a1677, a1678)?
                };
                nat_mul(a1679, a1680)?
            };
            nat_mul(a1681, a1682)?
        };
        let a1685 = 0u64;
        nat_quot(a1683, a1684, a1685)
    };
    let v5: bool = {
        let a1694 = {
            let a1692 = {
                let a1686 = 1u64;
                let a1687 = 1u64;
                nat_add(a1686, a1687)?
            };
            let a1693 = {
                let a1690 = {
                    let a1688 = 1u64;
                    let a1689 = 1u64;
                    nat_add(a1688, a1689)?
                };
                let a1691 = 1u64;
                nat_mul(a1690, a1691)?
            };
            nat_mul(a1692, a1693)?
        };
        let a1695 = v3;
        if nat_lt(a1694, a1695) {
            true
        } else {
            let a1704 = v3;
            let a1705 = {
                let a1702 = {
                    let a1696 = 1u64;
                    let a1697 = 1u64;
                    nat_add(a1696, a1697)?
                };
                let a1703 = {
                    let a1700 = {
                        let a1698 = 1u64;
                        let a1699 = 1u64;
                        nat_add(a1698, a1699)?
                    };
                    let a1701 = 1u64;
                    nat_mul(a1700, a1701)?
                };
                nat_mul(a1702, a1703)?
            };
            if nat_eq(a1704, a1705) {
                let a1713 = {
                    let a1710 = v4;
                    let a1711 = {
                        let a1708 = {
                            let a1706 = 1u64;
                            let a1707 = 1u64;
                            nat_add(a1706, a1707)?
                        };
                        let a1709 = 1u64;
                        nat_mul(a1708, a1709)?
                    };
                    let a1712 = 0u64;
                    nat_rem(a1710, a1711, a1712)
                };
                let a1714 = 1u64;
                nat_eq(a1713, a1714)
            } else {
                false
            }
        }
    };
    let v6: u64 = {
        let a1715 = v4;
        let a1716 = if v5 {
            1u64
        } else {
            0u64
        };
        nat_add(a1715, a1716)?
    };
    let v7: bool = {
        let a1745 = {
            let a1743 = {
                let a1719 = {
                    let a1717 = 1u64;
                    let a1718 = 1u64;
                    nat_add(a1717, a1718)?
                };
                let a1720 = 1u64;
                nat_mul(a1719, a1720)?
            };
            let a1744 = f10({
                let a1741 = {
                    let a1739 = {
                        let a1721 = 1u64;
                        let a1722 = 1u64;
                        nat_add(a1721, a1722)?
                    };
                    let a1740 = {
                        let a1737 = {
                            let a1735 = {
                                let a1723 = 1u64;
                                let a1724 = 1u64;
                                nat_add(a1723, a1724)?
                            };
                            let a1736 = {
                                let a1733 = {
                                    let a1731 = {
                                        let a1725 = 1u64;
                                        let a1726 = 1u64;
                                        nat_add(a1725, a1726)?
                                    };
                                    let a1732 = {
                                        let a1729 = {
                                            let a1727 = 1u64;
                                            let a1728 = 1u64;
                                            nat_add(a1727, a1728)?
                                        };
                                        let a1730 = 1u64;
                                        nat_mul(a1729, a1730)?
                                    };
                                    nat_mul(a1731, a1732)?
                                };
                                let a1734 = 1u64;
                                nat_add(a1733, a1734)?
                            };
                            nat_mul(a1735, a1736)?
                        };
                        let a1738 = 1u64;
                        nat_add(a1737, a1738)?
                    };
                    nat_mul(a1739, a1740)?
                };
                let a1742 = 1u64;
                nat_add(a1741, a1742)?
            })?;
            nat_mul(a1743, a1744)?
        };
        let a1746 = v6;
        nat_le(a1745, a1746)
    };
    let v8: u64 = if v7 {
        let a1751 = v6;
        let a1752 = {
            let a1749 = {
                let a1747 = 1u64;
                let a1748 = 1u64;
                nat_add(a1747, a1748)?
            };
            let a1750 = 1u64;
            nat_mul(a1749, a1750)?
        };
        let a1753 = 0u64;
        nat_quot(a1751, a1752, a1753)
    } else {
        v6
    };
    let v9: u64 = if v7 {
        let a1754 = v1;
        let a1755 = 1u64;
        nat_add(a1754, a1755)?
    } else {
        v1
    };
    let v10: u64 = {
        let a1794 = v9;
        let a1795 = {
            let a1792 = {
                let a1790 = {
                    let a1756 = 1u64;
                    let a1757 = 1u64;
                    nat_add(a1756, a1757)?
                };
                let a1791 = {
                    let a1788 = {
                        let a1758 = 1u64;
                        let a1759 = 1u64;
                        nat_add(a1758, a1759)?
                    };
                    let a1789 = {
                        let a1786 = {
                            let a1760 = 1u64;
                            let a1761 = 1u64;
                            nat_add(a1760, a1761)?
                        };
                        let a1787 = {
                            let a1784 = {
                                let a1762 = 1u64;
                                let a1763 = 1u64;
                                nat_add(a1762, a1763)?
                            };
                            let a1785 = {
                                let a1782 = {
                                    let a1764 = 1u64;
                                    let a1765 = 1u64;
                                    nat_add(a1764, a1765)?
                                };
                                let a1783 = {
                                    let a1780 = {
                                        let a1766 = 1u64;
                                        let a1767 = 1u64;
                                        nat_add(a1766, a1767)?
                                    };
                                    let a1781 = {
                                        let a1778 = {
                                            let a1768 = 1u64;
                                            let a1769 = 1u64;
                                            nat_add(a1768, a1769)?
                                        };
                                        let a1779 = {
                                            let a1776 = {
                                                let a1770 = 1u64;
                                                let a1771 = 1u64;
                                                nat_add(a1770, a1771)?
                                            };
                                            let a1777 = {
                                                let a1774 = {
                                                    let a1772 = 1u64;
                                                    let a1773 = 1u64;
                                                    nat_add(a1772, a1773)?
                                                };
                                                let a1775 = 1u64;
                                                nat_mul(a1774, a1775)?
                                            };
                                            nat_mul(a1776, a1777)?
                                        };
                                        nat_mul(a1778, a1779)?
                                    };
                                    nat_mul(a1780, a1781)?
                                };
                                nat_mul(a1782, a1783)?
                            };
                            nat_mul(a1784, a1785)?
                        };
                        nat_mul(a1786, a1787)?
                    };
                    nat_mul(a1788, a1789)?
                };
                nat_mul(a1790, a1791)?
            };
            let a1793 = 1u64;
            nat_add(a1792, a1793)?
        };
        if if nat_eq(a1794, a1795) {
            let a1818 = v8;
            let a1819 = f10({
                let a1816 = {
                    let a1814 = {
                        let a1796 = 1u64;
                        let a1797 = 1u64;
                        nat_add(a1796, a1797)?
                    };
                    let a1815 = {
                        let a1812 = {
                            let a1810 = {
                                let a1798 = 1u64;
                                let a1799 = 1u64;
                                nat_add(a1798, a1799)?
                            };
                            let a1811 = {
                                let a1808 = {
                                    let a1806 = {
                                        let a1800 = 1u64;
                                        let a1801 = 1u64;
                                        nat_add(a1800, a1801)?
                                    };
                                    let a1807 = {
                                        let a1804 = {
                                            let a1802 = 1u64;
                                            let a1803 = 1u64;
                                            nat_add(a1802, a1803)?
                                        };
                                        let a1805 = 1u64;
                                        nat_mul(a1804, a1805)?
                                    };
                                    nat_mul(a1806, a1807)?
                                };
                                let a1809 = 1u64;
                                nat_add(a1808, a1809)?
                            };
                            nat_mul(a1810, a1811)?
                        };
                        let a1813 = 1u64;
                        nat_add(a1812, a1813)?
                    };
                    nat_mul(a1814, a1815)?
                };
                let a1817 = 1u64;
                nat_add(a1816, a1817)?
            })?;
            nat_lt(a1818, a1819)
        } else {
            false
        } {
            0u64
        } else {
            let a1856 = v9;
            let a1857 = {
                let a1854 = {
                    let a1820 = 1u64;
                    let a1821 = 1u64;
                    nat_add(a1820, a1821)?
                };
                let a1855 = {
                    let a1852 = {
                        let a1822 = 1u64;
                        let a1823 = 1u64;
                        nat_add(a1822, a1823)?
                    };
                    let a1853 = {
                        let a1850 = {
                            let a1824 = 1u64;
                            let a1825 = 1u64;
                            nat_add(a1824, a1825)?
                        };
                        let a1851 = {
                            let a1848 = {
                                let a1826 = 1u64;
                                let a1827 = 1u64;
                                nat_add(a1826, a1827)?
                            };
                            let a1849 = {
                                let a1846 = {
                                    let a1828 = 1u64;
                                    let a1829 = 1u64;
                                    nat_add(a1828, a1829)?
                                };
                                let a1847 = {
                                    let a1844 = {
                                        let a1830 = 1u64;
                                        let a1831 = 1u64;
                                        nat_add(a1830, a1831)?
                                    };
                                    let a1845 = {
                                        let a1842 = {
                                            let a1832 = 1u64;
                                            let a1833 = 1u64;
                                            nat_add(a1832, a1833)?
                                        };
                                        let a1843 = {
                                            let a1840 = {
                                                let a1834 = 1u64;
                                                let a1835 = 1u64;
                                                nat_add(a1834, a1835)?
                                            };
                                            let a1841 = {
                                                let a1838 = {
                                                    let a1836 = 1u64;
                                                    let a1837 = 1u64;
                                                    nat_add(a1836, a1837)?
                                                };
                                                let a1839 = 1u64;
                                                nat_mul(a1838, a1839)?
                                            };
                                            nat_mul(a1840, a1841)?
                                        };
                                        nat_mul(a1842, a1843)?
                                    };
                                    nat_mul(a1844, a1845)?
                                };
                                nat_mul(a1846, a1847)?
                            };
                            nat_mul(a1848, a1849)?
                        };
                        nat_mul(a1850, a1851)?
                    };
                    nat_mul(a1852, a1853)?
                };
                nat_mul(a1854, a1855)?
            };
            nat_sub(a1856, a1857)
        }
    };
    let a1858 = v8;
    let a1859 = 0u64;
    if nat_eq(a1858, a1859) {
        f17(v0)
    } else {
        let a1912 = {
            let a1910 = {
                let a1908 = {
                    let a1860 = 1u64;
                    let a1861 = 1u64;
                    nat_add(a1860, a1861)?
                };
                let a1909 = {
                    let a1906 = {
                        let a1904 = {
                            let a1862 = 1u64;
                            let a1863 = 1u64;
                            nat_add(a1862, a1863)?
                        };
                        let a1905 = {
                            let a1902 = {
                                let a1900 = {
                                    let a1864 = 1u64;
                                    let a1865 = 1u64;
                                    nat_add(a1864, a1865)?
                                };
                                let a1901 = {
                                    let a1898 = {
                                        let a1896 = {
                                            let a1866 = 1u64;
                                            let a1867 = 1u64;
                                            nat_add(a1866, a1867)?
                                        };
                                        let a1897 = {
                                            let a1894 = {
                                                let a1892 = {
                                                    let a1868 = 1u64;
                                                    let a1869 = 1u64;
                                                    nat_add(a1868, a1869)?
                                                };
                                                let a1893 = {
                                                    let a1890 = {
                                                        let a1888 = {
                                                            let a1870 = 1u64;
                                                            let a1871 = 1u64;
                                                            nat_add(a1870, a1871)?
                                                        };
                                                        let a1889 = {
                                                            let a1886 = {
                                                                let a1884 = {
                                                                    let a1872 = 1u64;
                                                                    let a1873 = 1u64;
                                                                    nat_add(a1872, a1873)?
                                                                };
                                                                let a1885 = {
                                                                    let a1882 = {
                                                                        let a1880 = {
                                                                            let a1874 = 1u64;
                                                                            let a1875 = 1u64;
                                                                            nat_add(a1874, a1875)?
                                                                        };
                                                                        let a1881 = {
                                                                            let a1878 = {
                                                                                let a1876 = 1u64;
                                                                                let a1877 = 1u64;
                                                                                nat_add(a1876, a1877)?
                                                                            };
                                                                            let a1879 = 1u64;
                                                                            nat_mul(a1878, a1879)?
                                                                        };
                                                                        nat_mul(a1880, a1881)?
                                                                    };
                                                                    let a1883 = 1u64;
                                                                    nat_add(a1882, a1883)?
                                                                };
                                                                nat_mul(a1884, a1885)?
                                                            };
                                                            let a1887 = 1u64;
                                                            nat_add(a1886, a1887)?
                                                        };
                                                        nat_mul(a1888, a1889)?
                                                    };
                                                    let a1891 = 1u64;
                                                    nat_add(a1890, a1891)?
                                                };
                                                nat_mul(a1892, a1893)?
                                            };
                                            let a1895 = 1u64;
                                            nat_add(a1894, a1895)?
                                        };
                                        nat_mul(a1896, a1897)?
                                    };
                                    let a1899 = 1u64;
                                    nat_add(a1898, a1899)?
                                };
                                nat_mul(a1900, a1901)?
                            };
                            let a1903 = 1u64;
                            nat_add(a1902, a1903)?
                        };
                        nat_mul(a1904, a1905)?
                    };
                    let a1907 = 1u64;
                    nat_add(a1906, a1907)?
                };
                nat_mul(a1908, a1909)?
            };
            let a1911 = 1u64;
            nat_add(a1910, a1911)?
        };
        let a1913 = v9;
        if nat_le(a1912, a1913) {
            f15(v0)
        } else {
            let a1965 = {
                let a1938 = f17(v0)?;
                let a1939 = {
                    let a1936 = v10;
                    let a1937 = f10({
                        let a1934 = {
                            let a1932 = {
                                let a1914 = 1u64;
                                let a1915 = 1u64;
                                nat_add(a1914, a1915)?
                            };
                            let a1933 = {
                                let a1930 = {
                                    let a1928 = {
                                        let a1916 = 1u64;
                                        let a1917 = 1u64;
                                        nat_add(a1916, a1917)?
                                    };
                                    let a1929 = {
                                        let a1926 = {
                                            let a1924 = {
                                                let a1918 = 1u64;
                                                let a1919 = 1u64;
                                                nat_add(a1918, a1919)?
                                            };
                                            let a1925 = {
                                                let a1922 = {
                                                    let a1920 = 1u64;
                                                    let a1921 = 1u64;
                                                    nat_add(a1920, a1921)?
                                                };
                                                let a1923 = 1u64;
                                                nat_mul(a1922, a1923)?
                                            };
                                            nat_mul(a1924, a1925)?
                                        };
                                        let a1927 = 1u64;
                                        nat_add(a1926, a1927)?
                                    };
                                    nat_mul(a1928, a1929)?
                                };
                                let a1931 = 1u64;
                                nat_add(a1930, a1931)?
                            };
                            nat_mul(a1932, a1933)?
                        };
                        let a1935 = 1u64;
                        nat_add(a1934, a1935)?
                    })?;
                    nat_mul(a1936, a1937)?
                };
                nat_add(a1938, a1939)?
            };
            let a1966 = {
                let a1962 = v8;
                let a1963 = f10({
                    let a1960 = {
                        let a1958 = {
                            let a1940 = 1u64;
                            let a1941 = 1u64;
                            nat_add(a1940, a1941)?
                        };
                        let a1959 = {
                            let a1956 = {
                                let a1954 = {
                                    let a1942 = 1u64;
                                    let a1943 = 1u64;
                                    nat_add(a1942, a1943)?
                                };
                                let a1955 = {
                                    let a1952 = {
                                        let a1950 = {
                                            let a1944 = 1u64;
                                            let a1945 = 1u64;
                                            nat_add(a1944, a1945)?
                                        };
                                        let a1951 = {
                                            let a1948 = {
                                                let a1946 = 1u64;
                                                let a1947 = 1u64;
                                                nat_add(a1946, a1947)?
                                            };
                                            let a1949 = 1u64;
                                            nat_mul(a1948, a1949)?
                                        };
                                        nat_mul(a1950, a1951)?
                                    };
                                    let a1953 = 1u64;
                                    nat_add(a1952, a1953)?
                                };
                                nat_mul(a1954, a1955)?
                            };
                            let a1957 = 1u64;
                            nat_add(a1956, a1957)?
                        };
                        nat_mul(a1958, a1959)?
                    };
                    let a1961 = 1u64;
                    nat_add(a1960, a1961)?
                })?;
                let a1964 = 0u64;
                nat_rem(a1962, a1963, a1964)
            };
            nat_add(a1965, a1966)
        }
    }
}

pub fn f38(v0: Fn2, p1971: u64, p1972: Adt2) -> R<(Adt2, bool)> {
    let m1970 = (p1971, p1972);
    {
        let mut m1970 = m1970;
        loop {
            let step = {
                let (v1, v2) = m1970;
                let m1967 = v1;
                if m1967 == 0 {
                    Err::<(u64, Adt2), R<(Adt2, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1967 - 1;
                    let m1969 = {
                        let c1968: Fn2 = v0.clone();
                        c1968.apply(v2.clone())?
                    };
                    match m1969 {
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
                Ok(next) => { m1970 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f39(v0: Adt2) -> R<Option<Adt2>> {
    f40(v0)
}

pub fn f40(v0: Adt2) -> R<Option<Adt2>> {
    let a1974 = 0u64;
    let a1975 = match v0.clone() {
        Adt2::C0(_, h1973, _, _) => {
            h1973
        }
    };
    if if if nat_lt(a1974, a1975) {
        let a1978 = match v0.clone() {
            Adt2::C0(_, h1976, _, _) => {
                h1976
            }
        };
        let a1979 = match v0.clone() {
            Adt2::C0(_, _, _, h1977) => {
                h1977
            }
        };
        nat_lt(a1978, a1979)
    } else {
        false
    } {
        let a1982 = match v0.clone() {
            Adt2::C0(_, _, h1980, _) => {
                h1980
            }
        };
        let a1983 = match v0.clone() {
            Adt2::C0(h1981, _, _, _) => {
                h1981
            }
        };
        nat_lt(a1982, a1983)
    } else {
        false
    } {
        Ok(Some(Adt2::C0({
            let a1985 = match v0.clone() {
                Adt2::C0(h1984, _, _, _) => {
                    h1984
                }
            };
            let a1986 = 1u64;
            nat_sub(a1985, a1986)
        }, {
            let a1989 = match v0.clone() {
                Adt2::C0(_, h1987, _, _) => {
                    h1987
                }
            };
            let a1990 = match v0.clone() {
                Adt2::C0(_, h1988, _, _) => {
                    h1988
                }
            };
            nat_add(a1989, a1990)?
        }, match v0.clone() {
            Adt2::C0(_, _, h1991, _) => {
                h1991
            }
        }, match v0 {
            Adt2::C0(_, _, _, h1992) => {
                h1992
            }
        })))
    } else {
        Ok(None::<Adt2>)
    }
}

pub fn run(p0: Adt0, p1: u64) -> R<Adt1> {
    f0(p0, p1)
}
