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
    C8,
    C9,
    C10,
    C11,
    C12,
    C13,
    C14,
    C15,
}

#[derive(Clone)]
pub enum Adt1 {
    C0(u64, u64, u64, u64),
    C1(u64, u64),
    C2(u64, u64),
    C3(u64, u64, u64, u64),
    C4(u64, u64),
    C5(u64, u64),
    C6,
}

pub fn f0(v0: Adt0, v1: u64, v2: u64) -> Adt1 {
    let m1 = v0;
    match m1 {
        Adt0::C0 => {
            f1(v1, v2)
        }
        Adt0::C1 => {
            f2(v1, v2)
        }
        Adt0::C2 => {
            f3(v1, v2)
        }
        Adt0::C3 => {
            f4(v1, v2)
        }
        Adt0::C4 => {
            f5(v1, v2)
        }
        Adt0::C5 => {
            f6(v1, v2)
        }
        Adt0::C6 => {
            f7(v1, v2)
        }
        Adt0::C7 => {
            f8(v1, v2)
        }
        Adt0::C8 => {
            f9(v1, v2)
        }
        Adt0::C9 => {
            f10(v1, v2)
        }
        Adt0::C10 => {
            f11(v1, v2)
        }
        Adt0::C11 => {
            f12(v1, v2)
        }
        Adt0::C12 => {
            f13(v1, v2)
        }
        Adt0::C13 => {
            f14(v1, v2)
        }
        Adt0::C14 => {
            f15(v1, v2)
        }
        Adt0::C15 => {
            f16(v1, v2)
        }
    }
}

pub fn f1(_: u64, v1: u64) -> Adt1 {
    let a2 = v1;
    let a3 = 1u64;
    if nat_lt(a2, a3) {
        Adt1::C0(2u64, 0u64, 0u64, 2u64)
    } else {
        let a4 = v1;
        let a5 = 2u64;
        if nat_lt(a4, a5) {
            Adt1::C0(0u64, 1u64, 2u64, 1u64)
        } else {
            Adt1::C6
        }
    }
}

pub fn f2(_: u64, v1: u64) -> Adt1 {
    let a6 = v1;
    let a7 = 2u64;
    if nat_lt(a6, a7) {
        let a8 = v1;
        let a9 = 1u64;
        if nat_lt(a8, a9) {
            Adt1::C1(1107296256u64, 1u64)
        } else {
            Adt1::C0(3u64, 0u64, 1u64, 0u64)
        }
    } else {
        let a10 = v1;
        let a11 = 3u64;
        if nat_lt(a10, a11) {
            Adt1::C0(4u64, 0u64, 0u64, 0u64)
        } else {
            Adt1::C6
        }
    }
}

pub fn f3(v0: u64, v1: u64) -> Adt1 {
    let a12 = v1;
    let a13 = 1u64;
    if nat_lt(a12, a13) {
        Adt1::C0(7u64, 0u64, 0u64, 0u64)
    } else {
        let a14 = v1;
        let a15 = 2u64;
        if nat_lt(a14, a15) {
            let a16 = v0;
            let a17 = 2u64;
            if nat_eq(a16, a17) {
                Adt1::C6
            } else {
                let a18 = v0;
                let a19 = 4u64;
                let _ = nat_eq(a18, a19);
                Adt1::C0(0u64, 1u64, 0u64, 0u64)
            }
        } else {
            Adt1::C6
        }
    }
}

pub fn f4(_: u64, v1: u64) -> Adt1 {
    let a20 = v1;
    let a21 = 2u64;
    if nat_lt(a20, a21) {
        let a22 = v1;
        let a23 = 1u64;
        if nat_lt(a22, a23) {
            Adt1::C0(0u64, 2u64, 1u64, 2u64)
        } else {
            Adt1::C0(2u64, 1u64, 0u64, 4u64)
        }
    } else {
        let a24 = v1;
        let a25 = 3u64;
        if nat_lt(a24, a25) {
            Adt1::C0(0u64, 3u64, 4u64, 3u64)
        } else {
            Adt1::C6
        }
    }
}

pub fn f5(_: u64, v1: u64) -> Adt1 {
    let a26 = v1;
    let a27 = 3u64;
    if nat_lt(a26, a27) {
        let a28 = v1;
        let a29 = 1u64;
        if nat_lt(a28, a29) {
            Adt1::C0(1u64, 0u64, 1u64, 3u64)
        } else {
            let a30 = v1;
            let a31 = 2u64;
            if nat_lt(a30, a31) {
                Adt1::C2(2u64, 4u64)
            } else {
                Adt1::C0(3u64, 4u64, 3u64, 0u64)
            }
        }
    } else {
        let a32 = v1;
        let a33 = 4u64;
        if nat_lt(a32, a33) {
            Adt1::C1(1065353216u64, 4u64)
        } else {
            let a34 = v1;
            let a35 = 5u64;
            if nat_lt(a34, a35) {
                Adt1::C0(3u64, 4u64, 0u64, 1u64)
            } else {
                Adt1::C6
            }
        }
    }
}

pub fn f6(v0: u64, v1: u64) -> Adt1 {
    let a36 = v1;
    let a37 = 5u64;
    if nat_lt(a36, a37) {
        let a38 = v1;
        let a39 = 2u64;
        if nat_lt(a38, a39) {
            let a40 = v1;
            let a41 = 1u64;
            if nat_lt(a40, a41) {
                Adt1::C2(2u64, 2u64)
            } else {
                Adt1::C1(1036831949u64, 4u64)
            }
        } else {
            let a42 = v1;
            let a43 = 3u64;
            if nat_lt(a42, a43) {
                Adt1::C0(2u64, 4u64, 2u64, 2u64)
            } else {
                let a44 = v1;
                let a45 = 4u64;
                if nat_lt(a44, a45) {
                    Adt1::C1({
                        let a46 = v0;
                        let a47 = 4u64;
                        if nat_eq(a46, a47) {
                            3212836864u64
                        } else {
                            3204448256u64
                        }
                    }, 4u64)
                } else {
                    Adt1::C0(0u64, 4u64, 2u64, 2u64)
                }
            }
        }
    } else {
        let a48 = v1;
        let a49 = 7u64;
        if nat_lt(a48, a49) {
            let a50 = v1;
            let a51 = 6u64;
            if nat_lt(a50, a51) {
                Adt1::C2(3u64, 4u64)
            } else {
                Adt1::C0(0u64, 2u64, 4u64, 2u64)
            }
        } else {
            let a52 = v1;
            let a53 = 8u64;
            if nat_lt(a52, a53) {
                Adt1::C0(1u64, 0u64, 1u64, 4u64)
            } else {
                let a54 = v1;
                let a55 = 9u64;
                if nat_lt(a54, a55) {
                    Adt1::C0(3u64, 2u64, 4u64, 0u64)
                } else {
                    Adt1::C6
                }
            }
        }
    }
}

pub fn f7(_: u64, v1: u64) -> Adt1 {
    let a56 = v1;
    let a57 = 3u64;
    if nat_lt(a56, a57) {
        let a58 = v1;
        let a59 = 1u64;
        if nat_lt(a58, a59) {
            Adt1::C0(1u64, 0u64, 1u64, 0u64)
        } else {
            let a60 = v1;
            let a61 = 2u64;
            if nat_lt(a60, a61) {
                Adt1::C0(2u64, 2u64, 0u64, 0u64)
            } else {
                Adt1::C1(1262485504u64, 4u64)
            }
        }
    } else {
        let a62 = v1;
        let a63 = 4u64;
        if nat_lt(a62, a63) {
            Adt1::C0(0u64, 0u64, 4u64, 0u64)
        } else {
            let a64 = v1;
            let a65 = 5u64;
            if nat_lt(a64, a65) {
                Adt1::C3(0u64, 3u64, 4u64, 0u64)
            } else {
                Adt1::C6
            }
        }
    }
}

pub fn f8(_: u64, v1: u64) -> Adt1 {
    let a66 = v1;
    let a67 = 4u64;
    if nat_lt(a66, a67) {
        let a68 = v1;
        let a69 = 2u64;
        if nat_lt(a68, a69) {
            let a70 = v1;
            let a71 = 1u64;
            if nat_lt(a70, a71) {
                Adt1::C2(2u64, 2u64)
            } else {
                Adt1::C0(2u64, 1u64, 2u64, 6u64)
            }
        } else {
            let a72 = v1;
            let a73 = 3u64;
            if nat_lt(a72, a73) {
                Adt1::C0(0u64, 3u64, 6u64, 3u64)
            } else {
                Adt1::C0(2u64, 6u64, 2u64, 7u64)
            }
        }
    } else {
        let a74 = v1;
        let a75 = 6u64;
        if nat_lt(a74, a75) {
            let a76 = v1;
            let a77 = 5u64;
            if nat_lt(a76, a77) {
                Adt1::C0(0u64, 4u64, 7u64, 4u64)
            } else {
                Adt1::C0(2u64, 6u64, 0u64, 7u64)
            }
        } else {
            let a78 = v1;
            let a79 = 7u64;
            if nat_lt(a78, a79) {
                Adt1::C0(0u64, 5u64, 7u64, 5u64)
            } else {
                Adt1::C6
            }
        }
    }
}

pub fn f9(_: u64, v1: u64) -> Adt1 {
    let a80 = v1;
    let a81 = 2u64;
    if nat_lt(a80, a81) {
        let a82 = v1;
        let a83 = 1u64;
        if nat_lt(a82, a83) {
            Adt1::C0(2u64, 0u64, 2u64, 3u64)
        } else {
            Adt1::C0(2u64, 1u64, 1u64, 4u64)
        }
    } else {
        let a84 = v1;
        let a85 = 3u64;
        if nat_lt(a84, a85) {
            Adt1::C0(1u64, 3u64, 4u64, 0u64)
        } else {
            Adt1::C6
        }
    }
}

pub fn f10(_: u64, v1: u64) -> Adt1 {
    let a86 = v1;
    let a87 = 4u64;
    if nat_lt(a86, a87) {
        let a88 = v1;
        let a89 = 2u64;
        if nat_lt(a88, a89) {
            let a90 = v1;
            let a91 = 1u64;
            if nat_lt(a90, a91) {
                Adt1::C0(2u64, 0u64, 4u64, 6u64)
            } else {
                Adt1::C0(2u64, 1u64, 2u64, 7u64)
            }
        } else {
            let a92 = v1;
            let a93 = 3u64;
            if nat_lt(a92, a93) {
                Adt1::C0(1u64, 6u64, 7u64, 6u64)
            } else {
                Adt1::C0(3u64, 6u64, 5u64, 6u64)
            }
        }
    } else {
        let a94 = v1;
        let a95 = 6u64;
        if nat_lt(a94, a95) {
            let a96 = v1;
            let a97 = 5u64;
            if nat_lt(a96, a97) {
                Adt1::C0(2u64, 3u64, 1u64, 7u64)
            } else {
                Adt1::C0(2u64, 2u64, 4u64, 8u64)
            }
        } else {
            let a98 = v1;
            let a99 = 7u64;
            if nat_lt(a98, a99) {
                Adt1::C0(1u64, 7u64, 8u64, 7u64)
            } else {
                let a100 = v1;
                let a101 = 8u64;
                if nat_lt(a100, a101) {
                    Adt1::C0(3u64, 7u64, 5u64, 7u64)
                } else {
                    Adt1::C6
                }
            }
        }
    }
}

pub fn f11(_: u64, v1: u64) -> Adt1 {
    let a102 = v1;
    let a103 = 1u64;
    if nat_lt(a102, a103) {
        Adt1::C0(3u64, 0u64, 1u64, 0u64)
    } else {
        Adt1::C6
    }
}

pub fn f12(v0: u64, v1: u64) -> Adt1 {
    let a104 = v1;
    let a105 = 4u64;
    if nat_lt(a104, a105) {
        let a106 = v1;
        let a107 = 2u64;
        if nat_lt(a106, a107) {
            let a108 = v1;
            let a109 = 1u64;
            if nat_lt(a108, a109) {
                Adt1::C2(2u64, 2u64)
            } else {
                Adt1::C0(2u64, 3u64, 2u64, 2u64)
            }
        } else {
            let a110 = v1;
            let a111 = 3u64;
            if nat_lt(a110, a111) {
                Adt1::C0(0u64, 2u64, 4u64, 2u64)
            } else {
                Adt1::C0(1u64, 2u64, 0u64, 2u64)
            }
        }
    } else {
        let a112 = v1;
        let a113 = 6u64;
        if nat_lt(a112, a113) {
            let a114 = v1;
            let a115 = 5u64;
            if nat_lt(a114, a115) {
                let a116 = v0;
                let a117 = 2u64;
                if nat_eq(a116, a117) {
                    Adt1::C0(7u64, 2u64, 2u64, 2u64)
                } else {
                    let a118 = v0;
                    let a119 = 4u64;
                    let _ = nat_eq(a118, a119);
                    Adt1::C0(2u64, 2u64, 2u64, 2u64)
                }
            } else {
                Adt1::C0(2u64, 1u64, 2u64, 2u64)
            }
        } else {
            let a120 = v1;
            let a121 = 7u64;
            if nat_lt(a120, a121) {
                Adt1::C0(0u64, 5u64, 2u64, 5u64)
            } else {
                Adt1::C6
            }
        }
    }
}

pub fn f13(_: u64, v1: u64) -> Adt1 {
    let a122 = v1;
    let a123 = 3u64;
    if nat_lt(a122, a123) {
        let a124 = v1;
        let a125 = 1u64;
        if nat_lt(a124, a125) {
            Adt1::C2(1u64, 1u64)
        } else {
            let a126 = v1;
            let a127 = 2u64;
            if nat_lt(a126, a127) {
                Adt1::C0(3u64, 0u64, 1u64, 2u64)
            } else {
                Adt1::C4(2u64, 2u64)
            }
        }
    } else {
        let a128 = v1;
        let a129 = 4u64;
        if nat_lt(a128, a129) {
            Adt1::C5(2u64, 3u64)
        } else {
            let a130 = v1;
            let a131 = 5u64;
            if nat_lt(a130, a131) {
                Adt1::C0(3u64, 1u64, 0u64, 1u64)
            } else {
                Adt1::C6
            }
        }
    }
}

pub fn f14(_: u64, v1: u64) -> Adt1 {
    let a132 = v1;
    let a133 = 2u64;
    if nat_lt(a132, a133) {
        let a134 = v1;
        let a135 = 1u64;
        if nat_lt(a134, a135) {
            Adt1::C0(2u64, 0u64, 1u64, 0u64)
        } else {
            Adt1::C1(1262485504u64, 4u64)
        }
    } else {
        let a136 = v1;
        let a137 = 3u64;
        if nat_lt(a136, a137) {
            Adt1::C0(0u64, 4u64, 0u64, 0u64)
        } else {
            let a138 = v1;
            let a139 = 4u64;
            if nat_lt(a138, a139) {
                Adt1::C3(0u64, 2u64, 4u64, 0u64)
            } else {
                Adt1::C6
            }
        }
    }
}

pub fn f15(_: u64, v1: u64) -> Adt1 {
    let a140 = v1;
    let a141 = 1u64;
    if nat_lt(a140, a141) {
        Adt1::C2(1u64, 1u64)
    } else {
        let a142 = v1;
        let a143 = 2u64;
        if nat_lt(a142, a143) {
            Adt1::C0(2u64, 0u64, 1u64, 0u64)
        } else {
            Adt1::C6
        }
    }
}

pub fn f16(_: u64, v1: u64) -> Adt1 {
    let a144 = v1;
    let a145 = 3u64;
    if nat_lt(a144, a145) {
        let a146 = v1;
        let a147 = 1u64;
        if nat_lt(a146, a147) {
            Adt1::C0(0u64, 0u64, 2u64, 0u64)
        } else {
            let a148 = v1;
            let a149 = 2u64;
            if nat_lt(a148, a149) {
                Adt1::C0(3u64, 0u64, 1u64, 0u64)
            } else {
                Adt1::C1(1262485504u64, 4u64)
            }
        }
    } else {
        let a150 = v1;
        let a151 = 4u64;
        if nat_lt(a150, a151) {
            Adt1::C0(0u64, 4u64, 0u64, 0u64)
        } else {
            let a152 = v1;
            let a153 = 5u64;
            if nat_lt(a152, a153) {
                Adt1::C3(0u64, 3u64, 4u64, 0u64)
            } else {
                Adt1::C6
            }
        }
    }
}

pub fn run(p0: Adt0, p1: u64, p2: u64) -> Adt1 {
    f0(p0, p1, p2)
}
