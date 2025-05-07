// File: module/core/former/tests/inc/former_enum_tests/tuple_multi_scalar_derive.rs

//! # Derive Test: #[scalar] Attribute on Multi-Field Tuple Variants
//!
//! This test file verifies the `#[derive(Former)]` macro's handling of enums
//! with multi-field tuple variants when explicitly marked with `#[scalar]`.
//!
//! ## Purpose:
//!
//! - To ensure the derive macro generates a direct static constructor method for
//!   multi-field tuple variants marked with `#[scalar]`, correctly handling multiple fields
//!   and the `impl Into<...>` signatures.
//! - It uses the shared test logic from `tuple_multi_scalar_only_test.rs`.

// use super::*; // Imports testing infrastructure
use former::Former; // Import derive macro

// === Enum Definition ===

/// Enum using derive for #[scalar] multi-field tuple variant behavior.
#[ derive( Debug, PartialEq, Clone, Former ) ]
// #[ debug ] // Uncomment to see generated code later
pub enum TestEnumMultiScalar // Consistent name
{
  /// A multi-field tuple variant with #[scalar].
  #[ scalar ] // Explicitly request scalar constructor
  VariantMultiScalar( i32, bool ), // Multi-field tuple variant
}

// === Include Test Logic ===
// This file contains the actual #[ test ] functions.
include!( "tuple_multi_scalar_only_test.rs" );