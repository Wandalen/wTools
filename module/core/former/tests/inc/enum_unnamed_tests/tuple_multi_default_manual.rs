// File: module/core/former/tests/inc/former_enum_tests/tuple_multi_default_manual.rs

//! # Manual Test: Default Behavior on Multi-Field Tuple Variants
//!
//! This file provides a manual implementation of the scalar-like static constructor
//! for an enum (`TestEnumMulti`) with a multi-field tuple variant (`VariantMulti(i32, bool)`),
//! demonstrating the expected default behavior without the `#[scalar]` attribute.
//!
//! ## Purpose:
//!
//! - To serve as a reference implementation demonstrating how the scalar-like static constructor
//!   should behave for multi-field tuple variants by default.
//! - To manually implement the static method (`variant_multi`), ensuring correct
//!   handling of multiple fields and the `impl Into<...>` signatures.
//! - To validate the logic used by the `#[derive(Former)]` macro by comparing its generated
//!   code's behavior against this manual implementation using the shared tests in
//!   `tuple_multi_default_only_test.rs`.

// use super::*; // Imports testing infrastructure

// === Enum Definition ===

/// Enum for manual testing of default multi-field tuple variant behavior.
#[ derive( Debug, PartialEq, Clone ) ]
pub enum TestEnumMulti // Consistent name
{
  /// A multi-field tuple variant.
  VariantMulti( i32, bool ), // Multi-field tuple variant
}

// === Manual implementation of static methods on TestEnumMulti ===
impl TestEnumMulti
{
  /// Manually implemented constructor for the VariantMulti variant (scalar style).
  #[ inline( always ) ]
  pub fn variant_multi( field1 : impl Into< i32 >, field2 : impl Into< bool > ) -> Self
  {
    Self::VariantMulti( field1.into(), field2.into() )
  }
}

// === Include the Test Logic ===
// This file contains the actual #[ test ] functions.
include!( "tuple_multi_default_only_test.rs" );