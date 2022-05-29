
use test_tools::*;

use super::TheModule;

//

tests_impls!
{
  #[ test ]
  fn basic()
  {
    let src = "abc";
    let iter = TheModule::string::split()
    .src( src )
    .perform();
    a_id!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "", "", "a", "", "b", "", "c", "", "", ] );
  }

  //

  #[ test ]
  fn split_with_option_preserving_empty()
  {
    let src = "a b c";
    let iter = TheModule::string::split()
    .src( src )
    .delimeter( " " )
    .preserving_empty( true )
    .stripping( false )
    .perform();
    a_id!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "a", " ", "b", " ", "c" ] );

    let src = "a b c";
    let iter = TheModule::string::split()
    .src( src )
    .delimeter( " " )
    .preserving_empty( false )
    .stripping( false )
    .perform();
    a_id!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "a", " ", "b", " ", "c" ] );

    /* */

    let src = "a b c";
    let iter = TheModule::string::split()
    .src( src )
    .delimeter( " " )
    .preserving_empty( true )
    .stripping( true )
    .perform();
    a_id!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "a", "", "b", "", "c" ] );

    let src = "a b c";
    let iter = TheModule::string::split()
    .src( src )
    .delimeter( " " )
    .preserving_empty( false )
    .stripping( true )
    .perform();
    a_id!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "a", "b", "c" ] );
  }

  //

  #[ test ]
  fn split_with_option_preserving_delimeters()
  {
    let src = "a b c";
    let iter = TheModule::string::split()
    .src( src )
    .delimeter( " " )
    .preserving_delimeters( true )
    .stripping( false )
    .perform();
    a_id!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "a", " ", "b", " ", "c" ] );

    let src = "a b c";
    let iter = TheModule::string::split()
    .src( src )
    .delimeter( " " )
    .preserving_delimeters( false )
    .stripping( false )
    .perform();
    a_id!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "a", "b", "c" ] );
  }

  //

  #[ test ]
  fn split_with_option_stripping()
  {
    let src = "a b c";
    let iter = TheModule::string::split()
    .src( src )
    .delimeter( " " )
    .stripping( true )
    .perform();
    a_id!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "a", "", "b", "", "c" ] );

    let src = "a b c";
    let iter = TheModule::string::split()
    .src( src )
    .delimeter( " " )
    .stripping( false )
    .perform();
    a_id!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "a", " ", "b", " ", "c" ] );

    /* */

    let src = "a b c";
    let iter = TheModule::string::split()
    .src( src )
    .delimeter( "b" )
    .stripping( true )
    .perform();
    a_id!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "a", "b", "c" ] );

    let src = "a b c";
    let iter = TheModule::string::split()
    .src( src )
    .delimeter( "b" )
    .preserving_delimeters( false )
    .stripping( true )
    .perform();
    a_id!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "a", "c" ] );
  }
}

//

tests_index!
{
  basic,
  split_with_option_preserving_empty,
  split_with_option_preserving_delimeters,
  split_with_option_stripping,
}
