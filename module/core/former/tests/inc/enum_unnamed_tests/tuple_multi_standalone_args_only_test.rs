#[allow(clippy::used_underscore_binding, clippy::all, warnings)]
// Purpose: Provides shared test assertions and logic for both the derived and manual implementations
// of standalone scalar constructors for multi-field tuple variants with `#[ arg_for_constructor ]`
// fields. It tests that standalone constructors generated/implemented when the enum has
// `#[ standalone_constructors ]` and all variant fields have `#[ arg_for_constructor ]` behave as
// expected (scalar style, taking field arguments).
//
// Coverage:
// - Rule 4a (#[ standalone_constructors ]): Tests the existence and functionality of the top-level constructor function (`variant`).
// - Rule 4b (Option 2 Logic): Tests that the standalone constructor takes arguments corresponding to the `#[ arg_for_constructor ]` fields and returns the final enum instance.
// - Rule 3f (Tuple + Multi-Field + Default): Implicitly tested via the `Variant` variant.
//
// Test Relevance/Acceptance Criteria:
// - Defines the `TestEnum` enum structure with a multi-field tuple variant `Variant(u32, String)`.
// - Contains a test function (`variant_test`) that is included by the derive and manual test files.
// - Calls the standalone constructor function `variant(value1, value2)` provided by the including file.
// - Asserts that the returned enum instance matches a manually constructed `TestEnum::Variant(value1, value2)`. This verifies that both derived and manual standalone constructors correctly handle field arguments and produce the final enum variant.

#[ cfg( test ) ]
mod tests
{
  use super::TestEnum;
  use super::variant;

  #[ test ]
  fn variant_test()
  {
    // Test Matrix Row: T19.1 (Implicitly, as this tests the behavior expected by the matrix)
    // Tests the standalone scalar constructor for Variant (multi field, #[ arg_for_constructor ] on all fields)
    let value1 = 123;
    let value2 = "abc".to_string();
    let got = variant( value1, value2.clone() ); // Call the standalone constructor

    let expected = TestEnum::Variant( value1, value2 );
    assert_eq!( got, expected );
  }
}