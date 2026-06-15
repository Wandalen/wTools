//! Tree formatter configuration types

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
  pub fn with_show_branches( mut self, show : bool ) -> Self
  {
    self.show_branches = show;
    self
  }

  /// Set whether to show root node
  #[ must_use ]
  pub fn with_show_root( mut self, show : bool ) -> Self
  {
    self.show_root = show;
    self
  }

  /// Set indentation size in spaces
  #[ must_use ]
  pub fn with_indent_size( mut self, size : usize ) -> Self
  {
    self.indent_size = size;
    self
  }

  /// Set maximum display depth
  #[ must_use ]
  pub fn with_max_depth( mut self, depth : Option< usize > ) -> Self
  {
    self.max_depth = depth;
    self
  }

  /// Set column separator for aligned format
  #[ must_use ]
  pub fn with_column_separator( mut self, separator : String ) -> Self
  {
    self.column_separator = separator;
    self
  }

  /// Set minimum column width for aligned format
  #[ must_use ]
  pub fn with_min_column_width( mut self, width : usize ) -> Self
  {
    self.min_column_width = width;
    self
  }

  /// Set ANSI color code for branch symbols
  #[ must_use ]
  pub fn with_branch_color( mut self, color : impl Into< String > ) -> Self
  {
    self.branch_color = color.into();
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
