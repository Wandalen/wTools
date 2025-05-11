//! Purpose: Provides shared test assertions and logic for both the derived and manual implementations
//! of static constructors and standalone constructors for zero-field tuple variants. It tests that
//! constructors generated/implemented for default and `#[scalar]` zero-field tuple variants behave
//! as expected (scalar style, returning the enum instance directly).
//!
//! Coverage:
//! - Rule 3b (Tuple + Zero-Field + Default): Tests the static method `variant_zero_default()` and the standalone constructor `standalone_variant_zero_default()`.
//! - Rule 1b (Tuple + Zero-Field + `#[scalar]`): Tests the static method `variant_zero_scalar()` and the standalone constructor `standalone_variant_zero_scalar()`.
//! - Rule 4a (#[standalone_constructors]): Tests the existence and functionality of the top-level constructor functions (`standalone_variant_zero_default`, `standalone_variant_zero_scalar`).
//!
//! Test Relevance/Acceptance Criteria:
//! - Defines the `EnumWithZeroFieldTuple` enum structure with zero-field tuple variants `VariantZeroDefault` and `VariantZeroScalar`.
//! - Contains test functions (`test_zero_field_default`, `test_zero_field_scalar`, `test_zero_field_default_standalone`, `test_zero_field_scalar_standalone`) that are included by the derive and manual test files.
//! - Calls the static methods (`variant_zero_default`, `variant_zero_scalar`) and standalone constructors (`standalone_variant_zero_default`, `standalone_variant_zero_scalar`) provided by the including file.
//! - Asserts that the returned enum instances match the direct enum variants (`EnumWithZeroFieldTuple::VariantZeroDefault`, `EnumWithZeroFieldTuple::VariantZeroScalar`). This verifies that both derived and manual implementations correctly provide scalar constructors for zero-field tuple variants, including standalone constructors.

// Test Matrix Row: T0.1 (Default, None)
#[ test ]
fn test_zero_field_default()
{
  use super::*;
  let got = EnumWithZeroFieldTuple::variant_zero_default();
  let expected = EnumWithZeroFieldTuple::VariantZeroDefault;
  assert_eq!( got, expected );
}

// Test Matrix Row: T0.2 (#[scalar], None)
#[ test ]
fn test_zero_field_scalar()
{
  use super::*;
  let got = EnumWithZeroFieldTuple::variant_zero_scalar();
  let expected = EnumWithZeroFieldTuple::VariantZeroScalar;
  assert_eq!( got, expected );
}

// Test Matrix Row: T0.3 (Default, #[standalone_constructors])
#[ test ]
fn test_zero_field_default_standalone()
{
  use super::*;
  let got = standalone_variant_zero_default();
  let expected = EnumWithZeroFieldTuple::VariantZeroDefault;
  assert_eq!( got, expected );
}

// Test Matrix Row: T0.4 (#[scalar], #[standalone_constructors])
#[ test ]
fn test_zero_field_scalar_standalone()
{
  use super::*;
  let got = standalone_variant_zero_scalar();
  let expected = EnumWithZeroFieldTuple::VariantZeroScalar;
  assert_eq!( got, expected );
}