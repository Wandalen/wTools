//! Adapters for converting raw string splits into rich, classified tokens.

#![allow(clippy::std_instead_of_alloc)]
#![allow(clippy::std_instead_of_core)]

use crate::error::{ ParseError, SourceLocation };
use strs_tools::string::split::{ Split, SplitType };
use core::fmt; // Import fmt for Display trait

/// Represents a token with its original split information and classified kind.
#[ derive( Debug, Clone ) ] // Added Clone derive
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

  // 1. Quoted strings: Check if the string starts and ends with a quote, and has length >= 2
  if s.string.starts_with( '"' ) && s.string.ends_with( '"' ) && s.string.len() >= 2
  {
    let inner_str = &s.string[ 1 .. s.string.len() - 1 ]; // Strip quotes
    let adjusted_start = s.start + 1;
    let adjusted_end = s.end - 1;
    let adjusted_location = SourceLocation::StrSpan { start : adjusted_start, end : adjusted_end };

    match unescape_string_with_errors( inner_str, adjusted_start )
    {
      Ok( unescaped ) => return Ok(( UnilangTokenKind::QuotedValue( unescaped ), adjusted_location )),
      Err( e ) => return Err( e ), // Propagate the error directly
    }
  }

  // 2. Known operators/delimiters
  match s.string
  {
    "::" => Ok(( UnilangTokenKind::Operator( "::" ), original_location )),
    "?" => Ok(( UnilangTokenKind::Operator( "?" ), original_location )),
    "." => Ok(( UnilangTokenKind::Delimiter( "." ), original_location )),
    " " => Ok(( UnilangTokenKind::Delimiter( " " ), original_location )),
    "\n" => Ok(( UnilangTokenKind::Delimiter( "\n" ), original_location )), // Classify newline as delimiter
    "#" => Ok(( UnilangTokenKind::Delimiter( "#" ), original_location )), // Classify hash as delimiter
    "!" => Ok(( UnilangTokenKind::Unrecognized( "!".to_string() ), original_location )), // Classify '!' as unrecognized
    _ =>
    {
      // 3. Identifiers or unrecognized
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
/// Returns the unescaped string or a ParseError if an invalid escape sequence is found.
/// `offset` is the starting position of the `src` string in the original input,
/// used for accurate error reporting.
fn unescape_string_with_errors( src : &str, mut offset : usize ) -> Result< String, ParseError >
{
  let mut result = String::with_capacity( src.len() );
  let mut chars = src.chars().peekable();

  while let Some( c ) = chars.next()
  {
    if c == '\\'
    {
      let escape_start = offset; // Start of the escape sequence
      match chars.next()
      {
        Some( 'n' ) =>
        {
          result.push( '\n' );
          offset += 2; // Advance past '\n'
        },
        Some( 't' ) =>
        {
          result.push( '\t' );
          offset += 2; // Advance past '\t'
        },
        Some( 'r' ) =>
        {
          result.push( '\r' );
          offset += 2; // Advance past '\r'
        },
        Some( '\\' ) =>
        {
          result.push( '\\' );
          offset += 2; // Advance past '\\'
        },
        Some( '"' ) =>
        {
          result.push( '"' );
          offset += 2; // Advance past '\"'
        },
        Some( c ) =>
        {
          // For invalid escape sequences like '\x', the span should be '\x' (2 chars)
          offset += 2; // Advance past '\c'
          return Err( ParseError
          {
            kind : crate::error::ErrorKind::InvalidEscapeSequence( format!( "\\{}", c ) ),
            location : Some( SourceLocation::StrSpan { start : escape_start, end : escape_start + 2 } ), // Corrected end
          });
        },
        None =>
        {
          // For trailing '\', the span should be '\' (1 char)
          offset += 1; // Advance past '\'
          return Err( ParseError
          {
            kind : crate::error::ErrorKind::InvalidEscapeSequence( "\\".to_string() ),
            location : Some( SourceLocation::StrSpan { start : escape_start, end : escape_start + 1 } ), // Corrected end
          });
        },
      }
    }
    else
    {
      result.push( c );
      offset += c.len_utf8(); // Advance for non-escaped character
    }
  }
  Ok( result )
}
