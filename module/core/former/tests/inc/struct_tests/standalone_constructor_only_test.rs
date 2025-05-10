//
// Contains the shared test logic for standalone constructors.
// This file is included by both the manual and derive test files.
// It uses consistent names defined in the including files.
//

// Use the items defined in the including file (manual or derive)
use super::*;

/// Tests the standalone constructor for a struct with no arguments.
#[ test ]
fn no_args_test() // Generic test name
{
  // Call the constructor function (manual or derived)
  // Assumes `test_struct_no_args` is defined in the including scope
  let former = test_struct_no_args();

  // Use the former to build the struct
  let instance = former
  .field1( 42 ) // Set the field using the regular setter
  .form();

  // Define the expected struct instance (using the consistent struct name)
  let expected = TestStructNoArgs
  {
    field1 : 42,
  };

  // Assert that the formed instance matches the expected one
  assert_eq!( instance, expected );
}

// qqq : Uncomment tests below once arg_for_constructor is implemented for structs // Removed comment block start
/// Tests the standalone constructor for a struct with arguments.
#[ test ]
fn with_args_test() // Generic test name
{
  // Call the constructor function (manual or derived) with arguments
  // Assumes `test_struct_with_args` is defined in the including scope
  let former = test_struct_with_args( "hello", true ); // Use literal args

  // Use the former to set the remaining optional field and build the struct
  let instance = former
  .field_c( std::f32::consts::PI ) // Set the non-constructor field
  .form();

  // Define the expected struct instance (using the consistent struct name)
  let expected = TestStructWithArgs
  {
    field_a : "hello".to_string(),
    field_b : true,
    field_c : Some( std::f32::consts::PI ),
  };

  // Assert that the formed instance matches the expected one
  assert_eq!( instance, expected );

  // Test case where the non-constructor field is not set
  let former2 = test_struct_with_args( "world", false );
  let instance2 = former2.form(); // field_c remains None

  let expected2 = TestStructWithArgs
  {
    field_a : "world".to_string(),
    field_b : false,
    field_c : None,
  };
  assert_eq!( instance2, expected2 );
}