//
// Contains the test logic for standalone enum constructors.
// This file is included by both the manual and derive test files.
//

// Use the items defined in the including file (either manual or derive)
use super::*;

/// Tests the standalone constructor for a unit variant (manual version).
#[ test ]
fn unit_variant_manual()
{
  // Call the manually defined standalone constructor
  let instance = manual_unit_variant();

  // Define the expected enum instance
  let expected = ManualEnum::UnitVariant;

  // Assert that the formed instance matches the expected one
  assert_eq!( instance, expected );
}

/// Tests the standalone constructor for a tuple variant (manual version).
#[ test ]
fn tuple_variant_manual()
{
  // Call the manually defined standalone constructor (returns former)
  let former = manual_tuple_variant();

  // Use the former to build the variant
  let instance = former
  ._0( 101 ) // Set the tuple field using the generated setter
  .form();

  // Define the expected enum instance
  let expected = ManualEnum::TupleVariant( 101 );

  // Assert that the formed instance matches the expected one
  assert_eq!( instance, expected );
}

/// Tests the standalone constructor for a struct variant (manual version).
#[ test ]
fn struct_variant_manual()
{
  // Call the manually defined standalone constructor (returns former)
  let former = manual_struct_variant();

  // Use the former to build the variant
  let instance = former
  .field( "value".to_string() ) // Set the struct field using the generated setter
  .form();

  // Define the expected enum instance
  let expected = ManualEnum::StructVariant { field : "value".to_string() };

  // Assert that the formed instance matches the expected one
  assert_eq!( instance, expected );
}