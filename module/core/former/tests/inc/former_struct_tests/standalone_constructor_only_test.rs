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

// Tests for the struct with arguments will be added here later.
// fn with_args_manual() { ... }