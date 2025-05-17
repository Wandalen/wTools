// File: module/core/former/tests/inc/former_enum_tests/named_tests/standalone_constructor_args_named_only_test.rs

// Use the items defined in the including file (manual or derive for args)
use super::*;

/// Tests the standalone constructor for a struct variant that takes arguments.
#[ test ]
fn struct_variant_args_test() // New test name
{
  // Assumes `struct_variant_args` takes a String argument and returns Self (Option 2)
  let instance = struct_variant_args( "arg_value" ); // Call directly
  let expected = TestEnumArgs::StructVariantArgs { field : "arg_value".to_string() };
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