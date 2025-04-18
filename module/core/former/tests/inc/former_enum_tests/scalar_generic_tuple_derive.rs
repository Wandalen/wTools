// File: module/core/former/tests/inc/former_enum_tests/scalar_generic_tuple_derive.rs

//! # Derive Test: #[scalar] Attribute on Generic Tuple Variants
//!
//! This test file verifies the `#[derive(Former)]` macro's handling of tuple variants
//! containing generic types when the variant is explicitly marked with `#[scalar]`.
//!
//! ## Purpose:
//!
//! - To ensure the derive macro generates a direct static constructor method for
//!   `#[scalar]` tuple variants, correctly handling generic parameters and bounds.
//! - To confirm the generated constructor signature accepts arguments via `impl Into<...>`
//!   for each field in the tuple, including generic ones.
//! - It uses the shared test logic from `scalar_generic_tuple_only_test.rs`.

use super::*; // Imports testing infrastructure and potentially other common items

// --- Bound, Types, and Inner Struct ---
// Are defined in the included _only_test.rs file

// --- Enum Definition with Bounds and #[scalar] Variants ---
// Apply Former derive here. This is what we are testing.
#[ derive( Debug, PartialEq, Clone, former::Former ) ]
#[ debug ] // Uncomment to see generated code later
pub enum EnumScalarGeneric< T : Bound > // Enum bound
{
  #[ scalar ] // Explicitly request scalar constructor
  Variant1( InnerScalar< T > ), // Tuple variant with one generic field

  #[ scalar ] // Explicitly request scalar constructor
  Variant2( InnerScalar< T >, bool ), // Tuple variant with generic and non-generic fields
}

// --- Include the Test Logic ---
// This file contains the actual #[test] functions.
include!( "scalar_generic_tuple_only_test.rs" );