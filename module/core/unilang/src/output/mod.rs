//!
//! Output processing utilities for CLI applications.
//!
//! This module provides utilities for processing and truncating CLI output,
//! including ANSI-aware and Unicode-aware truncation.
//!

/// Output truncation utilities.
pub mod truncation;

/// Internal namespace.
mod private
{
}

mod_interface::mod_interface!
{
  exposed use
  {
    truncation::TruncationConfig,
    truncation::OutputFilter,
    truncation::TruncatedOutput,
    truncation::apply_truncation,
    truncation::truncate_head,
    truncation::truncate_tail,
    truncation::truncate_width,
  };
}
