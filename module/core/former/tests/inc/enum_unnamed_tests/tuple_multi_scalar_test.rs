//! Test for `tuple_multi_fields_scalar` handler
use super::*;
#[ allow( unused_imports ) ]
use ::former::prelude::*;
use ::former::Former;

#[ derive( Debug, PartialEq, Former ) ]
pub enum TupleMultiScalarEnum
{
  #[ scalar ]
  Variant(i32, String, bool),
}

#[ test ]
fn tuple_multi_scalar_test()
{
  let got = TupleMultiScalarEnum::variant(42, "test".to_string(), true);
  let expected = TupleMultiScalarEnum::Variant(42, "test".to_string(), true);
  assert_eq!(got, expected);
}

#[ test ]
fn tuple_multi_scalar_into_test()
{
  // Test that impl Into<T> works correctly for multiple fields
  let got = TupleMultiScalarEnum::variant(24i8, "test", false); // i8 should convert to i32
  let expected = TupleMultiScalarEnum::Variant(24, "test".to_string(), false);
  assert_eq!(got, expected);
}