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
  let instance = unit_variant_args(); // Returns Enum directly
  let expected = TestEnumArgs::UnitVariantArgs;
  assert_eq!( instance, expected );
}

/// Tests the standalone constructor for a tuple variant that takes arguments.
#[ test ]
fn tuple_variant_args_test() // New test name
{
  // Assumes `tuple_variant_args` takes an i32 argument and returns Self (Option 2)
  let instance = tuple_variant_args( 202 ); // Call directly
  let expected = TestEnumArgs::TupleVariantArgs( 202 );
  assert_eq!( instance, expected );
}

/// Tests the standalone constructor for a struct variant that takes arguments.
#[ test ]
fn struct_variant_args_test() // New test name
{
  // Assumes `struct_variant_args` takes a String argument and returns Self (Option 2)
  let instance = struct_variant_args( "arg_value" ); // Call directly
  let expected = TestEnumArgs::StructVariantArgs { field : "arg_value".to_string() };
  assert_eq!( instance, expected );
}

/// Tests the standalone constructor for a multi-field tuple variant that takes arguments.
#[ test ]
fn multi_tuple_variant_args_test()
{
  // Based on derive file, `MultiTupleArgs` has no #[arg_for_constructor] fields.
  // Option 2 dictates constructor takes 0 args and returns Former.
  let former = multi_tuple_args(); // Call with no args
  let instance = former.form(); // Form the instance
  // The default values will be used since no args were provided to the former
  let expected = TestEnumArgs::MultiTupleArgs( i32::default(), bool::default() );
  assert_eq!( instance, expected );
}

/// Tests the standalone constructor for a multi-field struct variant that takes arguments.
#[ test ]
fn multi_struct_variant_args_test()
{
  // Assumes `multi_struct_args` takes i32 and bool arguments and returns Self (Option 2)
  let instance = multi_struct_args( -1, false ); // Call directly
  let expected = TestEnumArgs::MultiStructArgs { a : -1, b : false };
  assert_eq!( instance, expected );
}
