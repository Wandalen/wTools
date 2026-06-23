#[ allow( unused_imports ) ]
use super :: *;

//

#[ cfg(feature = "std") ]
#[ test ]
fn basic()
{
  use the_module ::string ::indentation;

  /* test.case( "basic" ) */
  {
  let src = "a\nbc";
  let exp = "---a\n---bc";
  let got = indentation("---", src, "");
  assert_eq!(got, exp);
 }

  /* test.case( "empty string" ) */
  {
  let src = "";
  let exp = "";
  let got = indentation("---", src, "");
  assert_eq!(got, exp);
 }

  /* test.case( "two strings" ) */
  {
  let src = "a\nb";
  let exp = "---a+++\n---b+++";
  let got = indentation("---", src, "+++");
  assert_eq!(got, exp);
 }

  /* test.case( "last empty" ) */
  {
  let src = "a\n";
  let exp = "---a+++\n---+++";
  let got = indentation("---", src, "+++");
  // println!( "got: '{}'", got );
  assert_eq!(got, exp);
 }

  /* test.case( "first empty" ) */
  {
  let src = "\nb";
  let exp = "---+++\n---b+++";
  let got = indentation("---", src, "+++");
  // println!( "got: '{}'", got );
  assert_eq!(got, exp);
 }

  /* test.case( "two empty string" ) */
  {
  let src = "\n";
  let exp = "---+++\n---+++";
  let got = indentation("---", src, "+++");
  // println!( "got: '{}'", got );
  assert_eq!(got, exp);
 }
}

/// Multiple consecutive newlines produce one prefixed empty line per newline boundary.
#[ cfg(feature = "std") ]
#[ test ]
fn multiple_newlines_only()
{
  use the_module::string::indentation;

  let src = "\n\n\n";
  let got = indentation( ">>", src, "" );
  let lines : Vec< &str > = got.split( '\n' ).collect();
  assert_eq!( lines.len(), 4, "3 newlines split into 4 lines, got {}", lines.len() );
  for ( i, line ) in lines.iter().enumerate()
  {
    assert_eq!( *line, ">>", "line {i} should be just the prefix, got '{line}'" );
  }
}

/// Long prefix that exceeds typical terminal width still applies correctly.
#[ cfg(feature = "std") ]
#[ test ]
fn long_prefix_boundary()
{
  use the_module::string::indentation;

  let long_prefix = "X".repeat( 200 );
  let src = "a\nb";
  let got = indentation( &long_prefix, src, "" );
  let lines : Vec< &str > = got.split( '\n' ).collect();
  assert_eq!( lines.len(), 2, "two source lines, got {}", lines.len() );
  assert_eq!( lines[ 0 ], format!( "{long_prefix}a" ), "first line with long prefix" );
  assert_eq!( lines[ 1 ], format!( "{long_prefix}b" ), "second line with long prefix" );
}
