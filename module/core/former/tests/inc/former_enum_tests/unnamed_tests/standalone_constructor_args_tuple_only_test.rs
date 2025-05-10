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
  // Based on derive file, `MultiTupleArgs` has no #[arg_for_constructor] fields.
  // Option 2 dictates constructor takes 0 args and returns Former.
  let former = multi_tuple_args(); // Call with no args
  let instance = former.form(); // Form the instance
  // The default values will be used since no args were provided to the former
  let expected = TestEnumArgs::MultiTupleArgs( i32::default(), bool::default() );
  assert_eq!( instance, expected );
}
