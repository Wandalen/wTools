//! Test for `struct_zero_fields_handler` with standalone constructors
use super::*;
#[allow(unused_imports)]
use ::former::prelude::*;
use ::former::Former;

#[derive(Debug, PartialEq, Former)]
pub enum StructZeroEnum
{
  #[scalar]
  VariantZero {},
}

#[test]
fn struct_zero_scalar_test()
{
  let got = StructZeroEnum::variant_zero();
  let expected = StructZeroEnum::VariantZero {};
  assert_eq!(got, expected);
}

// #[test]
// fn standalone_variant_zero_test()
// {
//   let got = standalone_variant_zero();
//   let expected = StructZeroEnum::VariantZero {};
//   assert_eq!(got, expected);
// }