//! Adapts items from `strs_tools::string::split` and classifies them for unilang parsing.

use crate::config::UnilangParserOptions;
use crate::error::SourceLocation;
use strs_tools::string::split::{ Split, SplitType };
use std::borrow::Cow;

/// Represents the classified kind of a token relevant to unilang syntax.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnilangTokenKind<'a> // Added lifetime 'a
{
  /// An identifier (e.g., command name, argument name).
  Identifier( Cow<'a, str> ), // Changed 'static to 'a
  /// An operator (e.g., "?").
  Operator( Cow<'a, str> ),   // Changed 'static to 'a
  /// A delimiter (e.g., "::", ";;").
  Delimiter( Cow<'a, str> ),  // Changed 'static to 'a
  /// A value that was enclosed in quotes. The Cow contains the raw string content.
  QuotedValue( Cow<'a, str> ), // Changed 'static to 'a
  /// A value that was not enclosed in quotes.
  UnquotedValue( Cow<'a, str> ),// Changed 'static to 'a
  /// A token that could not be classified or is not recognized in the current context.
  Unrecognized( Cow<'a, str> ),// Changed 'static to 'a
  // Note: Whitespace and comments are expected to be handled/filtered
  // before or during the SplitIterator phase, or by parser logic skipping them.
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
  pub kind : UnilangTokenKind<'a>, // Added lifetime 'a
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
/// This is a crucial step as `strs_tools::string::split::Split` only distinguishes
/// between `Delimeted` content and `Delimeter`s. This function adds the
/// unilang-specific semantic meaning.
///
/// TODO: This initial classification is basic. It needs to be more robust,
/// especially for `Delimeted` content (distinguishing identifiers from unquoted values)
/// and potentially handling quoted values if `SplitOptionsFormer` is configured
/// with `preserving_quoting: true`.
pub fn classify_split<'a>
(
  split : &Split<'a>,
  _options : &UnilangParserOptions // options might be needed for context-sensitive classification
) -> UnilangTokenKind<'a> // Added lifetime 'a
{
  match split.typ
  {
    SplitType::Delimeter =>
    {
      // Delimiters from UnilangParserOptions are "::", ";;", "?"
      match split.string
      {
        "::" => UnilangTokenKind::Delimiter( Cow::Borrowed( "::" ) ),
        ";;" => UnilangTokenKind::Delimiter( Cow::Borrowed( ";;" ) ),
        "?"  => UnilangTokenKind::Operator( Cow::Borrowed( "?" ) ),
        _    => UnilangTokenKind::Unrecognized( Cow::Borrowed( split.string ) ),
      }
    }
    SplitType::Delimeted =>
    {
      // Basic classification for delimited content.
      // This needs to be smarter.
      if split.string.chars().all( |c| c.is_alphanumeric() || c == '_' ) && !split.string.is_empty()
      {
        UnilangTokenKind::Identifier( Cow::Borrowed( split.string ) ) // Prefer Borrowed if possible
      }
      else if !split.string.is_empty()
      {
        UnilangTokenKind::UnquotedValue( Cow::Borrowed( split.string ) ) // Prefer Borrowed
      }
      else
      {
        UnilangTokenKind::Unrecognized( Cow::Borrowed( "" ) )
      }
    }
  }
}