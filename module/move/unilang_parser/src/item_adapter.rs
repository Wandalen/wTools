//! Adapters for converting raw string splits into rich, classified tokens.

#![ allow( clippy::std_instead_of_alloc ) ]
#![ allow( clippy::std_instead_of_core ) ]

use crate::error::{ ParseError, SourceLocation };
use alloc::string::{ String, ToString };
use alloc::borrow::Cow;
use core::fmt;

/// Temporary simple replacement for `strs_tools` Split
#[ derive( Debug, Clone ) ]
pub struct Split< 'a >
{
  /// The string content of this split
  pub string : Cow< 'a, str >,
  /// The byte bounds in the original string  
  pub bounds : ( usize, usize ),
  /// Start position in the original string
  pub start : usize,
  /// End position in the original string
  pub end : usize,
  /// Type of this split segment
  pub typ : SplitType,
  /// Whether this segment was originally quoted
  pub was_quoted : bool,
}

/// Type of split segment
#[ derive( Debug, Clone, PartialEq ) ]
pub enum SplitType
{
  /// A delimiter segment
  Delimiter,
  /// A non-delimiter segment
  NonDelimiter,
}

/// Represents a token with its original split information and classified kind.
#[ derive( Debug, Clone ) ]
pub struct RichItem< 'a >
{
  /// The original string split.
  pub inner : Split< 'a >,
  /// The classified kind of the token.
  pub kind : UnilangTokenKind,
  /// The source location adjusted for things like quotes.
  pub adjusted_source_location : SourceLocation,
}

impl< 'a > RichItem< 'a >
{
  /// Creates a new `RichItem`.
  #[ must_use ]
  pub fn new
  (
    inner : Split< 'a >,
    kind : UnilangTokenKind,
    adjusted_source_location : SourceLocation,
  )
  ->
  Self
  {
    Self
    {
      inner,
      kind,
      adjusted_source_location,
    }
  }

  /// Returns the source location of the item.
  #[ must_use ]
  pub fn source_location( &self ) -> SourceLocation
  {
    self.adjusted_source_location.clone()
  }
}

/// Represents the classified kind of a unilang token.
#[ derive( Debug, PartialEq, Eq, Clone ) ]
pub enum UnilangTokenKind
{
  /// An identifier (e.g., a command name, argument name, or unquoted value).
  Identifier( String ),
  /// A number literal.
  Number( String ),

  /// An operator (e.g., `::`, `?`).
  Operator( &'static str ),
  /// A delimiter (e.g., space, dot, newline).
  Delimiter( &'static str ),
  /// An unrecognized token, indicating a parsing error.
  Unrecognized( String ),
}

impl fmt::Display for UnilangTokenKind
{
  fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
  {
    match self
    {
      UnilangTokenKind::Identifier( s ) | UnilangTokenKind::Unrecognized( s ) | UnilangTokenKind::Number( s ) => write!( f, "{s}" ),
      UnilangTokenKind::Operator( s ) | UnilangTokenKind::Delimiter( s ) => write!( f, "{s}" ),
    }
  }
}

/// Checks if a character is a valid part of a Unilang identifier.
/// Valid characters are lowercase alphanumeric (`a-z`, `0-9`) and underscore (`_`).
fn is_valid_identifier( s : &str ) -> bool
{
  !s.is_empty()
    && s.chars()
    .next()
    .is_some_and( | c | c.is_ascii_lowercase() || c == '_' )
    && !s.ends_with( '-' )
    && s
    .chars()
    .all( | c | c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' || c == '-' )
}

/// Classifies a `strs_tools::Split` into a `UnilangTokenKind` and returns its adjusted source location.
/// Classifies a `strs_tools::Split` into a `UnilangTokenKind` and adjusts its `SourceLocation`.
///
/// # Errors
/// Returns a `ParseError` if the split represents an invalid escape sequence.
pub fn classify_split( s : &Split< '_ > ) -> Result< ( UnilangTokenKind, SourceLocation ), ParseError >
{
  let original_location = SourceLocation::StrSpan
  {
    start : s.start,
    end : s.end,
  };

  let result = match s.string
  {
    Cow::Borrowed( "::" ) => Ok( ( UnilangTokenKind::Operator( "::" ), original_location ) ),
    Cow::Borrowed( "?" ) => Ok( ( UnilangTokenKind::Operator( "?" ), original_location ) ),
    Cow::Borrowed( ":" ) => Ok( ( UnilangTokenKind::Operator( ":" ), original_location ) ),
    Cow::Borrowed( "." ) => Ok( ( UnilangTokenKind::Delimiter( "." ), original_location ) ),
    Cow::Borrowed( " " ) => Ok( ( UnilangTokenKind::Delimiter( " " ), original_location ) ),
    Cow::Borrowed( "\t" ) => Ok( ( UnilangTokenKind::Delimiter( "\t" ), original_location ) ),
    Cow::Borrowed( "\r" ) => Ok( ( UnilangTokenKind::Delimiter( "\r" ), original_location ) ),
    Cow::Borrowed( "\n" ) => Ok( ( UnilangTokenKind::Delimiter( "\n" ), original_location ) ),
    Cow::Borrowed( "#" ) => Ok( ( UnilangTokenKind::Delimiter( "#" ), original_location ) ),
    Cow::Borrowed( "!" ) => Ok( ( UnilangTokenKind::Unrecognized( "!".to_string() ), original_location ) ),
    _ =>
    {
      if s.typ == SplitType::Delimiter
      {
        if s.was_quoted
        {
          Ok( ( UnilangTokenKind::Identifier( s.string.to_string() ), original_location ) )
        }
        else if s.string.parse::< i64 >().is_ok()
        {
          Ok( ( UnilangTokenKind::Number( s.string.to_string() ), original_location ) )
        }
        else if is_valid_identifier( s.string.as_ref() )
        {
          Ok( ( UnilangTokenKind::Identifier( s.string.to_string() ), original_location ) )
        }
        else
        {
          Ok( ( UnilangTokenKind::Unrecognized( s.string.to_string() ), original_location ) )
        }
      }
      else
      {
        Ok( ( UnilangTokenKind::Unrecognized( s.string.to_string() ), original_location ) )
      }
    }
  };
  result
}
