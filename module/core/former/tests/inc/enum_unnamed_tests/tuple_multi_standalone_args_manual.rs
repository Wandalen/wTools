// File: module/core/former/tests/inc/former_enum_tests/tuple_multi_standalone_args_manual.rs

//! # Manual Test: #[standalone_constructors] and #[arg_for_constructor] on Multi-Field Tuple Variants (Returns Self)
//!
//! This file provides a manual implementation of the standalone constructor that takes arguments
//! and returns Self for an enum (`TestEnumMultiStandaloneArgs`) with a multi-field tuple variant
//! (`VariantMultiStandaloneArgs(i32, bool)`), demonstrating the expected behavior under
//! `#[standalone_constructors]` with `#[arg_for_constructor]` on the fields.
//!
//! ## Purpose:
//!
//! - To serve as a reference implementation demonstrating how the standalone constructor should
//!   behave for multi-field tuple variants when it takes arguments and returns Self.
//! - To manually implement the standalone constructor function (`variant_multi_standalone_args`).
//! - To validate the logic used by the `#[derive(Former)]` macro by comparing its generated
//!   code's behavior against this manual implementation using the shared tests in
//!   `tuple_multi_standalone_args_only_test.rs`.

// use super::*; // Imports testing infrastructure

// === Enum Definition ===

/// Enum for manual testing of #[standalone_constructors] with #[arg_for_constructor] on multi-field tuple variants.
#[ derive( Debug, PartialEq, Clone ) ]
pub enum TestEnumMultiStandaloneArgs // Consistent name
{
  /// A multi-field tuple variant with #[standalone_constructors] and #[arg_for_constructor].
  VariantMultiStandaloneArgs( i32, bool ), // Multi-field tuple variant
}

// === Manual implementation of static methods on TestEnumMultiStandaloneArgs ===
impl TestEnumMultiStandaloneArgs
{
  /// Manually implemented standalone constructor for the VariantMultiStandaloneArgs variant.
  /// Takes arguments for fields marked with #[arg_for_constructor] and returns Self.
  #[ inline( always ) ]
  pub fn variant_multi_standalone_args( field1 : impl Into< i32 >, field2 : impl Into< bool > ) -> Self
  {
    Self::VariantMultiStandaloneArgs( field1.into(), field2.into() )
  }
}

// === Include the Test Logic ===
include!( "tuple_multi_standalone_args_only_test.rs" );