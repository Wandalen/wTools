// Purpose: Provides shared test assertions and logic for both the derived and manual implementations
// of standalone scalar constructors for named (struct-like) variants with `#[arg_for_constructor]`
// fields. It tests that standalone constructors generated/implemented when the enum has
// `#[standalone_constructors]` and all variant fields have `#[arg_for_constructor]` behave as
// expected (scalar style, taking field arguments).
//
// Coverage:
// - Rule 4a (#[standalone_constructors]): Tests the existence and functionality of top-level constructor functions (`struct_variant_args`, `multi_struct_args`).
// - Rule 4b (Option 2 Logic): Tests that these standalone constructors take arguments corresponding to the `#[arg_for_constructor]` fields and return the final enum instance.
// - Rule 1e (Struct + Single-Field + `#[scalar]`): Implicitly tested via `StructVariantArgs`.
// - Rule 3e (Struct + Single-Field + Default): Implicitly tested via `StructVariantArgs`.
// - Rule 1g (Struct + Multi-Field + `#[scalar]`): Implicitly tested via `MultiStructArgs`.
// - Rule 3g (Struct + Multi-Field + Default): Implicitly tested via `MultiStructArgs`.
//
// Test Relevance/Acceptance Criteria:
// - Defines the `TestEnumArgs` enum structure with single-field (`StructVariantArgs { field: String }`) and multi-field (`MultiStructArgs { a: i32, b: bool }`) named variants.
// - Contains test functions (`struct_variant_args_test`, `multi_struct_variant_args_test`) that are included by the derive and manual test files.
// - Calls the standalone constructor functions (`struct_variant_args(value)`, `multi_struct_args(value1, value2)`) provided by the including file.
// - Asserts that the returned enum instances match manually constructed expected values (`TestEnumArgs::StructVariantArgs { field: value }`, `TestEnumArgs::MultiStructArgs { a: value1, b: value2 }`). This verifies that both derived and manual standalone constructors correctly handle field arguments and produce the final enum variant.

// File: module/core/former/tests/inc/former_enum_tests/named_tests/standalone_constructor_args_named_only_test.rs

// Use the items defined in the including file (manual or derive for args)
use super::*;

/// Tests the standalone constructor for a struct variant that takes arguments.
#[ test ]
fn struct_variant_args_test() // New test name
{
  // Test Matrix Row: T27.1 (Implicitly, as this tests the behavior expected by the matrix)
  // Assumes `struct_variant_args` takes a String argument and returns Self (Option 2)
  let instance = struct_variant_args( "arg_value" ); // Call directly
  let expected = TestEnumArgs::StructVariantArgs { field : "arg_value".to_string() };
  assert_eq!( instance, expected );
}

/// Tests the standalone constructor for a multi-field struct variant that takes arguments.
#[ test ]
fn multi_struct_variant_args_test()
{
  // Test Matrix Row: T27.2 (Implicitly, as this tests the behavior expected by the matrix)
  // Assumes `multi_struct_args` takes i32 and bool arguments and returns Self (Option 2)
  let instance = multi_struct_args( -1, false ); // Call directly
  let expected = TestEnumArgs::MultiStructArgs { a : -1, b : false };
  assert_eq!( instance, expected );
}