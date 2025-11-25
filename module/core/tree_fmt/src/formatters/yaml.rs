//! YAML formatter for `TableView` data
//!
//! ## Output Example
//! ```
//! # #[cfg(feature = "format_yaml")]
//! # {
//! # use tree_fmt::{ RowBuilder, YamlFormatter, Format };
//! # let view = RowBuilder::new(vec!["Name".into(), "Age".into()])
//! #   .add_row(vec!["Alice".into(), "30".into()])
//! #   .build_view();
//! let formatter = YamlFormatter::new();
//! // - Name: Alice
//! //   Age: '30'
//! # }
//! ```

use crate::{ TableView, formatters::{ Format, FormatError } };
use std::collections::HashMap;

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
/// use tree_fmt::{ RowBuilder, YamlFormatter, Format };
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
    // Convert TableView to Vec<HashMap<String, String>>
    // Each row becomes an object with column names as keys
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

    serde_yaml::to_string( &rows )
      .map_err( | e | FormatError::Serialization( e.to_string() ) )
  }
}

#[ cfg( test ) ]
mod tests
{
  use super::*;
  use crate::{ RowBuilder, TableMetadata };

  #[ test ]
  fn test_yaml_formatter_basic()
  {
    let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
      .add_row( vec![ "Alice".into(), "30".into() ] )
      .add_row( vec![ "Bob".into(), "25".into() ] )
      .build_view();

    let formatter = YamlFormatter::new();
    let yaml = formatter.format( &view ).unwrap();

    // Should be YAML list (starts with -)
    assert!( yaml.contains( '-' ) );

    // Should contain column names as keys
    assert!( yaml.contains( "Name:" ) );
    assert!( yaml.contains( "Age:" ) );

    // Should contain data
    assert!( yaml.contains( "Alice" ) );
    assert!( yaml.contains( "'30'" ) || yaml.contains( "\"30\"" ) || yaml.contains( "30" ) );
    assert!( yaml.contains( "Bob" ) );

    // Should NOT have JSON brackets (YAML uses dashes for lists)
    assert!( !yaml.contains( '{' ) );
  }

  #[ test ]
  fn test_yaml_formatter_empty_table()
  {
    let view = TableView::new(
      TableMetadata::new( vec![ "Column".into() ] ),
      vec![]
    );

    let formatter = YamlFormatter::new();
    let yaml = formatter.format( &view ).unwrap();

    // Empty table should produce empty array: []
    assert!( yaml.trim() == "[]" );
  }

  #[ test ]
  fn test_yaml_formatter_special_characters()
  {
    let view = RowBuilder::new( vec![ "Key".into() ] )
      .add_row( vec![ "value: with colon".into() ] )
      .build_view();

    let formatter = YamlFormatter::new();
    let yaml = formatter.format( &view ).unwrap();

    // YAML should properly escape/quote strings with colons
    assert!( yaml.contains( "Key:" ) );
    assert!( yaml.contains( "value: with colon" ) || yaml.contains( "'value: with colon'" ) || yaml.contains( "\"value: with colon\"" ) );
  }

  #[ test ]
  fn test_yaml_formatter_output_structure()
  {
    // Verify the output is actually parseable as Vec<HashMap<String, String>>
    let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
      .add_row( vec![ "Alice".into(), "30".into() ] )
      .add_row( vec![ "Bob".into(), "25".into() ] )
      .build_view();

    let formatter = YamlFormatter::new();
    let yaml_str = formatter.format( &view ).unwrap();

    // Parse back to verify structure
    let parsed : Vec< HashMap< String, String > > = serde_yaml::from_str( &yaml_str )
      .expect( "Output should be valid YAML list of objects" );

    assert_eq!( parsed.len(), 2 );
    assert_eq!( parsed[ 0 ][ "Name" ], "Alice" );
    assert_eq!( parsed[ 0 ][ "Age" ], "30" );
    assert_eq!( parsed[ 1 ][ "Name" ], "Bob" );
    assert_eq!( parsed[ 1 ][ "Age" ], "25" );
  }
}
