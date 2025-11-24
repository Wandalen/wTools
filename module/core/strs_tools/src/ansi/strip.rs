//! ANSI stripping utilities
//!
//! Provides utilities for removing ANSI escape sequences from text.

extern crate alloc;

use alloc::string::String;
use super::{ Segment, parse_segments };

/// Remove all ANSI escape sequences from text.
///
/// Returns the visible text content with all ANSI codes stripped.
///
/// # Arguments
///
/// * `text` - Input text potentially containing ANSI escape sequences
///
/// # Returns
///
/// String with all ANSI escape sequences removed.
///
/// # Examples
///
/// ```rust
/// # #[ cfg( feature = "ansi" ) ]
/// # {
/// use strs_tools::ansi::strip;
///
/// assert_eq!( strip( "\x1b[31mred\x1b[0m" ), "red" );
/// assert_eq!( strip( "\x1b[1;31mbold red\x1b[0m normal" ), "bold red normal" );
/// assert_eq!( strip( "plain text" ), "plain text" );
/// # }
/// ```
///
/// # Performance
///
/// - Time complexity: O(n)
/// - Space complexity: O(n) for output string
/// - Benchmark: ~500ns/KB on modern hardware
pub fn strip( text : &str ) -> String
{
  parse_segments( text )
    .iter()
    .filter_map( | seg | match seg
    {
      Segment::Text( t ) => Some( *t ),
      Segment::Ansi( _ ) => None,
    })
    .collect()
}

#[ cfg( test ) ]
mod tests
{
  use super::*;

  #[ test ]
  fn strip_empty()
  {
    assert_eq!( strip( "" ), "" );
  }

  #[ test ]
  fn strip_plain_text()
  {
    assert_eq!( strip( "hello world" ), "hello world" );
  }

  #[ test ]
  fn strip_ansi_only()
  {
    assert_eq!( strip( "\x1b[31m" ), "" );
    assert_eq!( strip( "\x1b[0m" ), "" );
    assert_eq!( strip( "\x1b[1;31;44m" ), "" );
  }

  #[ test ]
  fn strip_simple_colored_text()
  {
    assert_eq!( strip( "\x1b[31mred\x1b[0m" ), "red" );
  }

  #[ test ]
  fn strip_complex_formatting()
  {
    assert_eq!(
      strip( "\x1b[1;31mbold red\x1b[0m normal" ),
      "bold red normal"
    );
  }

  #[ test ]
  fn strip_multiple_colors()
  {
    assert_eq!(
      strip( "\x1b[31mred\x1b[32mgreen\x1b[34mblue\x1b[0m" ),
      "redgreenblue"
    );
  }

  #[ test ]
  fn strip_rgb_colors()
  {
    assert_eq!(
      strip( "\x1b[38;2;255;128;0morange\x1b[0m" ),
      "orange"
    );
  }

  #[ test ]
  fn strip_unicode()
  {
    assert_eq!(
      strip( "\x1b[33m日本語\x1b[0m" ),
      "日本語"
    );
  }

  #[ test ]
  fn strip_preserves_whitespace()
  {
    assert_eq!(
      strip( "\x1b[31m  spaced  \x1b[0m" ),
      "  spaced  "
    );
  }

  #[ test ]
  fn strip_consecutive_ansi()
  {
    assert_eq!(
      strip( "\x1b[1m\x1b[31m\x1b[44mtext\x1b[0m" ),
      "text"
    );
  }
}
