// File: module/core/former/tests/inc/former_enum_tests/unnamed_tests/standalone_constructor_tuple_only_test.rs

// Use the items defined in the including file (manual or derive)
use super::*;

/// Tests the standalone constructor for a tuple variant.
#[ test ]
fn tuple_variant_test() // Use enum-specific test name
{
  // Call the constructor function (manual or derived)
  let former = tuple_variant(); // <<< Call with zero args

  // Use the former to build the variant
  let instance = former
  ._0( 101 ) // Set the tuple field using the generated setter
  .form();

  // Define the expected enum instance (using the consistent enum name)
  let expected = TestEnum::TupleVariant( 101 ); // Use TestEnum

  // Assert that the formed instance matches the expected one
  assert_eq!( instance, expected );
}