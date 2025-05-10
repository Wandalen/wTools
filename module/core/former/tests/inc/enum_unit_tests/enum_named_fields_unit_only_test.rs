// Purpose: Provides shared test assertions and logic for verifying the constructors generated
// by `#[derive(Former)]` for enums with unit variants using named fields syntax.
// This file is included by both `enum_named_fields_unit_derive.rs` and `enum_named_fields_unit_manual.rs`.
//
// Coverage:
// - Rule 3a (Unit + Default): Tests static method `EnumWithNamedFields::unit_variant_default()`.
// - Rule 1a (Unit + `#[scalar]`): Tests static method `EnumWithNamedFields::unit_variant_scalar()`.
//
// Test Relevance/Acceptance Criteria:
// - Defines test functions (`unit_variant_scalar_test`, `unit_variant_default_construction`) that
//   invoke static methods provided by the including file (either derived or manual).
// - Asserts that the instances created by these constructors are equal to the expected
//   enum variants (`EnumWithNamedFields::UnitVariantScalar`, `EnumWithNamedFields::UnitVariantDefault`).
//
// File: module/core/former/tests/inc/former_enum_tests/unit_tests/enum_named_fields_unit_only_test.rs
use super::*;

// --- Unit Variant ---

#[ test ]
fn unit_variant_scalar_test() // New Test
{
  // Expect a direct static constructor taking no arguments.
  let got = EnumWithNamedFields::unit_variant_scalar();
  let expected = EnumWithNamedFields::UnitVariantScalar;
  assert_eq!( got, expected );
}

#[ test ]
fn unit_variant_default_construction() // Renamed Test
{
  // Expect a direct static constructor taking no arguments (default is scalar).
  let got = EnumWithNamedFields::unit_variant_default();
  let expected = EnumWithNamedFields::UnitVariantDefault;
  assert_eq!( got, expected );
}