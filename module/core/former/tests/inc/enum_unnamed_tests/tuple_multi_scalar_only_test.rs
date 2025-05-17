// File: module/core/former/tests/inc/former_enum_tests/tuple_multi_scalar_only_test.rs

/// # Test Logic: #[scalar] Attribute on Multi-Field Tuple Variants
///
/// This file contains the core test logic for verifying the `Former` derive macro's
/// handling of enums where a multi-field tuple variant is explicitly marked
/// with the `#[scalar]` attribute.
///
/// ## Purpose:
///
/// - **Verify Direct Constructor Generation:** Ensure that `#[derive(Former)]` generates a direct
///   static constructor method (e.g., `Enum::variant_name(T1, T2, ...) -> Enum`) for multi-field
///   tuple variants marked with `#[scalar]`.
/// - **Verify Argument Handling in Constructor:** Confirm that the generated constructor correctly
///   accepts arguments via `impl Into<...>` for each field in the tuple.
///
/// This file is included via `include!` by both the `_manual.rs` and `_derive.rs`
/// test files for this scenario.

// use super::*; // Imports items from the parent file (manual or derive)

#[ test ]
fn multi_field_tuple_scalar_construction()
{
  // Tests the direct constructor generated for a multi-field tuple variant
  // `VariantMultiScalar(i32, bool)` marked with `#[scalar]`.
  // Expect a direct static constructor `variant_multi_scalar` taking `impl Into<i32>` and `impl Into<bool>`.
  let got = TestEnumMultiScalar::variant_multi_scalar( 202, false );

  let expected = TestEnumMultiScalar::VariantMultiScalar( 202, false );
  assert_eq!( got, expected );
}