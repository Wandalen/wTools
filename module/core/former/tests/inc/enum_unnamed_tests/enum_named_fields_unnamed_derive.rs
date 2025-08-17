//! Purpose: Tests the `#[ derive( Former ) ]` macro's generation of constructors for zero-field
//! unnamed (tuple) variants, including with `#[ scalar ]` and `#[ standalone_constructors ]`.
//! This file focuses on verifying the derive-based implementation.
//!
//! Coverage:
//! - Rule 3b (Tuple + Zero-Field + Default): Tests static method `EnumWithNamedFields::variant_zero_unnamed_default()`.
//! - Rule 1b (Tuple + Zero-Field + `#[ scalar ]`): Tests static method `EnumWithNamedFields::variant_zero_unnamed_scalar()`.
//! - Rule 4a (#[`standalone_constructors`]): Verifies generation of top-level constructor functions (though not explicitly tested in `_only_test.rs`).
//!
//! Test Relevance/Acceptance Criteria:
//! - Defines an enum `EnumWithNamedFields` with two zero-field unnamed variants: `VariantZeroUnnamedDefault()` and `VariantZeroUnnamedScalar()`.
//! - `VariantZeroUnnamedScalar` is annotated with `#[ scalar ]`. The enum has `#[ derive( Former ) ]`, `#[ debug ]`, and `#[ standalone_constructors ]`.
//! - Relies on the derived static methods (`EnumWithNamedFields::variant_zero_unnamed_scalar()`, `EnumWithNamedFields::variant_zero_unnamed_default()`)
//!   defined in `enum_named_fields_unnamed_only_test.rs`.
//! - Asserts that these constructors produce the correct `EnumWithNamedFields` enum instances by comparing
//!   with manually constructed variants.

use super::*;

// Define the enum with zero-field unnamed (tuple) variants for testing.
#[ derive( Debug, PartialEq, former::Former ) ]
// #[ debug ]
#[ standalone_constructors ]
pub enum EnumWithNamedFields
{
  // --- Zero Fields (Unnamed - Tuple-like) ---
  VariantZeroUnnamedDefault(), // Expect: variant_zero_unnamed_default() -> Enum (Default is scalar for 0 fields)
  #[ scalar ] // Expect: variant_zero_unnamed_scalar() -> Enum
  VariantZeroUnnamedScalar(),
}

// Include the test logic file
include!( "enum_named_fields_unnamed_only_test.rs" );