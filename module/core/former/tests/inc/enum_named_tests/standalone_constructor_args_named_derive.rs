//! Purpose: Tests the `#[derive(Former)]` macro's generation of standalone scalar constructor functions
//! for named (struct-like) variants when the enum has the `#[standalone_constructors]` attribute and
//! fields within the variants have the `#[arg_for_constructor]` attribute. This file focuses on
//! verifying the derive-based implementation for both single-field and multi-field named variants.
//!
//! Coverage:
//! - Rule 4a (#[standalone_constructors]): Verifies the generation of top-level constructor functions (`struct_variant_args`, `multi_struct_args`).
//! - Rule 4b (Option 2 Logic): Verifies that when all fields in a named variant have `#[arg_for_constructor]`, the standalone constructor takes arguments for those fields and returns the final enum instance (scalar style).
//! - Rule 1e (Struct + Single-Field + `#[scalar]`): Implicitly relevant as `StructVariantArgs` is a single-field named variant.
//! - Rule 3e (Struct + Single-Field + Default): Implicitly relevant as `StructVariantArgs` is a single-field named variant.
//! - Rule 1g (Struct + Multi-Field + `#[scalar]`): Implicitly relevant as `MultiStructArgs` is a multi-field named variant.
//! - Rule 3g (Struct + Multi-Field + Default): Implicitly relevant as `MultiStructArgs` is a multi-field named variant.
//!
//! Test Relevance/Acceptance Criteria:
//! - Defines an enum `TestEnumArgs` with single-field (`StructVariantArgs { field: String }`) and multi-field (`MultiStructArgs { a: i32, b: bool }`) named variants.
//! - Applies `#[derive(Former)]`, `#[standalone_constructors]`, and `#[debug]` to the enum.
//! - Applies `#[arg_for_constructor]` to the fields within both variants.
//! - Includes shared test logic from `standalone_constructor_args_named_only_test.rs`.
//! - The included tests call the derived standalone constructor functions (`struct_variant_args(value)`, `multi_struct_args(value1, value2)`) and assert that the returned enum instances match manually constructed expected values. This verifies that the standalone constructors are generated correctly as scalar functions when all fields have `#[arg_for_constructor]`.

// File: module/core/former/tests/inc/former_enum_tests/named_tests/standalone_constructor_args_named_derive.rs

#[ allow( unused_imports ) ]
use ::former::prelude::*;
use ::former::Former; // Import derive macro

// === Enum Definition ===

/// Enum using derive for standalone constructors with arguments.
#[ derive( Debug, PartialEq, Clone, Former, debug ) ] // Added debug attribute
#[ standalone_constructors ] // Enable standalone constructors
pub enum TestEnumArgs // Use the distinct name
{
  /// A struct variant with one field marked as constructor arg.
  StructVariantArgs // Use the distinct name
  {
    #[ arg_for_constructor ] // Mark field as constructor arg
    field : String,
  },
  /// A struct variant with multiple fields marked as constructor args.
  // #[ scalar ] // <<< Keep scalar attribute
  MultiStructArgs // Use the distinct name
  {
    #[ arg_for_constructor ]
    a : i32,
    #[ arg_for_constructor ]
    b : bool,
  },
}

// === Include Test Logic ===
include!( "standalone_constructor_args_named_only_test.rs" ); // Include the specific test file