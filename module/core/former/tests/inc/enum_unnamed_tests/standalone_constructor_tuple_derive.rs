//! Purpose: Tests the `#[derive(Former)]` macro's generation of standalone former builder functions for tuple variants when the enum has the `#[standalone_constructors]` attribute and no fields within the variants have the `#[arg_for_constructor]` attribute. This file focuses on verifying the derive-based implementation.
//!
//! Coverage:
//! - Rule 4a (#[`standalone_constructors`]): Verifies the generation of top-level constructor functions (`variant1`, `variant2`).
//! - Rule 4b (Option 2 Logic): Verifies that when no fields in a tuple variant have `#[arg_for_constructor]`, the standalone constructor returns a former builder for the variant.
//! - Rule 3d (Tuple + Single-Field + Default): Implicitly relevant as `Variant1` is a single-field tuple variant.
//! - Rule 3f (Tuple + Multi-Field + Default): Implicitly relevant as `Variant2` is a multi-field tuple variant.
//!
//! Test Relevance/Acceptance Criteria:
//! - Defines an enum `TestEnum` with single-field (`Variant1(u32)`) and multi-field (`Variant2(u32, String)`) tuple variants.
//! - Applies `#[derive(Former)]` and `#[standalone_constructors]` to the enum.
//! - No `#[arg_for_constructor]` attributes are applied to fields.
//! - Includes shared test logic from `standalone_constructor_tuple_only_test.rs`.
//! - The included tests call the standalone constructor functions (`variant1()`, `variant2()`), use the returned former builders' setters (`._0()`, `._1()`), and call `.form()`.
//! - Asserts that the resulting enum instances match manually constructed expected values. This verifies that the standalone constructors are generated correctly and return former builders when no field arguments are specified.

use former::Former;

#[ derive( Former, Debug, PartialEq ) ]
#[ former( standalone_constructors ) ]
pub enum TestEnum
{
  Variant1( u32 ),
  Variant2( u32, String ),
}

// Temporarily inline the test to debug scope issues
#[test]
fn variant1_test()
{
  // Test the standalone constructor for Variant1 (single field, no #[arg_for_constructor])
  let value = 123;
  let got = variant_1() // Call the standalone constructor
    ._0( value ) // Use the setter for the field
    .form(); // Form the final enum instance

  let expected = TestEnum::Variant1( value );
  assert_eq!( got, expected );
}

#[test] 
fn variant2_test()
{
  // Test the standalone constructor for Variant2 (multi field, no #[arg_for_constructor])
  let value1 = 456;
  let value2 = "abc".to_string(); 
  let got = variant_2() // Call the standalone constructor
    ._0( value1 ) // Use the setter for the first field
    ._1( value2.clone() ) // Use the setter for the second field
    .form(); // Form the final enum instance

  let expected = TestEnum::Variant2( value1, value2 );
  assert_eq!( got, expected );
}