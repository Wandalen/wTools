//! Tests for ANSI truncation with boundary detection.

#[ cfg( feature = "ansi" ) ]
use strs_tools::ansi::{ truncate_if_needed, truncate_lines, TruncateOptions };

#[ test ]
#[ cfg( feature = "ansi" ) ]
fn truncate_if_needed_exact_width_no_truncation()
{
  let opts = TruncateOptions::new( 5 ).with_suffix( "→" );
  let result = truncate_if_needed( "hello", 5, &opts );
  assert_eq!( result, "hello" );
}

#[ test ]
#[ cfg( feature = "ansi" ) ]
fn truncate_if_needed_exceeds_width_truncates()
{
  let opts = TruncateOptions::new( 5 ).with_suffix( "→" );
  let result = truncate_if_needed( "hello world", 5, &opts );
  assert!( result.contains( "→" ) );
}

#[ test ]
#[ cfg( feature = "ansi" ) ]
fn truncate_if_needed_under_width_no_truncation()
{
  let opts = TruncateOptions::new( 10 ).with_suffix( "→" );
  let result = truncate_if_needed( "short", 10, &opts );
  assert_eq!( result, "short" );
}

#[ test ]
#[ cfg( feature = "ansi" ) ]
fn truncate_if_needed_ansi_exact_width()
{
  let opts = TruncateOptions::new( 5 ).with_suffix( "→" );
  let text = "\x1b[31mhello\x1b[0m";
  let result = truncate_if_needed( text, 5, &opts );
  assert!( result.contains( "hello" ) );
  assert!( result.contains( "\x1b[31m" ) );
}

#[ test ]
#[ cfg( feature = "ansi" ) ]
fn truncate_lines_mixed_lengths()
{
  let text = "short\nthis is a very long line\nmedium";
  let opts = TruncateOptions::new( 10 ).with_suffix( "→" );
  let ( result, truncated ) = truncate_lines( text, 10, &opts );

  assert!( truncated );
  let lines : Vec< &str > = result.lines().collect();
  assert_eq!( lines.len(), 3 );
  assert!( lines[ 0 ].contains( "short" ) );
  assert!( lines[ 1 ].contains( "→" ) );
  assert!( lines[ 2 ].contains( "medium" ) );
}

#[ test ]
#[ cfg( feature = "ansi" ) ]
fn truncate_lines_all_fit_no_truncation()
{
  let text = "a\nbb\nccc";
  let opts = TruncateOptions::new( 10 ).with_suffix( "→" );
  let ( result, truncated ) = truncate_lines( text, 10, &opts );

  assert!( !truncated );
  assert_eq!( result, "a\nbb\nccc" );
}
