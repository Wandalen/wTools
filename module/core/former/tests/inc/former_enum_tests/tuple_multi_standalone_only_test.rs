// File: module/core/former/tests/inc/former_enum_tests/tuple_multi_standalone_only_test.rs

/// # Test Logic: #[standalone_constructors] on Multi-Field Tuple Variants
///
/// This file contains the core test logic for verifying the `Former` derive macro's
/// handling of enums where a multi-field tuple variant is marked with
/// `#[standalone_constructors]` (on the enum) but *without* `#[arg_for_constructor]`
/// on the fields.
///
/// ## Purpose:
///
/// - **Verify Standalone Former Generation:** Ensure that `#[derive(Former)]` generates a standalone
///   constructor function (e.g., `enum_name::variant_name() -> VariantFormer<...>`) for multi-field
///   tuple variants under `#[standalone_constructors]` when fields are *not* marked with `#[arg_for_constructor]`.
/// - **Verify Setter Handling:** Confirm that the returned Former instance provides setters for each
///   field in the tuple.
///
/// This file is included via `include!` by both the `_manual.rs` and `_derive.rs`
/// test files for this scenario.

// use super::*; // Imports items from the parent file (manual or derive)

#[ test ]
fn multi_field_tuple_standalone_construction()
{
  // Tests the standalone constructor generated for a multi-field tuple variant
  // `VariantMultiStandalone(i32, bool)` with #[standalone_constructors] but no #[arg_for_constructor].
  // Expect a standalone constructor `TestEnumMultiStandalone::variant_multi_standalone()` returning a Former.
  let got = TestEnumMultiStandalone::variant_multi_standalone()
    ._0( 101 )
    ._1( true )
    .form();

  let expected = TestEnumMultiStandalone::VariantMultiStandalone( 101, true );
  assert_eq!( got, expected );
}