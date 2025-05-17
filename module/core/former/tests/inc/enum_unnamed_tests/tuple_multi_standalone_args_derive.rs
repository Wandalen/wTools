// File: module/core/former/tests/inc/former_enum_tests/tuple_multi_standalone_args_derive.rs

//! # Derive Test: #[standalone_constructors] and #[arg_for_constructor] on Multi-Field Tuple Variants (Returns Self)
//!
//! This test file verifies the `#[derive(Former)]` macro's handling of enums
//! where a multi-field tuple variant is marked with `#[standalone_constructors]`
//! (on the enum) and `#[arg_for_constructor]` on the fields.
//!
//! ## Purpose:
//!
//! - **Verify Standalone Direct Constructor Generation:** Ensure that `#[derive(Former)]` generates a standalone
//!   constructor function (e.g., `enum_name::variant_name(T1, T2, ...) -> Enum`) for multi-field
//!   tuple variants under `#[standalone_constructors]` when fields *are* marked with `#[arg_for_constructor]`.
//! - **Verify Argument Handling in Constructor:** Confirm that the generated constructor correctly
//!   accepts arguments via `impl Into<...>` for each field marked with `#[arg_for_constructor]`.
//! - It uses the shared test logic from `tuple_multi_standalone_args_only_test.rs`.

#[ allow( unused_imports ) ]
use ::former::prelude::*;
use ::former::Former; // Import derive macro

// === Enum Definition ===

/// Enum using derive for #[standalone_constructors] with #[arg_for_constructor] on multi-field tuple variants.
#[ derive( Debug, PartialEq, Clone, Former ) ]
#[ standalone_constructors ] // Enable standalone constructors
// #[ debug ] // Uncomment to see generated code later
pub enum TestEnumMultiStandaloneArgs // Consistent name
{
  /// A multi-field tuple variant with #[standalone_constructors] and #[arg_for_constructor].
  VariantMultiStandaloneArgs // Consistent name
  (
    #[ arg_for_constructor ] // Mark field as constructor arg
    i32,
    #[ arg_for_constructor ] // Mark field as constructor arg
    bool,
  ),
}

// === Include Test Logic ===
include!( "tuple_multi_standalone_args_only_test.rs" );