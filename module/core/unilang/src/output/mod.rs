//! Output processing utilities for CLI applications.
//!
//! # Deprecation Notice
//!
//! **This module is deprecated and will be removed in unilang 0.32.0.**
//!
//! Use `cli_tools::cli_output` instead. This module now re-exports from
//! cli_tools to maintain backward compatibility.
//!
//! ## Migration Guide
//!
//! **Before:**
//! ```rust,ignore
//! use unilang::output::*;
//!
//! let config = TruncationConfig {
//!   head: Some(10),
//!   width: Some(80),
//!   ..Default::default()
//! };
//! let result = apply_truncation(stdout, stderr, &config);
//! ```
//!
//! **After:**
//! ```rust,ignore
//! use cli_tools::cli_output::*;
//!
//! let config = OutputConfig::new()
//!   .with_head(10)
//!   .with_width(80);
//! let result = process_output(stdout, stderr, &config);
//! ```
//!
//! ## API Changes
//!
//! - `TruncationConfig` → `OutputConfig` (with builder pattern)
//! - `apply_truncation()` → `process_output()`
//! - `TruncatedOutput` → `ProcessedOutput`
//! - `truncate_head()` → `strs_tools::string::lines::head()`
//! - `truncate_tail()` → `strs_tools::string::lines::tail()`
//! - `truncate_width()` → `strs_tools::ansi::truncate_if_needed()`

// Allow deprecated items since this entire module is a deprecated compatibility layer
#[ allow( deprecated ) ]

#[ cfg( feature = "output_processing" ) ]
#[ deprecated(
  since = "0.31.0",
  note = "Use cli_tools::cli_output::OutputConfig instead. See module docs for migration guide."
) ]
pub use cli_tools::cli_output::OutputConfig as TruncationConfig;

#[ cfg( feature = "output_processing" ) ]
#[ deprecated( since = "0.31.0", note = "Use cli_tools::cli_output::StreamFilter" ) ]
pub use cli_tools::cli_output::StreamFilter as OutputFilter;

#[ cfg( feature = "output_processing" ) ]
#[ deprecated( since = "0.31.0", note = "Use cli_tools::cli_output::ProcessedOutput" ) ]
pub use cli_tools::cli_output::ProcessedOutput as TruncatedOutput;

#[ cfg( feature = "output_processing" ) ]
#[ deprecated( since = "0.31.0", note = "Use cli_tools::cli_output::process_output" ) ]
pub use cli_tools::cli_output::process_output as apply_truncation;

#[ cfg( feature = "output_processing" ) ]
#[ deprecated( since = "0.31.0", note = "Use strs_tools::string::lines::head" ) ]
pub use strs_tools::string::lines::head as truncate_head;

#[ cfg( feature = "output_processing" ) ]
#[ deprecated( since = "0.31.0", note = "Use strs_tools::string::lines::tail" ) ]
pub use strs_tools::string::lines::tail as truncate_tail;

/// Truncate each line to max visible width (ANSI and Unicode aware).
///
/// # Deprecation
///
/// This function is deprecated. Use `strs_tools::ansi::truncate_if_needed` directly.
///
/// # Arguments
///
/// * `text` - Input text (may be multiline)
/// * `max_width` - Maximum visible characters per line (0 = no limit)
///
/// # Returns
///
/// Text with each line truncated to max_width visible characters.
#[ cfg( feature = "output_processing" ) ]
#[ deprecated( since = "0.31.0", note = "Use strs_tools::ansi::truncate_if_needed" ) ]
#[ must_use ]
pub fn truncate_width( text : &str, max_width : usize ) -> String
{
  use strs_tools::ansi::{ truncate_if_needed, TruncateOptions };

  if max_width == 0
  {
    return text.to_string();
  }

  let opts = TruncateOptions::new( max_width ).with_reset( true );
  text.lines()
    .map( | line | truncate_if_needed( line, max_width, &opts ) )
    .collect::< Vec< _ > >()
    .join( "\n" )
}

// Internal namespace
mod private
{
}

mod_interface::mod_interface!
{
  #[ cfg( feature = "output_processing" ) ]
  #[ allow( deprecated ) ]
  exposed use
  {
    TruncationConfig,
    OutputFilter,
    TruncatedOutput,
    apply_truncation,
    truncate_head,
    truncate_tail,
    truncate_width,
  };
}
