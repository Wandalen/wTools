#[ allow( unused_imports ) ]
use super::*;
use the_module::ansi::{ parse_segments, Segment };

#[ test ]
fn empty_string()
{
  let result = parse_segments( "" );
  assert!( result.is_empty() );
}

#[ test ]
fn plain_text_no_ansi()
{
  let result = parse_segments( "hello world" );
  assert_eq!( result.len(), 1 );
  assert_eq!( result[ 0 ], Segment::Text( "hello world" ) );
}

#[ test ]
fn single_ansi_code()
{
  let result = parse_segments( "\x1b[31m" );
  assert_eq!( result.len(), 1 );
  assert_eq!( result[ 0 ], Segment::Ansi( "\x1b[31m" ) );
}

#[ test ]
fn ansi_with_text()
{
  let result = parse_segments( "\x1b[31mred\x1b[0m" );
  assert_eq!( result.len(), 3 );
  assert_eq!( result[ 0 ], Segment::Ansi( "\x1b[31m" ) );
  assert_eq!( result[ 1 ], Segment::Text( "red" ) );
  assert_eq!( result[ 2 ], Segment::Ansi( "\x1b[0m" ) );
}

#[ test ]
fn complex_formatting()
{
  // Bold + red, then reset
  let result = parse_segments( "\x1b[1;31mbold red\x1b[0m normal" );
  assert_eq!( result.len(), 4 );
  assert_eq!( result[ 0 ], Segment::Ansi( "\x1b[1;31m" ) );
  assert_eq!( result[ 1 ], Segment::Text( "bold red" ) );
  assert_eq!( result[ 2 ], Segment::Ansi( "\x1b[0m" ) );
  assert_eq!( result[ 3 ], Segment::Text( " normal" ) );
}

#[ test ]
fn text_before_ansi()
{
  let result = parse_segments( "prefix \x1b[32mgreen" );
  assert_eq!( result.len(), 3 );
  assert_eq!( result[ 0 ], Segment::Text( "prefix " ) );
  assert_eq!( result[ 1 ], Segment::Ansi( "\x1b[32m" ) );
  assert_eq!( result[ 2 ], Segment::Text( "green" ) );
}

#[ test ]
fn consecutive_ansi_codes()
{
  let result = parse_segments( "\x1b[1m\x1b[31m" );
  assert_eq!( result.len(), 2 );
  assert_eq!( result[ 0 ], Segment::Ansi( "\x1b[1m" ) );
  assert_eq!( result[ 1 ], Segment::Ansi( "\x1b[31m" ) );
}

#[ test ]
fn rgb_color_code()
{
  // 24-bit RGB color: \x1b[38;2;255;128;0m (orange foreground)
  let result = parse_segments( "\x1b[38;2;255;128;0morange\x1b[0m" );
  assert_eq!( result.len(), 3 );
  assert_eq!( result[ 0 ], Segment::Ansi( "\x1b[38;2;255;128;0m" ) );
  assert_eq!( result[ 1 ], Segment::Text( "orange" ) );
  assert_eq!( result[ 2 ], Segment::Ansi( "\x1b[0m" ) );
}

#[ test ]
fn lone_escape_treated_as_text()
{
  // Lone ESC without '[' should be treated as text
  let result = parse_segments( "before\x1bafter" );
  assert_eq!( result.len(), 1 );
  assert_eq!( result[ 0 ], Segment::Text( "before\x1bafter" ) );
}

#[ test ]
fn unicode_text_with_ansi()
{
  let result = parse_segments( "\x1b[33m日本語\x1b[0m" );
  assert_eq!( result.len(), 3 );
  assert_eq!( result[ 0 ], Segment::Ansi( "\x1b[33m" ) );
  assert_eq!( result[ 1 ], Segment::Text( "日本語" ) );
  assert_eq!( result[ 2 ], Segment::Ansi( "\x1b[0m" ) );
}
