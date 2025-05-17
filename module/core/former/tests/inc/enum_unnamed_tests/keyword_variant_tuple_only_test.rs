// File: module/core/former/tests/inc/former_enum_tests/unnamed_tests/keyword_variant_tuple_only_test.rs
use super::*;

#[ test ]
fn keyword_variant_constructors()
{
  // Test single-field variant (StringFormerStub) - Expects direct constructor due to #[scalar]
  let inner_string_stub = StringFormerStub { value : "stop".to_string() };
  let got_break = KeywordVariantEnum::r#break( inner_string_stub );
  let exp_break = KeywordVariantEnum::r#Break( StringFormerStub { value: "stop".to_string() } );
  assert_eq!( got_break, exp_break );

  // Test multi-field variant (bool, i32) - Expects former builder due to #[scalar] and multi-fields
  let got_if = KeywordVariantEnum::r#if()
    ._0( true )
    ._1( 10 )
    .form();
  let exp_if = KeywordVariantEnum::r#If( true, 10 );
  assert_eq!( got_if, exp_if );

  // Test single-field variant (u32) - Expects direct constructor due to #[scalar]
  let got_let = KeywordVariantEnum::r#let( 99_u32 );
  let exp_let = KeywordVariantEnum::r#Let( 99_u32 );
  assert_eq!( got_let, exp_let );

  // Test single-field variant (InnerData) - Expects subformer due to #[subform_scalar]
  let got_struct = KeywordVariantEnum::r#struct()
    .data1( -1 )
    .data2( false )
    .form();
  let exp_struct = KeywordVariantEnum::r#Struct( InnerData { data1: -1, data2: false } );
  assert_eq!( got_struct, exp_struct );

  // Test multi-field variant (usize, &'static str) - Expects former builder due to #[scalar] and multi-fields
  // Explicitly type the integer literal as usize
  let got_for = KeywordVariantEnum::r#for()
    ._0( 5_usize )
    ._1( "times" )
    .form();
  let exp_for = KeywordVariantEnum::r#For( 5, "times" );
  assert_eq!( got_for, exp_for );
}