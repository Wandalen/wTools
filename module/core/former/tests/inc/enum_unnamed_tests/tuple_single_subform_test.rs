#![allow(clippy::used_underscore_binding, clippy::all, warnings, missing_docs)]
//! Test for `tuple_single_field_subform` handler
use super::*;
#[ allow( unused_imports ) ]
use ::former::prelude::*;
use ::former::Former;

// Helper struct that derives Former for subform testing
#[ derive( Debug, PartialEq, Default, Clone, Former ) ]
pub struct InnerStruct {
  pub value: i64,
}

#[ derive( Debug, PartialEq, Former ) ]
pub enum TupleSingleSubformEnum
{
  #[ subform_scalar ]
  Variant(InnerStruct),
}

#[ test ]
fn tuple_single_subform_test()
{
  // Using fixed handler approach with ._0() indexed setter
  // TODO: Should delegate to field type's Former per spec Rule 2d
  let inner = InnerStruct { value: 100 };
  let got = TupleSingleSubformEnum::variant()
    ._0(inner)
    .form();
  let expected = TupleSingleSubformEnum::Variant(InnerStruct {
    value: 100,
  });
  assert_eq!(got, expected);
}

#[ test ]
fn tuple_single_subform_defaults_test()
{
  // Test using default values with fixed handler
  let got = TupleSingleSubformEnum::variant().form();
  let expected = TupleSingleSubformEnum::Variant(InnerStruct::default());
  assert_eq!(got, expected);
}