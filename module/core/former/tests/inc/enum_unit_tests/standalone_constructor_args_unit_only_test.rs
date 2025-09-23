#[allow(clippy::used_underscore_binding, clippy::all, warnings)]
// Purpose: Provides shared test assertions and logic for verifying the standalone constructor for a unit variant,
// intended to be included by both the derived (`standalone_constructor_args_unit_derive.rs`) and manual
// (`standalone_constructor_args_unit_manual.rs`) test files.
//
// Coverage:
// - Rule 3a (Unit + Default): Covered by the default behavior of unit variants.
// - Rule 1a (Unit + `#[ scalar ]`): Unit variants implicitly behave as scalar.
// - Rule 4a (#[ standalone_constructors ]): Verifies the functionality of the top-level constructor function.
//
// Test Relevance/Acceptance Criteria:
// - Contains the `unit_variant_args_test` function.
// - This test assumes the existence of a standalone constructor function `unit_variant_args()` and the enum `TestEnumArgs` in the including scope.
// - It invokes `unit_variant_args()` and asserts that the returned instance is equal to the direct enum variant `TestEnumArgs::UnitVariantArgs`.

// File: module/core/former/tests/inc/former_enum_tests/unit_tests/standalone_constructor_args_unit_only_test.rs

// Use the items defined in the including file (manual or derive for args)
use super::*;

/// Tests the standalone constructor for a unit variant (still takes no args).
#[ test ]
fn unit_variant_args_test() // New test name
{
  // Assumes `unit_variant_args` is defined in the including scope
  let instance = unit_variant_args(); // Returns Enum directly
  let expected = TestEnumArgs::UnitVariantArgs;
  assert_eq!( instance, expected );
}