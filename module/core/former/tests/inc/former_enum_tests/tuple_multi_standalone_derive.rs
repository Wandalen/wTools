// File: module/core/former/tests/inc/former_enum_tests/tuple_multi_standalone_derive.rs

//! # Derive Test: #[standalone_constructors] on Multi-Field Tuple Variants (Returns Former)
//!
//! This test file verifies the `#[derive(Former)]` macro's handling of enums
//! where a multi-field tuple variant is marked with `#[standalone_constructors]`
//! (on the enum) but *without* `#[arg_for_constructor]` on the fields.
//!
//! ## Purpose:
//!
//! - **Verify Standalone Former Generation:** Ensure that `#[derive(Former)]` generates a standalone
//!   constructor function (e.g., `enum_name::variant_name() -> VariantFormer<...>`) for multi-field
//!   tuple variants under `#[standalone_constructors]` when fields are *not* marked with `#[arg_for_constructor]`.
//! - **Verify Setter Handling:** Confirm that the returned Former instance provides setters for each
//!   field in the tuple.
//! - It uses the shared test logic from `tuple_multi_standalone_only_test.rs`.

#[ allow( unused_imports ) ]
use ::former::prelude::*;
use ::former::Former; // Import derive macro

// === Enum Definition ===

/// Enum using derive for #[standalone_constructors] on multi-field tuple variants.
#[ derive( Debug, PartialEq, Clone, Former ) ]
#[ standalone_constructors ] // Enable standalone constructors
// #[ debug ] // Uncomment to see generated code later
pub enum TestEnumMultiStandalone // Consistent name
{
  /// A multi-field tuple variant.
  VariantMultiStandalone( i32, bool ), // Multi-field tuple variant (no #[arg_for_constructor])
}

// === Include Test Logic ===
include!( "tuple_multi_standalone_only_test.rs" );