// File: module/core/former/tests/inc/former_enum_tests/keyword_variant_only_test.rs
use super::*;

#[ test ]
fn keyword_variant_constructors()
{
  // Test single-field variant subformer style
  // This expects Enum::r#break() to return a former for String
  let got_break = KeywordVariantEnum::r#break()
  .value( "stop".to_string() ) // Assuming StringFormer uses .value()
  .form();
  let exp_break = KeywordVariantEnum::r#Break( "stop".to_string() );
  assert_eq!( got_break, exp_break );

  // Test unit variant direct constructor style
  let got_loop = KeywordVariantEnum::r#loop(); // Expects static method
  let exp_loop = KeywordVariantEnum::r#Loop;
  assert_eq!( got_loop, exp_loop );

  // Test multi-field variant direct constructor style
  let got_if = KeywordVariantEnum::r#if( true, 10 ); // Expects static method
  let exp_if = KeywordVariantEnum::r#If( true, 10 );
  assert_eq!( got_if, exp_if );
}
