#[ allow( unused_imports ) ]
use super :: *;
#[ allow( unused_imports ) ]
use test_tools ::impls_index ::tests_impls;
#[ allow( unused_imports ) ]
use test_tools ::impls_index ::tests_index;
//

tests_impls! {
  #[ test ]
  fn basic()
  {

  /* test.case( "parse" ); */
  {
   a_id!( crate ::the_module ::string ::number ::parse :: < f32, _ >( "1.0" ), Ok( 1.0 ) );
 }

  /* test.case( "parse_partial" ); */
  {
   a_id!( crate ::the_module ::string ::number ::parse_partial :: < i32, _ >( "1a" ), Ok( ( 1, 1 ) ) );
 }

  /* test.case( "parse_partial_with_options" ); */
  {
   const FORMAT: u128 = crate ::the_module ::string ::number ::format ::STANDARD;
   let options = crate ::the_module ::string ::number ::ParseFloatOptions ::builder()
   .exponent( b'^' )
   .decimal_point( b',' )
   .build()
   .unwrap();
   let result = crate ::the_module ::string ::number ::parse_partial_with_options :: < f32, _, FORMAT >( "0", &options );
   assert_eq!( result, Ok( ( 0.0f32, 1usize ) ) );
 }

  /* test.case( "parse_with_options" ); */
  {
   const FORMAT: u128 = crate ::the_module ::string ::number ::format ::STANDARD;
   let options = crate ::the_module ::string ::number ::ParseFloatOptions ::builder()
   .exponent( b'^' )
   .decimal_point( b',' )
   .build()
   .unwrap();
   let result = crate ::the_module ::string ::number ::parse_with_options :: < f32, _, FORMAT >( "1,2345", &options );
   assert_eq!( result, Ok( 1.2345f32 ) );
 }

  /* test.case( "to_string" ); */
  {
   a_id!( crate ::the_module ::string ::number ::to_string( 5 ), "5" );
 }

 }
}

//

tests_index! {
  basic,
}
