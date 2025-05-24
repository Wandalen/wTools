//! Defines error types for the unilang instruction parser.
#![allow(clippy::std_instead_of_alloc)]
#![allow(clippy::std_instead_of_core)]
use std::fmt;

/// Represents the location of a token or parsing error within the input source.
///
/// This enum is used by [`ParseError`] to indicate where an issue occurred.
/// It can pinpoint a location either within a single continuous string (`StrSpan`)
/// or within a specific segment of a slice of strings (`SliceSegment`).
#[derive(Debug, PartialEq, Clone, Eq)] // Added Eq for consistency
pub enum SourceLocation
{
  /// Location within a single string input.
  /// The span represents a byte range.
  StrSpan
  {
    /// The starting byte index of the span in the original string (inclusive).
    start : usize,
    /// The ending byte index of the span in the original string (exclusive).
    end : usize,
  },
  /// Location within a segment of a slice input (e.g., when parsing `&[&str]`).
  /// The span represents a byte range within the specific segment.
  SliceSegment
  {
    /// The 0-based index of the segment in the input slice.
    segment_index : usize,
    /// The starting byte index of the span within its segment (inclusive).
    start_in_segment : usize,
    /// The ending byte index (exclusive) of the span within its segment.
    end_in_segment : usize,
  },
}

/// Specifies the kind of parsing error encountered.
///
/// This enum is used by [`ParseError`] to categorize the error.
#[derive(Debug, Clone, PartialEq, Eq)] // Added Clone, PartialEq, Eq for testability and consistency
pub enum ErrorKind
{
  // Note: Itemization errors from `strs_tools::string::split` are not directly wrapped
  // as `SplitIterator` does not return `Result`. Errors related to tokenization issues
  // (e.g., invalid characters not forming valid tokens by `strs_tools`'s rules)
  // would typically result in `Unrecognized` tokens, which the `unilang_instruction_parser`'s
  // own logic then flags as a `ErrorKind::Syntax` if they are unexpected.

  /// A general syntax error not covered by more specific kinds.
  /// The string contains a descriptive message.
  Syntax(String),
  /// An empty instruction segment caused by a trailing delimiter (e.g., "cmd ;;").
  TrailingDelimiter,
  // /// Unterminated quoted string.
  // /// Note: `strs_tools::string::split` with `preserving_quoting: true` typically handles
  // /// unterminated quotes by treating the content as an unquoted value up to the next delimiter
  // /// or end of input. This error kind might be less common unless pre-validation is done.
  // UnterminatedQuote, // Kept for potential future use, but may not be directly hit by current parser.
  // /// Invalid escape sequence within a string.
  // /// This is now typically reported as `Syntax(String)` by `unescape_string_with_errors`.
  // InvalidEscapeSequence, // Kept for potential future use, but Syntax(msg) is primary.
}

/// Represents an error encountered during the parsing of unilang instructions.
///
/// It includes a [`ErrorKind`] to categorize the error and an optional
/// [`SourceLocation`] to pinpoint where the error occurred in the input.
#[derive(Debug, Clone, PartialEq, Eq)] // Added Clone, PartialEq, Eq for testability and consistency
pub struct ParseError
{
  /// The kind of error.
  pub kind : ErrorKind,
  /// The location of the error in the source input, if available.
  /// This helps in providing user-friendly error messages.
  pub location : Option<SourceLocation>,
}

impl fmt::Display for ParseError
{
  fn fmt( &self, f : &mut fmt::Formatter<'_> ) -> fmt::Result
  {
    match &self.kind
    {
      ErrorKind::Syntax( msg ) => write!( f, "Syntax error: {}", msg )?,
      ErrorKind::TrailingDelimiter => write!( f, "Syntax error: Empty instruction segment due to trailing ';;'" )?,
      // ErrorKind::UnterminatedQuote => write!( f, "Syntax error: Unterminated quote" )?,
      // ErrorKind::InvalidEscapeSequence => write!( f, "Syntax error: Invalid escape sequence" )?,
    }
    if let Some( loc ) = &self.location
    {
      match loc
      {
        SourceLocation::StrSpan { start, end } =>
        {
          write!( f, " at bytes {}-{}", start, end )?;
        }
        SourceLocation::SliceSegment { segment_index, start_in_segment, end_in_segment } =>
        {
          write!( f, " in segment {} at bytes {}-{}", segment_index, start_in_segment, end_in_segment )?;
        }
      }
    }
    Ok( () )
  }
}

impl std::error::Error for ParseError
{
  fn source( &self ) -> Option< &( dyn std::error::Error + 'static ) >
  {
    // Currently, ParseError does not wrap other error types directly as its source.
    // Specific error information is contained within `ErrorKind`.
    None
  }
}
// Removed: impl From<strs_tools::string::tokenizer_core::ParseError> for ParseError
// as strs_tools::string::split::SplitIterator does not return a compatible Result/Error.
// Errors from unescape_string_with_errors are constructed directly as ParseError.