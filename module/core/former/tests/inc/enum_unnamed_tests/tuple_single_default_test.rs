#![allow(clippy::used_underscore_binding, clippy::all, warnings, missing_docs)]
//! Test for `tuple_single_field_subform` handler with default behavior (no attributes)
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
pub enum TupleSingleDefaultEnum
{
  // No attributes - should use default behavior (Rule 3d)
  Variant(InnerStruct),
}

#[ test ]
fn tuple_single_default_test()
{
  // Using fixed handler approach with ._0() indexed setter
  let inner = InnerStruct { value: 100 };
  let got = TupleSingleDefaultEnum::variant()
    ._0(inner)
    .form();
  let expected = TupleSingleDefaultEnum::Variant(InnerStruct {
    value: 100,
  });
  assert_eq!(got, expected);
}

#[ test ]
fn tuple_single_default_with_defaults_test()
{
  // Test using default values with fixed handler
  let got = TupleSingleDefaultEnum::variant().form();
  let expected = TupleSingleDefaultEnum::Variant(InnerStruct::default());
  assert_eq!(got, expected);
}