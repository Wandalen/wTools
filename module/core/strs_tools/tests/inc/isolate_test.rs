#[ allow( unused_imports ) ]
use super :: *;

//

#[ test ]
fn basic()
{
  let src = "";
  let mut options = the_module ::string ::isolate_left();
  options.src = the_module ::string ::isolate ::private ::Src( src );
  let req = options.isolate();
  let exp = ( "", None, "" );
  assert_eq!( req, exp );
}

//

#[ test ]
fn isolate_left_or_none()
{
  /* no entry */
  let src = "abaca";
  let mut options = the_module ::string ::isolate_left();
  options.src = the_module ::string ::isolate ::private ::Src( src );
  options.delimiter = the_module ::string ::isolate ::private ::Delimiter( "f" );
  options.none = the_module ::string ::isolate ::private ::NoneFlag( true );
  let req = options.isolate();
  let exp = ( "", None, "abaca" );
  assert_eq!( req, exp );

  /* default */
  let src = "abaca";
  let mut options = the_module ::string ::isolate_left();
  options.src = the_module ::string ::isolate ::private ::Src( src );
  options.delimiter = the_module ::string ::isolate ::private ::Delimiter( "a" );
  options.none = the_module ::string ::isolate ::private ::NoneFlag( true );
  let req = options.isolate();
  let exp = ( "", Some( "a" ), "baca" );
  assert_eq!( req, exp );

  /* times - 0 */
  let src = "abaca";
  let mut options = the_module ::string ::isolate_left();
  options.src = the_module ::string ::isolate ::private ::Src( src );
  options.delimiter = the_module ::string ::isolate ::private ::Delimiter( "a" );
  options.times = 0;
  options.none = the_module ::string ::isolate ::private ::NoneFlag( true );
  let req = options.isolate();
  let exp = ( "", None, "abaca" );
  assert_eq!( req, exp );

  /* times - 1 */
  let src = "abaca";
  let mut options = the_module ::string ::isolate_left();
  options.src = the_module ::string ::isolate ::private ::Src( src );
  options.delimiter = the_module ::string ::isolate ::private ::Delimiter( "a" );
  options.times = 1;
  options.none = the_module ::string ::isolate ::private ::NoneFlag( true );
  let req = options.isolate();
  let exp = ( "", Some( "a" ), "baca" );
  assert_eq!( req, exp );

  /* times - 2 */
  let src = "abaca";
  let mut options = the_module ::string ::isolate_left();
  options.src = the_module ::string ::isolate ::private ::Src( src );
  options.delimiter = the_module ::string ::isolate ::private ::Delimiter( "a" );
  options.times = 2;
  options.none = the_module ::string ::isolate ::private ::NoneFlag( true );
  let req = options.isolate();
  let exp = ( "ab", Some( "a" ), "ca" );
  assert_eq!( req, exp );

  /* times - 3 */
  let src = "abaca";
  let mut options = the_module ::string ::isolate_left();
  options.src = the_module ::string ::isolate ::private ::Src( src );
  options.delimiter = the_module ::string ::isolate ::private ::Delimiter( "a" );
  options.times = 3;
  options.none = the_module ::string ::isolate ::private ::NoneFlag( true );
  let req = options.isolate();
  let exp = ( "abac", Some( "a" ), "" );
  assert_eq!( req, exp );

  /* times - 4 */
  let src = "abaca";
  let mut options = the_module ::string ::isolate_left();
  options.src = the_module ::string ::isolate ::private ::Src( src );
  options.delimiter = the_module ::string ::isolate ::private ::Delimiter( "a" );
  options.times = 4;
  options.none = the_module ::string ::isolate ::private ::NoneFlag( true );
  let req = options.isolate();
  let exp = ( "", None, "abaca" );
  assert_eq!( req, exp );
}

//

#[ test ]
fn isolate_right_or_none()
{
  /* no entry */
  let src = "abaca";
  let mut options = the_module ::string ::isolate_right();
  options.src = the_module ::string ::isolate ::private ::Src( src );
  options.delimiter = the_module ::string ::isolate ::private ::Delimiter( "f" );
  options.none = the_module ::string ::isolate ::private ::NoneFlag( true );
  let req = options.isolate();
  let exp = ( "abaca", None, "" );
  assert_eq!( req, exp );

  /* default */
  let src = "abaca";
  let mut options = the_module ::string ::isolate_right();
  options.src = the_module ::string ::isolate ::private ::Src( src );
  options.delimiter = the_module ::string ::isolate ::private ::Delimiter( "a" );
  options.none = the_module ::string ::isolate ::private ::NoneFlag( true );
  let req = options.isolate();
  let exp = ( "abac", Some( "a" ), "" );
  assert_eq!( req, exp );

  /* times - 0 */
  let src = "abaca";
  let mut options = the_module ::string ::isolate_right();
  options.src = the_module ::string ::isolate ::private ::Src( src );
  options.delimiter = the_module ::string ::isolate ::private ::Delimiter( "a" );
  options.times = 0;
  options.none = the_module ::string ::isolate ::private ::NoneFlag( true );
  let req = options.isolate();
  let exp = ( "abaca", None, "" );
  assert_eq!( req, exp );

  /* times - 1 */
  let src = "abaca";
  let mut options = the_module ::string ::isolate_right();
  options.src = the_module ::string ::isolate ::private ::Src( src );
  options.delimiter = the_module ::string ::isolate ::private ::Delimiter( "a" );
  options.times = 1;
  options.none = the_module ::string ::isolate ::private ::NoneFlag( true );
  let req = options.isolate();
  let exp = ( "abac", Some( "a" ), "" );
  assert_eq!( req, exp );

  /* times - 2 */
  let src = "abaca";
  let mut options = the_module ::string ::isolate_right();
  options.src = the_module ::string ::isolate ::private ::Src( src );
  options.delimiter = the_module ::string ::isolate ::private ::Delimiter( "a" );
  options.times = 2;
  options.none = the_module ::string ::isolate ::private ::NoneFlag( true );
  let req = options.isolate();
  let exp = ( "ab", Some( "a" ), "ca" );
  assert_eq!( req, exp );

  /* times - 3 */
  let src = "abaca";
  let mut options = the_module ::string ::isolate_right();
  options.src = the_module ::string ::isolate ::private ::Src( src );
  options.delimiter = the_module ::string ::isolate ::private ::Delimiter( "a" );
  options.times = 3;
  options.none = the_module ::string ::isolate ::private ::NoneFlag( true );
  let req = options.isolate();
  let exp = ( "", Some( "a" ), "baca" );
  assert_eq!( req, exp );

  /* times - 4 */
  let src = "abaca";
  let mut options = the_module ::string ::isolate_right();
  options.src = the_module ::string ::isolate ::private ::Src( src );
  options.delimiter = the_module ::string ::isolate ::private ::Delimiter( "a" );
  options.times = 4;
  options.none = the_module ::string ::isolate ::private ::NoneFlag( true );
  let req = options.isolate();
  let exp = ( "abaca", None, "" );
  assert_eq!( req, exp );
}
