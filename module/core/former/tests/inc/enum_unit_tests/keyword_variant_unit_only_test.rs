// File: module/core/former/tests/inc/former_enum_tests/unit_tests/keyword_variant_unit_only_test.rs
use super::*;

#[ test ]
fn keyword_variant_constructors()
{
  // Test unit variant - Expects direct constructor
  let got_loop = KeywordVariantEnum::r#loop();
  let exp_loop = KeywordVariantEnum::r#Loop;
  assert_eq!( got_loop, exp_loop );
}