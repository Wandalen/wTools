//! Purpose: Provides shared test assertions and logic for both the derived and manual implementations
//! of standalone former builders for named (struct-like) variants without `#[arg_for_constructor]`
//! fields. It tests that standalone constructors generated/implemented when the enum has
//! `#[standalone_constructors]` and no variant fields have `#[arg_for_constructor]` behave as
//! expected (former builder style, allowing field setting via setters).
//!
//! Coverage:
//! - Rule 4a (#[standalone_constructors]): Tests the existence and functionality of the top-level constructor function (`struct_variant`).
//! - Rule 4b (Option 2 Logic): Tests that the standalone constructor returns a former builder for the variant and that its fields can be set using setters (`.field()`).
//! - Rule 1e (Struct + Single-Field + `#[scalar]`): Implicitly tested via `StructVariant`.
//! - Rule 3e (Struct + Single-Field + Default): Implicitly tested via `StructVariant`.
//!
//! Test Relevance/Acceptance Criteria:
//! - Defines the `TestEnum` enum structure with a single-field named variant `StructVariant { field: String }`.
//! - Contains a test function (`struct_variant_test`) that is included by the derive and manual test files.
//! - Calls the standalone constructor function `struct_variant()` provided by the including file.
//! - Uses the returned former builder's setter (`.field()`) to set the field.
//! - Calls `.form()` on the former builder to get the final enum instance.
//! - Asserts that the resulting enum instance matches a manually constructed `TestEnum::StructVariant { field: value }`. This verifies that both derived and manual standalone constructors correctly return former builders and allow setting fields via setters.

// File: module/core/former/tests/inc/former_enum_tests/named_tests/standalone_constructor_named_only_test.rs

// Use the items defined in the including file (manual or derive)
use super::*;

/// Tests the standalone constructor for a struct variant.
#[ test ]
fn struct_variant_test() // Use enum-specific test name
{
  // Test Matrix Row: T28.1 (Implicitly, as this tests the behavior expected by the matrix)
  // Call the constructor function (manual or derived)
  let former = struct_variant(); // <<< Call with zero args

  // Use the former to build the variant
  let instance = former
  .field( "value".to_string() ) // Set the struct field using the generated setter
  .form();

  // Define the expected enum instance (using the consistent enum name)
  let expected = TestEnum::StructVariant { field : "value".to_string() }; // Use TestEnum

  // Assert that the formed instance matches the expected one
  assert_eq!( instance, expected );
}