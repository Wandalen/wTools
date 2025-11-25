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

/// Formatter parameters for table output
///
/// Defines customizable parameters including borders, separators, padding,
/// and color options. Use preset methods like `bordered()` or `markdown()`
/// for common configurations, or customize individual parameters.
#[ derive( Debug, Clone ) ]
#[ allow( clippy::struct_excessive_bools ) ]
pub struct TableConfig
{
  /// Show table borders (deprecated - use `border_style`)
  #[ deprecated( note = "Use border_style instead" ) ]
  pub show_borders : bool,
  /// Column widths (empty = auto-size)
  pub column_widths : Vec< usize >,
  /// Align columns right (false = left align)
  pub align_right : Vec< bool >,
  /// Border rendering variant
  pub border_variant : BorderVariant,
  /// Header separator line variant
  pub header_separator_variant : HeaderSeparatorVariant,
  /// Column separator parameter
  pub column_separator : ColumnSeparator,
  /// Add padding at outer edges of table
  pub outer_padding : bool,
  /// Number of padding spaces within cells
  pub inner_padding : usize,
  /// Enable ANSI coloring for header row
  pub colorize_header : bool,
  /// ANSI color code for header (default: none)
  pub header_color : String,
  /// Enable alternating row colors
  pub alternating_rows : bool,
  /// First row color
  pub row_color1 : String,
  /// Second row color (for alternating)
  pub row_color2 : String,
  /// Minimum width for each column
  pub min_column_width : usize,
  /// Maximum width for columns (None = unlimited)
  pub max_column_width : Option< usize >,
  /// Marker string for truncated content
  pub truncation_marker : String,
}

impl Default for TableConfig
{
  fn default() -> Self
  {
    #[allow(deprecated)]
    Self
    {
      show_borders : false,
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
      min_column_width : 0,
      max_column_width : None,
      truncation_marker : "...".to_string(),
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

  /// Set whether to show table borders
  #[ deprecated( note = "Use border_variant() instead" ) ]
  #[ must_use ]
  pub fn show_borders( mut self, show : bool ) -> Self
  {
    if show
    {
      self.border_variant = BorderVariant::Ascii;
      self.header_separator_variant = HeaderSeparatorVariant::AsciiGrid;
    }
    else
    {
      self.border_variant = BorderVariant::None;
      self.header_separator_variant = HeaderSeparatorVariant::None;
      self.column_separator = ColumnSeparator::Spaces( 2 );
    }
    self
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
      key_color : "\x1b[90m".to_string(),  // Gray
      padding_side : PaddingSide::BeforeSeparator,
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
      colorize_keys : true,  // Gray keys by default for better readability
      key_color : "\x1b[90m".to_string(),
      padding_side : PaddingSide::AfterSeparator,
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
