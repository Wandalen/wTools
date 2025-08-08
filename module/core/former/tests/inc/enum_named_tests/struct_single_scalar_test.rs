//! Test for `struct_single_field_scalar` handler
use super::*;
#[ allow( unused_imports ) ]
use ::former::prelude::*;
use ::former::Former;

#[ derive( Debug, PartialEq, Former ) ]
pub enum SingleFieldEnum
{
  #[ scalar ]
  VariantOneScalar { field_a: String },
}

#[ test ]
fn single_field_scalar_test()
{
  let got = SingleFieldEnum::variant_one_scalar("value_a".to_string());
  let expected = SingleFieldEnum::VariantOneScalar { field_a: "value_a".to_string() };
  assert_eq!(got, expected);
}

#[ test ]
fn single_field_scalar_into_test()
{
  // Test that impl Into<String> works correctly
  let got = SingleFieldEnum::variant_one_scalar("value_b");
  let expected = SingleFieldEnum::VariantOneScalar { field_a: "value_b".to_string() };
  assert_eq!(got, expected);
}