//! ANSI-aware truncation utilities
//!
//! Provides utilities for truncating text while preserving ANSI formatting.

extern crate alloc;

use alloc::string::String;
use super::{ Segment, parse_segments };
use super::strip::strip;

/// Configuration options for ANSI-aware truncation.
///
/// # Examples
///
/// ```rust
/// # #[ cfg( feature = "ansi" ) ]
/// # {
/// use strs_tools::ansi::TruncateOptions;
///
/// let opts = TruncateOptions::new( 10 )
///   .with_suffix( "..." )
///   .with_reset( true );
/// # }
/// ```
#[ derive( Debug, Clone, PartialEq, Eq ) ]
pub struct TruncateOptions
{
  /// Maximum visible width (excluding ANSI codes)
  pub max_width : usize,
  /// Suffix to append when truncated (e.g., "...", "…")
  pub suffix : Option< String >,
  /// Whether to append reset code after truncation
  pub append_reset : bool,
}

impl TruncateOptions
{
  /// Create new truncation options with specified max width.
  ///
  /// # Arguments
  ///
  /// * `max_width` - Maximum visible character width
  ///
  /// # Panics
  ///
  /// Panics if `max_width` is 0 (Architectural Principle: Panic on Invalid Configuration).
  pub fn new( max_width : usize ) -> Self
  {
    assert!( max_width != 0, "TruncateOptions: max_width must be greater than 0" );

    Self
    {
      max_width,
      suffix : None,
      append_reset : false,
    }
  }

  /// Set the suffix to append when truncated.
  ///
  /// # Arguments
  ///
  /// * `suffix` - String to append (e.g., "...", "…")
  pub fn with_suffix( mut self, suffix : impl Into< String > ) -> Self
  {
    self.suffix = Some( suffix.into() );
    self
  }

  /// Set whether to append ANSI reset code after truncation.
  ///
  /// # Arguments
  ///
  /// * `reset` - If true, append `\x1b[0m` after truncation
  pub fn with_reset( mut self, reset : bool ) -> Self
  {
    self.append_reset = reset;
    self
  }
}

impl Default for TruncateOptions
{
  fn default() -> Self
  {
    Self
    {
      max_width : 80,
      suffix : None,
      append_reset : false,
    }
  }
}

/// Truncate text to max width while preserving ANSI codes (char-based, Tier 1).
///
/// # Arguments
///
/// * `text` - Input text potentially containing ANSI escape sequences
/// * `options` - Truncation configuration
///
/// # Returns
///
/// Truncated string with ANSI codes preserved.
///
/// # Examples
///
/// ```rust
/// # #[ cfg( feature = "ansi" ) ]
/// # {
/// use strs_tools::ansi::{ truncate, TruncateOptions };
///
/// let opts = TruncateOptions::new( 5 );
/// assert_eq!( truncate( "hello world", &opts ), "hello" );
///
/// // With suffix
/// let opts = TruncateOptions::new( 8 ).with_suffix( "..." );
/// assert_eq!( truncate( "hello world", &opts ), "hello..." );
///
/// // ANSI codes preserved
/// let opts = TruncateOptions::new( 3 ).with_reset( true );
/// assert_eq!( truncate( "\x1b[31mhello\x1b[0m", &opts ), "\x1b[31mhel\x1b[0m" );
/// # }
/// ```
///
/// # Performance
///
/// - Time complexity: O(n)
/// - Benchmark: ~10µs/KB on modern hardware
pub fn truncate( text : &str, options : &TruncateOptions ) -> String
{
  truncate_internal( text, options, &CharCounter )
}

/// Truncate text to max width while preserving ANSI codes (grapheme-based, Tier 2).
///
/// Uses Unicode grapheme clusters for accurate truncation of
/// CJK characters, emoji, and combining marks.
///
/// # Arguments
///
/// * `text` - Input text potentially containing ANSI escape sequences
/// * `options` - Truncation configuration
///
/// # Returns
///
/// Truncated string with ANSI codes preserved.
///
/// # Examples
///
/// ```rust
/// # #[ cfg( all( feature = "ansi_unicode" ) ) ]
/// # {
/// use strs_tools::ansi::{ truncate_unicode, TruncateOptions };
///
/// let opts = TruncateOptions::new( 3 );
///
/// // Grapheme-aware: emoji with modifier counts as 1
/// assert_eq!( truncate_unicode( "👋🏽ab", &opts ), "👋🏽ab" );
/// # }
/// ```
#[ cfg( feature = "ansi_unicode" ) ]
pub fn truncate_unicode( text : &str, options : &TruncateOptions ) -> String
{
  truncate_internal( text, options, &GraphemeCounter )
}

/// Truncate ANSI text only if it exceeds maximum width.
///
/// Unlike `truncate()` which unconditionally reserves space for the suffix,
/// this function first checks if truncation is needed by comparing
/// `visual_len(text)` with `max_width`. Only truncates when text genuinely
/// exceeds the limit.
///
/// # Bug Fix
///
/// This function prevents incorrect truncation of text that fits exactly
/// within the width limit. For example, "hello" (5 visible chars) with
/// `max_width=5` returns "hello" unchanged, not "hell→".
///
/// # Arguments
///
/// * `text` - Input text potentially containing ANSI escape sequences
/// * `max_width` - Maximum visible character width
/// * `options` - Truncation configuration (suffix, reset behavior)
///
/// # Returns
///
/// Original text if it fits, truncated text if it exceeds max_width.
///
/// # Examples
///
/// ```rust
/// # #[ cfg( feature = "ansi" ) ]
/// # {
/// use strs_tools::ansi::{ truncate_if_needed, TruncateOptions };
///
/// let opts = TruncateOptions::new( 5 ).with_suffix( "→" );
///
/// // Fits exactly - no truncation
/// assert_eq!( truncate_if_needed( "hello", 5, &opts ), "hello" );
///
/// // Exceeds limit - truncated
/// let result = truncate_if_needed( "hello world", 5, &opts );
/// assert!( result.contains( "→" ) );
///
/// // ANSI codes don't count toward width
/// let ansi_text = "\x1b[31mhello\x1b[0m";
/// assert!( truncate_if_needed( ansi_text, 5, &opts ).contains( "hello" ) );
/// # }
/// ```
///
/// # Performance
///
/// - Time complexity: O(n)
/// - Only performs truncation when necessary
/// - Single width calculation per call
pub fn truncate_if_needed( text : &str, max_width : usize, options : &TruncateOptions ) -> String
{
  truncate_if_needed_internal( text, max_width, options, &CharCounter )
}

/// Unicode-aware version of `truncate_if_needed()`.
///
/// Uses grapheme clusters for accurate width calculation of CJK characters,
/// emoji, and combining marks.
#[ cfg( feature = "ansi_unicode" ) ]
pub fn truncate_if_needed_unicode( text : &str, max_width : usize, options : &TruncateOptions ) -> String
{
  truncate_if_needed_internal( text, max_width, options, &GraphemeCounter )
}

fn truncate_if_needed_internal< C : VisibleCounter >(
  text : &str,
  max_width : usize,
  options : &TruncateOptions,
  counter : &C
) -> String
{
  // Fix(bug-width-truncation): Check boundary before truncating
  //
  // Root cause: truncate() reserves space for suffix within max_width,
  // so calling it unconditionally truncates text that fits exactly.
  //
  // Pitfall: Always validate width boundary before calling truncate().
  // Don't assume truncate() handles this internally.

  let visible_width = counter.count( &strip( text ) );

  if visible_width > max_width
  {
    truncate_internal( text, options, counter )
  }
  else
  {
    text.to_string()
  }
}

/// Truncate each line in a text block to maximum width.
///
/// Applies `truncate_if_needed()` to each line independently, tracking
/// whether any line required truncation. Returns both the processed text
/// and a boolean flag indicating if truncation occurred.
///
/// # Arguments
///
/// * `text` - Multi-line text potentially containing ANSI escape sequences
/// * `max_width` - Maximum visible character width per line
/// * `options` - Truncation configuration
///
/// # Returns
///
/// Tuple of (processed_text, any_line_truncated).
///
/// # Examples
///
/// ```rust
/// # #[ cfg( feature = "ansi" ) ]
/// # {
/// use strs_tools::ansi::{ truncate_lines, TruncateOptions };
///
/// let text = "short\nthis is a very long line\nmedium";
/// let opts = TruncateOptions::new( 10 ).with_suffix( "→" );
///
/// let ( result, truncated ) = truncate_lines( text, 10, &opts );
/// assert!( truncated ); // Long line was truncated
///
/// let lines : Vec< &str > = result.lines().collect();
/// assert!( lines[ 0 ].contains( "short" ) );
/// assert!( lines[ 1 ].contains( "→" ) ); // Truncation indicator
/// # }
/// ```
///
/// # Performance
///
/// - Time complexity: O(n × m) where n is text length, m is average line count
/// - Single-pass processing
/// - Minimal allocations (one String per line)
pub fn truncate_lines( text : &str, max_width : usize, options : &TruncateOptions ) -> ( String, bool )
{
  truncate_lines_internal( text, max_width, options, &CharCounter )
}

/// Unicode-aware version of `truncate_lines()`.
#[ cfg( feature = "ansi_unicode" ) ]
pub fn truncate_lines_unicode( text : &str, max_width : usize, options : &TruncateOptions ) -> ( String, bool )
{
  truncate_lines_internal( text, max_width, options, &GraphemeCounter )
}

fn truncate_lines_internal< C : VisibleCounter >(
  text : &str,
  max_width : usize,
  options : &TruncateOptions,
  counter : &C
) -> ( String, bool )
{
  let mut any_truncated = false;

  let lines : alloc::vec::Vec< String > = text
    .lines()
    .map( | line |
    {
      let visible_width = counter.count( &strip( line ) );

      if visible_width > max_width
      {
        any_truncated = true;
        truncate_internal( line, options, counter )
      }
      else
      {
        line.to_string()
      }
    } )
    .collect();

  ( lines.join( "\n" ), any_truncated )
}

// ==================== Internal Implementation ====================

/// Trait for counting visible units (chars or graphemes).
trait VisibleCounter
{
  /// Count visible units in text.
  fn count( &self, text : &str ) -> usize;

  /// Take first N visible units from text.
  fn take_first< 'a >( &self, text : &'a str, n : usize ) -> &'a str;
}

/// Char-based counter (Tier 1).
struct CharCounter;

impl VisibleCounter for CharCounter
{
  fn count( &self, text : &str ) -> usize
  {
    text.chars().count()
  }

  fn take_first< 'a >( &self, text : &'a str, n : usize ) -> &'a str
  {
    let end = text
      .char_indices()
      .nth( n )
      .map_or( text.len(), | ( idx, _ ) | idx );
    &text[ ..end ]
  }
}

/// Grapheme-based counter (Tier 2).
#[ cfg( feature = "ansi_unicode" ) ]
struct GraphemeCounter;

#[ cfg( feature = "ansi_unicode" ) ]
impl VisibleCounter for GraphemeCounter
{
  fn count( &self, text : &str ) -> usize
  {
    use unicode_segmentation::UnicodeSegmentation;
    text.graphemes( true ).count()
  }

  fn take_first< 'a >( &self, text : &'a str, n : usize ) -> &'a str
  {
    use unicode_segmentation::UnicodeSegmentation;

    let mut end = 0;
    for ( idx, grapheme ) in text.grapheme_indices( true ).take( n )
    {
      end = idx + grapheme.len();
    }
    &text[ ..end ]
  }
}

/// Internal truncation implementation using generic counter.
fn truncate_internal< C : VisibleCounter >(
  text : &str,
  options : &TruncateOptions,
  counter : &C,
) -> String
{
  let segments = parse_segments( text );

  // Calculate suffix length
  let suffix_len = options.suffix.as_ref().map_or( 0, | s | counter.count( s ) );

  // If suffix is longer than max_width, we can't use it
  let ( effective_max, use_suffix ) = if suffix_len >= options.max_width
  {
    ( options.max_width, false )
  }
  else
  {
    ( options.max_width - suffix_len, true )
  };

  let mut result = String::new();
  let mut visible_count = 0;
  let mut truncated = false;

  for segment in segments
  {
    match segment
    {
      Segment::Ansi( code ) =>
      {
        // Always include ANSI codes
        result.push_str( code );
      }
      Segment::Text( text_content ) =>
      {
        let text_len = counter.count( text_content );

        if visible_count + text_len <= effective_max
        {
          // Fits entirely
          result.push_str( text_content );
          visible_count += text_len;
        }
        else if visible_count < effective_max
        {
          // Partial fit - truncate
          let remaining = effective_max - visible_count;
          let truncated_text = counter.take_first( text_content, remaining );
          result.push_str( truncated_text );
          truncated = true;
          break;
        }
        else
        {
          // No more room
          truncated = true;
          break;
        }
      }
    }
  }

  // Append suffix if truncated, suffix configured, and suffix fits
  if truncated && use_suffix
  {
    if let Some( ref suffix ) = options.suffix
    {
      result.push_str( suffix );
    }
  }

  // Append reset if configured
  if options.append_reset
  {
    result.push_str( "\x1b[0m" );
  }

  result
}

#[ cfg( test ) ]
mod tests
{
  use super::*;

  // ==================== TruncateOptions tests ====================

  #[ test ]
  fn options_new()
  {
    let opts = TruncateOptions::new( 10 );
    assert_eq!( opts.max_width, 10 );
    assert!( opts.suffix.is_none() );
    assert!( !opts.append_reset );
  }

  #[ test ]
  #[ should_panic( expected = "max_width must be greater than 0" ) ]
  fn options_panic_on_zero()
  {
    TruncateOptions::new( 0 );
  }

  #[ test ]
  fn options_builder()
  {
    let opts = TruncateOptions::new( 10 )
      .with_suffix( "..." )
      .with_reset( true );

    assert_eq!( opts.max_width, 10 );
    assert_eq!( opts.suffix, Some( "...".to_string() ) );
    assert!( opts.append_reset );
  }

  #[ test ]
  fn options_default()
  {
    let opts = TruncateOptions::default();
    assert_eq!( opts.max_width, 80 );
    assert!( opts.suffix.is_none() );
    assert!( !opts.append_reset );
  }

  // ==================== truncate tests ====================

  #[ test ]
  fn truncate_no_change_when_fits()
  {
    let opts = TruncateOptions::new( 10 );
    assert_eq!( truncate( "hello", &opts ), "hello" );
  }

  #[ test ]
  fn truncate_plain_text()
  {
    let opts = TruncateOptions::new( 5 );
    assert_eq!( truncate( "hello world", &opts ), "hello" );
  }

  #[ test ]
  fn truncate_with_suffix()
  {
    let opts = TruncateOptions::new( 8 ).with_suffix( "..." );
    assert_eq!( truncate( "hello world", &opts ), "hello..." );
  }

  #[ test ]
  fn truncate_with_ellipsis()
  {
    let opts = TruncateOptions::new( 6 ).with_suffix( "…" );
    assert_eq!( truncate( "hello world", &opts ), "hello…" );
  }

  #[ test ]
  fn truncate_preserves_ansi()
  {
    let opts = TruncateOptions::new( 3 );
    assert_eq!( truncate( "\x1b[31mhello\x1b[0m", &opts ), "\x1b[31mhel" );
  }

  #[ test ]
  fn truncate_with_reset()
  {
    let opts = TruncateOptions::new( 3 ).with_reset( true );
    assert_eq!( truncate( "\x1b[31mhello\x1b[0m", &opts ), "\x1b[31mhel\x1b[0m" );
  }

  #[ test ]
  fn truncate_ansi_only_fits()
  {
    let opts = TruncateOptions::new( 10 );
    // ANSI codes don't count toward width
    assert_eq!( truncate( "\x1b[31m\x1b[0m", &opts ), "\x1b[31m\x1b[0m" );
  }

  #[ test ]
  fn truncate_multiple_ansi_segments()
  {
    let opts = TruncateOptions::new( 5 );
    let input = "\x1b[31mre\x1b[32md green\x1b[0m";
    // "re" (2) + "d gr" (4) = 6 > 5, so truncate
    // Should get: \x1b[31mre\x1b[32md g (5 visible chars)
    assert_eq!( truncate( input, &opts ), "\x1b[31mre\x1b[32md g" );
  }

  #[ test ]
  fn truncate_empty()
  {
    let opts = TruncateOptions::new( 5 );
    assert_eq!( truncate( "", &opts ), "" );
  }

  #[ test ]
  fn truncate_suffix_too_long()
  {
    // Suffix longer than max_width - should truncate without suffix
    let opts = TruncateOptions::new( 2 ).with_suffix( "..." );
    assert_eq!( truncate( "hello", &opts ), "he" );
  }

  #[ test ]
  fn truncate_unicode_char_based()
  {
    let opts = TruncateOptions::new( 2 );
    // Char-based: 日本語 = 3 chars, truncate to 2
    assert_eq!( truncate( "日本語", &opts ), "日本" );
  }

  // ==================== truncate_unicode tests ====================

  #[ cfg( feature = "ansi_unicode" ) ]
  mod unicode_tests
  {
    use crate::ansi::truncate::{ truncate_unicode, TruncateOptions };

    #[ test ]
    fn truncate_grapheme_emoji()
    {
      let opts = TruncateOptions::new( 2 );
      // 👋🏽 is 1 grapheme, so "👋🏽a" = 2 graphemes
      assert_eq!( truncate_unicode( "👋🏽ab", &opts ), "👋🏽a" );
    }

    #[ test ]
    fn truncate_grapheme_with_ansi()
    {
      let opts = TruncateOptions::new( 2 ).with_reset( true );
      assert_eq!(
        truncate_unicode( "\x1b[33m日本語\x1b[0m", &opts ),
        "\x1b[33m日本\x1b[0m"
      );
    }

    #[ test ]
    fn truncate_grapheme_combining()
    {
      let opts = TruncateOptions::new( 2 );
      // "e\u{0301}" (e + combining acute) is 1 grapheme
      // "e\u{0301}ab" = 3 graphemes, truncate to 2
      assert_eq!( truncate_unicode( "e\u{0301}ab", &opts ), "e\u{0301}a" );
    }
  }
}
