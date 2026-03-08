//! Display utilities for configuration output (feature = "display")
//!
//! Provides formatters for configuration data in multiple formats (table, JSON, YAML)

#[ cfg( feature = "display_table" ) ]
pub mod table;

#[ cfg( feature = "display_json" ) ]
pub mod json;

#[ cfg( feature = "display_yaml" ) ]
pub mod yaml;

/// Display options for configuration formatting
#[ derive( Debug, Clone ) ]
pub struct DisplayOptions
{
  /// Filter to single parameter key
  pub filter_key : Option< String >,
  /// Include configuration sources table
  pub include_sources : bool,
  /// Include validation warnings
  pub include_warnings : bool,
}

impl Default for DisplayOptions
{
  #[ inline ]
  fn default() -> Self
  {
    Self
    {
      filter_key : None,
      include_sources : true,
      include_warnings : true,
    }
  }
}
