//! Unified format trait for all formatters

use crate::TableView;
use error_tools::dependency::thiserror;

/// Error type for formatting operations
#[ derive( thiserror::Error, Debug ) ]
pub enum FormatError
{
  /// Serialization error (requires `serde_support` feature)
  #[ cfg( feature = "serde_support" ) ]
  #[ error( "Serialization error: {0}" ) ]
  Serialization( String ),
  /// Invalid or malformed data
  #[ error( "Invalid data: {0}" ) ]
  InvalidData( String ),
  /// Unsupported operation for this formatter
  #[ error( "Unsupported operation: {0}" ) ]
  UnsupportedOperation( String ),
}

/// Unified formatting interface for all output formats
///
/// All formatters (table, json, yaml, text, etc.) implement this trait
/// to provide a consistent interface for formatting `TableView` data.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "format_table")]
/// # {
/// use data_fmt::{ RowBuilder, TableFormatter, TableConfig, Format };
///
/// let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
///   .add_row( vec![ "Alice".into(), "30".into() ] )
///   .build_view();
///
/// let formatter = TableFormatter::with_config( TableConfig::plain() );
/// let output = Format::format( &formatter, &view ).unwrap();
/// assert!( output.contains( "Name" ) );
/// # }
/// ```
pub trait Format
{
  /// Format the table view to a string
  ///
  /// # Errors
  ///
  /// Returns `FormatError` if formatting fails due to invalid data,
  /// serialization errors, or unsupported operations.
  fn format( &self, data : &TableView ) -> Result< String, FormatError >;
}
