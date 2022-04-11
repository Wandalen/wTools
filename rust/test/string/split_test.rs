
use wtest_basic::*;

#[cfg( feature = "in_wtools" )]
use wtools::string as TheModule;
#[cfg( not( feature = "in_wtools" ) )]
use wstring_tools as TheModule;

//

fn _basic()
{

  // test.case( "delimeter : "b" );
  let src = "abc def";
  let iter = TheModule::string::split()
  .src( src )
  .delimeter( "bc" )
  .stripping( false )
  .form();
  // assert_eq!( iter.collect::< Vec< _ > >(), vec![ "a", "bc", " def" ] );
  assert_eq!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "a", "bc", " def" ] );

}

//

fn _split_with_option_preserving_empty()
{
  let src = "a b c";
  let iter = TheModule::string::split()
  .src( src )
  .delimeter( " " )
  .preserving_empty( true )
  .stripping( false )
  .form();
  assert_eq!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "a", " ", "b", " ", "c" ] );

  let src = "a b c";
  let iter = TheModule::string::split()
  .src( src )
  .delimeter( " " )
  .preserving_empty( false )
  .stripping( false )
  .form();
  assert_eq!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "a", " ", "b", " ", "c" ] );

  /* */

  let src = "a b c";
  let iter = TheModule::string::split()
  .src( src )
  .delimeter( " " )
  .preserving_empty( true )
  .stripping( true )
  .form();
  assert_eq!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "a", "", "b", "", "c" ] );

  let src = "a b c";
  let iter = TheModule::string::split()
  .src( src )
  .delimeter( " " )
  .preserving_empty( false )
  .stripping( true )
  .form();
  assert_eq!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "a", "b", "c" ] );
}

//

fn _split_with_option_preserving_delimeters()
{
  let src = "a b c";
  let iter = TheModule::string::split()
  .src( src )
  .delimeter( " " )
  .preserving_delimeters( true )
  .stripping( false )
  .form();
  assert_eq!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "a", " ", "b", " ", "c" ] );

  let src = "a b c";
  let iter = TheModule::string::split()
  .src( src )
  .delimeter( " " )
  .preserving_delimeters( false )
  .stripping( false )
  .form();
  assert_eq!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "a", "b", "c" ] );
}

//

fn _split_with_option_stripping()
{
  let src = "a b c";
  let iter = TheModule::string::split()
  .src( src )
  .delimeter( " " )
  .stripping( true )
  .form();
  assert_eq!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "a", "", "b", "", "c" ] );

  let src = "a b c";
  let iter = TheModule::string::split()
  .src( src )
  .delimeter( " " )
  .stripping( false )
  .form();
  assert_eq!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "a", " ", "b", " ", "c" ] );
}

//

test_suite!
{
  basic,
  split_with_option_preserving_empty,
  split_with_option_preserving_delimeters,
  split_with_option_stripping,
}
