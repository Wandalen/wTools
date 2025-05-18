//! Adapts items from `strs_tools::string::split` and classifies them for unilang parsing.

use crate::config::UnilangParserOptions;
use crate::error::SourceLocation;
use strs_tools::string::split::{ Split, SplitType };
use std::borrow::Cow;

/// Represents the classified kind of a token relevant to unilang syntax.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnilangTokenKind<'a>
{
  Identifier( Cow<'a, str> ),
  Operator( Cow<'a, str> ),
  Delimiter( Cow<'a, str> ),
  QuotedValue( Cow<'a, str> ), // Indicates it was quoted, content is raw (quotes stripped by SplitIterator)
  UnquotedValue( Cow<'a, str> ),
  Unrecognized( Cow<'a, str> ),
}

/// Represents an item from the `strs_tools::string::split::SplitIterator`,
/// enriched with segment information and a classified `UnilangTokenKind`.
#[derive(Debug, Clone)]
pub struct RichItem<'a>
{
  pub inner : Split<'a>,
  pub segment_idx : Option<usize>,
  pub kind : UnilangTokenKind<'a>,
}

impl<'a> RichItem<'a>
{
  pub fn source_location( &self ) -> SourceLocation
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

pub fn classify_split<'a>
(
  split : &Split<'a>,
  options : &UnilangParserOptions
) -> UnilangTokenKind<'a>
{
  match split.typ
  {
    SplitType::Delimeter =>
    {
      if split.string == "?"
      {
        UnilangTokenKind::Operator( Cow::Borrowed( "?" ) )
      }
      else if options.delimiters.contains( &split.string )
      {
        UnilangTokenKind::Delimiter( Cow::Borrowed( split.string ) )
      }
      else
      {
        UnilangTokenKind::Unrecognized( Cow::Borrowed( split.string ) )
      }
    }
    SplitType::Delimeted =>
    {
      // TODO: Refine this classification, especially for QuotedValue.
      // Current assumption: SplitIterator strips quotes.
      // The `classify_split` needs to know if the original was quoted to make it QuotedValue.
      // This might require `preserving_quoting: true` in SplitOptionsFormer and stripping here.
      // For now, we can't reliably distinguish QuotedValue from UnquotedValue/Identifier.
      if !split.string.is_empty() && split.string.chars().all( |c| c.is_alphanumeric() || c == '_' )
      {
        UnilangTokenKind::Identifier( Cow::Borrowed( split.string ) )
      }
      else if !split.string.is_empty()
      {
        UnilangTokenKind::UnquotedValue( Cow::Borrowed( split.string ) )
      }
      else
      {
        UnilangTokenKind::Unrecognized( Cow::Borrowed( "" ) )
      }
    }
  }
}

/// Unescapes string values. Returns Cow<'static, str> by always producing an owned String.
///
/// TODO: Implement full unescaping according to `unilang/spec.md` (R5, E1).
pub fn unescape_string(s: &str) -> Cow<'static, str> {
    // If it contains a backslash, assume it might need unescaping.
    // A real implementation would parse all escape sequences.
    if s.contains('\\') {
        // Basic example: replace common escapes.
        // This is NOT a complete or correct unescaper.
        let mut unescaped = String::with_capacity(s.len());
        let mut chars = s.chars();
        while let Some(c) = chars.next() {
            if c == '\\' {
                match chars.next() {
                    Some('\\') => unescaped.push('\\'),
                    Some('\"') => unescaped.push('\"'),
                    Some('\'') => unescaped.push('\''),
                    Some('n') => unescaped.push('\n'),
                    Some('t') => unescaped.push('\t'),
                    // Add other escapes like \r, \0, \xHH, \u{HHHH} as per spec
                    Some(other) => { // Invalid escape, push backslash and char
                        unescaped.push('\\');
                        unescaped.push(other);
                    }
                    None => unescaped.push('\\'), // Trailing backslash
                }
            } else {
                unescaped.push(c);
            }
        }
        Cow::Owned(unescaped)
    } else {
        // If no backslashes, can't be any standard escapes.
        // To return Cow<'static, str>, we must own it if it's not a 'static literal.
        Cow::Owned(s.to_string())
    }
}


#[cfg(test)]
mod tests
{
  use super::*;
  use strs_tools::string::split::Split;

  fn get_default_options() -> UnilangParserOptions
  {
    UnilangParserOptions::default()
  }

  #[test]
  fn classify_delimiters_and_operators()
  {
    let options = get_default_options();
    let split_colon = Split { string: "::", typ: SplitType::Delimeter, start:0, end:2 };
    let split_semicolon = Split { string: ";;", typ: SplitType::Delimeter, start:0, end:2 };
    let split_qmark = Split { string: "?", typ: SplitType::Delimeter, start:0, end:1 };
    let split_unknown_delim = Split { string: "&&", typ: SplitType::Delimeter, start:0, end:2 };

    assert_eq!( classify_split( &split_colon, &options ), UnilangTokenKind::Delimiter( Cow::Borrowed( "::" ) ) );
    assert_eq!( classify_split( &split_semicolon, &options ), UnilangTokenKind::Delimiter( Cow::Borrowed( ";;" ) ) );
    assert_eq!( classify_split( &split_qmark, &options ), UnilangTokenKind::Operator( Cow::Borrowed( "?" ) ) );
    assert_eq!( classify_split( &split_unknown_delim, &options ), UnilangTokenKind::Unrecognized( Cow::Borrowed( "&&" ) ) );
  }

  #[test]
  fn classify_delimited_content()
  {
    let options = get_default_options();
    let split_ident = Split { string: "command", typ: SplitType::Delimeted, start:0, end:7 };
    let split_ident_with_num = Split { string: "cmd1", typ: SplitType::Delimeted, start:0, end:4 };
    let split_unquoted_val = Split { string: "some-value/path", typ: SplitType::Delimeted, start:0, end:15 };
    let split_num_val = Split { string: "123.45", typ: SplitType::Delimeted, start:0, end:6 };

    assert_eq!( classify_split( &split_ident, &options ), UnilangTokenKind::Identifier( Cow::Borrowed( "command" ) ) );
    assert_eq!( classify_split( &split_ident_with_num, &options ), UnilangTokenKind::Identifier( Cow::Borrowed( "cmd1" ) ) );
    assert_eq!( classify_split( &split_unquoted_val, &options ), UnilangTokenKind::UnquotedValue( Cow::Borrowed( "some-value/path" ) ) );
    assert_eq!( classify_split( &split_num_val, &options ), UnilangTokenKind::UnquotedValue( Cow::Borrowed( "123.45" ) ) );
  }

  #[test]
  fn unescape_simple() {
      assert_eq!(unescape_string("simple"), Cow::Owned::<String>("simple".to_string()));
      assert_eq!(unescape_string("a\\\\b"), Cow::Owned("a\\b".to_string()));
      assert_eq!(unescape_string("a\\\"b"), Cow::Owned("a\"b".to_string()));
      assert_eq!(unescape_string("a\\\'b"), Cow::Owned("a\'b".to_string()));
      assert_eq!(unescape_string("a\\nb"), Cow::Owned("a\nb".to_string()));
      assert_eq!(unescape_string("a\\tb"), Cow::Owned("a\tb".to_string()));
      assert_eq!(unescape_string("complex\\\\path\\\"with\\\'quotes\\nnext"), Cow::Owned("complex\\path\"with\'quotes\nnext".to_string()));
      assert_eq!(unescape_string("trailing\\"), Cow::Owned("trailing\\".to_string()));
       assert_eq!(unescape_string("noescape"), Cow::Owned("noescape".to_string()));
  }
}