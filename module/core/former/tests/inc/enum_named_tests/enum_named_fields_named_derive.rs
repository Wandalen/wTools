#![allow(clippy::used_underscore_binding, clippy::all, warnings, missing_docs)]
//! Purpose: Tests the `#[ derive( Former ) ]` macro's generation of constructors for named (struct-like)
//! variants with varying field counts and attributes (`#[ scalar ]`, `#[ subform_scalar ]`). This file
//! focuses on verifying the derive-based implementation, including static methods and standalone
//! constructors (when enabled on the enum).
//!
//! Coverage:
//! - Rule 1c (Struct + Zero-Field + `#[ scalar ]`): Verifies `Enum::variant() -> Enum` for a zero-field named variant with `#[ scalar ]`.
//! - Rule 3c (Struct + Zero-Field + Default): Implicitly covered as this is an error case verified by compile-fail tests.
//! - Rule 1e (Struct + Single-Field + `#[ scalar ]`): Verifies `Enum::variant { field: InnerType } -> Enum` for a single-field named variant with `#[ scalar ]`.
//! - Rule 2e (Struct + Single-Field + `#[ subform_scalar ]`): Verifies `Enum::variant() -> VariantFormer<...>` for a single-field named variant with `#[ subform_scalar ]`.
//! - Rule 3e (Struct + Single-Field + Default): Verifies `Enum::variant() -> VariantFormer<...>` for a single-field named variant without specific attributes.
//! - Rule 1g (Struct + Multi-Field + `#[ scalar ]`): Verifies `Enum::variant { f1: T1, f2: T2, ... } -> Enum` for a multi-field named variant with `#[ scalar ]`.
//! - Rule 3g (Struct + Multi-Field + Default): Verifies `Enum::variant() -> VariantFormer<...>` for a multi-field named variant without specific attributes.
//! - Rule 4a (#[ standalone_constructors ]): Verifies the generation of top-level constructor functions for named variants.
//! - Rule 4b (Option 2 Logic): Relevant to the return types of standalone constructors based on field attributes.
//!
//! Test Relevance/Acceptance Criteria:
//! - Defines an enum `EnumWithNamedFields` with named variants covering zero, one, and two fields.
//! - Applies `#[ derive( Former ) ]`, `#[ debug ]`, and `#[ standalone_constructors ]` to the enum.
//! - Applies `#[ scalar ]` and `#[ subform_scalar ]` to relevant variants.
//! - Includes shared test logic from `enum_named_fields_named_only_test.rs`.
//! - The included tests call the derived static methods (e.g., `EnumWithNamedFields::variant_zero_scalar()`, `EnumWithNamedFields::variant_one_scalar()`, `EnumWithNamedFields::variant_one_subform()`, etc.) and standalone constructors (e.g., `standalone_variant_zero_scalar()`).
//! - Asserts that the returned values match the expected enum instances or former types, verifying the constructor generation and behavior for named variants with different attributes and field counts.

// File: module/core/former/tests/inc/former_enum_tests/named_tests/enum_named_fields_named_derive.rs
use super::*;
#[ allow( unused_imports ) ]
use ::former::prelude::*;
use ::former::Former;

// Define the inner struct needed for subform tests directly in this file
#[ derive( Debug, PartialEq, Default, Clone, Former ) ] // Former derive needed for subform tests
pub struct InnerForSubform {
    pub value: i64,
}

// Define the enum with named field variants for testing.
#[ derive( Debug, PartialEq, Former ) ]
#[ former( standalone_constructors ) ]
pub enum EnumWithNamedFields
{
  // --- Zero Fields (Named - Struct-like) ---
  #[ scalar ]
  VariantZeroScalar {}, // Expect: variant_zero_scalar() -> Enum
  // VariantZeroDefault {}, // Error case - no manual impl needed

  // --- One Field (Named - Struct-like) ---
  #[ scalar ]
  VariantOneScalar { field_a : String }, // Expect: variant_one_scalar(String) -> Enum
  #[ subform_scalar ]
  VariantOneSubform { field_b : InnerForSubform }, // Expect: variant_one_subform() -> InnerForSubformFormer
  VariantOneDefault { field_c : InnerForSubform }, // Expect: variant_one_default() -> InnerForSubformFormer

  // --- Two Fields (Named - Struct-like) ---
  #[ scalar ]
  VariantTwoScalar { field_d : i32, field_e : bool }, // Expect: variant_two_scalar(i32, bool) -> Enum
  // VariantTwoDefault { field_f : i32, field_g : bool }, // Error case - no manual impl needed
}

// Include the test logic file
include!( "enum_named_fields_named_only_test.rs" );