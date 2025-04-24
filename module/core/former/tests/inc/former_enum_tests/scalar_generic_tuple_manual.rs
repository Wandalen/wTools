// File: module/core/former/tests/inc/former_enum_tests/scalar_generic_tuple_manual.rs

//! # Manual Test: #[scalar] Attribute on Generic Tuple Variants
//!
//! This file provides a manual implementation of the `Former` pattern's static constructors
//! for an enum (`EnumScalarGeneric<T>`) with tuple variants containing generic types,
//! where those variants would conceptually be marked with `#[scalar]`.
//!
//! ## Purpose:
//!
//! - To serve as a reference implementation demonstrating how the static constructors
//!   should behave for `#[scalar]` tuple variants involving generics.
//! - To manually implement the static methods (`variant_1`, `variant_2`), ensuring correct
//!   handling of the enum's generic parameter `T`, its bounds, and the `impl Into<...>`
//!   signatures for the variant fields.
//! - To validate the logic used by the `#[derive(Former)]` macro by comparing its generated
//!   code's behavior against this manual implementation using the shared tests in
//!   `scalar_generic_tuple_only_test.rs`.

use super::*; // Imports testing infrastructure and potentially other common items

// --- Bound, Types, and Inner Struct ---
// Are defined in the included _only_test.rs file

// --- Enum Definition with Bounds ---
// Define the enum without the derive macro
#[ derive( Debug, PartialEq, Clone ) ]
pub enum EnumScalarGeneric< T : Bound > // Enum bound
{
  Variant1( InnerScalar< T > ), // Tuple variant with one generic field
  Variant2( InnerScalar< T >, bool ), // Tuple variant with generic and non-generic fields
}

// --- Manual implementation of static methods ---
impl< T : Bound > EnumScalarGeneric< T > // Apply bounds from enum definition
{
  /// Manually implemented constructor for the Variant1 variant (scalar style).
  #[ inline( always ) ]
  // FIX: Renamed to snake_case
  pub fn variant_1( value : impl Into< InnerScalar< T > > ) -> Self
  {
    Self::Variant1( value.into() )
  }

  /// Manually implemented former builder for the Variant2 variant.
  #[ inline( always ) ]
  // FIX: Renamed to snake_case
  pub fn variant_2() -> EnumScalarGenericVariant2Former< T >
  {
    EnumScalarGenericVariant2Former::begin( None, None, EnumScalarGenericVariant2End::< T >::default() )
  }
}

// --- Include the Test Logic ---
// This file contains the actual #[ test ] functions.
include!( "scalar_generic_tuple_only_test.rs" );