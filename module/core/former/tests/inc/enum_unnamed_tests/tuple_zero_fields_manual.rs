//! Purpose: Provides a hand-written implementation of the `Former` pattern's static constructors
//! for zero-field tuple variants, demonstrating the manual implementation corresponding to both
//! default behavior and the effect of the `#[scalar]` attribute.
//!
//! Coverage:
//! - Rule 3b (Tuple + Zero-Field + Default): Manually implements the static method `EnumWithZeroFieldTuple::variant_zero_default()` to return the enum instance.
//! - Rule 1b (Tuple + Zero-Field + `#[scalar]`): Manually implements the static method `EnumWithZeroFieldTuple::variant_zero_scalar()` to return the enum instance.
//! - Rule 4a (#[standalone_constructors]): Manually implements standalone constructor functions (`standalone_variant_zero_default`, `standalone_variant_zero_scalar`) to return the enum instance, corresponding to the tests in `_only_test.rs`.
//!
//! Test Relevance/Acceptance Criteria:
//! - Defines an enum `EnumWithZeroFieldTuple` with zero-field tuple variants `VariantZeroDefault` and `VariantZeroScalar`.
//! - Provides hand-written static methods (`variant_zero_default`, `variant_zero_scalar`) and standalone functions (`standalone_variant_zero_default`, `standalone_variant_zero_scalar`) that mimic the behavior expected from the `#[derive(Former)]` macro for zero-field tuple variants.
//! - Includes shared test logic from `tuple_zero_fields_only_test.rs`.
//! - The included tests call these manually implemented methods/functions and assert that the returned enum instances match the direct enum variants. This verifies the manual implementation of constructors for zero-field tuple variants.

#[ allow( unused_imports ) ]
use ::former::prelude::*;
use test_tools::exposed::*;
use core::fmt::Debug;
use core::marker::PhantomData;

// Helper struct used in tests
#[ derive( Debug, PartialEq, Default ) ]
pub struct InnerForSubform
{
  pub value : i32,
}

// Define the enums without the derive macro
#[derive(Debug, PartialEq)]
pub enum ZeroTuple { Variant() }

impl ZeroTuple
{
  #[inline(always)]
  pub fn variant() -> Self
  {
    Self::Variant()
  }
}

#[inline(always)]
pub fn zero_tuple_variant() -> ZeroTuple
{
  ZeroTuple::Variant()
}

#[derive(Debug, PartialEq)]
pub enum ZeroTupleScalar { Variant() }

impl ZeroTupleScalar
{
  #[inline(always)]
  pub fn variant() -> Self
  {
    Self::Variant()
  }
}

#[inline(always)]
pub fn zero_tuple_scalar_variant() -> ZeroTupleScalar
{
  ZeroTupleScalar::Variant()
}

// Include the shared test logic
include!( "./tuple_zero_fields_only_test.rs" );