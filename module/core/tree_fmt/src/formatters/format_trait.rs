//! Unified format trait for all formatters

use crate::TableView;

/// Error type for formatting operations
#[ derive( Debug ) ]
pub enum FormatError
{
  /// Serialization error (requires `serde_support` feature)
  #[ cfg( feature = "serde_support" ) ]
  Serialization( String ),
  /// Invalid or malformed data
  InvalidData( String ),
  /// Unsupported operation for this formatter
  UnsupportedOperation( String ),
}

impl std::fmt::Display for FormatError
{
  fn fmt( &self, f : &mut std::fmt::Formatter< '_ > ) -> std::fmt::Result
  {
    match self
    {
      #[ cfg( feature = "serde_support" ) ]
      Self::Serialization( msg ) => write!( f, "Serialization error: {msg}" ),
      Self::InvalidData( msg ) => write!( f, "Invalid data: {msg}" ),
      Self::UnsupportedOperation( msg ) => write!( f, "Unsupported operation: {msg}" ),
    }
  }
}

impl std::error::Error for FormatError {}

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
/// use tree_fmt::{ RowBuilder, TableFormatter, TableConfig, Format };
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
