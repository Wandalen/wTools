// File: module/core/former/tests/inc/former_enum_tests/tuple_multi_standalone_args_only_test.rs

/// # Test Logic: #[standalone_constructors] and #[arg_for_constructor] on Multi-Field Tuple Variants
///
/// This file contains the core test logic for verifying the `Former` derive macro's
/// handling of enums where a multi-field tuple variant is marked with
/// `#[standalone_constructors]` (on the enum) and `#[arg_for_constructor]`
/// on the fields.
///
/// ## Purpose:
///
/// - **Verify Standalone Direct Constructor Generation:** Ensure that `#[derive(Former)]` generates a standalone
///   constructor function (e.g., `enum_name::variant_name(T1, T2, ...) -> Enum`) for multi-field
///   tuple variants under `#[standalone_constructors]` when fields *are* marked with `#[arg_for_constructor]`.
/// - **Verify Argument Handling in Constructor:** Confirm that the generated constructor correctly
///   accepts arguments via `impl Into<...>` for each field marked with `#[arg_for_constructor]`.
///
/// This file is included via `include!` by both the `_manual.rs` and `_derive.rs`
/// test files for this scenario.

// use super::*; // Imports items from the parent file (manual or derive)

#[ test ]
fn multi_field_tuple_standalone_args_construction()
{
  // Tests the standalone constructor generated for a multi-field tuple variant
  // `VariantMultiStandaloneArgs(i32, bool)` with #[standalone_constructors] and #[arg_for_constructor].
  // Expect a standalone constructor `TestEnumMultiStandaloneArgs::variant_multi_standalone_args(i32, bool)` returning Self.
  let got = TestEnumMultiStandaloneArgs::variant_multi_standalone_args( 303, false );

  let expected = TestEnumMultiStandaloneArgs::VariantMultiStandaloneArgs( 303, false );
  assert_eq!( got, expected );
}