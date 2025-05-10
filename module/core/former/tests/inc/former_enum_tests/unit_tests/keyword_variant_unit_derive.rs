// File: module/core/former/tests/inc/former_enum_tests/unit_tests/keyword_variant_unit_derive.rs
use super::*;

#[ derive( Debug, PartialEq, the_module::Former ) ]
enum KeywordVariantEnum
{
  /// Unit: Expects r#loop()
  r#Loop,
}

// Include the test logic
include!( "keyword_variant_unit_only_test.rs" );