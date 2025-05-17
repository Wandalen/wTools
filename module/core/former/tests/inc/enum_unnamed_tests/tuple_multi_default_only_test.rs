// File: module/core/former/tests/inc/former_enum_tests/tuple_multi_default_only_test.rs

/// # Test Logic: Default Behavior on Multi-Field Tuple Variants
///
/// This file contains the core test logic for verifying the `Former` derive macro's
/// default handling of enums with multi-field tuple variants.
///
/// ## Purpose:
///
/// - **Verify Scalar-like Constructor Generation:** Ensure that `#[derive(Former)]` generates a direct
///   static constructor method (e.g., `Enum::variant_name(T1, T2, ...) -> Enum`) for multi-field
///   tuple variants by default, instead of a subformer starter.
/// - **Verify Argument Handling in Constructor:** Confirm that the generated constructor correctly
///   accepts arguments via `impl Into<...>` for each field in the tuple.
///
/// This file is included via `include!` by both the `_manual.rs` and `_derive.rs`
/// test files for this scenario.

// use super::*; // Imports items from the parent file (manual or derive)

#[ test ]
fn multi_field_tuple_default_construction()
{
  // Tests the direct constructor generated for a multi-field tuple variant
  // `VariantMulti(i32, bool)` with default behavior.
  // Expect a direct static constructor `variant_multi` taking `impl Into<i32>` and `impl Into<bool>`.
  let got = TestEnumMulti::variant_multi( 101, true );

  let expected = TestEnumMulti::VariantMulti( 101, true );
  assert_eq!( got, expected );
}