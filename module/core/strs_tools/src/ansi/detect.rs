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
pub fn is_reset_code( code : &str ) -> bool
{
  // Reset codes: \x1b[0m or \x1b[m
  code == "\x1b[0m" || code == "\x1b[m"
}

/// Check if an ANSI code is an SGR (Select Graphic Rendition) sequence.
pub fn is_sgr_code( code : &str ) -> bool
{
  // SGR sequences end with 'm'
  code.ends_with( 'm' ) && code.starts_with( "\x1b[" )
}
