#![allow(clippy::used_underscore_binding, clippy::all, warnings, missing_docs)]
//! Test for `tuple_multi_fields_subform` handler with default behavior (no attributes)
use super::*;
#[ allow( unused_imports ) ]
use ::former::prelude::*;
use ::former::Former;

#[ derive( Debug, PartialEq, Former ) ]
pub enum TupleMultiDefaultEnum
{
  // No attributes - should use default behavior (Rule 3f - multi-field subform)
  Variant(i32, String, bool),
}

#[ test ]
fn tuple_multi_default_test()
{
  let got = TupleMultiDefaultEnum::variant()
    ._0(42)
    ._1("test".to_string())
    ._2(true)
    .form();
  let expected = TupleMultiDefaultEnum::Variant(42, "test".to_string(), true);
  assert_eq!(got, expected);
}

#[ test ]
fn tuple_multi_default_into_test()
{
  // Test that impl Into<T> works correctly for multiple fields
  let got = TupleMultiDefaultEnum::variant()
    ._0(24i8) // i8 should convert to i32
    ._1("test") // &str should convert to String
    ._2(false)
    .form();
  let expected = TupleMultiDefaultEnum::Variant(24, "test".to_string(), false);
  assert_eq!(got, expected);
}