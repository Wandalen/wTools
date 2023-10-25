use super::*;

//
#[ cfg( feature = "enabled" ) ]
tests_impls!
{
  #[ test ]
  fn basic()
  {

    /* test.case( "parse" ); */
    {
      a_id!( TheModule::number::parse::< f32, _ >( "1.0" ), Ok( 1.0 ) );
    }

    /* test.case( "parse_partial" ); */
    {
      a_id!( TheModule::number::parse_partial::< i32, _ >( "1a" ), Ok( ( 1, 1 ) ) );
    }

    /* test.case( "parse_partial_with_options" ); */
    {
      const FORMAT : u128 = TheModule::number::format::STANDARD;
      let options = TheModule::number::ParseFloatOptions::builder()
      .exponent( b'^' )
      .decimal_point( b',' )
      .build()
      .unwrap();
      let got = TheModule::number::parse_partial_with_options::< f32, _, FORMAT >( "0", &options );
      let exp = Ok( ( 0.0, 1 ) );
      a_id!( got, exp );
    }

    /* test.case( "parse_with_options" ); */
    {
      const FORMAT: u128 = TheModule::number::format::STANDARD;
      let options = TheModule::number::ParseFloatOptions::builder()
      .exponent( b'^' )
      .decimal_point( b',' )
      .build()
      .unwrap();
      let got = TheModule::number::parse_with_options::< f32, _, FORMAT >( "1,2345", &options );
      let exp = Ok( 1.2345 );
      a_id!( got, exp );
    }

    /* test.case( "to_string" ); */
    {
      a_id!( TheModule::number::to_string( 5 ), "5" );
    }

  }
}

//

#[ cfg( feature = "enabled" ) ]
tests_index!
{
  basic,
}
