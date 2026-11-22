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
    C0(bool, bool),
    C1(bool, bool),
    C2,
}

#[derive(Clone)]
pub enum Adt1 {
    C0,
    C1(u64, u64, Adt2, Adt2),
    C2(bool, u64, Adt2, Adt2),
    C3(u64, Adt2),
    C4(bool, u64, Adt2),
    C5(u64, u64, Adt2),
    C6(u64, u64, Adt2),
    C7,
}

#[derive(Clone)]
pub enum Adt2 {
    C0(u64),
    C1(u64),
}

pub fn f0(v0: Adt0, v1: u64) -> Adt1 {
    let m1 = v0;
    match m1 {
        Adt0::C0(v2, v3) => {
            f1(v2, v3, v1)
        }
        Adt0::C1(v4, v5) => {
            f2(v4, v5, v1)
        }
        Adt0::C2 => {
            let a2 = v1;
            let a3 = 0u64;
            if nat_eq(a2, a3) {
                Adt1::C1(2u64, 4u64, Adt2::C0(0u64), Adt2::C0(2u64))
            } else {
                let a4 = v1;
                let a5 = 1u64;
                if nat_eq(a4, a5) {
                    Adt1::C1(2u64, 5u64, Adt2::C0(1u64), Adt2::C0(3u64))
                } else {
                    let a6 = v1;
                    let a7 = 2u64;
                    if nat_eq(a6, a7) {
                        Adt1::C1(1u64, 4u64, Adt2::C0(4u64), Adt2::C0(5u64))
                    } else {
                        let a8 = v1;
                        let a9 = 3u64;
                        if nat_eq(a8, a9) {
                            Adt1::C1(2u64, 5u64, Adt2::C0(0u64), Adt2::C0(3u64))
                        } else {
                            let a10 = v1;
                            let a11 = 4u64;
                            if nat_eq(a10, a11) {
                                Adt1::C1(2u64, 6u64, Adt2::C0(1u64), Adt2::C0(2u64))
                            } else {
                                let a12 = v1;
                                let a13 = 5u64;
                                if nat_eq(a12, a13) {
                                    Adt1::C1(0u64, 5u64, Adt2::C0(5u64), Adt2::C0(6u64))
                                } else {
                                    Adt1::C0
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f1(v0: bool, v1: bool, v2: u64) -> Adt1 {
    if v0 {
        let a14 = v2;
        let a15 = 0u64;
        if nat_eq(a14, a15) {
            Adt1::C4(false, 11u64, Adt2::C0(0u64))
        } else {
            let a16 = v2;
            let a17 = 1u64;
            if nat_eq(a16, a17) {
                Adt1::C4(true, 12u64, Adt2::C0(1u64))
            } else {
                let a18 = v2;
                let a19 = 2u64;
                if nat_eq(a18, a19) {
                    Adt1::C4(false, 13u64, Adt2::C0(14u64))
                } else {
                    let a20 = v2;
                    let a21 = 3u64;
                    if nat_eq(a20, a21) {
                        Adt1::C3(10u64, Adt2::C0(2u64))
                    } else {
                        let a22 = v2;
                        let a23 = 4u64;
                        if nat_eq(a22, a23) {
                            Adt1::C1(2u64, 10u64, Adt2::C1(1073741824u64), Adt2::C0(10u64))
                        } else {
                            let a24 = v2;
                            let a25 = 5u64;
                            if nat_eq(a24, a25) {
                                Adt1::C1(2u64, 8u64, Adt2::C0(6u64), Adt2::C1(1073741824u64))
                            } else {
                                let a26 = v2;
                                let a27 = 6u64;
                                if nat_eq(a26, a27) {
                                    Adt1::C1(2u64, 8u64, Adt2::C0(8u64), Adt2::C1(1078530011u64))
                                } else {
                                    let a28 = v2;
                                    let a29 = 7u64;
                                    if nat_eq(a28, a29) {
                                        Adt1::C1(3u64, 8u64, Adt2::C0(12u64), Adt2::C0(8u64))
                                    } else {
                                        let a30 = v2;
                                        let a31 = 8u64;
                                        if nat_eq(a30, a31) {
                                            Adt1::C3(8u64, Adt2::C0(8u64))
                                        } else {
                                            let a32 = v2;
                                            let a33 = 9u64;
                                            if nat_eq(a32, a33) {
                                                Adt1::C1(2u64, 8u64, Adt2::C0(11u64), Adt2::C0(8u64))
                                            } else {
                                                let a34 = v2;
                                                let a35 = 10u64;
                                                if nat_eq(a34, a35) {
                                                    Adt1::C1(3u64, 8u64, Adt2::C0(8u64), Adt2::C0(10u64))
                                                } else {
                                                    let a36 = v2;
                                                    let a37 = 11u64;
                                                    if nat_eq(a36, a37) {
                                                        if v1 {
                                                            Adt1::C5(9u64, 8u64, Adt2::C0(8u64))
                                                        } else {
                                                            Adt1::C7
                                                        }
                                                    } else {
                                                        let a38 = v2;
                                                        let a39 = 12u64;
                                                        if nat_eq(a38, a39) {
                                                            Adt1::C2(true, 8u64, Adt2::C1(0u64), Adt2::C0(8u64))
                                                        } else {
                                                            let a40 = v2;
                                                            let a41 = 13u64;
                                                            if nat_eq(a40, a41) {
                                                                Adt1::C1(2u64, 9u64, Adt2::C0(7u64), Adt2::C1(1073741824u64))
                                                            } else {
                                                                let a42 = v2;
                                                                let a43 = 14u64;
                                                                if nat_eq(a42, a43) {
                                                                    Adt1::C1(2u64, 9u64, Adt2::C0(9u64), Adt2::C1(1078530011u64))
                                                                } else {
                                                                    let a44 = v2;
                                                                    let a45 = 15u64;
                                                                    if nat_eq(a44, a45) {
                                                                        Adt1::C1(3u64, 9u64, Adt2::C0(12u64), Adt2::C0(9u64))
                                                                    } else {
                                                                        let a46 = v2;
                                                                        let a47 = 16u64;
                                                                        if nat_eq(a46, a47) {
                                                                            Adt1::C3(9u64, Adt2::C0(9u64))
                                                                        } else {
                                                                            let a48 = v2;
                                                                            let a49 = 17u64;
                                                                            if nat_eq(a48, a49) {
                                                                                Adt1::C1(2u64, 9u64, Adt2::C0(11u64), Adt2::C0(9u64))
                                                                            } else {
                                                                                let a50 = v2;
                                                                                let a51 = 18u64;
                                                                                if nat_eq(a50, a51) {
                                                                                    Adt1::C1(3u64, 9u64, Adt2::C0(9u64), Adt2::C0(10u64))
                                                                                } else {
                                                                                    let a52 = v2;
                                                                                    let a53 = 19u64;
                                                                                    if nat_eq(a52, a53) {
                                                                                        if v1 {
                                                                                            Adt1::C5(10u64, 9u64, Adt2::C0(9u64))
                                                                                        } else {
                                                                                            Adt1::C7
                                                                                        }
                                                                                    } else {
                                                                                        let a54 = v2;
                                                                                        let a55 = 20u64;
                                                                                        if nat_eq(a54, a55) {
                                                                                            Adt1::C2(false, 9u64, Adt2::C0(13u64), Adt2::C0(9u64))
                                                                                        } else {
                                                                                            let a56 = v2;
                                                                                            let a57 = 21u64;
                                                                                            if nat_eq(a56, a57) {
                                                                                                Adt1::C1(3u64, 10u64, Adt2::C1(1065353216u64), Adt2::C0(3u64))
                                                                                            } else {
                                                                                                let a58 = v2;
                                                                                                let a59 = 22u64;
                                                                                                if nat_eq(a58, a59) {
                                                                                                    Adt1::C3(10u64, Adt2::C0(10u64))
                                                                                                } else {
                                                                                                    let a60 = v2;
                                                                                                    let a61 = 23u64;
                                                                                                    if nat_eq(a60, a61) {
                                                                                                        Adt1::C1(2u64, 10u64, Adt2::C1(1036831949u64), Adt2::C0(10u64))
                                                                                                    } else {
                                                                                                        let a62 = v2;
                                                                                                        let a63 = 24u64;
                                                                                                        if nat_eq(a62, a63) {
                                                                                                            Adt1::C1(0u64, 10u64, Adt2::C1(1065353216u64), Adt2::C0(10u64))
                                                                                                        } else {
                                                                                                            let a64 = v2;
                                                                                                            let a65 = 25u64;
                                                                                                            if nat_eq(a64, a65) {
                                                                                                                Adt1::C1(2u64, 5u64, Adt2::C0(5u64), Adt2::C0(10u64))
                                                                                                            } else {
                                                                                                                Adt1::C0
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
                            }
                        }
                    }
                }
            }
        }
    } else {
        Adt1::C0
    }
}

pub fn f2(v0: bool, v1: bool, v2: u64) -> Adt1 {
    if v0 {
        let a66 = v2;
        let a67 = 0u64;
        if nat_eq(a66, a67) {
            Adt1::C1(3u64, 8u64, Adt2::C0(0u64), Adt2::C0(1u64))
        } else {
            let a68 = v2;
            let a69 = 1u64;
            if nat_eq(a68, a69) {
                Adt1::C1(2u64, 9u64, Adt2::C0(3u64), Adt2::C0(8u64))
            } else {
                let a70 = v2;
                let a71 = 2u64;
                if nat_eq(a70, a71) {
                    Adt1::C4(false, 10u64, Adt2::C0(2u64))
                } else {
                    let a72 = v2;
                    let a73 = 3u64;
                    if nat_eq(a72, a73) {
                        Adt1::C1(1u64, 10u64, Adt2::C0(10u64), Adt2::C0(6u64))
                    } else {
                        let a74 = v2;
                        let a75 = 4u64;
                        if nat_eq(a74, a75) {
                            Adt1::C1(1u64, 11u64, Adt2::C0(7u64), Adt2::C0(6u64))
                        } else {
                            let a76 = v2;
                            let a77 = 5u64;
                            if nat_eq(a76, a77) {
                                Adt1::C2(true, 11u64, Adt2::C1(981668463u64), Adt2::C0(11u64))
                            } else {
                                let a78 = v2;
                                let a79 = 6u64;
                                if nat_eq(a78, a79) {
                                    Adt1::C1(3u64, 10u64, Adt2::C0(10u64), Adt2::C0(11u64))
                                } else {
                                    let a80 = v2;
                                    let a81 = 7u64;
                                    if nat_eq(a80, a81) {
                                        Adt1::C2(true, 10u64, Adt2::C1(0u64), Adt2::C0(10u64))
                                    } else {
                                        let a82 = v2;
                                        let a83 = 8u64;
                                        if nat_eq(a82, a83) {
                                            Adt1::C2(false, 10u64, Adt2::C1(1065353216u64), Adt2::C0(10u64))
                                        } else {
                                            let a84 = v2;
                                            let a85 = 9u64;
                                            if nat_eq(a84, a85) {
                                                Adt1::C1(1u64, 10u64, Adt2::C1(1065353216u64), Adt2::C0(10u64))
                                            } else {
                                                let a86 = v2;
                                                let a87 = 10u64;
                                                if nat_eq(a86, a87) {
                                                    Adt1::C1(2u64, 10u64, Adt2::C0(10u64), Adt2::C0(4u64))
                                                } else {
                                                    let a88 = v2;
                                                    let a89 = 11u64;
                                                    if nat_eq(a88, a89) {
                                                        Adt1::C1(1u64, 11u64, Adt2::C1(1065353216u64), Adt2::C0(10u64))
                                                    } else {
                                                        let a90 = v2;
                                                        let a91 = 12u64;
                                                        if nat_eq(a90, a91) {
                                                            Adt1::C1(2u64, 9u64, Adt2::C0(9u64), Adt2::C0(11u64))
                                                        } else {
                                                            let a92 = v2;
                                                            let a93 = 13u64;
                                                            if nat_eq(a92, a93) {
                                                                Adt1::C1(2u64, 8u64, Adt2::C0(8u64), Adt2::C0(10u64))
                                                            } else {
                                                                let a94 = v2;
                                                                let a95 = 14u64;
                                                                if nat_eq(a94, a95) {
                                                                    Adt1::C1(0u64, 9u64, Adt2::C0(9u64), Adt2::C0(8u64))
                                                                } else {
                                                                    let a96 = v2;
                                                                    let a97 = 15u64;
                                                                    if nat_eq(a96, a97) {
                                                                        Adt1::C6(8u64, 9u64, Adt2::C0(9u64))
                                                                    } else {
                                                                        let a98 = v2;
                                                                        let a99 = 16u64;
                                                                        if nat_eq(a98, a99) {
                                                                            Adt1::C1(2u64, 8u64, Adt2::C0(8u64), Adt2::C0(5u64))
                                                                        } else {
                                                                            let a100 = v2;
                                                                            let a101 = 17u64;
                                                                            if nat_eq(a100, a101) {
                                                                                Adt1::C1(2u64, 9u64, Adt2::C0(9u64), Adt2::C0(5u64))
                                                                            } else {
                                                                                let a102 = v2;
                                                                                let a103 = 18u64;
                                                                                if nat_eq(a102, a103) {
                                                                                    Adt1::C1(2u64, 9u64, Adt2::C0(9u64), if v1 {
                                                                                        Adt2::C1(3212836864u64)
                                                                                    } else {
                                                                                        Adt2::C1(1065353216u64)
                                                                                    })
                                                                                } else {
                                                                                    Adt1::C0
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
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        let a104 = v2;
        let a105 = 0u64;
        if nat_eq(a104, a105) {
            Adt1::C1(3u64, 8u64, Adt2::C0(0u64), Adt2::C0(1u64))
        } else {
            let a106 = v2;
            let a107 = 1u64;
            if nat_eq(a106, a107) {
                Adt1::C1(2u64, 9u64, Adt2::C0(3u64), Adt2::C0(8u64))
            } else {
                let a108 = v2;
                let a109 = 2u64;
                if nat_eq(a108, a109) {
                    Adt1::C6(8u64, 9u64, Adt2::C0(9u64))
                } else {
                    let a110 = v2;
                    let a111 = 3u64;
                    if nat_eq(a110, a111) {
                        Adt1::C1(2u64, 8u64, Adt2::C0(8u64), Adt2::C0(5u64))
                    } else {
                        let a112 = v2;
                        let a113 = 4u64;
                        if nat_eq(a112, a113) {
                            Adt1::C1(2u64, 9u64, Adt2::C0(9u64), Adt2::C0(5u64))
                        } else {
                            let a114 = v2;
                            let a115 = 5u64;
                            if nat_eq(a114, a115) {
                                Adt1::C1(2u64, 9u64, Adt2::C0(9u64), if v1 {
                                    Adt2::C1(3212836864u64)
                                } else {
                                    Adt2::C1(1065353216u64)
                                })
                            } else {
                                Adt1::C0
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn run(p0: Adt0, p1: u64) -> Adt1 {
    f0(p0, p1)
}
