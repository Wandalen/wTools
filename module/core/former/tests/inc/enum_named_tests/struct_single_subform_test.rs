#![allow(clippy::used_underscore_binding, clippy::all, warnings, missing_docs)]
//! Test for `struct_single_field_subform` handler
use super::*;
#[ allow( unused_imports ) ]
use ::former::prelude::*;
use ::former::Former;

// Define the inner struct needed for subform tests
#[ derive( Debug, PartialEq, Default, Clone, Former ) ]
pub struct InnerForSubform {
    pub value: i64,
}

#[ derive( Debug, PartialEq, Former ) ]
pub enum SingleSubformEnum
{
  #[ subform_scalar ]
  VariantOneSubform { field_b: InnerForSubform },
}

#[ test ]
fn single_field_subform_test()
{
  // Test using default behavior - the field should default to InnerForSubform::default()
  let got = SingleSubformEnum::variant_one_subform()
    .form();
  let expected = SingleSubformEnum::VariantOneSubform { field_b: InnerForSubform::default() };
  assert_eq!(got, expected);
}

#[ test ]
fn single_field_subform_field_setter_test()
{
  // Test using the field setter directly
  let got = SingleSubformEnum::variant_one_subform()
    .field_b(InnerForSubform { value: 202 })
    .form();
  let expected = SingleSubformEnum::VariantOneSubform { field_b: InnerForSubform { value: 202 } };
  assert_eq!(got, expected);
}