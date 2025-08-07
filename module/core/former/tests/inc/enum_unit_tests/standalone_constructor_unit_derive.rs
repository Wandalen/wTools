//! Purpose: Tests the `#[derive(Former)]` macro's generation of standalone constructors
//! for unit variants. This file focuses on verifying the derive-based implementation.
//!
//! Coverage:
//! - Rule 3a (Unit + Default): Verifies `TestEnum::unit_variant() -> TestEnum` (implicitly, as default is scalar).
//! - Rule 1a (Unit + `#[scalar]`): Verifies `TestEnum::unit_variant() -> TestEnum` (implicitly, as default is scalar).
//! - Rule 4a (#[standalone_constructors]): Verifies generation of the top-level constructor function `unit_variant()`.
//!
//! Test Relevance/Acceptance Criteria:
//! - Defines an enum `TestEnum` with a unit variant `UnitVariant`, and the `#[derive(Former)]` and `#[standalone_constructors]` attributes.
//! - Relies on the derived top-level function `unit_variant()` defined in `standalone_constructor_unit_only_test.rs`.
//! - Asserts that the instance created by this constructor is equal to the expected
//!   enum variant (`TestEnum::UnitVariant`).
// File: module/core/former/tests/inc/former_enum_tests/unit_tests/standalone_constructor_unit_derive.rs
#[ allow( unused_imports ) ]
use ::former::prelude::*;
use ::former::Former; // Import derive macro

// === Enum Definition ===

/// Enum using derive for standalone constructors.
#[ derive( Debug, PartialEq, Clone, Former ) ]
#[ standalone_constructors ] // New attribute is active
pub enum TestEnum // Consistent name
{
  /// A unit variant.
  UnitVariant,
}

// === Include Test Logic ===
include!( "standalone_constructor_unit_only_test.rs" ); // Use the consistent name