#[ allow( unused_imports ) ]
use super :: *;

//

#[ test ]
fn basic()
{

  /* test.case( "parse" ); */
  {
    assert_eq!( crate ::the_module ::string ::number ::parse :: < f32, _ >( "1.0" ), Ok( 1.0 ) );
  }

  /* test.case( "parse_partial" ); */
  {
    assert_eq!( crate ::the_module ::string ::number ::parse_partial :: < i32, _ >( "1a" ), Ok( ( 1, 1 ) ) );
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
    assert_eq!( crate ::the_module ::string ::number ::to_string( 5 ), "5" );
  }

}

/// Integer parsing via the generic parse function.
#[ test ]
fn integer_basic()
{
  let result = crate::the_module::string::number::parse::< i32, _ >( "42" );
  assert_eq!( result, Ok( 42i32 ), "basic integer parse failed" );
}

/// Scientific notation parsed as f64.
#[ test ]
fn scientific_notation()
{
  let result = crate::the_module::string::number::parse::< f64, _ >( "1.5e10" );
  assert_eq!( result, Ok( 1.5e10f64 ), "scientific notation parse failed" );
}

/// Non-numeric input returns an error.
#[ test ]
fn invalid_input()
{
  let result = crate::the_module::string::number::parse::< i32, _ >( "not_a_number" );
  assert!( result.is_err(), "expected Err for non-numeric input, got {result:?}" );
}

/// Overflowing integer returns an error.
#[ test ]
fn overflow_i32()
{
  let result = crate::the_module::string::number::parse::< i32, _ >( "99999999999999999999" );
  assert!( result.is_err(), "expected Err for overflow, got {result:?}" );
}

/// f64 boundary value at MAX parses successfully.
#[ test ]
fn f64_boundary()
{
  let result = crate::the_module::string::number::parse::< f64, _ >( "1.7976931348623157e+308" );
  assert!( result.is_ok(), "f64::MAX should parse, got {result:?}" );
  let val = result.unwrap();
  assert!( ( val - f64::MAX ).abs() < 1e292, "parsed value should be close to f64::MAX" );
}
