// File: module/core/former/tests/inc/former_enum_tests/tuple_multi_default_derive.rs

//! # Derive Test: Default Behavior on Multi-Field Tuple Variants
//!
//! This test file verifies the `#[derive(Former)]` macro's default handling of enums
//! with multi-field tuple variants.
//!
//! ## Purpose:
//!
//! - To ensure the derive macro generates a direct static constructor method for
//!   multi-field tuple variants by default, correctly handling multiple fields
//!   and the `impl Into<...>` signatures.
//! - It uses the shared test logic from `tuple_multi_default_only_test.rs`.

// use super::*; // Imports testing infrastructure
use former::Former; // Import derive macro

// === Enum Definition ===

/// Enum using derive for default multi-field tuple variant behavior.
#[ derive( Debug, PartialEq, Clone, Former ) ]
// #[ debug ] // Uncomment to see generated code later
pub enum TestEnumMulti // Consistent name
{
  /// A multi-field tuple variant.
  VariantMulti( i32, bool ), // Multi-field tuple variant (default behavior)
}

// === Include Test Logic ===
// This file contains the actual #[ test ] functions.
include!( "tuple_multi_default_only_test.rs" );