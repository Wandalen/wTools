//! Enum types for table style configuration

/// Border rendering variant for tables
#[ derive( Debug, Clone, Copy, PartialEq, Eq, Default ) ]
pub enum BorderVariant
{
  /// No borders, space-separated columns
  None,
  /// ASCII borders with pipes: | + -
  #[ default ]
  Ascii,
  /// Full ASCII grid with row separators: +---+
  AsciiGrid,
  /// Unicode box drawing: ┌─┬─┐ ├─┼─┤ └─┴─┘
  Unicode,
  /// Markdown table format: | col | col |
  Markdown,
}

/// Header separator line variant
#[ derive( Debug, Clone, Copy, PartialEq, Eq, Default ) ]
pub enum HeaderSeparatorVariant
{
  /// No separator line below header
  None,
  /// Dashes only: -----
  Dash,
  /// ASCII grid separator: +-----+
  #[ default ]
  AsciiGrid,
  /// Unicode separator: ├─────┤
  Unicode,
  /// Markdown separator: |-----|
  Markdown,
}

/// Column separator parameter
#[ derive( Debug, Clone, PartialEq, Eq ) ]
pub enum ColumnSeparator
{
  /// N spaces between columns
  Spaces( usize ),
  /// Single character separator (|, ,, \t, etc.)
  Character( char ),
  /// Custom string separator
  String( String ),
}

impl Default for ColumnSeparator
{
  fn default() -> Self
  {
    Self::Character( '|' )
  }
}

/// Column flexibility classification for auto-wrapping budget allocation
///
/// Determines how a column's width is handled during auto-fit:
/// - `Fixed`: keeps natural content width, never wrapped
/// - `Flex`: shrinks to fit the terminal budget, content wraps at budget boundary
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub enum ColumnFlex
{
  /// Keep natural content width; never wrapped by auto-fit
  Fixed,
  /// Shrink to fit budget; content wraps at budget boundary
  Flex,
}

/// Determines how overflow columns are rendered as continuation lines below a row.
///
/// Used with `TableConfig::fold_style` to control the format of continuation lines
/// when `auto_fold` moves overflow columns below the primary table row.
#[ derive( Debug, Clone, Copy, PartialEq, Eq, Default ) ]
pub enum FoldStyle
{
  /// Values only — no column labels on continuation lines.
  Bare,
  /// `"ColName: value"` pairs — default continuation format.
  #[ default ]
  Labeled,
  /// Each overflow column on its own indented line with label.
  Stacked,
}
