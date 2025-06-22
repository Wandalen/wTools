//!
//! Error types for the command aggregator parser.
//!

use super::input::Location;

///
/// Represents an error that occurred during parsing.
///
#[ derive( Debug, Clone, PartialEq, Eq ) ]
pub enum ParseError
{
  /// An unexpected character or sequence was encountered.
  UnexpectedToken
  {
    /// The location of the unexpected token.
    location : Location,
    /// The unexpected token.
    token : String,
  },
  /// An unquoted value contained internal whitespace (based on E5 decision).
  UnquotedValueWithWhitespace
  {
    /// The location of the value.
    location : Location,
    /// The value containing whitespace.
    value : String,
  },
  /// An unterminated quote was found.
  UnterminatedQuote
  {
    /// The location of the unterminated quote.
    location : Location,
    /// The quote character that was not terminated.
    quote_char : char,
  },
  /// End of input was reached unexpectedly.
  UnexpectedEndOfInput
  {
    /// The location where the end of input was unexpected.
    location : Location,
  },
  /// A required element was missing.
  MissingElement
  {
    /// The location where the element was expected.
    location : Location,
    /// A description of the missing element.
    element_description : String,
  },
  // Add other specific error variants as needed during parser implementation.
}