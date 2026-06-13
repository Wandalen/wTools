//! YAML formatter for `TableView` data
//!
//! ## Output Example
//! ```
//! # #[cfg(feature = "format_yaml")]
//! # {
//! # use data_fmt::{ RowBuilder, YamlFormatter, Format };
//! # let view = RowBuilder::new(vec!["Name".into(), "Age".into()])
//! #   .add_row(vec!["Alice".into(), "30".into()])
//! #   .build_view();
//! let formatter = YamlFormatter::new();
//! // - Name: Alice
//! //   Age: '30'
//! # }
//! ```

use crate::{ TableView, formatters::{ Format, FormatError } };

/// YAML output formatter
///
/// Converts `TableView` data to array of objects where each row becomes
/// an object with column names as keys.
///
/// Output format: list of dictionaries in YAML
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "format_yaml")]
/// # {
/// use data_fmt::{ RowBuilder, YamlFormatter, Format };
///
/// let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
///   .add_row( vec![ "Alice".into(), "30".into() ] )
///   .build_view();
///
/// let formatter = YamlFormatter::new();
/// let yaml = formatter.format( &view ).unwrap();
/// assert!( yaml.contains( "Name:" ) );
/// assert!( yaml.contains( "Alice" ) );
/// # }
/// ```
#[ derive( Debug, Clone ) ]
pub struct YamlFormatter;

impl YamlFormatter
{
  /// Create new YAML formatter
  pub fn new() -> Self
  {
    Self
  }
}

impl Default for YamlFormatter
{
  fn default() -> Self
  {
    Self::new()
  }
}

impl Format for YamlFormatter
{
  fn format( &self, data : &TableView ) -> Result< String, FormatError >
  {
    let rows = super::table_view_to_row_maps( data );

    serde_yaml_ng::to_string( &rows )
      .map_err( | e | FormatError::Serialization( e.to_string() ) )
  }
}
