// module/core/former/tests/inc/former_enum_tests/standalone_constructor_args_only_test.rs
//
// Contains the shared test logic for *argument-taking* standalone enum constructors.
// This file is included by both the manual and derive test files for the args case.
//

// Use the items defined in the including file (manual or derive for args)
use super::*;

/// Tests the standalone constructor for a unit variant (still takes no args).
#[ test ]
fn unit_variant_args_test() // New test name
{
  // Assumes `unit_variant_args` is defined in the including scope
  let instance = unit_variant_args();
  let expected = TestEnumArgs::UnitVariantArgs; // Use new enum name
  assert_eq!( instance, expected );
}

/// Tests the standalone constructor for a tuple variant that takes arguments.
#[ test ]
fn tuple_variant_args_test() // New test name
{
  // Assumes `tuple_variant_args` takes an i32 argument
  let former = tuple_variant_args( 202 ); // Call constructor with arg

  // Form directly (no other fields to set in this simple case)
  let instance = former.form();

  // Define the expected enum instance
  let expected = TestEnumArgs::TupleVariantArgs( 202 ); // Use new enum name

  assert_eq!( instance, expected );

  // Test setting another value via setter (optional, but good check)
  let former2 = tuple_variant_args( 0 ); // Init with 0
  let instance2 = former2._0( 303 ).form(); // Override with setter
  let expected2 = TestEnumArgs::TupleVariantArgs( 303 );
  assert_eq!( instance2, expected2 );

}

/// Tests the standalone constructor for a struct variant that takes arguments.
#[ test ]
fn struct_variant_args_test() // New test name
{
  // Assumes `struct_variant_args` takes a String argument
  let former = struct_variant_args( "arg_value" ); // Call constructor with arg

  // Form directly (no other fields to set in this simple case)
  let instance = former.form();

  // Define the expected enum instance
  let expected = TestEnumArgs::StructVariantArgs { field : "arg_value".to_string() }; // Use new enum name

  assert_eq!( instance, expected );

  // Test setting another value via setter (optional, but good check)
  let former2 = struct_variant_args( "" ); // Init with ""
  let instance2 = former2.field( "override".to_string() ).form(); // Override with setter
  let expected2 = TestEnumArgs::StructVariantArgs { field : "override".to_string() };
  assert_eq!( instance2, expected2 );
}