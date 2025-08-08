//! Purpose: Provides a manual implementation of constructors for an enum with unit variants
//! using named fields syntax, including static methods, to serve as a reference for verifying
//! the `#[ derive( Former ) ]` macro's behavior.
//!
//! Coverage:
//! - Rule 3a (Unit + Default): Manual implementation of static method `EnumWithNamedFields::unit_variant_default()`.
//! - Rule 1a (Unit + `#[ scalar ]`): Manual implementation of static method `EnumWithNamedFields::unit_variant_scalar()`.
//!
//! Test Relevance/Acceptance Criteria:
//! - Defines an enum `EnumWithNamedFields` with unit variants `UnitVariantDefault` and `UnitVariantScalar`.
//! - Manually implements static methods (`EnumWithNamedFields::unit_variant_scalar()`, `EnumWithNamedFields::unit_variant_default()`)
//!   that mirror the expected generated code for scalar unit variants.
//! - This file is included by `enum_named_fields_unit_only_test.rs` to provide the manual implementations
//!   that the shared tests compare against.
// File: module/core/former/tests/inc/former_enum_tests/unit_tests/enum_named_fields_unit_manual.rs
use super::*;
use former::{
  FormingEnd, StoragePreform, FormerDefinition, FormerDefinitionTypes, Storage, ReturnPreformed, FormerBegin, FormerMutator,
};
use core::marker::PhantomData;

// Define the enum with unit variants for manual testing.
#[ derive( Debug, PartialEq ) ]
pub enum EnumWithNamedFields {
  // --- Unit Variant ---
  UnitVariantScalar,  // New
  UnitVariantDefault, // Renamed
}

// --- Manual implementation of static methods on the Enum ---
impl EnumWithNamedFields {
  // --- Unit Variant ---
  #[ inline( always ) ]
  pub fn unit_variant_scalar() -> Self {
    Self::UnitVariantScalar
  } // New
  #[ inline( always ) ]
  pub fn unit_variant_default() -> Self {
    Self::UnitVariantDefault
  } // Renamed (Default is scalar)
}

// Include the test logic file
include!("enum_named_fields_unit_only_test.rs");
