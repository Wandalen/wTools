
use super::*;

//

tests_impls!
{
  fn basic()
  {
    let src = "abc";
    let iter = the_module::string::split()
    .src( src )
    .perform();
    assert_eq!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "", "", "a", "", "b", "", "c", "", "", ] );
  }

  //

  fn basic_form_and_methods()
  {
    let src = "abc";
    let opts = the_module::string::split()
    .src( src )
    .form();
    let iter = opts.split();
    assert_eq!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "", "", "a", "", "b", "", "c", "", "", ] );

    let src = "abc";
    let opts = the_module::string::split()
    .src( src )
    .form();
    let iter = opts.split_fast();
    assert_eq!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "", "", "a", "", "b", "", "c", "", "", ] );
  }

  //

  fn split_with_option_preserving_empty()
  {
    let src = "a b c";
    let iter = the_module::string::split()
    .src( src )
    .delimeter( " " )
    .preserving_empty( true )
    .stripping( false )
    .perform();
    assert_eq!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "a", " ", "b", " ", "c" ] );

    let src = "a b c";
    let iter = the_module::string::split()
    .src( src )
    .delimeter( " " )
    .preserving_empty( false )
    .stripping( false )
    .perform();
    assert_eq!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "a", " ", "b", " ", "c" ] );

    /* */

    let src = "a b c";
    let iter = the_module::string::split()
    .src( src )
    .delimeter( " " )
    .preserving_empty( true )
    .stripping( true )
    .perform();
    assert_eq!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "a", "", "b", "", "c" ] );

    let src = "a b c";
    let iter = the_module::string::split()
    .src( src )
    .delimeter( " " )
    .preserving_empty( false )
    .stripping( true )
    .perform();
    assert_eq!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "a", "b", "c" ] );
  }

  //

  fn split_with_option_preserving_delimeters()
  {
    let src = "a b c";
    let iter = the_module::string::split()
    .src( src )
    .delimeter( " " )
    .preserving_delimeters( true )
    .stripping( false )
    .perform();
    assert_eq!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "a", " ", "b", " ", "c" ] );

    let src = "a b c";
    let iter = the_module::string::split()
    .src( src )
    .delimeter( " " )
    .preserving_delimeters( false )
    .stripping( false )
    .perform();
    assert_eq!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "a", "b", "c" ] );
  }

  //

  fn split_with_option_preserving_quoting()
  {
    let src = "a 'b' c";
    let iter = the_module::string::split()
    .src( src )
    .delimeter( " " )
    .quoting( false )
    .preserving_delimeters( false )
    .preserving_empty( false )
    .preserving_quoting( true )
    .stripping( true )
    .perform();
    assert_eq!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "a", "'b'", "c" ] );

    let src = "a 'b' c";
    let iter = the_module::string::split()
    .src( src )
    .delimeter( " " )
    .quoting( false )
    .preserving_delimeters( false )
    .preserving_empty( false )
    .preserving_quoting( false )
    .stripping( true )
    .perform();
    assert_eq!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "a", "'b'", "c" ] );

    let src = "a 'b' c";
    let iter = the_module::string::split()
    .src( src )
    .delimeter( " " )
    .quoting( true )
    .preserving_delimeters( false )
    .preserving_empty( false )
    .preserving_quoting( true )
    .stripping( true )
    .perform();
    assert_eq!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "a", "'b'", "c" ] );

    let src = "a 'b' c";
    let iter = the_module::string::split()
    .src( src )
    .delimeter( " " )
    .quoting( true )
    .preserving_delimeters( false )
    .preserving_empty( false )
    .preserving_quoting( false )
    .stripping( true )
    .perform();
    assert_eq!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "a", "b", "c" ] );
  }

  //

  fn split_with_option_stripping()
  {
    let src = "a b c";
    let iter = the_module::string::split()
    .src( src )
    .delimeter( " " )
    .stripping( true )
    .perform();
    assert_eq!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "a", "", "b", "", "c" ] );

    let src = "a b c";
    let iter = the_module::string::split()
    .src( src )
    .delimeter( " " )
    .stripping( false )
    .perform();
    assert_eq!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "a", " ", "b", " ", "c" ] );

    /* */

    let src = "a b c";
    let iter = the_module::string::split()
    .src( src )
    .delimeter( "b" )
    .stripping( true )
    .perform();
    assert_eq!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "a", "b", "c" ] );

    let src = "a b c";
    let iter = the_module::string::split()
    .src( src )
    .delimeter( "b" )
    .preserving_delimeters( false )
    .stripping( true )
    .perform();
    assert_eq!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "a", "c" ] );
  }

  //

  fn split_with_option_quoting()
  {
    let src = "a b c d";
    let iter = the_module::string::split()
    .src( src )
    .delimeter( " " )
    .stripping( false )
    .preserving_delimeters( true )
    .quoting( true )
    .perform();
    assert_eq!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "a", " ", "b", " ", "c", " ", "d" ] );

    let src = "a 'b' c d";
    let iter = the_module::string::split()
    .src( src )
    .delimeter( " " )
    .stripping( false )
    .preserving_delimeters( true )
    .preserving_empty( true )
    .quoting( true )
    .perform();
    assert_eq!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "a", " ", "'b'", " ", "c", " ", "d" ] );

    let src = "a 'b ' c d";
    let iter = the_module::string::split()
    .src( src )
    .delimeter( " " )
    .stripping( false )
    .preserving_delimeters( true )
    .preserving_empty( true )
    .quoting( true )
    .perform();
    assert_eq!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "a", " ", "'b '", " ", "c", " ", "d" ] );

    let src = "a 'b 'c d";
    let iter = the_module::string::split()
    .src( src )
    .delimeter( " " )
    .stripping( false )
    .preserving_delimeters( true )
    .preserving_empty( true )
    .quoting( true )
    .perform();
    assert_eq!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "a", " ", "'b '", "c", " ", "d" ] );

    let src = "'a 'b 'c d";
    let iter = the_module::string::split()
    .src( src )
    .delimeter( " " )
    .stripping( false )
    .preserving_delimeters( true )
    .preserving_empty( true )
    .quoting( true )
    .perform();
    assert_eq!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "'a '", "b", " ", "'c d" ] );

    /* */

    let src = "a b c d";
    let iter = the_module::string::split()
    .src( src )
    .delimeter( " " )
    .stripping( false )
    .preserving_delimeters( false )
    .quoting( true )
    .perform();
    assert_eq!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "a", "b", "c", "d" ] );

    let src = "a 'b' c d";
    let iter = the_module::string::split()
    .src( src )
    .delimeter( " " )
    .stripping( false )
    .preserving_delimeters( false )
    .preserving_empty( true )
    .quoting( true )
    .perform();
    assert_eq!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "a", "'b'", "c", "d" ] );

    let src = "a 'b ' c d";
    let iter = the_module::string::split()
    .src( src )
    .delimeter( " " )
    .stripping( false )
    .preserving_delimeters( false )
    .preserving_empty( true )
    .quoting( true )
    .perform();
    assert_eq!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "a", "'b '", "c", "d" ] );

    let src = "a 'b 'c d";
    let iter = the_module::string::split()
    .src( src )
    .delimeter( " " )
    .stripping( false )
    .preserving_delimeters( false )
    .preserving_empty( true )
    .quoting( true )
    .perform();
    assert_eq!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "a", "'b '", "c", "d" ] );

    let src = "'a 'b 'c d";
    let iter = the_module::string::split()
    .src( src )
    .delimeter( " " )
    .stripping( false )
    .preserving_delimeters( false )
    .preserving_empty( true )
    .quoting( true )
    .perform();
    assert_eq!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "'a '", "b", "'c d" ] );

    /* */

    let src = "a 'b' c d";
    let iter = the_module::string::split()
    .src( src )
    .delimeter( " " )
    .stripping( true )
    .preserving_delimeters( true )
    .preserving_empty( false )
    .quoting( true )
    .perform();
    assert_eq!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "a", "'b'", "c", "d" ] );

    let src = "a 'b ' c d";
    let iter = the_module::string::split()
    .src( src )
    .delimeter( " " )
    .stripping( true )
    .preserving_delimeters( true )
    .preserving_empty( false )
    .quoting( true )
    .perform();
    assert_eq!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "a", "'b '", "c", "d" ] );

    let src = "a 'b 'c d";
    let iter = the_module::string::split()
    .src( src )
    .delimeter( " " )
    .stripping( true )
    .preserving_delimeters( true )
    .preserving_empty( false )
    .quoting( true )
    .perform();
    assert_eq!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "a", "'b '", "c", "d" ] );

    let src = "'a 'b 'c d";
    let iter = the_module::string::split()
    .src( src )
    .delimeter( " " )
    .stripping( true )
    .preserving_delimeters( true )
    .preserving_empty( false )
    .quoting( true )
    .perform();
    assert_eq!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "'a '", "b", "'c d" ] );
  }

  //

  fn basic_split_with_vector()
  {
    let src = "abc";
    let iter = the_module::string::split()
    .src( src )
    .delimeter( vec![] )
    .perform();
    assert_eq!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "abc", ] );

    let src = "abc";
    let iter = the_module::string::split()
    .src( src )
    .delimeter( vec![ "a", "b", "" ] )
    .perform();
    assert_eq!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "", "", "a", "", "b", "", "c", "", "", ] );

    let src = "abc";
    let iter = the_module::string::split()
    .src( src )
    .delimeter( vec![ "b", "d" ] )
    .perform();
    assert_eq!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "a", "b", "c" ] );
  }
}

//

tests_index!
{
  basic,
  basic_form_and_methods,
  split_with_option_preserving_empty,
  split_with_option_preserving_delimeters,
  split_with_option_preserving_quoting,
  split_with_option_stripping,
  split_with_option_quoting,
  basic_split_with_vector,
}
