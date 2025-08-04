//! Minimal test for struct_zero_fields_handler to verify it's working
use super::*;
#[allow(unused_imports)]
use ::former::prelude::*;
use ::former::Former;

#[derive(Debug, PartialEq, Former)]
pub enum MinimalEnum
{
  #[scalar]
  ZeroField {},
}

#[test]
fn minimal_zero_field_test()
{
  let got = MinimalEnum::zero_field();
  let expected = MinimalEnum::ZeroField {};
  assert_eq!(got, expected);
}