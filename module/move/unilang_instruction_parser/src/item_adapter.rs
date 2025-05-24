//! Adapts items from `strs_tools::string::split` and classifies them for unilang parsing.
#![allow(clippy::elidable_lifetime_names)]

//!
//! This module provides structures and functions to take the raw `Split` items from
//! `strs_tools` and convert them into `RichItem`s, which include a classified
//! `UnilangTokenKind`. This classification is crucial for the parser engine to
//! understand the syntactic role of each token. It also includes the `unescape_string_with_errors`
//! function for processing escape sequences within string literals.

use crate::config::UnilangParserOptions;
use crate::error::SourceLocation;
use crate::error::{ErrorKind, ParseError};
use strs_tools::string::split::{ Split, SplitType };

/// Represents the classified kind of a token relevant to unilang syntax.
///
/// Each variant stores the string content of the token. For `QuotedValue`,
/// this is the raw inner content of the string, before unescaping.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnilangTokenKind
{
  /// An identifier, typically used for command names, path segments, or argument names.
  Identifier( String ),
  /// An operator, like `?` for help.
  Operator( String ),
  /// A delimiter, like `::` for named arguments or `;;` for instruction separation.
  Delimiter( String ),
  /// The inner content of a quoted string (e.g., `hello` from `"hello"`). Unescaping is handled later.
  QuotedValue( String ),
  /// An unquoted value that is not an identifier, operator, or delimiter.
  Unrecognized( String ),
}

/// Represents an item (token) from the input string after initial splitting and classification.
///
/// It wraps a `strs_tools::string::split::Split` item, adding a `segment_idx` (for slice inputs)
/// and a `UnilangTokenKind` which categorizes the token based on unilang syntax rules.
#[derive(Debug, Clone)]
pub struct RichItem<'input_lifetime>
{
  /// The original `Split` item from `strs_tools`.
  pub inner : Split<'input_lifetime>,
  /// The index of the string segment this item originated from, if parsing a slice `&[&str]`.
  /// `None` if parsing a single `&str`.
  pub segment_idx : Option<usize>,
  /// The classified kind of this token according to unilang syntax.
  pub kind : UnilangTokenKind,
}

impl<'input_lifetime> RichItem<'input_lifetime>
{
  /// Calculates the [`SourceLocation`] of this `RichItem` in the original input.
  ///
  /// This considers whether the input was a single string or a slice of strings.
  #[allow(clippy::must_use_candidate)]
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

  /// Returns a string slice of the payload of the token kind, if applicable.
  ///
  /// For example, for `UnilangTokenKind::Identifier("cmd")`, this returns `Some("cmd")`.
  #[allow(clippy::must_use_candidate)]
  pub fn kind_payload_as_str( &self ) -> Option<&str>
  {
    match &self.kind
    {
      UnilangTokenKind::Identifier(s) |
      UnilangTokenKind::Operator(s) |
      UnilangTokenKind::Delimiter(s) |
      UnilangTokenKind::QuotedValue(s) |
      UnilangTokenKind::Unrecognized(s) => Some(s.as_str()),
    }
  }
}

/// Classifies a `strs_tools::string::split::Split` item into a [`UnilangTokenKind`].
///
/// This function applies a set of rules based on the `UnilangParserOptions` and the
/// content and type of the `Split` item to determine its syntactic role in unilang.
///
/// The classification order is roughly:
/// 1. Quoted values (based on `options.quote_pairs`).
/// 2. Known operators and delimiters (from `options.main_delimiters`, e.g., `?`, `::`, `;;`).
/// 3. Identifiers (alphanumeric, `_`, `-`, starting with alpha or `_`).
/// 4. Unrecognized tokens (single punctuation not fitting other categories, excluding single unrecognized punctuation).
/// 5. Unrecognized tokens (single punctuation not otherwise classified, or other fallbacks).
///
/// Note: For `QuotedValue`, this function extracts and stores the *inner content* of the quotes.
/// The actual unescaping of this inner content is handled by [`unescape_string_with_errors`].
#[must_use]
#[allow(clippy::missing_panics_doc)]
#[allow(clippy::needless_return)]
#[allow(clippy::elidable_lifetime_names)]
pub fn classify_split<'input_lifetime>
(
  split : &Split<'input_lifetime>,
  options : &UnilangParserOptions
) -> UnilangTokenKind
{
  let s = split.string;

  if split.typ == SplitType::Delimeted {
      for (prefix, postfix) in &options.quote_pairs {
          if s.starts_with(prefix) && s.ends_with(postfix) && s.len() >= prefix.len() + postfix.len() {
              let inner_content = &s[prefix.len()..(s.len() - postfix.len())];
              return UnilangTokenKind::QuotedValue(inner_content.to_string());
          }
      }
  }

  if s == "?" { return UnilangTokenKind::Operator("?".to_string()); }
  if s == "::" { return UnilangTokenKind::Delimiter("::".to_string()); }
  if s == ";;" { return UnilangTokenKind::Delimiter(";;".to_string()); }
  if s == ":" { return UnilangTokenKind::Delimiter(":".to_string()); }

  #[allow(clippy::collapsible_if)]
  if split.typ == SplitType::Delimeted && !s.is_empty() {
      let mut chars = s.chars();
      if let Some(first_char) = chars.next() {
          if (first_char.is_alphabetic() || first_char == '_') && chars.all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
              return UnilangTokenKind::Identifier(s.to_string());
          }
      }
  }

  #[allow(clippy::collapsible_if)]
  if split.typ == SplitType::Delimeted && !s.is_empty() && !(options.whitespace_is_separator && s.trim().is_empty()) {
      if s.chars().count() == 1 {
          let first_char = s.chars().next().unwrap();
          if first_char.is_ascii_punctuation() {
              return UnilangTokenKind::Unrecognized(s.to_string());
          }
      }
      return UnilangTokenKind::Unrecognized(s.to_string());
  }

  return UnilangTokenKind::Unrecognized(s.to_string());
}

/// Unescapes string values, handling standard escape sequences and reporting errors for invalid ones.
///
/// Takes the raw string content `s` (e.g., the inner content of a quoted string)
/// and a `base_location` which represents the [`SourceLocation`] of `s` within the
/// original, complete input string or input slice segment.
///
/// Supported standard escapes: `\\`, `\"`, `\'`, `\n`, `\t`.
///
/// If an invalid escape sequence (e.g., `\x`, `\z`) or a trailing backslash is encountered,
/// this function returns a [`ParseError`] with an appropriate message and a `SourceLocation`
/// pinpointing the invalid sequence in the original input.
#[allow(clippy::missing_errors_doc)]
pub fn unescape_string_with_errors(
    s: &str,
    base_location: &SourceLocation,
) -> Result<String, ParseError> {
    if !s.contains('\\') {
        return Ok(s.to_string());
    }

    let mut unescaped = String::with_capacity(s.len());
    let mut chars = s.char_indices();

    while let Some((idx, c)) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some((_escape_char_idx, '\\')) => unescaped.push('\\'),
                Some((_escape_char_idx, '\"')) => unescaped.push('\"'),
                Some((_escape_char_idx, '\'')) => unescaped.push('\''),
                Some((_escape_char_idx, 'n')) => unescaped.push('\n'),
                Some((_escape_char_idx, 't')) => unescaped.push('\t'),
                Some((escape_char_idx_val, other_char)) => {
                    let error_start_offset = idx;
                    let error_end_offset = escape_char_idx_val + other_char.len_utf8();

                    let error_location = match base_location {
                        SourceLocation::StrSpan { start: base_start, .. } => {
                            SourceLocation::StrSpan { start: base_start + error_start_offset, end: base_start + error_end_offset }
                        }
                        SourceLocation::SliceSegment { segment_index, start_in_segment: base_start_in_seg, .. } => {
                            SourceLocation::SliceSegment {
                                segment_index: *segment_index,
                                start_in_segment: base_start_in_seg + error_start_offset,
                                end_in_segment: base_start_in_seg + error_end_offset,
                            }
                        }
                    };
                    return Err(ParseError {
                        kind: ErrorKind::Syntax(format!("Invalid escape sequence: \\{}", other_char)),
                        location: Some(error_location),
                    });
                }
                None => {
                    let error_location = match base_location {
                        SourceLocation::StrSpan { start: base_start, .. } => {
                            SourceLocation::StrSpan { start: base_start + idx, end: base_start + idx + 1 }
                        }
                        SourceLocation::SliceSegment { segment_index, start_in_segment: base_start_in_seg, .. } => {
                            SourceLocation::SliceSegment {
                                segment_index: *segment_index,
                                start_in_segment: base_start_in_seg + idx,
                                end_in_segment: base_start_in_seg + idx + 1,
                            }
                        }
                    };
                    return Err(ParseError {
                        kind: ErrorKind::Syntax("Trailing backslash".to_string()),
                        location: Some(error_location),
                    });
                }
            }
        } else {
            unescaped.push(c);
        }
    }
    Ok(unescaped)
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

    assert_eq!( classify_split( &split_colon, &options ), UnilangTokenKind::Delimiter( "::".to_string() ) );
    assert_eq!( classify_split( &split_semicolon, &options ), UnilangTokenKind::Delimiter( ";;".to_string() ) );
    assert_eq!( classify_split( &split_qmark, &options ), UnilangTokenKind::Operator( "?".to_string() ) );

    let split_unknown_punct = Split { string: "&", typ: SplitType::Delimeted, start:0, end:1 };
    assert_eq!( classify_split( &split_unknown_punct, &options ), UnilangTokenKind::Unrecognized( "&".to_string() ) );

    let split_bang = Split { string: "!", typ: SplitType::Delimeted, start:0, end:1 };
    assert_eq!( classify_split( &split_bang, &options ), UnilangTokenKind::Unrecognized( "!".to_string() ) );

    let split_single_colon = Split { string: ":", typ: SplitType::Delimeter, start:0, end:1 };
    assert_eq!( classify_split( &split_single_colon, &options ), UnilangTokenKind::Delimiter( ":".to_string() ) );
  }

  #[test]
  fn classify_delimited_content()
  {
    let options = get_default_options();

    let split_quoted = Split { string: "\"hello world\"", typ: SplitType::Delimeter, start:0, end:13 };
    assert_eq!( classify_split( &split_quoted, &options ), UnilangTokenKind::QuotedValue( "hello world".to_string() ) );

    let split_single_quoted = Split { string: "'another value'", typ: SplitType::Delimeter, start:0, end:15 };
    assert_eq!( classify_split( &split_single_quoted, &options ), UnilangTokenKind::QuotedValue( "another value".to_string() ) );

    let split_empty_quoted = Split { string: "\"\"", typ: SplitType::Delimeted, start:0, end:2 };
    assert_eq!( classify_split( &split_empty_quoted, &options ), UnilangTokenKind::QuotedValue( String::new() ) );

    let split_ident = Split { string: "command", typ: SplitType::Delimeted, start:0, end:7 };
    let split_ident_with_hyphen = Split { string: "cmd-name", typ: SplitType::Delimeter, start:0, end:8 };
    let split_ident_with_num = Split { string: "cmd1", typ: SplitType::Delimeter, start:0, end:4 };

    assert_eq!( classify_split( &split_ident, &options ), UnilangTokenKind::Identifier( "command".to_string() ) );
    assert_eq!( classify_split( &split_ident_with_hyphen, &options ), UnilangTokenKind::Identifier( "cmd-name".to_string() ) );
    assert_eq!( classify_split( &split_ident_with_num, &options ), UnilangTokenKind::Identifier( "cmd1".to_string() ) );

    let split_unquoted_val_path = Split { string: "some-value/path", typ: SplitType::Delimeted, start:0, end:15 };
    let split_num_val = Split { string: "123.45", typ: SplitType::Delimeter, start:0, end:6 };
    assert_eq!( classify_split( &split_num_val, &options ), UnilangTokenKind::UnquotedValue( "123.45".to_string() ) );
    assert_eq!( classify_split( &split_unquoted_val_path, &options ), UnilangTokenKind::UnquotedValue( "some-value/path".to_string() ) );

    let split_just_quote = Split { string: "\"", typ: SplitType::Delimeted, start:0, end:1 };
    assert_eq!( classify_split( &split_just_quote, &options ), UnilangTokenKind::Unrecognized( "\"".to_string() ) );

    let split_unclosed_quote = Split { string: "\"open", typ: SplitType::Delimeted, start:0, end:5 };
    assert_eq!( classify_split( &split_unclosed_quote, &options ), UnilangTokenKind::UnquotedValue( "\"open".to_string() ) );
  }

  #[test]
  fn unescape_with_errors_logic() {
      let base_loc_str = SourceLocation::StrSpan { start: 10, end: 30 };
      assert_eq!(unescape_string_with_errors("simple", &base_loc_str).unwrap(), "simple");
      assert_eq!(unescape_string_with_errors("a\\\\b", &base_loc_str).unwrap(), "a\\b");
      assert_eq!(unescape_string_with_errors("a\\\"b", &base_loc_str).unwrap(), "a\"b");
      assert_eq!(unescape_string_with_errors("a\\\'b", &base_loc_str).unwrap(), "a\'b");
      assert_eq!(unescape_string_with_errors("a\\nb", &base_loc_str).unwrap(), "a\nb");
      assert_eq!(unescape_string_with_errors("a\\tb", &base_loc_str).unwrap(), "a\tb");

      let res_invalid = unescape_string_with_errors("invalid\\z esc", &base_loc_str);
      assert!(res_invalid.is_err());
      let err = res_invalid.unwrap_err();
      assert!(matches!(err.kind, ErrorKind::Syntax(_)));
      assert!(err.to_string().contains("Invalid escape sequence: \\z"));
      assert_eq!(err.location, Some(SourceLocation::StrSpan { start: 10 + 7, end: 10 + 7 + 2 }));


      let res_trailing = unescape_string_with_errors("trailing\\", &base_loc_str);
      assert!(res_trailing.is_err());
      let err_trailing = res_trailing.unwrap_err();
      assert!(matches!(err_trailing.kind, ErrorKind::Syntax(_)));
      assert!(err_trailing.to_string().contains("Trailing backslash"));
      assert_eq!(err_trailing.location, Some(SourceLocation::StrSpan { start: 10 + 8, end: 10 + 8 + 1 }));

      let base_loc_slice = SourceLocation::SliceSegment { segment_index: 1, start_in_segment: 5, end_in_segment: 25 };
      let res_invalid_slice = unescape_string_with_errors("test\\x", &base_loc_slice);
      assert!(res_invalid_slice.is_err());
      let err_slice = res_invalid_slice.unwrap_err();
      assert!(err_slice.to_string().contains("Invalid escape sequence: \\x"));
      assert_eq!(err_slice.location, Some(SourceLocation::SliceSegment { segment_index: 1, start_in_segment: 5 + 4, end_in_segment: 5 + 4 + 2}));
  }
}