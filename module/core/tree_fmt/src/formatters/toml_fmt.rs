//! TOML formatter for `TableView` data
//!
//! ## Output Example
//! ```
//! # #[cfg(feature = "format_toml")]
//! # {
//! # use tree_fmt::{ RowBuilder, TomlFormatter, Format };
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
use std::collections::HashMap;

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
/// use tree_fmt::{ RowBuilder, TomlFormatter, Format };
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
    // Convert TableView to Vec<HashMap<String, String>>
    // Each row becomes a table with column names as keys
    let column_names = &data.metadata.column_names;

    let rows : Vec< HashMap< String, String > > = data.rows
      .iter()
      .map( | row |
      {
        column_names
          .iter()
          .zip( row.iter() )
          .map( | ( name, value ) | ( name.clone(), value.clone() ) )
          .collect()
      })
      .collect();

    // TOML requires wrapping in a structure for array of tables
    #[ derive( serde::Serialize ) ]
    struct TomlWrapper
    {
      row : Vec< HashMap< String, String > >,
    }

    let wrapper = TomlWrapper { row : rows };
    toml::to_string( &wrapper )
      .map_err( | e | FormatError::Serialization( e.to_string() ) )
  }
}

#[ cfg( test ) ]
mod tests
{
  use super::*;
  use crate::{ RowBuilder, TableMetadata };

  #[ test ]
  fn test_toml_formatter_basic()
  {
    let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
      .add_row( vec![ "Alice".into(), "30".into() ] )
      .add_row( vec![ "Bob".into(), "25".into() ] )
      .build_view();

    let formatter = TomlFormatter::new();
    let toml_str = formatter.format( &view ).unwrap();

    // Should be TOML array of tables format: [[row]]
    assert!( toml_str.contains( "[[row]]" ) );

    // Should contain column names as keys
    assert!( toml_str.contains( "Name" ) );
    assert!( toml_str.contains( "Age" ) );

    // Should contain data
    assert!( toml_str.contains( "Alice" ) );
    assert!( toml_str.contains( "30" ) );
    assert!( toml_str.contains( "Bob" ) );
    assert!( toml_str.contains( "25" ) );
  }

  #[ test ]
  fn test_toml_formatter_empty_table()
  {
    let view = TableView::new(
      TableMetadata::new( vec![ "Column".into() ] ),
      vec![]
    );

    let formatter = TomlFormatter::new();
    let toml_str = formatter.format( &view ).unwrap();

    // Empty table should produce no [[row]] sections
    assert!( !toml_str.contains( "[[row]]" ) );
  }

  #[ test ]
  fn test_toml_formatter_output_structure()
  {
    // Verify the output is actually parseable TOML with [[row]] structure
    let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
      .add_row( vec![ "Alice".into(), "30".into() ] )
      .add_row( vec![ "Bob".into(), "25".into() ] )
      .build_view();

    let formatter = TomlFormatter::new();
    let toml_str = formatter.format( &view ).unwrap();

    // Parse back to verify structure
    #[ derive( serde::Deserialize ) ]
    struct TomlWrapper
    {
      row : Vec< HashMap< String, String > >,
    }

    let parsed : TomlWrapper = toml::from_str( &toml_str )
      .expect( "Output should be valid TOML with [[row]] tables" );

    assert_eq!( parsed.row.len(), 2 );
    assert_eq!( parsed.row[ 0 ][ "Name" ], "Alice" );
    assert_eq!( parsed.row[ 0 ][ "Age" ], "30" );
    assert_eq!( parsed.row[ 1 ][ "Name" ], "Bob" );
    assert_eq!( parsed.row[ 1 ][ "Age" ], "25" );
  }
}
