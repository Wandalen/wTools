//
// Contains the test logic for standalone constructors.
// This file is included by both the manual and derive test files.
//

// Use the items defined in the including file (either manual or derive)
use super::*;

/// Tests the standalone constructor for a struct with no arguments (manual version).
#[ test ]
fn no_args_manual()
{
  // Call the manually defined standalone constructor
  let former = manual_no_args_struct();

  // Use the former to build the struct
  let instance = former
  .field1( 42 ) // Set the field using the regular setter
  .form();

  // Define the expected struct instance
  let expected = ManualNoArgsStruct
  {
    field1 : 42,
  };

  // Assert that the formed instance matches the expected one
  assert_eq!( instance, expected );
}

/// Tests the standalone constructor for a struct with arguments (manual version).
#[ test ]
fn with_args_manual()
{
  // Call the manually defined standalone constructor with arguments
  let former = manual_with_args_struct( "hello", true );

  // Use the former to set the remaining optional field and build the struct
  let instance = former
  .field_c( 3.14 ) // Set the non-constructor field
  .form();

  // Define the expected struct instance
  let expected = ManualWithArgsStruct
  {
    field_a : "hello".to_string(),
    field_b : true,
    field_c : Some( 3.14 ),
  };

  // Assert that the formed instance matches the expected one
  assert_eq!( instance, expected );

  // Test case where the non-constructor field is not set
  let former2 = manual_with_args_struct( "world", false );
  let instance2 = former2.form(); // field_c remains None

  let expected2 = ManualWithArgsStruct
  {
    field_a : "world".to_string(),
    field_b : false,
    field_c : None,
  };
  assert_eq!( instance2, expected2 );
}