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
  #[ must_use ]
  pub fn new( inner : Split<'a>, kind : UnilangTokenKind, adjusted_source_location : SourceLocation ) -> Self
  {
    Self { inner, kind, adjusted_source_location }
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
      UnilangTokenKind::Identifier( s ) | UnilangTokenKind::Unrecognized( s ) => write!( f, "{s}" ),
      UnilangTokenKind::Operator( s ) | UnilangTokenKind::Delimiter( s ) => write!( f, "{s}" ),
    }
  }
}

/// Classifies a `strs_tools::Split` into a `UnilangTokenKind` and returns its adjusted source location.
/// Classifies a `strs_tools::Split` into a `UnilangTokenKind` and adjusts its `SourceLocation`.
///
/// # Errors
/// Returns a `ParseError` if the split represents an invalid escape sequence.
pub fn classify_split( s : &Split<'_> ) -> Result<( UnilangTokenKind, SourceLocation ), ParseError>
{
  let original_location = SourceLocation::StrSpan { start : s.start, end : s.end };



  match s.string
  {
    std::borrow::Cow::Borrowed("::") => Ok(( UnilangTokenKind::Operator( "::" ), original_location )),
    std::borrow::Cow::Borrowed("?") => Ok(( UnilangTokenKind::Operator( "?" ), original_location )),
    std::borrow::Cow::Borrowed(":") => Ok(( UnilangTokenKind::Operator( ":" ), original_location )),
    std::borrow::Cow::Borrowed(".") => Ok(( UnilangTokenKind::Delimiter( "." ), original_location )),
    std::borrow::Cow::Borrowed(" ") => Ok(( UnilangTokenKind::Delimiter( " " ), original_location )),
    std::borrow::Cow::Borrowed("\t") => Ok(( UnilangTokenKind::Delimiter( "\t" ), original_location )),
    std::borrow::Cow::Borrowed("\r") => Ok(( UnilangTokenKind::Delimiter( "\r" ), original_location )),
    std::borrow::Cow::Borrowed("\n") => Ok(( UnilangTokenKind::Delimiter( "\n" ), original_location )),
    std::borrow::Cow::Borrowed("#") => Ok(( UnilangTokenKind::Delimiter( "#" ), original_location )),
    std::borrow::Cow::Borrowed("!") => Ok(( UnilangTokenKind::Unrecognized( "!".to_string() ), original_location )),
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
