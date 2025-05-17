//! Error types for the unilang parser.

use super::input::Location;

/// Represents an error that occurred during parsing.
#[ derive( Debug, Clone, PartialEq, Eq ) ]
pub enum ParseError
{
  /// An unexpected character or sequence was encountered.
  UnexpectedToken
  {
    location : Location,
    token : String,
  },
  /// An unquoted value contained internal whitespace (based on E5 decision).
  UnquotedValueWithWhitespace
  {
    location : Location,
    value : String,
  },
  /// An unterminated quote was found.
  UnterminatedQuote
  {
    location : Location,
    quote_char : char,
  },
  /// End of input was reached unexpectedly.
  UnexpectedEndOfInput
  {
    location : Location,
  },
  /// A required element was missing.
  MissingElement
  {
    location : Location,
    element_description : String,
  },
  // Add other specific error variants as needed during parser implementation.
}