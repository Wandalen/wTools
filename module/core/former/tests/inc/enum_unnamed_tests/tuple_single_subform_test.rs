//! Test for tuple_single_field_subform handler
use super::*;
#[allow(unused_imports)]
use ::former::prelude::*;
use ::former::Former;

// Helper struct that derives Former for subform testing
#[derive(Debug, PartialEq, Default, Clone, Former)]
pub struct InnerStruct {
  pub value: i64,
}

#[derive(Debug, PartialEq, Former)]
pub enum TupleSingleSubformEnum
{
  #[subform_scalar]
  Variant(InnerStruct),
}

#[test]
fn tuple_single_subform_test()
{
  let got = TupleSingleSubformEnum::variant()
    .value(100)
    .form();
  let expected = TupleSingleSubformEnum::Variant(InnerStruct {
    value: 100,
  });
  assert_eq!(got, expected);
}

#[test]
fn tuple_single_subform_defaults_test()
{
  // Test using default values
  let got = TupleSingleSubformEnum::variant().form();
  let expected = TupleSingleSubformEnum::Variant(InnerStruct::default());
  assert_eq!(got, expected);
}