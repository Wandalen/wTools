// File: module/core/former/tests/inc/former_enum_tests/keyword_variant_derive.rs
use super::*;

// Assume StringFormer exists for the derive macro to find for the r#Break variant
// (Normally provided by former crate itself or derived)
#[ derive( Debug, PartialEq, former::Former ) ]
struct StringFormerStub { value : String } // Minimal stub

#[ derive( Debug, PartialEq, the_module::Former ) ]
enum KeywordVariantEnum
{
  r#Break( String ), // Single-field tuple -> Expects subformer method `r#break()`
  r#Loop,            // Unit -> Expects direct constructor `r#loop()`
  r#If( bool, i32 ), // Multi-field tuple -> Expects direct constructor `r#if()`
}

include!( "keyword_variant_only_test.rs" );