//! Line-based text operations
//!
//! Provides utilities for head/tail line extraction and line-based truncation.
//!
//! # Examples
//!
//! ```rust
//! # #[ cfg( all( feature = "string_split", feature = "std" ) ) ]
//! # {
//! use strs_tools::string::lines::{ head, tail, head_and_tail };
//!
//! let text = "line1\nline2\nline3\nline4\nline5";
//!
//! // Get first 2 lines
//! assert_eq!( head( text, 2 ), "line1\nline2" );
//!
//! // Get last 2 lines
//! assert_eq!( tail( text, 2 ), "line4\nline5" );
//!
//! // Get head + tail with omission count
//! let ( result, omitted ) = head_and_tail( text, 2, 2 );
//! assert_eq!( result, "line1\nline2\nline4\nline5" );
//! assert_eq!( omitted, 1 );
//! # }
//! ```

#[ cfg( feature = "std" ) ]
use std::string::String;
#[ cfg( all( feature = "use_alloc", not( feature = "std" ) ) ) ]
use alloc::string::String;

/// Extract first N lines from text.
///
/// # Arguments
///
/// * `text` - Input text (multiline)
/// * `count` - Number of lines to keep
///
/// # Returns
///
/// String containing only the first N lines.
///
/// # Examples
///
/// ```rust
/// # #[ cfg( all( feature = "string_split", feature = "std" ) ) ]
/// # {
/// use strs_tools::string::lines::head;
///
/// let text = "line1\nline2\nline3";
/// assert_eq!( head( text, 2 ), "line1\nline2" );
///
/// // Requesting more than available returns all
/// assert_eq!( head( text, 10 ), "line1\nline2\nline3" );
/// # }
/// ```
#[ must_use ]
pub fn head( text : &str, count : usize ) -> String
{
  text.lines()
    .take( count )
    .collect::< Vec< _ > >()
    .join( "\n" )
}

/// Extract last N lines from text.
///
/// # Arguments
///
/// * `text` - Input text (multiline)
/// * `count` - Number of lines to keep
///
/// # Returns
///
/// String containing only the last N lines.
///
/// # Examples
///
/// ```rust
/// # #[ cfg( all( feature = "string_split", feature = "std" ) ) ]
/// # {
/// use strs_tools::string::lines::tail;
///
/// let text = "line1\nline2\nline3";
/// assert_eq!( tail( text, 2 ), "line2\nline3" );
///
/// // Requesting more than available returns all
/// assert_eq!( tail( text, 10 ), "line1\nline2\nline3" );
/// # }
/// ```
#[ must_use ]
pub fn tail( text : &str, count : usize ) -> String
{
  let all_lines : Vec< _ > = text.lines().collect();
  let start = all_lines.len().saturating_sub( count );
  all_lines[ start.. ].join( "\n" )
}

/// Extract head + tail with omission marker.
///
/// When head + tail counts exceed total lines, returns all lines with no omission.
/// Otherwise, returns head lines concatenated with tail lines, plus count of omitted middle lines.
///
/// # Arguments
///
/// * `text` - Input text (multiline)
/// * `head_count` - Lines to keep from start
/// * `tail_count` - Lines to keep from end
///
/// # Returns
///
/// Tuple of (selected_lines, omitted_count)
///
/// # Examples
///
/// ```rust
/// # #[ cfg( all( feature = "string_split", feature = "std" ) ) ]
/// # {
/// use strs_tools::string::lines::head_and_tail;
///
/// let text = "line1\nline2\nline3\nline4\nline5";
///
/// // Head 2 + tail 2 = omit 1 middle line
/// let ( result, omitted ) = head_and_tail( text, 2, 2 );
/// assert_eq!( result, "line1\nline2\nline4\nline5" );
/// assert_eq!( omitted, 1 );
///
/// // Overlap returns all
/// let ( result, omitted ) = head_and_tail( text, 3, 3 );
/// assert_eq!( result, "line1\nline2\nline3\nline4\nline5" );
/// assert_eq!( omitted, 0 );
/// # }
/// ```
#[ must_use ]
pub fn head_and_tail( text : &str, head_count : usize, tail_count : usize ) -> ( String, usize )
{
  let lines : Vec< &str > = text.lines().collect();
  let total = lines.len();

  if head_count + tail_count >= total
  {
    // Overlap or exact fit - show all
    return ( text.to_string(), 0 );
  }

  // Need to omit middle section
  let omit_count = total - head_count - tail_count;
  let mut result = Vec::with_capacity( head_count + tail_count );
  result.extend_from_slice( &lines[ ..head_count ] );
  result.extend_from_slice( &lines[ total - tail_count.. ] );

  ( result.join( "\n" ), omit_count )
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
  pub use super::{ head, tail, head_and_tail };
}
