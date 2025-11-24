//! ANSI detection utilities
//!
//! Provides utilities for detecting ANSI escape sequences in text.

use super::{ Segment, parse_segments };

/// Check if text contains any ANSI escape sequences.
///
/// # Arguments
///
/// * `text` - Input text to check
///
/// # Returns
///
/// `true` if text contains ANSI escape sequences, `false` otherwise.
///
/// # Examples
///
/// ```rust
/// # #[ cfg( feature = "ansi" ) ]
/// # {
/// use strs_tools::ansi::has_ansi;
///
/// assert!( has_ansi( "\x1b[31mred\x1b[0m" ) );
/// assert!( !has_ansi( "plain text" ) );
/// # }
/// ```
///
/// # Performance
///
/// - Early exit on first ANSI code found
/// - Worst case O(n) if no ANSI codes present
pub fn has_ansi( text : &str ) -> bool
{
  // Fast path: check for ESC character
  if !text.contains( '\x1b' )
  {
    return false;
  }

  // Verify it's actually a CSI sequence
  parse_segments( text )
    .iter()
    .any( | seg | matches!( seg, Segment::Ansi( _ ) ) )
}

/// Detect unclosed ANSI formatting (non-reset sequences without terminating reset).
///
/// Useful for detecting when ANSI formatting "leaks" beyond intended scope,
/// which can cause visual corruption in terminal output.
///
/// # Arguments
///
/// * `text` - Input text to check
///
/// # Returns
///
/// `true` if text has ANSI formatting that isn't properly reset.
///
/// # Examples
///
/// ```rust
/// # #[ cfg( feature = "ansi" ) ]
/// # {
/// use strs_tools::ansi::has_unclosed_formatting;
///
/// // Properly closed
/// assert!( !has_unclosed_formatting( "\x1b[31mred\x1b[0m" ) );
///
/// // Unclosed - no reset after color
/// assert!( has_unclosed_formatting( "\x1b[31mred" ) );
///
/// // Plain text - no formatting
/// assert!( !has_unclosed_formatting( "plain" ) );
/// # }
/// ```
///
/// # Implementation Notes
///
/// This function tracks whether non-reset SGR sequences are followed by
/// a reset sequence (`\x1b[0m` or `\x1b[m`). It doesn't track individual
/// attributes, only whether formatting is "active" at end of string.
pub fn has_unclosed_formatting( text : &str ) -> bool
{
  let mut formatting_active = false;

  for segment in parse_segments( text )
  {
    if let Segment::Ansi( code ) = segment
    {
      if is_reset_code( code )
      {
        formatting_active = false;
      }
      else if is_sgr_code( code )
      {
        formatting_active = true;
      }
    }
  }

  formatting_active
}

/// Check if an ANSI code is a reset sequence.
fn is_reset_code( code : &str ) -> bool
{
  // Reset codes: \x1b[0m or \x1b[m
  code == "\x1b[0m" || code == "\x1b[m"
}

/// Check if an ANSI code is an SGR (Select Graphic Rendition) sequence.
fn is_sgr_code( code : &str ) -> bool
{
  // SGR sequences end with 'm'
  code.ends_with( 'm' ) && code.starts_with( "\x1b[" )
}

#[ cfg( test ) ]
mod tests
{
  use super::*;

  // ==================== has_ansi tests ====================

  #[ test ]
  fn has_ansi_empty()
  {
    assert!( !has_ansi( "" ) );
  }

  #[ test ]
  fn has_ansi_plain_text()
  {
    assert!( !has_ansi( "hello world" ) );
    assert!( !has_ansi( "no colors here" ) );
  }

  #[ test ]
  fn has_ansi_with_ansi()
  {
    assert!( has_ansi( "\x1b[31mred\x1b[0m" ) );
    assert!( has_ansi( "\x1b[0m" ) );
    assert!( has_ansi( "prefix \x1b[32mgreen" ) );
  }

  #[ test ]
  fn has_ansi_lone_escape()
  {
    // Lone ESC without '[' is not a valid ANSI CSI sequence
    assert!( !has_ansi( "before\x1bafter" ) );
  }

  #[ test ]
  fn has_ansi_rgb_color()
  {
    assert!( has_ansi( "\x1b[38;2;255;128;0morange\x1b[0m" ) );
  }

  // ==================== has_unclosed_formatting tests ====================

  #[ test ]
  fn unclosed_empty()
  {
    assert!( !has_unclosed_formatting( "" ) );
  }

  #[ test ]
  fn unclosed_plain_text()
  {
    assert!( !has_unclosed_formatting( "hello world" ) );
  }

  #[ test ]
  fn unclosed_properly_closed()
  {
    assert!( !has_unclosed_formatting( "\x1b[31mred\x1b[0m" ) );
    assert!( !has_unclosed_formatting( "\x1b[1;31mbold\x1b[0m" ) );
  }

  #[ test ]
  fn unclosed_not_closed()
  {
    assert!( has_unclosed_formatting( "\x1b[31mred" ) );
    assert!( has_unclosed_formatting( "\x1b[1m" ) );
  }

  #[ test ]
  fn unclosed_reset_only()
  {
    // Reset without prior formatting is fine
    assert!( !has_unclosed_formatting( "\x1b[0m" ) );
    assert!( !has_unclosed_formatting( "\x1b[m" ) );
  }

  #[ test ]
  fn unclosed_multiple_sequences()
  {
    // Closed in middle, unclosed at end
    assert!( has_unclosed_formatting( "\x1b[31mred\x1b[0m\x1b[32mgreen" ) );

    // All closed
    assert!( !has_unclosed_formatting( "\x1b[31mred\x1b[0m\x1b[32mgreen\x1b[0m" ) );
  }

  #[ test ]
  fn unclosed_reset_abbreviation()
  {
    // \x1b[m is equivalent to \x1b[0m
    assert!( !has_unclosed_formatting( "\x1b[31mred\x1b[m" ) );
  }

  // ==================== helper function tests ====================

  #[ test ]
  fn is_reset_code_test()
  {
    assert!( is_reset_code( "\x1b[0m" ) );
    assert!( is_reset_code( "\x1b[m" ) );
    assert!( !is_reset_code( "\x1b[31m" ) );
    assert!( !is_reset_code( "\x1b[1m" ) );
  }

  #[ test ]
  fn is_sgr_code_test()
  {
    assert!( is_sgr_code( "\x1b[0m" ) );
    assert!( is_sgr_code( "\x1b[31m" ) );
    assert!( is_sgr_code( "\x1b[1;31;44m" ) );
    assert!( is_sgr_code( "\x1b[38;2;255;128;0m" ) );
    // Non-SGR CSI sequences
    assert!( !is_sgr_code( "\x1b[2J" ) ); // Clear screen
    assert!( !is_sgr_code( "\x1b[H" ) );  // Cursor home
  }
}
