// Purpose: Provides shared test assertions and logic for both the derived and manual implementations
// of the static scalar constructor for a multi-field tuple variant when it is explicitly marked
// with the `#[scalar]` attribute. It tests that the constructors generated/implemented for this
// scenario behave as expected (scalar style).
//
// Coverage:
// - Rule 1f (Tuple + Multi-Field + `#[scalar]`): Tests that the constructor for a multi-field tuple variant with the `#[scalar]` attribute is scalar, taking arguments for each field and returning the enum instance.
//
// Test Relevance/Acceptance Criteria:
// - Defines the `TestEnum` enum structure with a multi-field tuple variant `Variant(u32, String)`.
// - Contains a test function (`variant_test`) that is included by the derive and manual test files.
// - Calls the static method `variant(value1, value2)` provided by the including file.
// - Asserts that the returned enum instance matches a manually constructed `TestEnum::Variant(value1, value2)`. This verifies that both derived and manual implementations correctly provide a scalar constructor for multi-field tuple variants when `#[scalar]` is applied.

#[ cfg( test ) ]
mod tests
{
  use crate::inc::enum_unnamed_tests::tuple_multi_scalar_derive::TestEnum;

  #[ test ]
  fn variant_test()
  {
    // Test Matrix Row: T18.1 (Implicitly, as this tests the behavior expected by the matrix)
    // Tests the scalar constructor for Variant (multi field, #[scalar])
    let value1 = 123;
    let value2 = "abc".to_string();
    let got = TestEnum::variant( value1, value2.clone() ); // Call the static method

    let expected = TestEnum::Variant( value1, value2 );
    assert_eq!( got, expected );
  }
}