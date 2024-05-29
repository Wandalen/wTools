
use super::*;

//

tests_impls!
{
  fn basic()
  {
    let src = "";
    let req = the_module::string::isolate_left()
    .src( src )
    .perform();
    let mut exp = ( "", None, "" );
    assert_eq!( req, exp );
  }

  //

  fn isolate_left_or_none()
  {
    /* no entry */
    let src = "abaca";
    let req = the_module::string::isolate_left()
    .src( src )
    .delimeter( "f" )
    .none( true )
    .perform();
    let mut exp = ( "", None, "abaca" );
    assert_eq!( req, exp );

    /* default */
    let src = "abaca";
    let req = the_module::string::isolate_left()
    .src( src )
    .delimeter( "a" )
    .none( true )
    .perform();
    let mut exp = ( "", Some( "a" ), "baca" );
    assert_eq!( req, exp );

    /* times - 0 */
    let src = "abaca";
    let req = the_module::string::isolate_left()
    .src( src )
    .delimeter( "a" )
    .times( 0 )
    .none( true )
    .perform();
    let mut exp = ( "", None, "abaca" );
    assert_eq!( req, exp );

    /* times - 1 */
    let src = "abaca";
    let req = the_module::string::isolate_left()
    .src( src )
    .delimeter( "a" )
    .times( 1 )
    .none( true )
    .perform();
    let mut exp = ( "", Some( "a" ), "baca" );
    assert_eq!( req, exp );

    /* times - 2 */
    let src = "abaca";
    let req = the_module::string::isolate_left()
    .src( src )
    .delimeter( "a" )
    .times( 2 )
    .none( true )
    .perform();
    let mut exp = ( "ab", Some( "a" ), "ca" );
    assert_eq!( req, exp );

    /* times - 3 */
    let src = "abaca";
    let req = the_module::string::isolate_left()
    .src( src )
    .delimeter( "a" )
    .times( 3 )
    .none( true )
    .perform();
    let mut exp = ( "abac", Some( "a" ), "" );
    assert_eq!( req, exp );

    /* times - 4 */
    let src = "abaca";
    let req = the_module::string::isolate_left()
    .src( src )
    .delimeter( "a" )
    .times( 4 )
    .none( true )
    .perform();
    let mut exp = ( "", None, "abaca" );
    assert_eq!( req, exp );
  }

  //

  fn isolate_right_or_none()
  {
    /* no entry */
    let src = "abaca";
    let req = the_module::string::isolate_right()
    .src( src )
    .delimeter( "f" )
    .none( true )
    .perform();
    let mut exp = ( "abaca", None, "" );
    assert_eq!( req, exp );

    /* default */
    let src = "abaca";
    let req = the_module::string::isolate_right()
    .src( src )
    .delimeter( "a" )
    .none( true )
    .perform();
    let mut exp = ( "abac", Some( "a" ), "" );
    assert_eq!( req, exp );

    /* times - 0 */
    let src = "abaca";
    let req = the_module::string::isolate_right()
    .src( src )
    .delimeter( "a" )
    .times( 0 )
    .none( true )
    .perform();
    let mut exp = ( "abaca", None, "" );
    assert_eq!( req, exp );

    /* times - 1 */
    let src = "abaca";
    let req = the_module::string::isolate_right()
    .src( src )
    .delimeter( "a" )
    .times( 1 )
    .none( true )
    .perform();
    let mut exp = ( "abac", Some( "a" ), "" );
    assert_eq!( req, exp );

    /* times - 2 */
    let src = "abaca";
    let req = the_module::string::isolate_right()
    .src( src )
    .delimeter( "a" )
    .times( 2 )
    .none( true )
    .perform();
    let mut exp = ( "ab", Some( "a" ), "ca" );
    assert_eq!( req, exp );

    /* times - 3 */
    let src = "abaca";
    let req = the_module::string::isolate_right()
    .src( src )
    .delimeter( "a" )
    .times( 3 )
    .none( true )
    .perform();
    let mut exp = ( "", Some( "a" ), "baca" );
    assert_eq!( req, exp );

    /* times - 4 */
    let src = "abaca";
    let req = the_module::string::isolate_right()
    .src( src )
    .delimeter( "a" )
    .times( 4 )
    .none( true )
    .perform();
    let mut exp = ( "abaca", None, "" );
    assert_eq!( req, exp );
  }
}

//

tests_index!
{
  basic,
  isolate_left_or_none,
  isolate_right_or_none,
}
