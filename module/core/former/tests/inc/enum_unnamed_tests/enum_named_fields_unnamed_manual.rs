//! Purpose: Provides a manual implementation of constructors for an enum with zero-field
//! unnamed (tuple) variants using named fields syntax, including static methods, to serve
//! as a reference for verifying the `#[derive(Former)]` macro's behavior.
//!
//! Coverage:
//! - Rule 3b (Tuple + Zero-Field + Default): Manual implementation of static method `EnumWithNamedFields::variant_zero_unnamed_default()`.
//! - Rule 1b (Tuple + Zero-Field + `#[scalar]`): Manual implementation of static method `EnumWithNamedFields::variant_zero_unnamed_scalar()`.
//!
//! Test Relevance/Acceptance Criteria:
//! - Defines an enum `EnumWithNamedFields` with two zero-field unnamed variants: `VariantZeroUnnamedDefault()` and `VariantZeroUnnamedScalar()`.
//! - Manually implements static methods (`EnumWithNamedFields::variant_zero_unnamed_scalar()`, `EnumWithNamedFields::variant_zero_unnamed_default()`)
//!   that mirror the expected generated code for scalar zero-field variants.
//! - This file is included by `enum_named_fields_unnamed_only_test.rs` to provide the manual implementations
//!   that the shared tests compare against.
// File: module/core/former/tests/inc/former_enum_tests/unnamed_tests/enum_named_fields_unnamed_manual.rs
use super::*;
use former::
{
  FormingEnd, StoragePreform, FormerDefinition, FormerDefinitionTypes, Storage,
  ReturnPreformed, FormerBegin, FormerMutator,
};
use std::marker::PhantomData;

// Define the enum with zero-field unnamed (tuple) variants for manual testing.
#[ derive( Debug, PartialEq ) ]
pub enum EnumWithNamedFields
{
  // --- Zero Fields (Unnamed - Tuple-like) ---
  VariantZeroUnnamedScalar(), // New
  VariantZeroUnnamedDefault(), // New
}

// --- Manual implementation of static methods on the Enum ---
impl EnumWithNamedFields
{
  // --- Zero Fields (Unnamed - Tuple-like) ---
  #[ inline( always ) ]
  pub fn variant_zero_unnamed_scalar() -> Self { Self::VariantZeroUnnamedScalar() } // New
  #[ inline( always ) ]
  pub fn variant_zero_unnamed_default() -> Self { Self::VariantZeroUnnamedDefault() } // New (Default is scalar)
}

// Include the test logic file
include!( "enum_named_fields_unnamed_only_test.rs" );