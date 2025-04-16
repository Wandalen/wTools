// File: module/core/former/tests/inc/former_enum_tests/keyword_variant_derive.rs
use super::*;

// Assume StringFormer exists for the derive macro to find for the r#Break variant
// (Normally provided by former crate itself or derived)
#[ derive( Debug, Default, PartialEq, former::Former ) ]
struct StringFormerStub
{
  value : String,
}

// Define an inner struct that also derives Former
#[ derive( Debug, Default, PartialEq, former::Former ) ]
pub struct InnerData
{
  data1 : i32,
  data2 : bool,
}

#[ derive( Debug, PartialEq, the_module::Former ) ]
enum KeywordVariantEnum
{
  /// Explicitly scalar: Expects r#break(StringFormerStub)
  #[ scalar ]
  r#Break( StringFormerStub ),
  /// Unit: Expects r#loop()
  r#Loop,
  /// Multi-field tuple: Explicitly scalar required -> Expects r#if(bool, i32)
  #[ scalar ]
  r#If( bool, i32 ),
  /// Explicitly scalar: Expects r#let(u32)
  #[ scalar ]
  r#Let( u32 ),
  /// Explicit Subform: Expects r#struct() -> InnerDataFormer<...>
  #[ subform_scalar ] // Apply attribute to variant
  r#Struct( InnerData ),
  /// Multi-field tuple: Explicitly scalar required -> Expects r#for(usize, &'static str)
  #[ scalar ]
  r#For( usize, &'static str ),
}

include!( "keyword_variant_only_test.rs" );