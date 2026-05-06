//! Alignment testing utilities
//!
//! Common helper functions for alignment tests to eliminate code duplication.
//!
//! ## Functions
//!
//! - `visual_position()` - Calculate visual position of substring considering Unicode
//!
//! ## Note on `dead_code` suppression
//!
//! Each integration test binary independently compiles `mod inc`, making helpers that are
//! not called in that specific binary appear as dead code. This is a Rust compilation
//! artifact of the integration test model — the functions ARE used across the test suite.
//! Suppression is permanent for shared test infrastructure; remove if test layout changes
//! to per-file helper inclusion.

#![ allow( dead_code ) ]

use data_fmt::visual_len;

/// Calculate visual position of a substring in a string
///
/// This function properly handles Unicode characters by calculating visual display width
/// rather than byte length. Used in alignment verification tests.
///
/// ## Arguments
///
/// - `line` - The full line of text
/// - `target` - The substring to find
///
/// ## Returns
///
/// `Some(usize)` with visual position if found, `None` if target not in line
pub fn visual_position( line : &str, target : &str ) -> Option< usize >
{
  let byte_pos = line.find( target )?;
  let before = &line[ ..byte_pos ];
  Some( visual_len( before ) )
}
