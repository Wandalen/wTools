
use wtest_basic::*;

#[cfg( feature = "in_wtools" )]
use wtools::string as TheModule;
#[cfg( not( feature = "in_wtools" ) )]
use wstring_tools as TheModule;

// use TheModule::prelude::*;

//

fn _basic()
{
  let src = "abc";
  let iter = TheModule::string::split()
  .src( src )
  .perform();
  assert_eq!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "", "", "a", "", "b", "", "c", "", "", ] );
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
  .perform();
  assert_eq!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "a", " ", "b", " ", "c" ] );

  let src = "a b c";
  let iter = TheModule::string::split()
  .src( src )
  .delimeter( " " )
  .preserving_empty( false )
  .stripping( false )
  .perform();
  assert_eq!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "a", " ", "b", " ", "c" ] );

  /* */

  let src = "a b c";
  let iter = TheModule::string::split()
  .src( src )
  .delimeter( " " )
  .preserving_empty( true )
  .stripping( true )
  .perform();
  assert_eq!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "a", "", "b", "", "c" ] );

  let src = "a b c";
  let iter = TheModule::string::split()
  .src( src )
  .delimeter( " " )
  .preserving_empty( false )
  .stripping( true )
  .perform();
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
  .perform();
  assert_eq!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "a", " ", "b", " ", "c" ] );

  let src = "a b c";
  let iter = TheModule::string::split()
  .src( src )
  .delimeter( " " )
  .preserving_delimeters( false )
  .stripping( false )
  .perform();
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
  .perform();
  assert_eq!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "a", "", "b", "", "c" ] );

  let src = "a b c";
  let iter = TheModule::string::split()
  .src( src )
  .delimeter( " " )
  .stripping( false )
  .perform();
  assert_eq!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "a", " ", "b", " ", "c" ] );

  /* */

  let src = "a b c";
  let iter = TheModule::string::split()
  .src( src )
  .delimeter( "b" )
  .stripping( true )
  .perform();
  assert_eq!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "a", "b", "c" ] );

  let src = "a b c";
  let iter = TheModule::string::split()
  .src( src )
  .delimeter( "b" )
  .preserving_delimeters( false )
  .stripping( true )
  .perform();
  assert_eq!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "a", "c" ] );
}

//

fn _basic_split_with_vector()
{
  let src = "abc";
  let iter = TheModule::string::split()
  .src( src )
  .delimeter( vec![] )
  .perform();
  assert_eq!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "abc", ] );

  let src = "abc";
  let iter = TheModule::string::split()
  .src( src )
  .delimeter( vec![ "a", "b", "" ] )
  .perform();
  assert_eq!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "", "", "a", "", "b", "", "c", "", "", ] );

  let src = "abc";
  let iter = TheModule::string::split()
  .src( src )
  .delimeter( vec![ "b", "d" ] )
  .perform();
  assert_eq!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "a", "b", "c" ] );
}

//

test_suite!
{
  basic,
  split_with_option_preserving_empty,
  split_with_option_preserving_delimeters,
  split_with_option_stripping,
  basic_split_with_vector,
}
