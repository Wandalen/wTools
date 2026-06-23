//! ANSI stripping utilities
//!
//! Provides utilities for removing ANSI escape sequences from text.

extern crate alloc;

use alloc::string::String;
use super::{ Segment, parse_segments };

/// Remove all ANSI escape sequences from text.
///
/// Returns the visible text content with all ANSI codes stripped.
///
/// # Arguments
///
/// * `text` - Input text potentially containing ANSI escape sequences
///
/// # Returns
///
/// String with all ANSI escape sequences removed.
///
/// # Examples
///
/// ```rust
/// # #[ cfg( feature = "ansi" ) ]
/// # {
/// use strs_tools::ansi::strip;
///
/// assert_eq!( strip( "\x1b[31mred\x1b[0m" ), "red" );
/// assert_eq!( strip( "\x1b[1;31mbold red\x1b[0m normal" ), "bold red normal" );
/// assert_eq!( strip( "plain text" ), "plain text" );
/// # }
/// ```
///
/// # Performance
///
/// - Time complexity: O(n)
/// - Space complexity: O(n) for output string
/// - Benchmark: ~500ns/KB on modern hardware
pub fn strip( text : &str ) -> String
{
  parse_segments( text )
    .iter()
    .filter_map( | seg | match seg
    {
      Segment::Text( t ) => Some( *t ),
      Segment::Ansi( _ ) => None,
    })
    .collect()
}
