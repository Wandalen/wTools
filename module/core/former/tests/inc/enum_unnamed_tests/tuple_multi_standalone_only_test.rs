#[allow(clippy::used_underscore_binding, clippy::all, warnings)]
// Purpose: Tests standalone constructor functionality for multi-field tuple variants
// This file is included by tuple_multi_standalone derive/manual files

#[ test ]
fn multi_tuple_standalone_constructor()
{
  // Test that the standalone constructor returns a former that can be used to build the variant
  let got = variant()  // Use module-level function (manual) or static method (derive)
    ._0( 42u32 )  // Fix type: use u32 literal
    ._1( "test".to_string() )
    .form();
  let expected = TestEnum::Variant( 42u32, "test".to_string() );
  assert_eq!( got, expected );
}