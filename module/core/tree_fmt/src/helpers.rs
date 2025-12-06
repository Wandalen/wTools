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
//! ### Implementation Note
//!
//! These utilities are re-exported from `strs_tools::ansi` for centralized ANSI
//! handling across the wTools ecosystem. The implementations provide:
//! - Tier 1: Zero-dependency char-based counting (used here)
//! - Tier 2: Unicode grapheme-aware counting (available via `strs_tools::ansi_unicode`)
//!
//! ### Testing
//!
//! See integration tests for real-world scenarios with colored output. All formatters
//! use these functions throughout for consistent ANSI-aware behavior.

// Re-export ANSI utilities from strs_tools for backward compatibility
pub use strs_tools::ansi::visual_len;
pub use strs_tools::ansi::pad_to_width;

/// Truncate text to maximum visual width with ANSI code preservation
///
/// Truncates text to fit within `max_width` visual characters, appending
/// `marker` if truncation occurs. Preserves ANSI color codes in the output.
///
/// For multiline text (containing `\n`), each line is truncated independently.
///
/// # Arguments
///
/// * `text` - Input text (may contain ANSI codes and newlines)
/// * `max_width` - Maximum visual width per line (ANSI codes don't count)
/// * `marker` - String to append when truncated (default: "...")
///
/// # Returns
///
/// Truncated string with preserved ANSI codes and marker appended.
/// If text fits within `max_width`, returns original text unchanged.
/// For multiline text, each line is truncated independently.
///
/// # Examples
///
/// ```
/// use tree_fmt::truncate_cell;
///
/// // Basic truncation
/// let result = truncate_cell( "Very long text here", 10, "..." );
/// assert_eq!( result, "Very lo..." );
///
/// // Multiline truncation (per-line)
/// let result = truncate_cell( "Long line 1\nLong line 2", 8, "..." );
/// assert!( result.contains( "..." ) );  // Both lines truncated
///
/// // ANSI codes preserved
/// let colored = "\x1b[31mRed text\x1b[0m";
/// let result = truncate_cell( colored, 5, "..." );
/// assert!( result.contains( "\x1b[31m" ) );  // Color preserved
/// ```
///
/// # Implementation Details
///
/// For single-line text: character-by-character iteration tracking visual
/// position while skipping ANSI escape sequences (`\x1b[...m` pattern).
///
/// For multiline text: splits on `\n`, truncates each line independently,
/// then joins back with `\n`. This ensures proper per-line truncation.
pub fn truncate_cell( text : &str, max_width : usize, marker : &str ) -> String
{
  // Handle multiline cells: truncate each line independently
  if text.contains( '\n' )
  {
    let lines : Vec<&str> = text.lines().collect();
    let truncated_lines : Vec<String> = lines
      .iter()
      .map( |line| truncate_single_line( line, max_width, marker ) )
      .collect();

    return truncated_lines.join( "\n" );
  }

  // Single line: use direct truncation
  truncate_single_line( text, max_width, marker )
}

/// Truncate a single line of text (internal helper)
///
/// Does not handle newlines - use `truncate_cell` for multiline text.
fn truncate_single_line( text : &str, max_width : usize, marker : &str ) -> String
{
  let text_visual_len = visual_len( text );

  // No truncation needed
  if text_visual_len <= max_width
  {
    return text.to_string();
  }

  // Calculate space available for content (reserve space for marker)
  let marker_len = visual_len( marker );
  let content_width = max_width.saturating_sub( marker_len );

  // Build truncated string while preserving ANSI codes
  let mut result = String::new();
  let mut visual_count = 0;
  let mut in_escape = false;

  for ch in text.chars()
  {
    // Track ANSI escape sequences
    if ch == '\x1b'
    {
      in_escape = true;
      result.push( ch );
      continue;
    }

    if in_escape
    {
      result.push( ch );
      if ch == 'm'
      {
        in_escape = false;
      }
      continue;
    }

    // Regular visible character
    if visual_count < content_width
    {
      result.push( ch );
      visual_count += 1;
    }
    else
    {
      // Reached truncation point
      break;
    }
  }

  // Append marker
  result.push_str( marker );

  result
}
