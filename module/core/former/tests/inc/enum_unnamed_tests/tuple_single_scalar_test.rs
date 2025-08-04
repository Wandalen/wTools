//! Test for tuple_single_field_scalar handler
use super::*;
#[allow(unused_imports)]
use ::former::prelude::*;
use ::former::Former;

#[derive(Debug, PartialEq, Former)]
pub enum TupleSingleScalarEnum
{
  #[scalar]
  Variant(String),
}

#[test]
fn tuple_single_scalar_test()
{
  let got = TupleSingleScalarEnum::variant("test_value".to_string());
  let expected = TupleSingleScalarEnum::Variant("test_value".to_string());
  assert_eq!(got, expected);
}

#[test]
fn tuple_single_scalar_into_test()  
{
  // Test that impl Into<String> works correctly
  let got = TupleSingleScalarEnum::variant("test_value");
  let expected = TupleSingleScalarEnum::Variant("test_value".to_string()); 
  assert_eq!(got, expected);
}