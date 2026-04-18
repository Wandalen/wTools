//! Formatter parameter types and configuration structures
//!
//! Defines formatter parameters and their preset combinations (Config types).
//! Each formatter has customizable parameters that control its output format.

/// Formatter parameters for tree output
///
/// Defines customizable parameters for tree rendering including branch symbols,
/// indentation, depth limits, and column formatting.
#[ derive( Debug, Clone ) ]
pub struct TreeConfig
{
  /// Show branch symbols (├──, └──, │)
  pub show_branches : bool,
  /// Show root node name
  pub show_root : bool,
  /// Number of spaces per indentation level
  pub indent_size : usize,
  /// Maximum depth to display (None = unlimited)
  pub max_depth : Option< usize >,
  /// Column separator for aligned format (default: "  ")
  pub column_separator : String,
  /// Minimum column width for aligned format (default: 0)
  pub min_column_width : usize,
  /// ANSI color code for branch symbols (default: none)
  pub branch_color : String,
}

impl Default for TreeConfig
{
  fn default() -> Self
  {
    Self
    {
      show_branches : true,
      show_root : false,
      indent_size : 4,
      max_depth : None,
      column_separator : "  ".to_string(),
      min_column_width : 0,
      branch_color : String::new(),
    }
  }
}

impl TreeConfig
{
  /// Create new config with defaults
  pub fn new() -> Self
  {
    Self::default()
  }

  /// Set whether to show branch symbols
  #[ must_use ]
  pub fn show_branches( mut self, show : bool ) -> Self
  {
    self.show_branches = show;
    self
  }

  /// Set whether to show root node
  #[ must_use ]
  pub fn show_root( mut self, show : bool ) -> Self
  {
    self.show_root = show;
    self
  }

  /// Set indentation size in spaces
  #[ must_use ]
  pub fn indent_size( mut self, size : usize ) -> Self
  {
    self.indent_size = size;
    self
  }

  /// Set maximum display depth
  #[ must_use ]
  pub fn max_depth( mut self, depth : Option< usize > ) -> Self
  {
    self.max_depth = depth;
    self
  }

  /// Set column separator for aligned format
  #[ must_use ]
  pub fn column_separator( mut self, separator : String ) -> Self
  {
    self.column_separator = separator;
    self
  }

  /// Set minimum column width for aligned format
  #[ must_use ]
  pub fn min_column_width( mut self, width : usize ) -> Self
  {
    self.min_column_width = width;
    self
  }

  /// Set ANSI color code for branch symbols
  #[ must_use ]
  pub fn branch_color( mut self, color : impl Into< String > ) -> Self
  {
    self.branch_color = color.into();
    self
  }
}

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
#[ derive( Debug, Clone ) ]
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
  pub fn column_widths( mut self, widths : Vec< usize > ) -> Self
  {
    self.column_widths = widths;
    self
  }

  /// Set column alignment (true = right, false = left)
  #[ must_use ]
  pub fn align_right( mut self, align : Vec< bool > ) -> Self
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
    Self
    {
      border_variant : BorderVariant::None,
      header_separator_variant : HeaderSeparatorVariant::None,
      column_separator : ColumnSeparator::Spaces( 2 ),
      outer_padding : true,
      inner_padding : 0,
      ..Self::default()
    }
  }

  /// Bordered variant: traditional pipe-separated table
  /// PostgreSQL-style output
  pub fn bordered() -> Self
  {
    Self
    {
      border_variant : BorderVariant::Ascii,
      header_separator_variant : HeaderSeparatorVariant::AsciiGrid,
      column_separator : ColumnSeparator::Character( '|' ),
      outer_padding : true,
      inner_padding : 1,
      ..Self::default()
    }
  }

  /// Markdown variant: GitHub-flavored Markdown table
  /// Ready for documentation and README files
  pub fn markdown() -> Self
  {
    Self
    {
      border_variant : BorderVariant::Markdown,
      header_separator_variant : HeaderSeparatorVariant::Markdown,
      column_separator : ColumnSeparator::Character( '|' ),
      outer_padding : true,
      inner_padding : 1,
      ..Self::default()
    }
  }

  /// Grid variant: full ASCII box with intersections
  /// Maximum visual clarity for formal reports
  pub fn grid() -> Self
  {
    Self
    {
      border_variant : BorderVariant::AsciiGrid,
      header_separator_variant : HeaderSeparatorVariant::AsciiGrid,
      column_separator : ColumnSeparator::Character( '|' ),
      outer_padding : true,
      inner_padding : 1,
      ..Self::default()
    }
  }

  /// Unicode box variant: Unicode box-drawing characters
  /// Modern, professional appearance for terminal UIs
  pub fn unicode_box() -> Self
  {
    Self
    {
      border_variant : BorderVariant::Unicode,
      header_separator_variant : HeaderSeparatorVariant::Unicode,
      column_separator : ColumnSeparator::Character( '│' ),
      outer_padding : true,
      inner_padding : 1,
      ..Self::default()
    }
  }

  /// CSV variant: comma-separated values
  /// Standard format for data export and Excel import
  pub fn csv() -> Self
  {
    Self
    {
      border_variant : BorderVariant::None,
      header_separator_variant : HeaderSeparatorVariant::None,
      column_separator : ColumnSeparator::Character( ',' ),
      outer_padding : false,
      inner_padding : 0,
      auto_wrap : false,
      auto_fold : false,
      ..Self::default()
    }
  }

  /// TSV variant: tab-separated values
  /// Excel and spreadsheet compatible
  pub fn tsv() -> Self
  {
    Self
    {
      border_variant : BorderVariant::None,
      header_separator_variant : HeaderSeparatorVariant::None,
      column_separator : ColumnSeparator::Character( '\t' ),
      outer_padding : false,
      inner_padding : 0,
      auto_wrap : false,
      auto_fold : false,
      ..Self::default()
    }
  }

  /// Compact variant: single-space separator, minimal padding
  /// Maximum information density for narrow terminals
  pub fn compact() -> Self
  {
    Self
    {
      border_variant : BorderVariant::None,
      header_separator_variant : HeaderSeparatorVariant::None,
      column_separator : ColumnSeparator::Spaces( 1 ),
      outer_padding : false,
      inner_padding : 0,
      ..Self::default()
    }
  }

  // Builder methods for new fields

  /// Set border rendering variant
  #[ must_use ]
  pub fn border_variant( mut self, variant : BorderVariant ) -> Self
  {
    self.border_variant = variant;
    self
  }

  /// Set header separator line variant
  #[ must_use ]
  pub fn header_separator_variant( mut self, variant : HeaderSeparatorVariant ) -> Self
  {
    self.header_separator_variant = variant;
    self
  }

  /// Set column separator parameter
  #[ must_use ]
  pub fn column_separator( mut self, sep : ColumnSeparator ) -> Self
  {
    self.column_separator = sep;
    self
  }

  /// Enable/disable padding at outer table edges
  #[ must_use ]
  pub fn outer_padding( mut self, enabled : bool ) -> Self
  {
    self.outer_padding = enabled;
    self
  }

  /// Set number of padding spaces within cells
  #[ must_use ]
  pub fn inner_padding( mut self, spaces : usize ) -> Self
  {
    self.inner_padding = spaces;
    self
  }

  /// Enable/disable header row coloring
  #[ must_use ]
  pub fn colorize_header( mut self, enabled : bool ) -> Self
  {
    self.colorize_header = enabled;
    self
  }

  /// Set ANSI color code for header row
  #[ must_use ]
  pub fn header_color( mut self, color : String ) -> Self
  {
    self.header_color = color;
    self
  }

  /// Enable/disable alternating row colors
  #[ must_use ]
  pub fn alternating_rows( mut self, enabled : bool ) -> Self
  {
    self.alternating_rows = enabled;
    self
  }

  /// Set colors for alternating rows
  #[ must_use ]
  pub fn row_colors( mut self, color1 : String, color2 : String ) -> Self
  {
    self.row_color1 = color1;
    self.row_color2 = color2;
    self
  }

  /// Set the ANSI reset sequence used after colored rows and headers
  ///
  /// Defaults to `"\x1b[0m"`. Set to `""` to disable ANSI reset (for plain-text output).
  #[ must_use ]
  pub fn color_reset( mut self, reset : impl Into< String > ) -> Self
  {
    self.color_reset = reset.into();
    self
  }

  /// Set minimum column width
  #[ must_use ]
  pub fn min_column_width( mut self, width : usize ) -> Self
  {
    self.min_column_width = width;
    self
  }

  /// Set maximum column width (None for unlimited)
  #[ must_use ]
  pub fn max_column_width( mut self, width : Option< usize > ) -> Self
  {
    self.max_column_width = width;
    self
  }

  /// Set truncation marker string
  #[ must_use ]
  pub fn truncation_marker( mut self, marker : String ) -> Self
  {
    self.truncation_marker = marker;
    self
  }

  /// Set indent prefix for sub-row detail lines
  #[ must_use ]
  pub fn sub_row_indent( mut self, indent : String ) -> Self
  {
    self.sub_row_indent = indent;
    self
  }

  /// Set target terminal width for auto-wrapping (None = auto-detect)
  #[ must_use ]
  pub fn terminal_width( mut self, width : Option< usize > ) -> Self
  {
    self.terminal_width = width;
    self
  }

  /// Enable or disable cell auto-wrapping at budget boundary
  #[ must_use ]
  pub fn auto_wrap( mut self, enabled : bool ) -> Self
  {
    self.auto_wrap = enabled;
    self
  }

  /// Set per-column flex classification (empty = auto-classify by heuristic)
  #[ must_use ]
  pub fn column_flex( mut self, flex : Vec< ColumnFlex > ) -> Self
  {
    self.column_flex = flex;
    self
  }

  /// Enable or disable column folding: overflow columns move to continuation lines
  #[ must_use ]
  pub fn auto_fold( mut self, enabled : bool ) -> Self
  {
    self.auto_fold = enabled;
    self
  }

  /// Set continuation line format for folded overflow columns
  #[ must_use ]
  pub fn fold_style( mut self, style : FoldStyle ) -> Self
  {
    self.fold_style = style;
    self
  }

  /// Set indent prefix for folded continuation lines
  #[ must_use ]
  pub fn fold_indent( mut self, indent : String ) -> Self
  {
    self.fold_indent = indent;
    self
  }
}

/// Internal accessors for formatters (pub(crate) methods, not fields — satisfies AF1).
///
/// These methods allow sibling formatter modules (e.g., `formatters::table`) to read
/// `TableConfig` fields without exposing them as `pub` to external crates.
/// Accessor names are distinct from setter method names to avoid Rust method name conflicts.
#[ allow( dead_code ) ]
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

/// Formatter parameters for expanded (vertical record) output
///
/// Defines customizable parameters for vertical key-value display including
/// record separators, key-value separators, and color options.
#[ derive( Debug, Clone ) ]
pub struct ExpandedConfig
{
  /// Record separator line (empty string disables record headers)
  pub record_separator : String,
  /// Key-value separator
  pub key_value_separator : String,
  /// Show record numbers in separator
  pub show_record_numbers : bool,
  /// Enable ANSI color for keys
  pub colorize_keys : bool,
  /// ANSI color code for keys (default: gray)
  pub key_color : String,
  /// Where to place padding for alignment
  pub padding_side : PaddingSide,
  /// Prefix string prepended to each key-value line (default: empty)
  pub indent_prefix : String,
}

impl Default for ExpandedConfig
{
  fn default() -> Self
  {
    Self
    {
      record_separator : "-[ RECORD {} ]".to_string(),
      key_value_separator : " | ".to_string(),
      show_record_numbers : true,
      colorize_keys : false,
      key_color : String::new(),
      padding_side : PaddingSide::BeforeSeparator,
      indent_prefix : String::new(),
    }
  }
}

impl ExpandedConfig
{
  /// Create new config with defaults (`PostgreSQL` \x style)
  pub fn new() -> Self
  {
    Self::default()
  }

  /// Create config for `PostgreSQL` \x style (default)
  pub fn postgres_style() -> Self
  {
    Self::new()
  }

  /// Create config for property list style
  /// Default: gray colored keys, colon separator, no record headers
  pub fn property_style() -> Self
  {
    Self
    {
      record_separator : String::new(),
      key_value_separator : ": ".to_string(),
      show_record_numbers : false,
      colorize_keys : true,
      key_color : String::new(),
      padding_side : PaddingSide::AfterSeparator,
      indent_prefix : String::new(),
    }
  }

  /// Set record separator format string
  #[ must_use ]
  pub fn record_separator( mut self, separator : String ) -> Self
  {
    self.record_separator = separator;
    self
  }

  /// Set key-value separator
  #[ must_use ]
  pub fn key_value_separator( mut self, separator : String ) -> Self
  {
    self.key_value_separator = separator;
    self
  }

  /// Set whether to show record numbers
  #[ must_use ]
  pub fn show_record_numbers( mut self, show : bool ) -> Self
  {
    self.show_record_numbers = show;
    self
  }

  /// Enable or disable colored keys
  #[ must_use ]
  pub fn colorize_keys( mut self, enable : bool ) -> Self
  {
    self.colorize_keys = enable;
    self
  }

  /// Set custom ANSI color code for keys
  #[ must_use ]
  pub fn key_color( mut self, color : String ) -> Self
  {
    self.key_color = color;
    self
  }

  /// Set padding side for alignment
  #[ must_use ]
  pub fn padding_side( mut self, side : PaddingSide ) -> Self
  {
    self.padding_side = side;
    self
  }

  /// Set indent prefix prepended to each key-value line
  #[ must_use ]
  pub fn indent_prefix( mut self, prefix : String ) -> Self
  {
    self.indent_prefix = prefix;
    self
  }
}

/// Tree symbols used for rendering
#[ derive( Debug, Clone ) ]
pub struct TreeSymbols
{
  /// Branch connector: ├──
  pub branch : &'static str,
  /// Last branch connector: └──
  pub last_branch : &'static str,
  /// Vertical line: │
  pub vertical : &'static str,
  /// Space for indentation
  pub space : &'static str,
}

impl Default for TreeSymbols
{
  fn default() -> Self
  {
    Self
    {
      branch : "├──",
      last_branch : "└──",
      vertical : "│",
      space : "    ",
    }
  }
}
