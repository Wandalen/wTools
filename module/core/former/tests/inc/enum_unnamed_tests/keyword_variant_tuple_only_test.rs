#[allow(clippy::used_underscore_binding, clippy::all, warnings)]
// Purpose: Tests keyword variant handling in tuple context
// This file is included by keyword_variant_tuple_derive files

#[ test ]
fn keyword_variant_test()
{
  // Test the scalar constructor with keyword identifier
  let got = KeywordVariantEnum::r#use( 42u32 );
  let expected = KeywordVariantEnum::r#use( 42u32 );
  assert_eq!( got, expected );
  
  // Test the scalar constructor for break variant
  let break_val = Break { value: 100 };
  let got_break = KeywordVariantEnum::r#break( break_val.clone() );
  let expected_break = KeywordVariantEnum::r#break( break_val );
  assert_eq!( got_break, expected_break );
}