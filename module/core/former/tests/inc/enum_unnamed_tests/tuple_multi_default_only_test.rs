#[allow(clippy::used_underscore_binding, clippy::all, warnings)]
// Purpose: Provides shared test assertions and logic for both the derived and manual implementations
// of the implicit variant former for a multi-field tuple variant when no specific variant
// attribute is applied (default behavior). It tests that the constructors generated/implemented
// for this scenario behave as expected (implicit variant former style).
//
// Coverage:
// - Rule 3f (Tuple + Multi-Field + Default): Tests that the constructor for a multi-field tuple variant without specific attributes returns an implicit variant former with setters like ._0() and ._1().
//
// Test Relevance/Acceptance Criteria:
// - Defines the `TestEnum` enum structure with a multi-field tuple variant `Variant(u32, String)`.
// - Contains a test function (`variant_test`) that is included by the derive and manual test files.
// - Calls the static method `variant()` that returns a former, then uses setters ._0() and ._1() and calls .form().
// - Asserts that the returned enum instance matches a manually constructed `TestEnum::Variant(value1, value2)`. This verifies that both derived and manual implementations correctly provide an implicit variant former for multi-field tuple variants by default.

#[ cfg( test ) ]
mod tests
{
  use crate::inc::enum_unnamed_tests::tuple_multi_default_derive::TestEnum;

  #[ test ]
  fn variant_test()
  {
    // Test Matrix Row: T17.1 (Implicitly, as this tests the behavior expected by the matrix)
    // Tests the implicit variant former for Variant (multi field, default behavior)
    let value1 = 123;
    let value2 = "abc".to_string();
    let got = TestEnum::variant()
      ._0( value1 )
      ._1( value2.clone() )
      .form(); // Call the implicit variant former

    let expected = TestEnum::Variant( value1, value2 );
    assert_eq!( got, expected );
  }
}