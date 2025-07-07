//! Provides utilities for adapting `strs_tools::string::split::Split` items into `RichItem`s,
//! which include a classification of the token kind.
//!
//! This module also handles unescaping of strings.

use crate::config::UnilangParserOptions;
use crate::error::{ ParseError, ErrorKind, SourceLocation };
use strs_tools::string::split::{ Split };

/// Represents a tokenized item with its original `Split` data,
/// its segment index (if part of a slice of strings), and its classified `UnilangTokenKind`.
#[ derive( Debug, Clone ) ]
pub struct RichItem< 'a >
{
  /// The original split item from `strs_tools`.
  pub inner : Split< 'a >,
  /// The index of the original string segment if parsing from a slice.
  pub segment_idx : Option< usize >,
  /// The classified kind of the token.
  pub kind : UnilangTokenKind,
}

impl< 'a > RichItem< 'a >
{
  /// Returns the source location of this item.
  pub fn source_location( &'a self ) -> SourceLocation
  {
    if let Some( segment_idx ) = self.segment_idx
    {
      SourceLocation::SliceSegment
      {
        segment_index : segment_idx,
        start_in_segment : self.inner.start,
        end_in_segment : self.inner.end,
      }
    }
    else
    {
      SourceLocation::StrSpan
      {
        start : self.inner.start,
        end : self.inner.end,
      }
    }
  }
}

/// Classifies a `Split` item into a `UnilangTokenKind`.
///
/// This function determines if a split string is an identifier, operator, delimiter,
/// or an unrecognized token based on the parser options.
pub fn classify_split<'a>
(
  split : &'a Split< 'a >,
  options : &UnilangParserOptions,
) -> UnilangTokenKind
{
  let s = split.string;
  eprintln!("DEBUG: classify_split: s: '{}', split.typ: {:?}", s, split.typ); // DEBUG PRINT

  // 1. Check for known operators
  if options.operators.contains(&s)
  {
    return UnilangTokenKind::Operator( s.to_string() );
  }

  // 2. Check for configured delimiters (must be exact match, not part of a larger string)
  if options.main_delimiters.contains(&s)
  {
    return UnilangTokenKind::Delimiter( s.to_string() );
  }

  // 3. Check for quoted values (strs_tools with quoting(false) will return the whole quoted string)
  for (prefix, postfix) in &options.quote_pairs {
      let is_quoted = s.starts_with(*prefix) && s.ends_with(*postfix) && s.len() >= prefix.len_utf8() + postfix.len_utf8();
      eprintln!("DEBUG: classify_split: checking quote pair ('{}', '{}'), is_quoted: {}", prefix, postfix, is_quoted); // DEBUG PRINT
      if is_quoted {
          return UnilangTokenKind::QuotedValue(s.to_string());
      }
  }
 
  // 4. Check if it's an identifier (alphanumeric, underscore, etc.)
  // This is a simplified check. A more robust parser would use a regex or a more
  // detailed character-by-character validation.
  if !s.is_empty() && s.chars().all(|c| c.is_alphanumeric() || c == '_')
  {
    return UnilangTokenKind::Identifier( s.to_string() );
  }

  // 5. Any other unrecognized token.
  UnilangTokenKind::Unrecognized( s.to_string() )
}

/// Represents the classified kind of a token.
#[ derive( Debug, Clone, PartialEq, Eq ) ]
pub enum UnilangTokenKind
{
  /// An identifier, typically a command name or argument name.
  Identifier( String ),
  /// A quoted string value. The inner string is already unescaped.
  QuotedValue( String ),
  /// An operator, e.g., `?`.
  Operator( String ),
  /// A delimiter, e.g., `::`, `;;`.
  Delimiter( String ),
  /// Any other unrecognized token.
  Unrecognized( String ),
}

/// Unescapes a string, handling common escape sequences.
///
/// Supports `\"`, `\'`, `\\`, `\n`, `\r`, `\t`, `\b`.
pub fn unescape_string_with_errors(s: &str, location: &SourceLocation) -> Result<String, ParseError> {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('"') => result.push('"'),
                Some('\'') => result.push('\''),
                Some('\\') => result.push('\\'), // Corrected: unescape \\ to \
                Some('n') => result.push('\n'),
                Some('r') => result.push('\r'),
                Some('t') => result.push('\t'),
                Some('b') => result.push('\x08'), // Backspace
                Some(other) => {
                    return Err(ParseError {
                        kind: ErrorKind::Syntax(format!("Invalid escape sequence: \\{}", other)),
                        location: Some(location.clone()),
                    });
                }
                None => {
                    return Err(ParseError {
                        kind: ErrorKind::Syntax("Incomplete escape sequence at end of string".to_string()),
                        location: Some(location.clone()),
                    });
                }
            }
        } else {
            result.push(c);
        }
    }
    Ok(result)
}
