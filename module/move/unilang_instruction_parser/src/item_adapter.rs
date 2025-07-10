//! Adapters for converting raw string splits into rich, classified tokens.

#![allow(clippy::std_instead_of_alloc)]
#![allow(clippy::std_instead_of_core)]

use crate::error::{ ParseError, SourceLocation };
use strs_tools::string::split::{ Split, SplitType };
use core::fmt;

/// Represents a token with its original split information and classified kind.
#[ derive( Debug, Clone ) ]
pub struct RichItem<'a>
{
  /// The original string split.
  pub inner : Split<'a>,
  /// The classified kind of the token.
  pub kind : UnilangTokenKind,
  /// The source location adjusted for things like quotes.
  pub adjusted_source_location : SourceLocation,
}

impl<'a> RichItem<'a>
{
  /// Creates a new `RichItem`.
  pub fn new( inner : Split<'a>, kind : UnilangTokenKind, adjusted_source_location : SourceLocation ) -> Self
  {
    Self { inner, kind, adjusted_source_location }
  }

  /// Returns the source location of the item.
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
  /// A quoted string value.
  QuotedValue( String ),
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
      UnilangTokenKind::Identifier( s ) => write!( f, "{}", s ),
      UnilangTokenKind::QuotedValue( s ) => write!( f, "\"{}\"", s ),
      UnilangTokenKind::Operator( s ) => write!( f, "{}", s ),
      UnilangTokenKind::Delimiter( s ) => write!( f, "{}", s ),
      UnilangTokenKind::Unrecognized( s ) => write!( f, "{}", s ),
    }
  }
}

/// Classifies a `strs_tools::Split` into a `UnilangTokenKind` and returns its adjusted source location.
pub fn classify_split( s : &Split<'_> ) -> Result<( UnilangTokenKind, SourceLocation ), ParseError>
{
  let original_location = SourceLocation::StrSpan { start : s.start, end : s.end };

  if s.string.starts_with('"') && s.string.ends_with('"') && s.string.len() >= 2
  {
    let inner_str = &s.string[ 1 .. s.string.len() - 1 ];
    let adjusted_location = SourceLocation::StrSpan { start : s.start + 1, end : s.end - 1 };
    return Ok(( UnilangTokenKind::QuotedValue( inner_str.to_string() ), adjusted_location ));
  }

  match s.string
  {
    "::" => Ok(( UnilangTokenKind::Operator( "::" ), original_location )),
    "?" => Ok(( UnilangTokenKind::Operator( "?" ), original_location )),
    ":" => Ok(( UnilangTokenKind::Operator( ":" ), original_location )),
    "." => Ok(( UnilangTokenKind::Delimiter( "." ), original_location )),
    " " => Ok(( UnilangTokenKind::Delimiter( " " ), original_location )),
    "\n" => Ok(( UnilangTokenKind::Delimiter( "\n" ), original_location )),
    "#" => Ok(( UnilangTokenKind::Delimiter( "#" ), original_location )),
    "!" => Ok(( UnilangTokenKind::Unrecognized( "!".to_string() ), original_location )),
    _ =>
    {
      if s.typ == SplitType::Delimeted
      {
        Ok(( UnilangTokenKind::Identifier( s.string.to_string() ), original_location ))
      }
      else
      {
        Ok(( UnilangTokenKind::Unrecognized( s.string.to_string() ), original_location ))
      }
    }
  }
}
