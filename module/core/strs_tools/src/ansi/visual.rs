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

/// Pad text to target width while respecting ANSI codes.
///
/// Adds spaces to reach the target width based on visible character count.
/// ANSI escape sequences are preserved but don't count toward width.
///
/// # Arguments
///
/// * `text` - Input text potentially containing ANSI escape sequences
/// * `target_width` - Desired visible width
/// * `align_right` - If true, pad on the left; if false, pad on the right
///
/// # Returns
///
/// Padded string with ANSI codes preserved.
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
/// # }
/// ```
pub fn pad_to_width( text : &str, target_width : usize, align_right : bool ) -> String
{
  let current_width = visual_len( text );

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

#[ cfg( test ) ]
mod tests
{
  use super::*;

  // ==================== visual_len tests ====================

  #[ test ]
  fn visual_len_empty()
  {
    assert_eq!( visual_len( "" ), 0 );
  }

  #[ test ]
  fn visual_len_plain_text()
  {
    assert_eq!( visual_len( "hello" ), 5 );
    assert_eq!( visual_len( "hello world" ), 11 );
  }

  #[ test ]
  fn visual_len_ansi_only()
  {
    assert_eq!( visual_len( "\x1b[31m" ), 0 );
    assert_eq!( visual_len( "\x1b[0m" ), 0 );
    assert_eq!( visual_len( "\x1b[1;31;44m" ), 0 );
  }

  #[ test ]
  fn visual_len_ansi_with_text()
  {
    assert_eq!( visual_len( "\x1b[31mred\x1b[0m" ), 3 );
    assert_eq!( visual_len( "\x1b[1;31mbold red\x1b[0m" ), 8 );
  }

  #[ test ]
  fn visual_len_multiple_ansi()
  {
    assert_eq!( visual_len( "\x1b[1m\x1b[31mtest\x1b[0m" ), 4 );
  }

  #[ test ]
  fn visual_len_unicode_codepoints()
  {
    // Tier 1: char-based - counts codepoints
    assert_eq!( visual_len( "日本語" ), 3 );
    // Emoji may be multiple codepoints in char-based counting
    assert_eq!( visual_len( "🎉" ), 1 );
  }

  // ==================== pad_to_width tests ====================

  #[ test ]
  fn pad_left_align()
  {
    assert_eq!( pad_to_width( "hi", 5, false ), "hi   " );
    assert_eq!( pad_to_width( "test", 10, false ), "test      " );
  }

  #[ test ]
  fn pad_right_align()
  {
    assert_eq!( pad_to_width( "hi", 5, true ), "   hi" );
    assert_eq!( pad_to_width( "test", 10, true ), "      test" );
  }

  #[ test ]
  fn pad_no_change_when_equal_or_larger()
  {
    assert_eq!( pad_to_width( "hello", 5, false ), "hello" );
    assert_eq!( pad_to_width( "hello world", 5, false ), "hello world" );
  }

  #[ test ]
  fn pad_with_ansi()
  {
    let result = pad_to_width( "\x1b[31mhi\x1b[0m", 5, false );
    assert_eq!( result, "\x1b[31mhi\x1b[0m   " );

    let result = pad_to_width( "\x1b[31mhi\x1b[0m", 5, true );
    assert_eq!( result, "   \x1b[31mhi\x1b[0m" );
  }

  #[ test ]
  fn pad_empty_string()
  {
    assert_eq!( pad_to_width( "", 3, false ), "   " );
    assert_eq!( pad_to_width( "", 3, true ), "   " );
  }

  // ==================== visual_len_unicode tests ====================

  #[ cfg( feature = "ansi_unicode" ) ]
  mod unicode_tests
  {
    use crate::ansi::visual::visual_len_unicode;

    #[ test ]
    fn grapheme_emoji_with_modifier()
    {
      // Emoji with skin tone modifier = 1 grapheme
      assert_eq!( visual_len_unicode( "👋🏽" ), 1 );
    }

    #[ test ]
    fn grapheme_flag_emoji()
    {
      // Flag emoji (2 regional indicators = 1 grapheme)
      assert_eq!( visual_len_unicode( "🇺🇸" ), 1 );
    }

    #[ test ]
    fn grapheme_combining_marks()
    {
      // e + combining acute accent = 1 grapheme
      assert_eq!( visual_len_unicode( "e\u{0301}" ), 1 );
    }

    #[ test ]
    fn grapheme_cjk()
    {
      assert_eq!( visual_len_unicode( "日本語" ), 3 );
    }

    #[ test ]
    fn grapheme_with_ansi()
    {
      assert_eq!( visual_len_unicode( "\x1b[33m日本語\x1b[0m" ), 3 );
      assert_eq!( visual_len_unicode( "\x1b[31m👋🏽\x1b[0m" ), 1 );
    }
  }
}
