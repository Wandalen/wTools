//! Helper utilities for ANSI-aware string handling
//!
//! ## Why ANSI-Aware Functions Are Needed
//!
//! ### The Problem: Color Codes Break Alignment
//!
//! ANSI escape sequences (color codes) affect byte length but not visual length, breaking
//! table and tree alignment when columns contain colored text.
//!
//! **Example Problem**:
//! ```text
//! // String: "\x1b[36mHello\x1b[0m"
//! byte_len: 17 bytes (escape codes included)
//! visual_len: 5 chars (only "Hello" is visible)
//! ```
//!
//! If we use standard `str.len()` for column width calculation, colored cells get
//! way too much padding:
//!
//! ```text
//! // WITHOUT visual_len (BROKEN):
//! Name              | Value
//! ------------------+------------------
//! [cyan]Hello[reset]                  | World  // Too much space after "Hello"!
//!
//! // WITH visual_len (CORRECT):
//! Name  | Value
//! ------+-------
//! Hello | World  // Proper alignment despite color codes
//! ```
//!
//! ### ANSI Escape Sequence Format
//!
//! Color codes follow the pattern: `\x1b[...m`
//! - Start: `\x1b[` (ESC + left bracket)
//! - Parameters: Numbers and semicolons (e.g., `31` for red, `1;36` for bold cyan)
//! - End: `m`
//!
//! Common examples:
//! - Red text: `\x1b[31m`
//! - Reset: `\x1b[0m`
//! - Bold cyan: `\x1b[1;36m`
//!
//! ### Solution: `visual_len()` and `pad_to_width()`
//!
//! **`visual_len()`**: Counts only visible characters by skipping escape sequences
//! **`pad_to_width()`**: Pads based on visual length, preserving ANSI codes
//!
//! ### Edge Cases Handled
//!
//! 1. **Nested escape sequences**: (uncommon but supported)
//!    - Example: `\x1b[31m\x1b[1mBold Red\x1b[0m`
//! 2. **Malformed sequences**: Counted as visible characters (graceful degradation)
//!    - Example: `\x1b[31Hello` (missing 'm' terminator)
//! 3. **Empty strings**: `visual_len("") == 0` (no special handling needed)
//! 4. **No ANSI codes**: Fast path, same as `str.chars().count()`
//!
//! ### Historical Context
//!
//! Added in v0.1.0 during initial linter improvements. Before this, colored header
//! rows in tables were completely misaligned, making tools like `cargo list` output
//! unreadable.
//!
//! ### Testing
//!
//! See integration tests for real-world scenarios with colored output. All formatters
//! use these functions throughout for consistent ANSI-aware behavior.

/// Calculate visual length of string, excluding ANSI escape sequences
///
/// ANSI color codes are invisible in terminal output but count toward string length.
/// This function calculates the actual visible character count for proper alignment.
///
/// # Examples
///
/// ```
/// use tree_fmt::visual_len;
///
/// assert_eq!( visual_len( "hello" ), 5 );
/// assert_eq!( visual_len( "\x1b[31mred\x1b[0m" ), 3 ); // "red" in red color
/// ```
pub fn visual_len( text : &str ) -> usize
{
  let mut len = 0;
  let mut in_escape = false;

  for ch in text.chars()
  {
    if ch == '\x1b'
    {
      in_escape = true;
    }
    else if in_escape
    {
      if ch == 'm'
      {
        in_escape = false;
      }
    }
    else
    {
      len += 1;
    }
  }

  len
}

/// Pad string to target visual width, accounting for ANSI escape sequences
///
/// # Arguments
///
/// * `text` - String to pad (may contain ANSI codes)
/// * `target_width` - Target visual width (excluding ANSI codes)
/// * `align_right` - Right-align if true, left-align if false
///
/// # Examples
///
/// ```
/// use tree_fmt::pad_to_width;
///
/// assert_eq!( pad_to_width( "hello", 10, false ), "hello     " );
/// assert_eq!( pad_to_width( "\x1b[31mred\x1b[0m", 10, false ), "\x1b[31mred\x1b[0m       " );
/// ```
pub fn pad_to_width( text : &str, target_width : usize, align_right : bool ) -> String
{
  let visible_len = visual_len( text );

  if visible_len >= target_width
  {
    return text.to_string();
  }

  let padding_needed = target_width - visible_len;
  let padding = " ".repeat( padding_needed );

  if align_right
  {
    format!( "{padding}{text}" )
  }
  else
  {
    format!( "{text}{padding}" )
  }
}
