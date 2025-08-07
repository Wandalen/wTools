//! Purpose: Tests the `#[derive(Former)]` macro's generation of a standalone former builder
//! for a named (struct-like) variant when the enum has the `#[standalone_constructors]` attribute
//! and no fields within the variants have the `#[arg_for_constructor]` attribute. This file focuses
//! on verifying the derive-based implementation for a single-field named variant.
//!
//! Coverage:
//! - Rule 4a (#[`standalone_constructors`]): Verifies the generation of the top-level constructor function (`struct_variant`).
//! - Rule 4b (Option 2 Logic): Verifies that when no fields in a named variant have `#[arg_for_constructor]`, the standalone constructor returns a former builder for the variant.
//! - Rule 1e (Struct + Single-Field + `#[scalar]`): Implicitly relevant as `StructVariant` is a single-field named variant.
//! - Rule 3e (Struct + Single-Field + Default): Implicitly relevant as `StructVariant` is a single-field named variant.
//!
//! Test Relevance/Acceptance Criteria:
//! - Defines an enum `TestEnum` with a single-field named variant `StructVariant { field: String }`.
//! - Applies `#[derive(Former)]` and `#[standalone_constructors]` to the enum.
//! - No `#[arg_for_constructor]` attributes are applied to fields.
//! - Includes shared test logic from `standalone_constructor_named_only_test.rs`.
//! - The included test calls the derived standalone constructor function `struct_variant()`, uses the returned former builder's setter (`.field()`), and calls `.form()`.
//! - Asserts that the resulting enum instance matches a manually constructed `TestEnum::StructVariant { field: value }`. This verifies that the standalone constructor is generated correctly as a former builder when no field arguments are specified.

// File: module/core/former/tests/inc/former_enum_tests/named_tests/standalone_constructor_named_derive.rs

#[ allow( unused_imports ) ]
use ::former::prelude::*;
use ::former::Former; // Import derive macro

// === Enum Definition ===

/// Enum using derive for standalone constructors.
#[ derive( Debug, PartialEq, Clone, Former ) ]
#[ standalone_constructors ] // New attribute is active
pub enum TestEnum // Consistent name
{
  /// A struct variant with one field.
  StructVariant // Defaults to subformer behavior
  {
    // #[ arg_for_constructor ] // <<< Keep commented out for this increment
    field : String,
  },
}

// === Include Test Logic ===
include!( "standalone_constructor_named_only_test.rs" ); // Use the consistent name