//! Adapters for converting raw string splits into rich, classified tokens.

#![allow(clippy::std_instead_of_alloc)]
#![allow(clippy::std_instead_of_core)]

use crate::error::{ ErrorKind, ParseError, SourceLocation };
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

  // Check for quoted strings first, as they are a form of Delimited split but need special handling.
  if s.string.starts_with('"') && s.string.ends_with('"') && s.string.len() >= 2
  {
    let inner_str = &s.string[ 1 .. s.string.len() - 1 ];
    let adjusted_start = s.start + 1;
    let adjusted_location = SourceLocation::StrSpan { start : adjusted_start, end : s.end - 1 };
    let unescaped = unescape_string_with_errors( inner_str, adjusted_start )?;
    return Ok(( UnilangTokenKind::QuotedValue( unescaped ), adjusted_location ));
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

/// Unescapes a string, handling common escape sequences.
fn unescape_string_with_errors( src : &str, offset : usize ) -> Result< String, ParseError >
{
  let mut result = String::with_capacity( src.len() );
  let mut chars = src.chars().peekable();
  let mut current_offset = offset;

  while let Some( c ) = chars.next()
  {
    if c == '\\'
    {
      let escape_start = current_offset;
      current_offset += 1; // for the '\'
      match chars.next()
      {
        Some( 'n' ) => { result.push( '\n' ); current_offset += 1; },
        Some( 't' ) => { result.push( '\t' ); current_offset += 1; },
        Some( 'r' ) => { result.push( '\r' ); current_offset += 1; },
        Some( '\\' ) => { result.push( '\\' ); current_offset += 1; },
        Some( '"' ) => { result.push( '"' ); current_offset += 1; },
        Some( next_c ) =>
        {
          let escape_sequence = format!( "\\{}", next_c );
          return Err( ParseError
          {
            kind : ErrorKind::InvalidEscapeSequence( escape_sequence ),
            location : Some( SourceLocation::StrSpan { start : escape_start, end : escape_start + 2 } ),
          });
        },
        None =>
        {
          return Err( ParseError
          {
            kind : ErrorKind::InvalidEscapeSequence( "\\".to_string() ),
            location : Some( SourceLocation::StrSpan { start : escape_start, end : escape_start + 1 } ),
          });
        },
      }
    }
    else
    {
      result.push( c );
      current_offset += c.len_utf8();
    }
  }
  Ok( result )
}
