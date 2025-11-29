//! CLI output processing utilities
//!
//! Provides comprehensive output filtering, truncation, and stream merging
//! for command-line applications.
//!
//! # Features
//!
//! - Head/tail line limiting
//! - ANSI-aware width truncation
//! - Stream selection (stdout/stderr filtering)
//! - Combined processing pipeline
//!
//! # Design Rationale
//!
//! This module consolidates CLI output processing functionality previously duplicated
//! in `unilang::output` (449 lines, deprecated in 0.31.0). The migration fixed three
//! critical issues:
//!
//! 1. **Architectural Violation**: Output formatting is presentation-layer functionality,
//!    not command framework responsibility. The `unilang` framework spec explicitly states
//!    it "does not render UI itself" (FR-SCOPE-2).
//!
//! 2. **Code Duplication**: The `unilang::output` implementation was 90% duplicated with
//!    `strs_tools::ansi` module, violating the single source of truth principle.
//!
//! 3. **API Limitations**: The old struct-based API required verbose initialization.
//!    The new builder pattern enables cleaner configuration chains.
//!
//! ## API Improvements
//!
//! The refactoring introduced several enhancements:
//!
//! - **Builder Pattern**: `OutputConfig::default().with_head(10).with_width(80)`
//!   vs `TruncationConfig { head: Some(10), width: Some(80), ..Default::default() }`
//!
//! - **Configurable Suffix**: User-defined truncation indicators vs hardcoded arrow
//!
//! - **Correct Width Detection**: Only truncates when `visual_len(line) > max_width`,
//!   fixing bug where exact-width text was incorrectly truncated
//!
//! - **Two-Tier Unicode**: Char-based (Tier 1) vs grapheme-aware (Tier 2) support
//!
//! # Examples
//!
//! ```rust
//! # #[ cfg( all( feature = "output", feature = "std" ) ) ]
//! # {
//! use cli_fmt::output::*;
//!
//! let config = OutputConfig::default()
//!   .with_head( 10 )
//!   .with_width( 80 );
//!
//! let result = process_output( "stdout content", "stderr content", &config );
//! println!( "Processed: {}", result.content );
//! # }
//! ```

#[ cfg( feature = "std" ) ]
use std::
{
  string::{ String, ToString },
};
#[ cfg( all( feature = "use_alloc", not( feature = "std" ) ) ) ]
use alloc::
{
  string::{ String, ToString },
};
#[ cfg( all( feature = "use_alloc", not( feature = "std" ) ) ) ]
extern crate alloc;

// Use ansi functions from strs_tools
use strs_tools::ansi::{ truncate_lines, TruncateOptions };

#[ cfg( feature = "ansi_unicode" ) ]
use strs_tools::ansi::truncate_lines_unicode;

#[ cfg( all( feature = "string_split", feature = "std" ) ) ]
use strs_tools::string::lines::{ head, tail, head_and_tail };

/// Configuration for CLI output processing.
#[ derive( Clone, Debug ) ]
pub struct OutputConfig
{
  /// Show only first N lines (None = no limit).
  pub head : Option< usize >,
  /// Show only last N lines (None = no limit).
  pub tail : Option< usize >,
  /// Maximum line width in visible characters (None = no limit).
  pub width : Option< usize >,
  /// Suffix to append when truncated by width (default: "→").
  pub width_suffix : String,
  /// Which output stream(s) to show.
  pub stream_filter : StreamFilter,
  /// Use Unicode grapheme clusters for width (vs char-based).
  pub unicode_aware : bool,
}

/// Which output stream to display.
#[ derive( Clone, Debug, Default, PartialEq, Eq ) ]
pub enum StreamFilter
{
  /// Display both stdout and stderr combined (stderr first).
  #[ default ]
  Both,
  /// Display only stdout.
  Stdout,
  /// Display only stderr.
  Stderr,
}

/// Result of output processing.
#[ derive( Clone, Debug ) ]
pub struct ProcessedOutput
{
  /// The processed content.
  pub content : String,
  /// Number of lines omitted by head/tail filtering.
  pub lines_omitted : usize,
  /// Whether any lines were truncated by width.
  pub width_truncated : bool,
}

impl Default for OutputConfig
{
  fn default() -> Self
  {
    Self
    {
      head : None,
      tail : None,
      width : None,
      width_suffix : "→".to_string(),
      stream_filter : StreamFilter::Both,
      unicode_aware : false,
    }
  }
}

impl OutputConfig
{
  /// Create new configuration with defaults.
  #[ must_use ]
  pub fn new() -> Self
  {
    Self::default()
  }

  /// Set head line limit (builder pattern).
  #[ must_use ]
  pub fn with_head( mut self, count : usize ) -> Self
  {
    self.head = Some( count );
    self
  }

  /// Set tail line limit (builder pattern).
  #[ must_use ]
  pub fn with_tail( mut self, count : usize ) -> Self
  {
    self.tail = Some( count );
    self
  }

  /// Set width limit (builder pattern).
  #[ must_use ]
  pub fn with_width( mut self, width : usize ) -> Self
  {
    self.width = Some( width );
    self
  }

  /// Set width truncation suffix (builder pattern).
  #[ must_use ]
  pub fn with_suffix( mut self, suffix : impl Into< String > ) -> Self
  {
    self.width_suffix = suffix.into();
    self
  }

  /// Set stream filter (builder pattern).
  #[ must_use ]
  pub fn with_stream_filter( mut self, filter : StreamFilter ) -> Self
  {
    self.stream_filter = filter;
    self
  }

  /// Enable Unicode-aware width calculation (builder pattern).
  #[ must_use ]
  pub fn with_unicode_aware( mut self, enabled : bool ) -> Self
  {
    self.unicode_aware = enabled;
    self
  }

  /// Check if any processing is configured.
  #[ must_use ]
  pub fn has_processing( &self ) -> bool
  {
    self.head.is_some() || self.tail.is_some() || self.width.is_some()
  }

  /// Check if default (no modifications).
  #[ must_use ]
  pub fn is_default( &self ) -> bool
  {
    self.head.is_none()
      && self.tail.is_none()
      && self.width.is_none()
      && self.stream_filter == StreamFilter::Both
      && self.width_suffix == "→"
      && !self.unicode_aware
  }
}

/// Process CLI output with filtering and truncation.
///
/// # Arguments
///
/// * `stdout` - Standard output content
/// * `stderr` - Standard error content
/// * `config` - Processing configuration
///
/// # Returns
///
/// Processed output with metadata about truncation.
///
/// # Examples
///
/// ```rust
/// # #[ cfg( all( feature = "output", feature = "std" ) ) ]
/// # {
/// use cli_fmt::output::*;
///
/// let config = OutputConfig::new()
///   .with_head( 5 )
///   .with_width( 80 );
///
/// let result = process_output( "line1\nline2\nline3", "", &config );
/// assert_eq!( result.content, "line1\nline2\nline3" );
/// assert_eq!( result.lines_omitted, 0 );
/// # }
/// ```
#[ must_use ]
pub fn process_output(
  stdout : &str,
  stderr : &str,
  config : &OutputConfig
) -> ProcessedOutput
{
  // 1. Select stream(s)
  let content = merge_streams( stdout, stderr, &config.stream_filter );

  // 2. Apply head/tail line filtering
  let ( lines_result, omitted ) = apply_line_filtering( &content, config.head, config.tail );

  // 3. Apply width truncation
  let ( final_content, width_truncated ) = apply_width_filtering( &lines_result, config );

  ProcessedOutput
  {
    content : final_content,
    lines_omitted : omitted,
    width_truncated,
  }
}

/// Merge stdout and stderr streams.
///
/// # Stream Ordering
///
/// When `StreamFilter::Both` is selected, stderr appears **before** stdout.
/// This follows CLI convention that errors should be visible immediately
/// without scrolling past normal output.
///
/// # Arguments
///
/// * `stdout` - Standard output content
/// * `stderr` - Standard error content
/// * `filter` - Which stream(s) to include
///
/// # Returns
///
/// Merged stream content.
///
/// # Examples
///
/// ```rust
/// # #[ cfg( all( feature = "output", feature = "std" ) ) ]
/// # {
/// use cli_fmt::output::{ merge_streams, StreamFilter };
///
/// // Both streams (stderr first)
/// let result = merge_streams( "out", "err", &StreamFilter::Both );
/// assert_eq!( result, "err\nout" );
///
/// // Only stdout
/// let result = merge_streams( "out", "err", &StreamFilter::Stdout );
/// assert_eq!( result, "out" );
/// # }
/// ```
#[ must_use ]
pub fn merge_streams( stdout : &str, stderr : &str, filter : &StreamFilter ) -> String
{
  match filter
  {
    StreamFilter::Stdout => stdout.to_string(),
    StreamFilter::Stderr => stderr.to_string(),
    StreamFilter::Both =>
    {
      // Fix(issue-stderr-ordering): stderr MUST appear before stdout
      // Root cause: Alphabetical ordering (stdout first) violated CLI convention
      //             that errors should be visible immediately without scrolling.
      // Pitfall: Stream ordering is easy to overlook in tests. Always test not
      //          just "is content present" but "is content in correct order".
      let mut merged = String::with_capacity( stdout.len() + stderr.len() );
      if !stderr.is_empty()
      {
        merged.push_str( stderr );
        // Ensure newline between streams
        if !stderr.ends_with( '\n' ) && !stdout.is_empty()
        {
          merged.push( '\n' );
        }
      }
      if !stdout.is_empty()
      {
        merged.push_str( stdout );
      }
      merged
    }
  }
}

// ============================================================================
// Internal implementation functions
// ============================================================================

#[ cfg( feature = "string_split" ) ]
fn apply_line_filtering(
  content : &str,
  head_opt : Option< usize >,
  tail_opt : Option< usize >
) -> ( String, usize )
{
  if content.is_empty()
  {
    return ( String::new(), 0 );
  }

  match ( head_opt, tail_opt )
  {
    ( None, None ) => ( content.to_string(), 0 ),
    ( Some( h ), None ) =>
    {
      let total = content.lines().count();
      if h >= total
      {
        ( content.to_string(), 0 )
      }
      else
      {
        ( head( content, h ), total - h )
      }
    }
    ( None, Some( t ) ) =>
    {
      let total = content.lines().count();
      if t >= total
      {
        ( content.to_string(), 0 )
      }
      else
      {
        ( tail( content, t ), total - t )
      }
    }
    ( Some( h ), Some( t ) ) =>
    {
      head_and_tail( content, h, t )
    }
  }
}

#[ cfg( not( all( feature = "string_split", feature = "std" ) ) ) ]
fn apply_line_filtering(
  content : &str,
  _head_opt : Option< usize >,
  _tail_opt : Option< usize >
) -> ( String, usize )
{
  ( content.to_string(), 0 )
}

fn apply_width_filtering( content : &str, config : &OutputConfig ) -> ( String, bool )
{
  let Some( max_width ) = config.width else
  {
    return ( content.to_string(), false );
  };

  if max_width == 0
  {
    return ( content.to_string(), false );
  }

  let opts = TruncateOptions::new( max_width )
    .with_suffix( &config.width_suffix )
    .with_reset( true );

  // Use general-purpose function from strs_tools
  if config.unicode_aware
  {
    #[ cfg( feature = "ansi_unicode" ) ]
    { truncate_lines_unicode( content, max_width, &opts ) }
    #[ cfg( not( feature = "ansi_unicode" ) ) ]
    { truncate_lines( content, max_width, &opts ) }
  }
  else
  {
    truncate_lines( content, max_width, &opts )
  }
}

/// Own namespace of the module.
#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use own::*;

/// Own namespace of the module.
#[ allow( unused_imports ) ]
pub mod own
{
  #[ allow( unused_imports ) ]
  use super::*;
  pub use orphan::*;
}

/// Parented namespace of the module.
#[ allow( unused_imports ) ]
pub mod orphan
{
  #[ allow( unused_imports ) ]
  use super::*;
  pub use exposed::*;
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  #[ allow( unused_imports ) ]
  use super::*;
  pub use prelude::*;
}

/// Namespace of the module to include with `use module::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  #[ allow( unused_imports ) ]
  use super::*;
  pub use super::
  {
    OutputConfig,
    StreamFilter,
    ProcessedOutput,
    process_output,
    merge_streams,
  };
}
