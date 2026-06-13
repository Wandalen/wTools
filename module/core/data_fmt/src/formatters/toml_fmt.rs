//! TOML formatter for `TableView` data
//!
//! ## Output Example
//! ```
//! # #[cfg(feature = "format_toml")]
//! # {
//! # use data_fmt::{ RowBuilder, TomlFormatter, Format };
//! # let view = RowBuilder::new(vec!["Name".into(), "Age".into()])
//! #   .add_row(vec!["Alice".into(), "30".into()])
//! #   .build_view();
//! let formatter = TomlFormatter::new();
//! // [[row]]
//! // Name = "Alice"
//! // Age = "30"
//! # }
//! ```

use crate::{ TableView, formatters::{ Format, FormatError } };

/// TOML output formatter
///
/// Converts `TableView` data to array of tables where each row becomes
/// a table with column names as keys.
///
/// Output format: TOML array of tables `[[row]]`
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "format_toml")]
/// # {
/// use data_fmt::{ RowBuilder, TomlFormatter, Format };
///
/// let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
///   .add_row( vec![ "Alice".into(), "30".into() ] )
///   .build_view();
///
/// let formatter = TomlFormatter::new();
/// let toml_str = formatter.format( &view ).unwrap();
/// assert!( toml_str.contains( "Name" ) );
/// # }
/// ```
#[ derive( Debug, Clone ) ]
pub struct TomlFormatter;

impl TomlFormatter
{
  /// Create new TOML formatter
  pub fn new() -> Self
  {
    Self
  }
}

impl Default for TomlFormatter
{
  fn default() -> Self
  {
    Self::new()
  }
}

impl Format for TomlFormatter
{
  fn format( &self, data : &TableView ) -> Result< String, FormatError >
  {
    let rows = super::table_view_to_row_maps( data );

    // TOML requires wrapping in a structure for array of tables
    #[ derive( serde::Serialize ) ]
    struct TomlWrapper
    {
      row : Vec< std::collections::HashMap< String, String > >,
    }

    let wrapper = TomlWrapper { row : rows };
    toml::to_string( &wrapper )
      .map_err( | e | FormatError::Serialization( e.to_string() ) )
  }
}
