//! Visual length calculation and padding utilities
//!
//! Provides utilities for calculating visible character count
//! and padding text while respecting ANSI escape sequences.

extern crate alloc;

use alloc::string::{ String, ToString };
use alloc::format;
use super::{ Segment, parse_segments };

/// Calculate the visible character count (char-based, Tier 1).
///
/// Counts characters that would be visible in terminal output,
/// ignoring ANSI escape sequences.
///
/// # Arguments
///
/// * `text` - Input text potentially containing ANSI escape sequences
///
/// # Returns
///
/// Number of visible characters (Unicode codepoints, not graphemes).
///
/// # Examples
///
/// ```rust
/// # #[ cfg( feature = "ansi" ) ]
/// # {
/// use strs_tools::ansi::visual_len;
///
/// assert_eq!( visual_len( "hello" ), 5 );
/// assert_eq!( visual_len( "\x1b[31mred\x1b[0m" ), 3 );
/// assert_eq!( visual_len( "\x1b[1;31mbold red\x1b[0m" ), 8 );
/// # }
/// ```
///
/// # Performance
///
/// - Time complexity: O(n)
/// - Benchmark: ~100ns/KB on modern hardware
pub fn visual_len( text : &str ) -> usize
{
  parse_segments( text )
    .iter()
    .filter_map( | seg | match seg
    {
      Segment::Text( t ) => Some( t.chars().count() ),
      Segment::Ansi( _ ) => None,
    })
    .sum()
}

/// Calculate the visible character count (grapheme-based, Tier 2).
///
/// Uses Unicode grapheme clusters for accurate counting of
/// CJK characters, emoji, and combining marks.
///
/// # Arguments
///
/// * `text` - Input text potentially containing ANSI escape sequences
///
/// # Returns
///
/// Number of visible grapheme clusters.
///
/// # Examples
///
/// ```rust
/// # #[ cfg( all( feature = "ansi_unicode" ) ) ]
/// # {
/// use strs_tools::ansi::visual_len_unicode;
///
/// // Emoji with skin tone modifier counts as 1 grapheme
/// assert_eq!( visual_len_unicode( "👋🏽" ), 1 );
/// // CJK characters count correctly
/// assert_eq!( visual_len_unicode( "日本語" ), 3 );
/// # }
/// ```
#[ cfg( feature = "ansi_unicode" ) ]
pub fn visual_len_unicode( text : &str ) -> usize
{
  use unicode_segmentation::UnicodeSegmentation;

  parse_segments( text )
    .iter()
    .filter_map( | seg | match seg
    {
      Segment::Text( t ) => Some( t.graphemes( true ).count() ),
      Segment::Ansi( _ ) => None,
    })
    .sum()
}

/// Calculate terminal display columns after stripping ANSI escapes (char-based, Tier 1).
///
/// Unlike `visual_len` which counts Unicode codepoints, `visual_width` measures
/// terminal display columns. Wide characters (CJK, emoji) occupy 2 columns;
/// combining marks occupy 0 columns.
///
/// # Arguments
///
/// * `text` - Input text potentially containing ANSI escape sequences
///
/// # Returns
///
/// Number of terminal display columns.
///
/// # Examples
///
/// ```rust
/// # #[ cfg( feature = "ansi" ) ]
/// # {
/// use strs_tools::ansi::visual_width;
///
/// assert_eq!( visual_width( "hello" ), 5 );
/// assert_eq!( visual_width( "你好" ), 4 );  // CJK = 2 columns each
/// assert_eq!( visual_width( "\x1b[31mred\x1b[0m" ), 3 );
/// # }
/// ```
pub fn visual_width( text : &str ) -> usize
{
  use unicode_width::UnicodeWidthChar;

  parse_segments( text )
    .iter()
    .filter_map( | seg | match seg
    {
      Segment::Text( t ) => Some(
        t.chars().map( | c | c.width().unwrap_or( 0 ) ).sum::< usize >()
      ),
      Segment::Ansi( _ ) => None,
    })
    .sum()
}

/// Calculate terminal display columns using grapheme clusters (Tier 2).
///
/// Like `visual_width` but processes grapheme clusters rather than
/// individual codepoints, giving accurate results for combining marks
/// and complex emoji sequences.
///
/// # Arguments
///
/// * `text` - Input text potentially containing ANSI escape sequences
///
/// # Returns
///
/// Number of terminal display columns using grapheme-cluster boundaries.
///
/// # Examples
///
/// ```rust
/// # #[ cfg( all( feature = "ansi_unicode" ) ) ]
/// # {
/// use strs_tools::ansi::visual_width_unicode;
///
/// assert_eq!( visual_width_unicode( "e\u{0301}" ), 1 );
/// assert_eq!( visual_width_unicode( "日本語" ), 6 );
/// # }
/// ```
#[ cfg( feature = "ansi_unicode" ) ]
pub fn visual_width_unicode( text : &str ) -> usize
{
  use unicode_segmentation::UnicodeSegmentation;
  use unicode_width::UnicodeWidthStr;

  parse_segments( text )
    .iter()
    .filter_map( | seg | match seg
    {
      Segment::Text( t ) => Some(
        t.graphemes( true ).map( UnicodeWidthStr::width ).sum::< usize >()
      ),
      Segment::Ansi( _ ) => None,
    })
    .sum()
}

/// Pad text to target display width while respecting ANSI codes and wide Unicode characters.
///
/// Uses display width (terminal columns) instead of character count.
/// Correctly handles:
/// - Wide characters (CJK, emoji): 2 display width
/// - Normal characters (ASCII, Cyrillic): 1 display width
/// - Zero-width characters (combining marks): 0 display width
/// - ANSI escape sequences: 0 display width (filtered out)
///
/// # Arguments
///
/// * `text` - Input text potentially containing ANSI escape sequences
/// * `target_width` - Desired display width in terminal columns
/// * `align_right` - If true, pad on the left; if false, pad on the right
///
/// # Returns
///
/// Padded string with correct display width for terminal alignment.
///
/// # Examples
///
/// ```rust
/// # #[ cfg( feature = "ansi" ) ]
/// # {
/// use strs_tools::ansi::pad_to_width;
///
/// // Left-aligned (pad right)
/// assert_eq!( pad_to_width( "hi", 5, false ), "hi   " );
///
/// // Right-aligned (pad left)
/// assert_eq!( pad_to_width( "hi", 5, true ), "   hi" );
///
/// // With ANSI codes - padding doesn't count escape sequences
/// assert_eq!(
///   pad_to_width( "\x1b[31mhi\x1b[0m", 5, false ),
///   "\x1b[31mhi\x1b[0m   "
/// );
///
/// // CJK characters (wide, 2 display width per char)
/// let padded = pad_to_width( "日本語", 10, false );  // 3 chars, 6 display width
/// // Result: "日本語    " (6 + 4 spaces = 10 display width)
/// # }
/// ```
///
/// # Fix(issue-003)
///
/// Root cause: Previous implementation mixed character-count-based padding
/// with Rust's display-width-based formatting (`{:<N}`), causing
/// misalignment with wide Unicode characters (CJK, emoji).
///
/// Pitfall: Always use display width for terminal alignment, not char count.
/// Display width ≠ char count ≠ byte count for Unicode.
/// CJK/emoji have display width = 2, not 1.
pub fn pad_to_width( text : &str, target_width : usize, align_right : bool ) -> String
{
  use unicode_width::UnicodeWidthStr;

  // Calculate display width of visible text (excluding ANSI codes)
  let visible_text : String = parse_segments( text )
    .iter()
    .filter_map( | seg | match seg
    {
      Segment::Text( t ) => Some( *t ),
      Segment::Ansi( _ ) => None,
    })
    .collect();

  let current_width = visible_text.width();

  if current_width >= target_width
  {
    return text.to_string();
  }

  let padding = target_width - current_width;
  let spaces : String = " ".repeat( padding );

  if align_right
  {
    format!( "{}{}", spaces, text )
  }
  else
  {
    format!( "{}{}", text, spaces )
  }
}
