//! Expanded (vertical record) formatter configuration types

use super::PaddingSide;

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
  /// ANSI color code for keys; pre-loaded with `"\x1b[90m"` (gray) by default.
  /// Override via `.key_color()` builder or set to `String::new()` to disable.
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
      key_color : "\x1b[90m".to_string(),  // Gray — default color when colorize_keys is enabled
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
      key_color : "\x1b[90m".to_string(),  // Gray — default color when colorize_keys is enabled
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
