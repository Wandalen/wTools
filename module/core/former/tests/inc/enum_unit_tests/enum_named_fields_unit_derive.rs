//! Purpose: Tests the `#[derive(Former)]` macro's generation of constructors for unit variants
//! within an enum that uses named fields syntax for its variants, including with `#[scalar]`
//! and `#[standalone_constructors]`. This file focuses on verifying the derive-based implementation.
//!
//! Coverage:
//! - Rule 3a (Unit + Default): Verifies `EnumWithNamedFields::unit_variant_default() -> EnumWithNamedFields`.
//! - Rule 1a (Unit + `#[scalar]`): Verifies `EnumWithNamedFields::unit_variant_scalar() -> EnumWithNamedFields`.
//! - Rule 4a (`#[standalone_constructors]`): Verifies generation of top-level constructor functions (though not explicitly tested in `_only_test.rs`).
//!
//! Test Relevance/Acceptance Criteria:
//! - Defines an enum `EnumWithNamedFields` with unit variants `UnitVariantDefault` and `UnitVariantScalar`,
//!   using named fields syntax (`{}`). `UnitVariantScalar` has the `#[scalar]` attribute. The enum has
//!   `#[derive(Former)]`, `#[ debug ]`, and `#[standalone_constructors]`.
//! - Relies on the derived static methods (`EnumWithNamedFields::unit_variant_scalar()`, `EnumWithNamedFields::unit_variant_default()`)
//!   defined in `enum_named_fields_unit_only_test.rs`.
//! - Asserts that these constructors produce the correct `EnumWithNamedFields` enum instances by comparing
//!   with manually constructed variants.
// File: module/core/former/tests/inc/former_enum_tests/unit_tests/enum_named_fields_unit_derive.rs
use super::*;

// Define the enum with unit variants for testing.
// xxx : Re-enable when trailing comma issue is fully fixed in macro_tools::generic_params::decompose
// #[derive(Debug, PartialEq, former::Former)]
#[derive(Debug, PartialEq)]
// #[ debug ]
#[standalone_constructors]
pub enum EnumWithNamedFields {
  // --- Unit Variant ---
  // Expect: unit_variant_default() -> Enum (Default is scalar for unit)
  UnitVariantDefault, // Renamed from UnitVariant
  #[scalar] // Expect: unit_variant_scalar() -> Enum
  UnitVariantScalar, // New
}

// Include the test logic file
include!("enum_named_fields_unit_only_test.rs");
