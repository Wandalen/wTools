
use super::*;

//

tests_impls!
{
  fn basic()
  {
    let src = "";
    let mut options = the_module::string::isolate_left();
    options.src = the_module::string::isolate::private::Src( src );
    let req = options.isolate();
    let mut exp = ( "", None, "" );
    assert_eq!( req, exp );
  }

  //

  fn isolate_left_or_none()
  {
    /* no entry */
    let src = "abaca";
    let mut options = the_module::string::isolate_left();
    options.src = the_module::string::isolate::private::Src( src );
    options.delimeter = the_module::string::isolate::private::Delimeter( "f" );
    options.none = the_module::string::isolate::private::NoneFlag( true );
    let req = options.isolate();
    let mut exp = ( "", None, "abaca" );
    assert_eq!( req, exp );

    /* default */
    let src = "abaca";
    let mut options = the_module::string::isolate_left();
    options.src = the_module::string::isolate::private::Src( src );
    options.delimeter = the_module::string::isolate::private::Delimeter( "a" );
    options.none = the_module::string::isolate::private::NoneFlag( true );
    let req = options.isolate();
    let mut exp = ( "", Some( "a" ), "baca" );
    assert_eq!( req, exp );

    /* times - 0 */
    let src = "abaca";
    let mut options = the_module::string::isolate_left();
    options.src = the_module::string::isolate::private::Src( src );
    options.delimeter = the_module::string::isolate::private::Delimeter( "a" );
    options.times = 0;
    options.none = the_module::string::isolate::private::NoneFlag( true );
    let req = options.isolate();
    let mut exp = ( "", None, "abaca" );
    assert_eq!( req, exp );

    /* times - 1 */
    let src = "abaca";
    let mut options = the_module::string::isolate_left();
    options.src = the_module::string::isolate::private::Src( src );
    options.delimeter = the_module::string::isolate::private::Delimeter( "a" );
    options.times = 1;
    options.none = the_module::string::isolate::private::NoneFlag( true );
    let req = options.isolate();
    let mut exp = ( "", Some( "a" ), "baca" );
    assert_eq!( req, exp );

    /* times - 2 */
    let src = "abaca";
    let mut options = the_module::string::isolate_left();
    options.src = the_module::string::isolate::private::Src( src );
    options.delimeter = the_module::string::isolate::private::Delimeter( "a" );
    options.times = 2;
    options.none = the_module::string::isolate::private::NoneFlag( true );
    let req = options.isolate();
    let mut exp = ( "ab", Some( "a" ), "ca" );
    assert_eq!( req, exp );

    /* times - 3 */
    let src = "abaca";
    let mut options = the_module::string::isolate_left();
    options.src = the_module::string::isolate::private::Src( src );
    options.delimeter = the_module::string::isolate::private::Delimeter( "a" );
    options.times = 3;
    options.none = the_module::string::isolate::private::NoneFlag( true );
    let req = options.isolate();
    let mut exp = ( "abac", Some( "a" ), "" );
    assert_eq!( req, exp );

    /* times - 4 */
    let src = "abaca";
    let mut options = the_module::string::isolate_left();
    options.src = the_module::string::isolate::private::Src( src );
    options.delimeter = the_module::string::isolate::private::Delimeter( "a" );
    options.times = 4;
    options.none = the_module::string::isolate::private::NoneFlag( true );
    let req = options.isolate();
    let mut exp = ( "", None, "abaca" );
    assert_eq!( req, exp );
  }

  //

  fn isolate_right_or_none()
  {
    /* no entry */
    let src = "abaca";
    let mut options = the_module::string::isolate_right();
    options.src = the_module::string::isolate::private::Src( src );
    options.delimeter = the_module::string::isolate::private::Delimeter( "f" );
    options.none = the_module::string::isolate::private::NoneFlag( true );
    let req = options.isolate();
    let mut exp = ( "abaca", None, "" );
    assert_eq!( req, exp );

    /* default */
    let src = "abaca";
    let mut options = the_module::string::isolate_right();
    options.src = the_module::string::isolate::private::Src( src );
    options.delimeter = the_module::string::isolate::private::Delimeter( "a" );
    options.none = the_module::string::isolate::private::NoneFlag( true );
    let req = options.isolate();
    let mut exp = ( "abac", Some( "a" ), "" );
    assert_eq!( req, exp );

    /* times - 0 */
    let src = "abaca";
    let mut options = the_module::string::isolate_right();
    options.src = the_module::string::isolate::private::Src( src );
    options.delimeter = the_module::string::isolate::private::Delimeter( "a" );
    options.times = 0;
    options.none = the_module::string::isolate::private::NoneFlag( true );
    let req = options.isolate();
    let mut exp = ( "abaca", None, "" );
    assert_eq!( req, exp );

    /* times - 1 */
    let src = "abaca";
    let mut options = the_module::string::isolate_right();
    options.src = the_module::string::isolate::private::Src( src );
    options.delimeter = the_module::string::isolate::private::Delimeter( "a" );
    options.times = 1;
    options.none = the_module::string::isolate::private::NoneFlag( true );
    let req = options.isolate();
    let mut exp = ( "abac", Some( "a" ), "" );
    assert_eq!( req, exp );

    /* times - 2 */
    let src = "abaca";
    let mut options = the_module::string::isolate_right();
    options.src = the_module::string::isolate::private::Src( src );
    options.delimeter = the_module::string::isolate::private::Delimeter( "a" );
    options.times = 2;
    options.none = the_module::string::isolate::private::NoneFlag( true );
    let req = options.isolate();
    let mut exp = ( "ab", Some( "a" ), "ca" );
    assert_eq!( req, exp );

    /* times - 3 */
    let src = "abaca";
    let mut options = the_module::string::isolate_right();
    options.src = the_module::string::isolate::private::Src( src );
    options.delimeter = the_module::string::isolate::private::Delimeter( "a" );
    options.times = 3;
    options.none = the_module::string::isolate::private::NoneFlag( true );
    let req = options.isolate();
    let mut exp = ( "", Some( "a" ), "baca" );
    assert_eq!( req, exp );

    /* times - 4 */
    let src = "abaca";
    let mut options = the_module::string::isolate_right();
    options.src = the_module::string::isolate::private::Src( src );
    options.delimeter = the_module::string::isolate::private::Delimeter( "a" );
    options.times = 4;
    options.none = the_module::string::isolate::private::NoneFlag( true );
    let req = options.isolate();
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
