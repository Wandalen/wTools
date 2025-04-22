// module/core/former/tests/inc/former_enum_tests/standalone_constructor_only_test.rs
//
// Contains the shared test logic for standalone enum constructors.
// This file is included by both the manual and derive test files.
//

// Use the items defined in the including file (manual or derive)
use super::*;

/// Tests the standalone constructor for a unit variant.
#[ test ]
fn unit_variant_test() // Use enum-specific test name
{
  // Call the constructor function (manual or derived)
  // Assumes `unit_variant` is defined in the including scope
  let instance = unit_variant();

  // Define the expected enum instance (using the consistent enum name)
  let expected = TestEnum::UnitVariant; // Use TestEnum

  // Assert that the formed instance matches the expected one
  assert_eq!( instance, expected );
}

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

/// Tests the standalone constructor for a struct variant.
#[ test ]
fn struct_variant_test() // Use enum-specific test name
{
  // Call the constructor function (manual or derived)
  let former = struct_variant(); // <<< Call with zero args

  // Use the former to build the variant
  let instance = former
  .field( "value".to_string() ) // Set the struct field using the generated setter
  .form();

  // Define the expected enum instance (using the consistent enum name)
  let expected = TestEnum::StructVariant { field : "value".to_string() }; // Use TestEnum

  // Assert that the formed instance matches the expected one
  assert_eq!( instance, expected );
}
