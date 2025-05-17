//! Purpose: Provides shared test assertions and logic for both the derived and manual implementations of standalone former builder functions for tuple variants without `#[arg_for_constructor]` fields. It tests that standalone constructors generated/implemented when the enum has `#[standalone_constructors]` and no variant fields have `#[arg_for_constructor]` behave as expected (former builder style).
//!
//! Coverage:
//! - Rule 4a (#[standalone_constructors]): Tests the existence and functionality of top-level constructor functions (`variant1`, `variant2`).
//! - Rule 4b (Option 2 Logic): Tests that these standalone constructors return former builders for the variants.
//! - Rule 3d (Tuple + Single-Field + Default): Implicitly tested via `Variant1`.
//! - Rule 3f (Tuple + Multi-Field + Default): Implicitly tested via `Variant2`.
//!
//! Test Relevance/Acceptance Criteria:
//! - Defines the `TestEnum` enum structure with `Variant1(u32)` and `Variant2(u32, String)`.
//! - Contains test functions (`variant1_test`, `variant2_test`) that are included by the derive and manual test files.
//! - Calls the standalone constructor functions (`variant1()`, `variant2()`).
//! - Uses the returned former builders' setters (`._0()`, `._1()`) and calls `.form()`.
//! - Asserts that the resulting enum instances match manually constructed expected values (`TestEnum::Variant1(value)`, `TestEnum::Variant2(value1, value2)`). This verifies that both derived and manual standalone constructors correctly return former builders and allow setting fields via setters.

#[ cfg( test ) ]
mod tests
{
  // use super::TestEnum; // Assuming TestEnum is available from the including file

  #[ test ]
  fn variant1_test()
  {
    // Test Matrix Row: T16.1 (Implicitly, as this tests the behavior expected by the matrix)
    // Tests the standalone constructor for Variant1 (single field, no #[arg_for_constructor])
    let value = 123;
    let got = variant1() // Call the standalone constructor
      ._0( value ) // Use the setter for the field
      .form(); // Form the final enum instance

    let expected = TestEnum::Variant1( value );
    assert_eq!( got, expected );
  }

  #[ test ]
  fn variant2_test()
  {
    // Test Matrix Row: T16.2 (Implicitly, as this tests the behavior expected by the matrix)
    // Tests the standalone constructor for Variant2 (multi field, no #[arg_for_constructor])
    let value1 = 456;
    let value2 = "abc".to_string();
    let got = variant2() // Call the standalone constructor
      ._0( value1 ) // Use the setter for the first field
      ._1( value2.clone() ) // Use the setter for the second field
      .form(); // Form the final enum instance

    let expected = TestEnum::Variant2( value1, value2 );
    assert_eq!( got, expected );
  }
}