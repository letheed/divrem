//! Division and modulus traits and implementations.
//!
//! There are several definitions for the division and modulus
//! functions, each with different properties.
//! Probably the most common in computer science is truncated division
//! (rounding towards zero) since it is the one provided by most processors
//! and defined as the `/` (and matching `%`) operator in the ISO C99 standard.
//!
//! This crate provides the following definitions:
//!
//! * Floored division (rounding towards negative infinity).
//! * Ceiled division (rounding towards positive infinity).
//! * Euclidean division (sign of modulus is always positive).
//!
//! For every definition, we provide a `Div`, a `Rem` and a `DivRem` variant.
//!
//! A `DivRem` variant of the truncated division is also provided for
//! convenience since it does not exist in the standard library.

#![warn(missing_docs)]
#![warn(rust_2018_idioms)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![warn(clippy::nursery)]
#![deny(unsafe_code)]
#![no_std]

use core::ops::{Div, Rem};

/// Truncated division and remainder.
///
/// Truncates the quotient and effectively rounds towards zero.
/// The sign of the modulus is always the same as the sign of the dividend.
///
/// This is the same as the `/` and `%` operators.
///
/// This is equivalent to the `quotRem` function in Haskell.
pub trait DivRem<RHS = Self>: Div<RHS> + Rem<RHS> {
    /// The resulting type after applying the `/` and `%` operators.
    type Output;

    /// Performs the `/` and `%` operations.
    fn div_rem(self, other: RHS) -> <Self as DivRem<RHS>>::Output;
}

/// Floored division.
///
/// Floors the quotient and effectively rounds towards negative infinity.
///
/// This is equivalent to the `div` function in Haskell.
pub trait DivFloor<RHS = Self>: Div<RHS> {
    /// Performs the floored division operation.
    fn div_floor(self, other: RHS) -> <Self as Div<RHS>>::Output;
}

/// Floored division remainder.
///
/// The sign of the modulus is always the same as the sign of the divisor
/// or zero.
///
/// This is equivalent to the `mod` function in Haskell.
pub trait RemFloor<RHS = Self>: Rem<RHS> {
    /// Returns the remainder of the floored division operation.
    fn rem_floor(self, other: RHS) -> <Self as Rem<RHS>>::Output;
}

/// Floored division and remainder.
///
/// Floors the quotient and effectively rounds towards negative infinity.
/// The sign of the modulus is always the same as the sign of the divisor
/// or zero.
///
/// This is equivalent to the `divMod` function in Haskell.
pub trait DivRemFloor<RHS = Self>: DivRem<RHS> {
    /// Performs the floored division operation with remainder.
    fn div_rem_floor(self, other: RHS) -> <Self as DivRem<RHS>>::Output;
}

/// Euclidean division.
///
/// The sign of the modulus is always positive or zero.
pub trait DivEuclid<RHS = Self>: Div<RHS> {
    /// Performs the euclidean division operation.
    fn div_euclid(self, other: RHS) -> <Self as Div<RHS>>::Output;
}

/// Euclidean remainder.
///
/// The sign of the modulus is always positive or zero.
pub trait RemEuclid<RHS = Self>: Rem<RHS> {
    /// Returns the remainder of the euclidean division operation.
    fn rem_euclid(self, other: RHS) -> <Self as Rem<RHS>>::Output;
}

/// Euclidean division and remainder.
///
/// The sign of the modulus is always positive or zero.
pub trait DivRemEuclid<RHS = Self>: DivRem<RHS> {
    /// Performs the euclidean division operation with remainder.
    fn div_rem_euclid(self, other: RHS) -> <Self as DivRem<RHS>>::Output;
}

/// Ceiled division.
///
/// Ceils the quotient and effectively rounds towards positive infinity.
pub trait DivCeil<RHS = Self>: Div<RHS> {
    /// Performs the ceiled division operation.
    fn div_ceil(self, other: RHS) -> <Self as Div<RHS>>::Output;
}

/// Ceiled division remainder.
///
/// The sign of the modulus is always the opposite of the sign of the divisor
/// or zero.
pub trait RemCeil<RHS = Self>: Rem<RHS> {
    /// Returns the remainder of the ceiled division operation.
    fn rem_ceil(self, other: RHS) -> <Self as Rem<RHS>>::Output;
}

/// Ceiled division and remainder.
///
/// Ceils the quotient and effectively rounds towards positive infinity.
/// The sign of the modulus is always the opposite of the sign of the divisor
/// or zero.
pub trait DivRemCeil<RHS = Self>: DivRem<RHS> {
    /// Performs the ceiled division operation with remainder.
    fn div_rem_ceil(self, other: RHS) -> <Self as DivRem<RHS>>::Output;
}

#[macro_use]
mod macros;
mod ceil;
mod euclid;
mod floor;
mod trunc;
