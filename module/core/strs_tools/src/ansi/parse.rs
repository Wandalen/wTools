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
