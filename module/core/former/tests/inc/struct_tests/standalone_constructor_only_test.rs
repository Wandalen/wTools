#[allow(clippy::used_underscore_binding, clippy::all, warnings)]
//
// Contains the shared test logic for standalone constructors.
// This file is included by both the manual and derive test files.
// It uses consistent names defined in the including files.
//

// Use the items defined in the including file (manual or derive)
use super::*;

/// Tests the standalone constructor for a struct with no ignored fields.
/// According to new specification: no #[`former_ignore`] fields means constructor returns Self directly.
#[ test ]
fn no_args_test() // Generic test name
{
  // Call the constructor function - it now takes all fields as arguments and returns Self
  let instance = test_struct_no_args(42);

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
  .c( core::f32::consts::PI ) // Set the non-constructor field
  .form();

  // Define the expected struct instance (using the consistent struct name)
  let expected = TestStructWithArgs
  {
    a : "hello".to_string(),
    b : true,
    c : Some( core::f32::consts::PI ),
  };

  // Assert that the formed instance matches the expected one
  assert_eq!( instance, expected );

  // Test case where the non-constructor field is not set
  let former2 = test_struct_with_args( "world", false );
  let instance2 = former2.form(); // field_c remains None

  let expected2 = TestStructWithArgs
  {
    a : "world".to_string(),
    b : false,
    c : None,
  };
  assert_eq!( instance2, expected2 );
}