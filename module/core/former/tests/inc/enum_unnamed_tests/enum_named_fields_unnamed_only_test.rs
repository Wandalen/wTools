// Purpose: Provides shared test assertions and logic for verifying the constructors generated
// by `#[derive(Former)]` for enums with zero-field unnamed (tuple) variants using named fields syntax.
// This file is included by both `enum_named_fields_unnamed_derive.rs` and `enum_named_fields_unnamed_manual.rs`.
//
// Coverage:
// - Rule 3b (Tuple + Zero-Field + Default): Tests static method `EnumWithNamedFields::variant_zero_unnamed_default()`.
// - Rule 1b (Tuple + Zero-Field + `#[scalar]`): Tests static method `EnumWithNamedFields::variant_zero_unnamed_scalar()`.
//
// Test Relevance/Acceptance Criteria:
// - Defines test functions (`variant_zero_unnamed_scalar_test`, `variant_zero_unnamed_default_test`) that
//   invoke static methods provided by the including file (either derived or manual).
// - Asserts that the instances created by these constructors are equal to the expected
//   enum variants (`EnumWithNamedFields::VariantZeroUnnamedScalar()`, `EnumWithNamedFields::VariantZeroUnnamedDefault()`).
//
// File: module/core/former/tests/inc/former_enum_tests/unnamed_tests/enum_named_fields_unnamed_only_test.rs
use super::*; // Imports EnumWithNamedFields

// --- Zero Fields (Unnamed) ---

#[ test ]
fn variant_zero_unnamed_scalar_test() // New Test
{
  // Expect a direct static constructor taking no arguments.
  let got = EnumWithNamedFields::variant_zero_unnamed_scalar();
  let expected = EnumWithNamedFields::VariantZeroUnnamedScalar();
  assert_eq!( got, expected );
}

#[ test ]
fn variant_zero_unnamed_default_test() // New Test
{
  // Expect a direct static constructor taking no arguments (default is scalar).
  let got = EnumWithNamedFields::variant_zero_unnamed_default();
  let expected = EnumWithNamedFields::VariantZeroUnnamedDefault();
  assert_eq!( got, expected );
}