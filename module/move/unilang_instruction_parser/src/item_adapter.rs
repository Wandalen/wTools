//! Adapts items from `strs_tools::string::split` and classifies them for unilang parsing.

use crate::config::UnilangParserOptions;
use crate::error::SourceLocation;
use strs_tools::string::split::{ Split, SplitType };
use std::borrow::Cow;

/// Represents the classified kind of a token relevant to unilang syntax.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnilangTokenKind<'a>
{
  /// An identifier (e.g., command name, argument name).
  Identifier( Cow<'a, str> ),
  /// An operator (e.g., "?").
  Operator( Cow<'a, str> ),
  /// A delimiter (e.g., "::", ";;").
  Delimiter( Cow<'a, str> ),
  /// A value that was enclosed in quotes. The Cow contains the raw string content (quotes stripped by SplitIterator).
  QuotedValue( Cow<'a, str> ),
  /// A value that was not enclosed in quotes.
  UnquotedValue( Cow<'a, str> ),
  /// A token that could not be classified or is not recognized in the current context.
  Unrecognized( Cow<'a, str> ),
}

/// Represents an item from the `strs_tools::string::split::SplitIterator`,
/// enriched with segment information and a classified `UnilangTokenKind`.
#[derive(Debug, Clone)]
pub struct RichItem<'a>
{
  /// The inner item from the `strs_tools` splitter.
  pub inner : Split<'a>,
  /// The index of the input segment this item belongs to, if applicable.
  /// `None` if the input was a single string.
  pub segment_idx : Option<usize>,
  /// The classified kind of this unilang token.
  pub kind : UnilangTokenKind<'a>,
}

impl<'a> RichItem<'a>
{
  /// Helper to get `SourceLocation` from this item.
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

/// Classifies a `Split<'a>` item into a `UnilangTokenKind<'a>`.
///
/// This function uses `UnilangParserOptions` to understand which strings
/// are considered operators or delimiters.
///
/// TODO: Distinguishing QuotedValue vs UnquotedValue is currently challenging
/// because `SplitOptionsFormer` is configured with `preserving_quoting: false` (default),
/// meaning the `SplitIterator` strips quotes. If `Split.string` was originally quoted,
/// that information is lost by the time `classify_split` sees it.
/// This might require:
/// 1. Configuring `SplitOptionsFormer` with `preserving_quoting: true` and then
///    stripping quotes here while setting `QuotedValue`.
/// 2. Or, assuming all `Delimeted` content that isn't an Identifier is an `UnquotedValue`
///    and handling unescaping later (which is the current approach).
/// The `unilang/spec.md` will be key to defining robust rules for Identifiers.
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
      // Check if it's a known operator or delimiter from options.
      // UnilangParserOptions.delimiters includes "::", ";;", "?"
      // We'll treat "?" as an Operator, others as Delimiter.
      if split.string == "?"
      {
        UnilangTokenKind::Operator( Cow::Borrowed( "?" ) )
      }
      else if options.delimiters.contains( &split.string ) // Check against all configured delimiters
      {
        UnilangTokenKind::Delimiter( Cow::Borrowed( split.string ) )
      }
      else
      {
        // This case should ideally not be reached if SplitOptionsFormer
        // is configured only with delimiters from UnilangParserOptions.
        UnilangTokenKind::Unrecognized( Cow::Borrowed( split.string ) )
      }
    }
    SplitType::Delimeted =>
    {
      // If preserving_empty was false for SplitOptionsFormer, split.string should not be empty here.
      // Current heuristic:
      // - If it looks like an identifier (alphanumeric + '_').
      // - Otherwise, it's an UnquotedValue.
      // This needs to be refined based on unilang's spec for identifiers.
      // And as noted in TODO, QuotedValue detection is tricky with current SplitOptionsFormer settings.
      if !split.string.is_empty() && split.string.chars().all( |c| c.is_alphanumeric() || c == '_' )
      // A more robust check might involve checking if it's NOT a number, etc.
      // Or if it matches a specific identifier pattern from unilang spec.
      // For now, this is a basic heuristic.
      // Also, ensure it's not a string that looks like a number if numbers are treated differently.
      // Example: if "123" should be UnquotedValue, not Identifier.
      // Let's assume for now simple alphanumeric strings can be identifiers.
      {
        UnilangTokenKind::Identifier( Cow::Borrowed( split.string ) )
      }
      else
      {
        // If not an identifier by the simple heuristic, and not empty,
        // classify as UnquotedValue. This will also catch numbers, paths, etc.
        UnilangTokenKind::UnquotedValue( Cow::Borrowed( split.string ) )
      }
      // If split.string could be empty (e.g. if preserving_empty was true),
      // an additional check for `split.string.is_empty()` would be needed here,
      // potentially returning Unrecognized or a specific EmptyValue token.
      // Since `preserving_empty` is false in `to_split_options_former`, we assume non-empty.
    }
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
    // "&&" is not in default options.delimiters, but SplitOptionsFormer would only split by known delimiters.
    // If it somehow appeared as a Delimiter type, it would be Unrecognized by this classifier.
    // However, options.delimiters for UnilangParserOptions includes "?", "::", ";;"
    // So, if SplitOptionsFormer is built using these, only these should appear as SplitType::Delimeter.
    // For robustness, if an unexpected delimiter string appears, it's Unrecognized.
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
    // Empty string case: SplitOptionsFormer is configured with preserving_empty: false,
    // so we shouldn't receive an empty Delimeted split. If we did, current logic would make it UnquotedValue("").

    assert_eq!( classify_split( &split_ident, &options ), UnilangTokenKind::Identifier( Cow::Borrowed( "command" ) ) );
    assert_eq!( classify_split( &split_ident_with_num, &options ), UnilangTokenKind::Identifier( Cow::Borrowed( "cmd1" ) ) );
    assert_eq!( classify_split( &split_unquoted_val, &options ), UnilangTokenKind::UnquotedValue( Cow::Borrowed( "some-value/path" ) ) );
    assert_eq!( classify_split( &split_num_val, &options ), UnilangTokenKind::UnquotedValue( Cow::Borrowed( "123.45" ) ) );
  }
}