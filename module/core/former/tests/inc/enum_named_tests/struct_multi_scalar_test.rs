//! Test for struct_multi_fields_scalar handler
use super::*;
#[allow(unused_imports)]
use ::former::prelude::*;
use ::former::Former;

#[derive(Debug, PartialEq, Former)]
pub enum MultiFieldEnum
{
  #[scalar]
  VariantTwoScalar { field_d: i32, field_e: bool },
}

#[test]
fn multi_field_scalar_test()
{
  let got = MultiFieldEnum::variant_two_scalar(42, true);
  let expected = MultiFieldEnum::VariantTwoScalar { field_d: 42, field_e: true };
  assert_eq!(got, expected);
}

#[test]
fn multi_field_scalar_into_test()
{
  // Test that impl Into<T> works correctly for multiple fields
  let got = MultiFieldEnum::variant_two_scalar(24i8, false); // i8 should convert to i32
  let expected = MultiFieldEnum::VariantTwoScalar { field_d: 24, field_e: false };
  assert_eq!(got, expected);
}