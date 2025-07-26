// Purpose: Provides shared test assertions and logic for both the derived and manual implementations
// of standalone former builders for multi-field tuple variants without `#[arg_for_constructor]`
// fields. It tests that standalone constructors generated/implemented when the enum has
// `#[standalone_constructors]` and no variant fields have `#[arg_for_constructor]` behave as
// expected (former builder style, allowing field setting via setters).
//
// Coverage:
// - Rule 4a (#[standalone_constructors]): Tests the existence and functionality of the top-level constructor function (`variant`).
// - Rule 4b (Option 2 Logic): Tests that the standalone constructor returns a former builder for the variant and that its fields can be set using setters (`._0()`, `._1()`).
// - Rule 3f (Tuple + Multi-Field + Default): Implicitly tested via the `Variant` variant.
//
// Test Relevance/Acceptance Criteria:
// - Defines the `TestEnum` enum structure with a multi-field tuple variant `Variant(u32, String)`.
// - Contains a test function (`variant_test`) that is included by the derive and manual test files.
// - Calls the standalone constructor function `variant()` provided by the including file.
// - Uses the returned former builder's setters (`._0()`, `._1()`) to set the fields.
// - Calls `.form()` on the former builder to get the final enum instance.
// - Asserts that the resulting enum instance matches a manually constructed `TestEnum::Variant(value1, value2)`. This verifies that both derived and manual standalone constructors correctly return former builders and allow setting fields via setters.

#[ cfg( test ) ]
mod tests
{
  use super::TestEnum;
  use super::variant;

  #[ test ]
  fn variant_test()
  {
    // Test Matrix Row: T20.1 (Implicitly, as this tests the behavior expected by the matrix)
    // Tests the standalone former builder for Variant (multi field, no #[arg_for_constructor])
    let value1 = 123;
    let value2 = "abc".to_string();
    let got = variant() // Call the standalone constructor
      ._0( value1 ) // Use the setter for the first field
      ._1( value2.clone() ) // Use the setter for the second field
      .form(); // Form the final enum instance

    let expected = TestEnum::Variant( value1, value2 );
    assert_eq!( got, expected );
  }
}