use super::*;

//

tests_impls!
{
  #[ test ]
  fn basic()
  {

    /* test.case( "parse" ); */
    {
      a_id!( the_module::number::parse::< f32, _ >( "1.0" ), Ok( 1.0 ) );
    }

    /* test.case( "parse_partial" ); */
    {
      a_id!( the_module::number::parse_partial::< i32, _ >( "1a" ), Ok( ( 1, 1 ) ) );
    }

    /* test.case( "parse_partial_with_options" ); */
    {
      const FORMAT : u128 = the_module::number::format::STANDARD;
      let options = the_module::number::ParseFloatOptions::builder()
      .exponent( b'^' )
      .decimal_point( b',' )
      .build()
      .unwrap();
      let got = the_module::number::parse_partial_with_options::< f32, _, FORMAT >( "0", &options );
      let exp = Ok( ( 0.0, 1 ) );
      a_id!( got, exp );
    }

    /* test.case( "parse_with_options" ); */
    {
      const FORMAT: u128 = the_module::number::format::STANDARD;
      let options = the_module::number::ParseFloatOptions::builder()
      .exponent( b'^' )
      .decimal_point( b',' )
      .build()
      .unwrap();
      let got = the_module::number::parse_with_options::< f32, _, FORMAT >( "1,2345", &options );
      let exp = Ok( 1.2345 );
      a_id!( got, exp );
    }

    /* test.case( "to_string" ); */
    {
      a_id!( the_module::number::to_string( 5 ), "5" );
    }

  }
}

//

tests_index!
{
  basic,
}
