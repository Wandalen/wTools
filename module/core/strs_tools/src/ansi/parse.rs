//! ANSI escape sequence parsing
//!
//! Provides utilities for parsing text into ANSI and text segments.

extern crate alloc;

use alloc::vec::Vec;
use super::Segment;

/// Parse text into ANSI escape sequences and visible text segments.
///
/// # Arguments
///
/// * `text` - Input text potentially containing ANSI escape sequences
///
/// # Returns
///
/// Vector of segments, each either an ANSI escape code or visible text.
///
/// # Examples
///
/// ```rust
/// # #[ cfg( feature = "ansi" ) ]
/// # {
/// use strs_tools::ansi::{ parse_segments, Segment };
///
/// let segments = parse_segments( "\x1b[31mred\x1b[0m" );
/// assert_eq!( segments.len(), 3 );
/// assert_eq!( segments[ 0 ], Segment::Ansi( "\x1b[31m" ) );
/// assert_eq!( segments[ 1 ], Segment::Text( "red" ) );
/// assert_eq!( segments[ 2 ], Segment::Ansi( "\x1b[0m" ) );
/// # }
/// ```
///
/// # Performance
///
/// - Time complexity: O(n) where n is input length
/// - Space complexity: O(n) for output segments
/// - Benchmark: ~1µs/KB on modern hardware
pub fn parse_segments< 'a >( text : &'a str ) -> Vec< Segment< 'a > >
{
  if text.is_empty()
  {
    return Vec::new();
  }

  let mut segments = Vec::new();
  let mut chars = text.char_indices().peekable();
  let mut text_start : Option< usize > = None;

  while let Some( ( idx, ch ) ) = chars.next()
  {
    // Check for ANSI escape sequence start: ESC (0x1b) followed by '['
    if ch == '\x1b'
    {
      // Look for '[' to confirm CSI sequence
      if let Some( &( _, '[' ) ) = chars.peek()
      {
        // Flush any pending text segment (only when we confirm valid ANSI)
        if let Some( start ) = text_start.take()
        {
          segments.push( Segment::Text( &text[ start..idx ] ) );
        }

        let ansi_start = idx;
        chars.next(); // consume '['

        // Find end of ANSI sequence (letter that terminates SGR)
        let mut ansi_end = idx + 2; // at least ESC[
        while let Some( &( end_idx, c ) ) = chars.peek()
        {
          ansi_end = end_idx + c.len_utf8();
          chars.next();

          // SGR sequences end with 'm', but other CSI sequences end with letters
          if c.is_ascii_alphabetic()
          {
            break;
          }
        }

        segments.push( Segment::Ansi( &text[ ansi_start..ansi_end ] ) );
      }
      else
      {
        // Lone ESC without '[' - treat as text
        if text_start.is_none()
        {
          text_start = Some( idx );
        }
      }
    }
    else
    {
      // Regular character - start or continue text segment
      if text_start.is_none()
      {
        text_start = Some( idx );
      }
    }
  }

  // Flush remaining text segment
  if let Some( start ) = text_start
  {
    segments.push( Segment::Text( &text[ start.. ] ) );
  }

  segments
}

#[ cfg( test ) ]
mod tests
{
  use super::*;

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
}
