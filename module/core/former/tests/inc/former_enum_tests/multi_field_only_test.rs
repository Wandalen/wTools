// File: module/core/former/tests/inc/former_enum_tests/multi_field_only_test.rs

// Use the enum and its impl from the including file (`multi_field_manual.rs` or `multi_field_derive.rs`)
use super::*;

#[ test ]
fn manual_enum_variant_constructors()
{
  // Test the Simple variant "scalar setter" style constructor
  let got_simple = EnumWithMultiField::simple( "test simple" ); // Call directly
  let exp_simple = EnumWithMultiField::Simple( "test simple".to_string() );
  assert_eq!( got_simple, exp_simple );

  // Test the MultiTuple variant constructor
  let got_multi = EnumWithMultiField::multi_tuple( 42, "hello", true );
  let exp_multi = EnumWithMultiField::MultiTuple( 42, "hello".to_string(), true );
  assert_eq!( got_multi, exp_multi );

  // Test the Empty variant constructor
  let got_empty = EnumWithMultiField::empty();
  let exp_empty = EnumWithMultiField::Empty;
  assert_eq!( got_empty, exp_empty );
}