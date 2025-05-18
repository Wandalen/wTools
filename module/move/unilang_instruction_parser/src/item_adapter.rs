//! Adapts items from `strs_tools::string::split` and classifies them for unilang parsing.

use crate::config::UnilangParserOptions;
use crate::error::SourceLocation;
use strs_tools::string::split::{ Split, SplitType };

/// Represents the classified kind of a token relevant to unilang syntax.
/// String content is owned.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnilangTokenKind
{
  Identifier( String ),
  Operator( String ),
  Delimiter( String ),
  QuotedValue( String ),
  UnquotedValue( String ),
  Unrecognized( String ),
}

/// Represents an item from the `strs_tools::string::split::SplitIterator`,
/// enriched with segment information and a classified `UnilangTokenKind`.
/// It still needs a lifetime 'input_lifetime due to `inner: Split<'input_lifetime>`.
#[derive(Debug, Clone)]
pub struct RichItem<'input_lifetime>
{
  pub inner : Split<'input_lifetime>,
  pub segment_idx : Option<usize>,
  pub kind : UnilangTokenKind,
}

impl<'input_lifetime> RichItem<'input_lifetime>
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

  pub fn kind_payload_as_str( &self ) -> Option<&str>
  {
    match &self.kind
    {
      UnilangTokenKind::Identifier(s) |
      UnilangTokenKind::Operator(s) |
      UnilangTokenKind::Delimiter(s) |
      UnilangTokenKind::QuotedValue(s) |
      UnilangTokenKind::UnquotedValue(s) |
      UnilangTokenKind::Unrecognized(s) => Some(s.as_str()),
    }
  }
}

pub fn classify_split<'input_lifetime>
(
  split : &Split<'input_lifetime>,
  options : &UnilangParserOptions
) -> UnilangTokenKind
{
  match split.typ
  {
    SplitType::Delimeter =>
    {
      if split.string == "?"
      {
        UnilangTokenKind::Operator( "?".to_string() )
      }
      else if options.main_delimiters.iter().any( |d| d == &split.string )
      {
        UnilangTokenKind::Delimiter( split.string.to_string() )
      }
      else if options.whitespace_is_separator && split.string.trim().is_empty()
      {
        UnilangTokenKind::Unrecognized( split.string.to_string() )
      }
      else
      {
        UnilangTokenKind::Unrecognized( split.string.to_string() )
      }
    }
    SplitType::Delimeted =>
    {
      let s = split.string;
      // Check if the string s (which now includes outer quotes due to preserving_quoting: true)
      // matches any of the quote pairs.
      for (prefix, postfix) in &options.quote_pairs {
          if s.starts_with(prefix) && s.ends_with(postfix) && s.len() >= prefix.len() + postfix.len() {
              // It's a quoted string. Extract the inner content.
              let inner_content = &s[prefix.len()..(s.len() - postfix.len())];
              return UnilangTokenKind::QuotedValue(inner_content.to_string());
          }
      }

      // If not a recognized quoted string, proceed with other classifications.
      if !s.is_empty() && s.chars().all( |c| c.is_alphanumeric() || c == '_' )
      {
        UnilangTokenKind::Identifier( s.to_string() )
      }
      else if !s.is_empty()
      {
        UnilangTokenKind::UnquotedValue( s.to_string() )
      }
      else
      {
        UnilangTokenKind::Unrecognized( "".to_string() )
      }
    }
  }
}

/// Unescapes string values, returning an owned String.
/// This function now expects the *inner content* of a quoted string if it was quoted.
pub fn unescape_string(s: &str) -> String {
    if !s.contains('\\') {
        return s.to_string();
    }

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
                Some(other_char) => {
                    unescaped.push('\\');
                    unescaped.push(other_char);
                }
                None => {
                    unescaped.push('\\');
                }
            }
        } else {
            unescaped.push(c);
        }
    }
    unescaped
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

    assert_eq!( classify_split( &split_colon, &options ), UnilangTokenKind::Delimiter( "::".to_string() ) );
    assert_eq!( classify_split( &split_semicolon, &options ), UnilangTokenKind::Delimiter( ";;".to_string() ) );
    assert_eq!( classify_split( &split_qmark, &options ), UnilangTokenKind::Operator( "?".to_string() ) );
    assert_eq!( classify_split( &split_unknown_delim, &options ), UnilangTokenKind::Unrecognized( "&&".to_string() ) );
  }

  #[test]
  fn classify_delimited_content()
  {
    let mut options = get_default_options();
    // options.preserve_quotes_in_split = true; // Not needed, handled by SplitOptionsFormer.preserving_quoting

    // Test case for QuotedValue
    let split_quoted = Split { string: "\"hello world\"", typ: SplitType::Delimeted, start:0, end:13 };
    assert_eq!( classify_split( &split_quoted, &options ), UnilangTokenKind::QuotedValue( "hello world".to_string() ) );

    let split_single_quoted = Split { string: "'another value'", typ: SplitType::Delimeted, start:0, end:15 };
    assert_eq!( classify_split( &split_single_quoted, &options ), UnilangTokenKind::QuotedValue( "another value".to_string() ) );

    let split_empty_quoted = Split { string: "\"\"", typ: SplitType::Delimeted, start:0, end:2 };
    assert_eq!( classify_split( &split_empty_quoted, &options ), UnilangTokenKind::QuotedValue( "".to_string() ) );

    // Test cases for Identifier and UnquotedValue
    let split_ident = Split { string: "command", typ: SplitType::Delimeted, start:0, end:7 };
    let split_ident_with_num = Split { string: "cmd1", typ: SplitType::Delimeted, start:0, end:4 };
    let split_unquoted_val = Split { string: "some-value/path", typ: SplitType::Delimeted, start:0, end:15 };
    let split_num_val = Split { string: "123.45", typ: SplitType::Delimeted, start:0, end:6 };

    assert_eq!( classify_split( &split_ident, &options ), UnilangTokenKind::Identifier( "command".to_string() ) );
    assert_eq!( classify_split( &split_ident_with_num, &options ), UnilangTokenKind::Identifier( "cmd1".to_string() ) );
    assert_eq!( classify_split( &split_unquoted_val, &options ), UnilangTokenKind::UnquotedValue( "some-value/path".to_string() ) );
    assert_eq!( classify_split( &split_num_val, &options ), UnilangTokenKind::UnquotedValue( "123.45".to_string() ) );

    // Test case: string that looks like a quote but isn't complete or is just a quote char
    let split_just_quote = Split { string: "\"", typ: SplitType::Delimeted, start:0, end:1 };
    assert_eq!( classify_split( &split_just_quote, &options ), UnilangTokenKind::UnquotedValue( "\"".to_string() ) );

    let split_unclosed_quote = Split { string: "\"open", typ: SplitType::Delimeted, start:0, end:5 };
    assert_eq!( classify_split( &split_unclosed_quote, &options ), UnilangTokenKind::UnquotedValue( "\"open".to_string() ) );

  }

  #[test]
  fn unescape_logic_owned() {
      assert_eq!(unescape_string("simple"), "simple".to_string());
      assert_eq!(unescape_string("path/with/slashes"), "path/with/slashes".to_string());
      assert_eq!(unescape_string("a\\\\b"), "a\\b".to_string());
      assert_eq!(unescape_string("a\\\"b"), "a\"b".to_string());
      assert_eq!(unescape_string("a\\\'b"), "a\'b".to_string());
      assert_eq!(unescape_string("a\\nb"), "a\nb".to_string());
      assert_eq!(unescape_string("a\\tb"), "a\tb".to_string());
      assert_eq!(unescape_string("complex\\\\path\\\"with\\\'quotes\\nnext"), "complex\\path\"with\'quotes\nnext".to_string());
      assert_eq!(unescape_string("trailing\\"), "trailing\\".to_string());
      assert_eq!(unescape_string("invalid\\z escape"), "invalid\\z escape".to_string());
      assert_eq!(unescape_string(""), "".to_string());
      assert_eq!(unescape_string("\\\\\\"), "\\\\".to_string());
  }
}