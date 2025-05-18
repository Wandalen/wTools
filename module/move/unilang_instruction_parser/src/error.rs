//! Defines error types for the unilang instruction parser.
use std::fmt;

/// Represents the location of a parsing error.
#[derive(Debug, PartialEq, Clone)]
pub enum SourceLocation
{
  /// Location within a single string input.
  StrSpan
  {
    start : usize,
    end : usize,
  },
  /// Location within a segment of a slice input.
  SliceSegment
  {
    segment_index : usize,
    start_in_segment : usize,
    end_in_segment : usize,
  },
}

/// Specifies the kind of parsing error.
#[derive(Debug)]
pub enum ErrorKind
{
  /// Error originating from the `strs_tools` itemizer.
  Itemization(strs_tools::string::parse_request::ErrorKind),
  /// General syntax error.
  Syntax(String),
  /// Unterminated quoted string.
  UnterminatedQuote,
  /// Invalid escape sequence within a string.
  InvalidEscapeSequence,
}

/// Represents an error encountered during parsing.
#[derive(Debug)]
pub struct ParseError
{
  /// The kind of error.
  pub kind : ErrorKind,
  /// The location of the error, if available.
  pub location : Option<SourceLocation>,
}

impl fmt::Display for ParseError
{
  fn fmt( &self, f : &mut fmt::Formatter<'_> ) -> fmt::Result
  {
    match &self.kind
    {
      ErrorKind::Itemization( e ) => write!( f, "Itemization error: {}", e )?,
      ErrorKind::Syntax( msg ) => write!( f, "Syntax error: {}", msg )?,
      ErrorKind::UnterminatedQuote => write!( f, "Syntax error: Unterminated quote" )?,
      ErrorKind::InvalidEscapeSequence => write!( f, "Syntax error: Invalid escape sequence" )?,
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
    match &self.kind
    {
      // qqq: Consider if `strs_tools::string::parse_request::ErrorKind` should implement `std::error::Error` itself.
      // If it does, this can be `Some(e)`. For now, it doesn't.
      ErrorKind::Itemization( _e ) => None,
      _ => None,
    }
  }
}

impl From<strs_tools::string::parse_request::ParseError> for ParseError
{
  fn from( err : strs_tools::string::parse_request::ParseError ) -> Self
  {
    // For now, itemization errors from strs_tools are mapped to StrSpan.
    // If itemization is done per segment for slice inputs, this mapping will need
    // to be adjusted by the caller to include segment_index.
    let location = SourceLocation::StrSpan { start : err.location.start, end : err.location.end };
    ParseError { kind : ErrorKind::Itemization( err.kind ), location : Some( location ) }
  }
}