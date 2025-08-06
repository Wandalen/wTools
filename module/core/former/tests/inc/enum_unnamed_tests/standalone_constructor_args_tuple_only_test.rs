// File: module/core/former/tests/inc/former_enum_tests/unnamed_tests/standalone_constructor_args_tuple_only_test.rs

// Use the items defined in the including file (manual or derive for args)
use super::*;

/// Tests the standalone constructor for a tuple variant that takes arguments.
#[ test ]
fn tuple_variant_args_test() // New test name
{
  // Assumes `tuple_variant_args` takes an i32 argument and returns Self (Option 2)
  let instance = tuple_variant_args( 202 ); // Call directly
  let expected = TestEnumArgs::TupleVariantArgs( 202 );
  assert_eq!( instance, expected );
}

/// Tests the standalone constructor for a multi-field tuple variant that takes arguments.
#[ test ]
fn multi_tuple_variant_args_test()
{
  // Based on derive file, `MultiTupleArgs` has #[scalar] attribute and no #[arg_for_constructor] fields.
  // Scalar behavior: constructor takes ALL fields as arguments and returns enum directly
  let instance = multi_tuple_args( 42, true ); // Call with all field arguments
  let expected = TestEnumArgs::MultiTupleArgs( 42, true );
  assert_eq!( instance, expected );
}
