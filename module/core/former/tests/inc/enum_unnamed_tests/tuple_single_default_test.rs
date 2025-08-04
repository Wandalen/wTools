//! Test for tuple_single_field_subform handler with default behavior (no attributes)
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
pub enum TupleSingleDefaultEnum
{
  // No attributes - should use default behavior (Rule 3d)
  Variant(InnerStruct),
}

#[test]
fn tuple_single_default_test()
{
  let got = TupleSingleDefaultEnum::variant()
    .value(100)
    .form();
  let expected = TupleSingleDefaultEnum::Variant(InnerStruct {
    value: 100,
  });
  assert_eq!(got, expected);
}

#[test]
fn tuple_single_default_with_defaults_test()
{
  // Test using default values
  let got = TupleSingleDefaultEnum::variant().form();
  let expected = TupleSingleDefaultEnum::Variant(InnerStruct::default());
  assert_eq!(got, expected);
}