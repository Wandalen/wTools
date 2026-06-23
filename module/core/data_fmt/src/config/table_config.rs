//! `TableConfig` and `PaddingSide` formatter parameter types

use super::{ BorderVariant, HeaderSeparatorVariant, ColumnSeparator, ColumnFlex, FoldStyle };
use super::Heading;

/// Formatter parameters for table output
///
/// Defines customizable parameters including borders, separators, padding,
/// and color options. Use preset methods like `bordered()` or `markdown()`
/// for common configurations, or customize individual parameters.
///
/// ## Construction
///
/// Use preset constructors or builder setters:
///
/// ```rust
/// use data_fmt::TableConfig;
/// let config = TableConfig::unicode_box();
/// ```
///
/// Attempting struct literal construction outside this module is a compile error:
///
/// ```compile_fail
/// let _ = data_fmt::TableConfig
/// {
///   outer_padding : true,
///   ..Default::default()
/// };
/// ```
///
/// ## Known Pitfalls
///
/// **Default column separator**: `TableConfig::default()` (= `new()`) sets
/// `column_separator: Spaces(2)`, NOT `ColumnSeparator::default()` which is `Character('|')`.
/// Use `bordered()` if pipe-separated output is required without an explicit setter call.
// Fix(BUG-003): all fields made private; struct literal construction outside this module
// is now a compile error so callers must use presets or the builder chain.
// Root cause: struct literal initialization allowed setting `header_separator_variant:
// Unicode` while relying on `..Default::default()` for `column_separator` (= Spaces(2)),
// producing misaligned Unicode header separators paired with space-separated data rows.
// Pitfall: Unicode separator components are interdependent — always use
// `TableConfig::unicode_box()` rather than manually pairing fields.
#[ derive( Debug, Clone ) ]
// TableConfig contains multiple independent boolean display toggles (auto_wrap, auto_fold,
// show_header, outer_padding, etc.); each controls a distinct rendering axis and a
// bitfield or enum wrapper would reduce API discoverability without design gain.
#[ allow( clippy::struct_excessive_bools ) ]
pub struct TableConfig
{
  /// Column widths (empty = auto-size)
  column_widths : Vec< usize >,
  /// Align columns right (false = left align)
  align_right : Vec< bool >,
  /// Border rendering variant
  border_variant : BorderVariant,
  /// Header separator line variant
  header_separator_variant : HeaderSeparatorVariant,
  /// Column separator parameter
  column_separator : ColumnSeparator,
  /// Add padding at outer edges of table
  outer_padding : bool,
  /// Number of padding spaces within cells
  inner_padding : usize,
  /// Enable ANSI coloring for header row
  colorize_header : bool,
  /// ANSI color code for header (default: none)
  header_color : String,
  /// Enable alternating row colors
  alternating_rows : bool,
  /// First row color
  row_color1 : String,
  /// Second row color (for alternating)
  row_color2 : String,
  /// ANSI reset sequence applied after colored rows/headers (default: "\x1b[0m")
  color_reset : String,
  /// Minimum width for each column
  min_column_width : usize,
  /// Maximum width for columns (None = unlimited)
  max_column_width : Option< usize >,
  /// Marker string for truncated content
  truncation_marker : String,
  /// Indent prefix for sub-row detail lines
  sub_row_indent : String,
  /// Target terminal width for auto-wrapping (None = auto-detect, fallback 120)
  terminal_width : Option< usize >,
  /// Enable cell auto-wrapping at column budget boundary
  auto_wrap : bool,
  /// Per-column flex classification (empty = auto-classify by heuristic)
  column_flex : Vec< ColumnFlex >,
  /// Enable column folding: move overflow columns to continuation lines below row
  auto_fold : bool,
  /// Format for continuation lines when `auto_fold` is active
  fold_style : FoldStyle,
  /// Indent prefix for continuation lines
  fold_indent : String,
  /// ANSI escape code applied to every border/separator character (None = no coloring)
  border_color : Option< String >,
  /// Optional titled rule rendered above the table (None = no heading)
  heading : Option< Heading >,
}

impl Default for TableConfig
{
  fn default() -> Self
  {
    Self
    {
      column_widths : Vec::new(),
      align_right : Vec::new(),
      border_variant : BorderVariant::None,
      header_separator_variant : HeaderSeparatorVariant::Dash,
      column_separator : ColumnSeparator::Spaces( 2 ),
      outer_padding : true,
      inner_padding : 0,
      colorize_header : false,
      header_color : String::new(),
      alternating_rows : false,
      row_color1 : String::new(),
      row_color2 : String::new(),
      color_reset : "\x1b[0m".to_string(),
      min_column_width : 0,
      max_column_width : None,
      truncation_marker : "...".to_string(),
      sub_row_indent : "  ".to_string(),
      terminal_width : None,
      auto_wrap : true,
      column_flex : Vec::new(),
      auto_fold   : true,
      fold_style  : FoldStyle::Labeled,
      fold_indent : "    ".to_string(),
      border_color : None,
      heading : None,
    }
  }
}

impl TableConfig
{
  /// Create new config with defaults
  pub fn new() -> Self
  {
    Self::default()
  }

  /// Set column widths (empty = auto-size)
  #[ must_use ]
  pub fn with_column_widths( mut self, widths : Vec< usize > ) -> Self
  {
    self.column_widths = widths;
    self
  }

  /// Set column alignment (true = right, false = left)
  #[ must_use ]
  pub fn with_align_right( mut self, align : Vec< bool > ) -> Self
  {
    self.align_right = align;
    self
  }

  // Variant preset constructors

  /// Plain variant: space-separated with dash separator (default)
  /// Ideal for CLI tools output (ps, top, pmon, etc.)
  pub fn plain() -> Self
  {
    Self::default()
  }

  /// Minimal variant: space-separated, no separator
  /// Maximum simplicity and information density
  pub fn minimal() -> Self
  {
    Self::default()
      .with_header_separator_variant(HeaderSeparatorVariant::None )
  }

  /// Bordered variant: traditional pipe-separated table
  /// PostgreSQL-style output
  pub fn bordered() -> Self
  {
    Self::default()
      .with_border_variant(BorderVariant::Ascii )
      .with_header_separator_variant(HeaderSeparatorVariant::AsciiGrid )
      .with_column_separator(ColumnSeparator::Character( '|' ) )
      .with_inner_padding(1 )
  }

  /// Markdown variant: GitHub-flavored Markdown table
  /// Ready for documentation and README files
  pub fn markdown() -> Self
  {
    Self::default()
      .with_border_variant(BorderVariant::Markdown )
      .with_header_separator_variant(HeaderSeparatorVariant::Markdown )
      .with_column_separator(ColumnSeparator::Character( '|' ) )
      .with_inner_padding(1 )
  }

  /// Grid variant: full ASCII box with intersections
  /// Maximum visual clarity for formal reports
  pub fn grid() -> Self
  {
    Self::default()
      .with_border_variant(BorderVariant::AsciiGrid )
      .with_header_separator_variant(HeaderSeparatorVariant::AsciiGrid )
      .with_column_separator(ColumnSeparator::Character( '|' ) )
      .with_inner_padding(1 )
  }

  /// Unicode box variant: Unicode box-drawing characters
  /// Modern, professional appearance for terminal UIs
  pub fn unicode_box() -> Self
  {
    Self::default()
      .with_border_variant(BorderVariant::Unicode )
      .with_header_separator_variant(HeaderSeparatorVariant::Unicode )
      .with_column_separator(ColumnSeparator::Character( '│' ) )
      .with_inner_padding(1 )
  }

  /// CSV variant: comma-separated values
  /// Standard format for data export and Excel import
  pub fn csv() -> Self
  {
    Self::default()
      .with_header_separator_variant(HeaderSeparatorVariant::None )
      .with_column_separator(ColumnSeparator::Character( ',' ) )
      .with_outer_padding(false )
      .with_auto_wrap(false )
      .with_auto_fold(false )
  }

  /// TSV variant: tab-separated values
  /// Excel and spreadsheet compatible
  pub fn tsv() -> Self
  {
    Self::default()
      .with_header_separator_variant(HeaderSeparatorVariant::None )
      .with_column_separator(ColumnSeparator::Character( '\t' ) )
      .with_outer_padding(false )
      .with_auto_wrap(false )
      .with_auto_fold(false )
  }

  /// Compact variant: single-space separator, minimal padding
  /// Maximum information density for narrow terminals
  pub fn compact() -> Self
  {
    Self::default()
      .with_header_separator_variant(HeaderSeparatorVariant::None )
      .with_column_separator(ColumnSeparator::Spaces( 1 ) )
      .with_outer_padding(false )
  }

  // Builder methods for new fields

  /// Set border rendering variant
  #[ must_use ]
  pub fn with_border_variant( mut self, variant : BorderVariant ) -> Self
  {
    self.border_variant = variant;
    self
  }

  /// Set header separator line variant
  #[ must_use ]
  pub fn with_header_separator_variant( mut self, variant : HeaderSeparatorVariant ) -> Self
  {
    self.header_separator_variant = variant;
    self
  }

  /// Set column separator parameter
  #[ must_use ]
  pub fn with_column_separator( mut self, sep : ColumnSeparator ) -> Self
  {
    self.column_separator = sep;
    self
  }

  /// Enable/disable padding at outer table edges
  #[ must_use ]
  pub fn with_outer_padding( mut self, enabled : bool ) -> Self
  {
    self.outer_padding = enabled;
    self
  }

  /// Set number of padding spaces within cells
  #[ must_use ]
  pub fn with_inner_padding( mut self, spaces : usize ) -> Self
  {
    self.inner_padding = spaces;
    self
  }

  /// Enable/disable header row coloring
  #[ must_use ]
  pub fn with_colorize_header( mut self, enabled : bool ) -> Self
  {
    self.colorize_header = enabled;
    self
  }

  /// Set ANSI color code for header row
  #[ must_use ]
  pub fn with_header_color( mut self, color : String ) -> Self
  {
    self.header_color = color;
    self
  }

  /// Enable/disable alternating row colors
  #[ must_use ]
  pub fn with_alternating_rows( mut self, enabled : bool ) -> Self
  {
    self.alternating_rows = enabled;
    self
  }

  /// Set colors for alternating rows
  #[ must_use ]
  pub fn with_row_colors( mut self, color1 : String, color2 : String ) -> Self
  {
    self.row_color1 = color1;
    self.row_color2 = color2;
    self
  }

  /// Set the ANSI reset sequence used after colored rows and headers
  ///
  /// Defaults to `"\x1b[0m"`. Set to `""` to disable ANSI reset (for plain-text output).
  #[ must_use ]
  pub fn with_color_reset( mut self, reset : impl Into< String > ) -> Self
  {
    self.color_reset = reset.into();
    self
  }

  /// Set minimum column width
  #[ must_use ]
  pub fn with_min_column_width( mut self, width : usize ) -> Self
  {
    self.min_column_width = width;
    self
  }

  /// Set maximum column width (None for unlimited)
  #[ must_use ]
  pub fn with_max_column_width( mut self, width : Option< usize > ) -> Self
  {
    self.max_column_width = width;
    self
  }

  /// Set truncation marker string
  #[ must_use ]
  pub fn with_truncation_marker( mut self, marker : String ) -> Self
  {
    self.truncation_marker = marker;
    self
  }

  /// Set indent prefix for sub-row detail lines
  #[ must_use ]
  pub fn with_sub_row_indent( mut self, indent : String ) -> Self
  {
    self.sub_row_indent = indent;
    self
  }

  /// Set target terminal width for auto-wrapping (None = auto-detect)
  #[ must_use ]
  pub fn with_terminal_width( mut self, width : Option< usize > ) -> Self
  {
    self.terminal_width = width;
    self
  }

  /// Enable or disable cell auto-wrapping at budget boundary
  #[ must_use ]
  pub fn with_auto_wrap( mut self, enabled : bool ) -> Self
  {
    self.auto_wrap = enabled;
    self
  }

  /// Set per-column flex classification (empty = auto-classify by heuristic)
  #[ must_use ]
  pub fn with_column_flex( mut self, flex : Vec< ColumnFlex > ) -> Self
  {
    self.column_flex = flex;
    self
  }

  /// Enable or disable column folding: overflow columns move to continuation lines
  #[ must_use ]
  pub fn with_auto_fold( mut self, enabled : bool ) -> Self
  {
    self.auto_fold = enabled;
    self
  }

  /// Set continuation line format for folded overflow columns
  #[ must_use ]
  pub fn with_fold_style( mut self, style : FoldStyle ) -> Self
  {
    self.fold_style = style;
    self
  }

  /// Set indent prefix for folded continuation lines
  #[ must_use ]
  pub fn with_fold_indent( mut self, indent : String ) -> Self
  {
    self.fold_indent = indent;
    self
  }

  /// Set ANSI escape code applied to every border/separator character
  #[ must_use ]
  pub fn with_border_color( mut self, color : String ) -> Self
  {
    self.border_color = Some( color );
    self
  }

  /// Attach a titled heading rule rendered above the table
  #[ must_use ]
  pub fn with_heading( mut self, h : Heading ) -> Self
  {
    self.heading = Some( h );
    self
  }
}

/// Internal accessors for formatters (pub(crate) methods, not fields — satisfies AF1).
///
/// These methods allow sibling formatter modules (e.g., `formatters::table`) to read
/// `TableConfig` fields without exposing them as `pub` to external crates.
/// Accessor names are distinct from setter method names to avoid Rust method name conflicts.
impl TableConfig
{
  /// Column separator (accessor; distinct from `column_separator` setter)
  pub( crate ) fn col_sep( &self ) -> &ColumnSeparator
  {
    &self.column_separator
  }

  /// Header separator variant (accessor; distinct from `header_separator_variant` setter)
  pub( crate ) fn header_sep_variant( &self ) -> HeaderSeparatorVariant
  {
    self.header_separator_variant
  }

  /// Column alignment slice (accessor; distinct from `align_right` setter)
  pub( crate ) fn col_align_right( &self ) -> &[ bool ]
  {
    &self.align_right
  }

  /// Whether outer padding is enabled (accessor; distinct from `outer_padding` setter)
  pub( crate ) fn has_outer_padding( &self ) -> bool
  {
    self.outer_padding
  }

  /// Inner padding spaces per cell (accessor; distinct from `inner_padding` setter)
  pub( crate ) fn cell_inner_padding( &self ) -> usize
  {
    self.inner_padding
  }

  /// Maximum column width (accessor; distinct from `max_column_width` setter)
  pub( crate ) fn max_col_width( &self ) -> Option< usize >
  {
    self.max_column_width
  }

  /// Truncation marker string (accessor; distinct from `truncation_marker` setter)
  pub( crate ) fn trunc_marker( &self ) -> &str
  {
    &self.truncation_marker
  }

  /// Column widths override slice (accessor; distinct from `column_widths` setter)
  pub( crate ) fn col_widths_override( &self ) -> &[ usize ]
  {
    &self.column_widths
  }

  /// Minimum column width floor (accessor; distinct from `min_column_width` setter)
  pub( crate ) fn min_col_width( &self ) -> usize
  {
    self.min_column_width
  }

  /// Border rendering variant (accessor; distinct from `border_variant` setter)
  pub( crate ) fn bdr_variant( &self ) -> BorderVariant
  {
    self.border_variant
  }

  /// Whether header row coloring is enabled
  pub( crate ) fn colorize_header_enabled( &self ) -> bool
  {
    self.colorize_header
  }

  /// ANSI color string for the header row (empty = none)
  pub( crate ) fn header_color_str( &self ) -> &str
  {
    &self.header_color
  }

  /// Whether alternating row coloring is enabled
  pub( crate ) fn alternating_rows_enabled( &self ) -> bool
  {
    self.alternating_rows
  }

  /// ANSI color string for even-indexed data rows (empty = none)
  pub( crate ) fn row_color1_str( &self ) -> &str
  {
    &self.row_color1
  }

  /// ANSI color string for odd-indexed data rows (empty = none)
  pub( crate ) fn row_color2_str( &self ) -> &str
  {
    &self.row_color2
  }

  /// ANSI reset sequence applied after colored content (default: "\x1b[0m")
  pub( crate ) fn color_reset_str( &self ) -> &str
  {
    &self.color_reset
  }

  /// Sub-row detail line indent prefix (accessor; distinct from `sub_row_indent` setter)
  pub( crate ) fn detail_indent( &self ) -> &str
  {
    &self.sub_row_indent
  }

  /// Target terminal width override (accessor; distinct from `terminal_width` setter)
  pub( crate ) fn term_width( &self ) -> Option< usize >
  {
    self.terminal_width
  }

  /// Whether auto-wrapping is enabled (accessor; distinct from `auto_wrap` setter)
  pub( crate ) fn is_auto_wrap( &self ) -> bool
  {
    self.auto_wrap
  }

  /// Per-column flex classification (accessor; distinct from `column_flex` setter)
  pub( crate ) fn col_flex( &self ) -> &[ ColumnFlex ]
  {
    &self.column_flex
  }

  /// Whether column folding is enabled (accessor; distinct from `auto_fold` setter)
  pub( crate ) fn is_auto_fold( &self ) -> bool
  {
    self.auto_fold
  }

  /// Continuation line format for folded overflow columns
  pub( crate ) fn fold_style_val( &self ) -> FoldStyle
  {
    self.fold_style
  }

  /// Indent prefix for folded continuation lines
  pub( crate ) fn fold_indent_val( &self ) -> &str
  {
    &self.fold_indent
  }

  /// ANSI escape code for border coloring (accessor)
  pub( crate ) fn border_color_str( &self ) -> Option< &str >
  {
    self.border_color.as_deref()
  }

  /// Heading reference (accessor)
  pub( crate ) fn heading_ref( &self ) -> Option< &Heading >
  {
    self.heading.as_ref()
  }

  /// Whether column separator is comma or tab (CSV/TSV mode disables padding and ANSI)
  pub( crate ) fn is_csv_or_tsv( &self ) -> bool
  {
    matches!( self.column_separator, ColumnSeparator::Character( ',' | '\t' ) )
  }
}

/// Where to place alignment padding in key-value pairs
#[ derive( Debug, Clone, Copy, PartialEq, Eq, Default ) ]
pub enum PaddingSide
{
  /// Pad keys before separator: "Name   | Value"
  /// Keys align at separator, values start at same column
  #[ default ]
  BeforeSeparator,

  /// Pad values after separator: "Name: Value"
  /// Separators follow keys immediately, values align at same column
  AfterSeparator,
}
