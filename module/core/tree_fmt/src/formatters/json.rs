//! JSON formatter for `TableView` data with 2 modes
//!
//! ## Available Modes
//!
//! ### Pretty (default)
//! ```
//! # #[cfg(feature = "format_json")]
//! # {
//! # use tree_fmt::{ RowBuilder, JsonFormatter, Format };
//! # let view = RowBuilder::new(vec!["Name".into()]).add_row(vec!["Alice".into()]).build_view();
//! let formatter = JsonFormatter::new();  // or JsonFormatter::pretty()
//! // [
//! //   {
//! //     "Name": "Alice"
//! //   }
//! // ]
//! # }
//! ```
//!
//! ### Compact
//! ```
//! # #[cfg(feature = "format_json")]
//! # {
//! # use tree_fmt::{ RowBuilder, JsonFormatter, Format };
//! # let view = RowBuilder::new(vec!["Name".into()]).add_row(vec!["Alice".into()]).build_view();
//! let formatter = JsonFormatter::compact();
//! // [{"Name":"Alice"}]
//! # }
//! ```

use crate::{ TableView, formatters::{ Format, FormatError } };
use std::collections::HashMap;

/// JSON output formatter
///
/// Converts `TableView` data to array of objects where each row becomes
/// an object with column names as keys.
///
/// Output format: `[{"col1": "val1", "col2": "val2"}, ...]`
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "format_json")]
/// # {
/// use tree_fmt::{ RowBuilder, JsonFormatter, Format };
///
/// let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
///   .add_row( vec![ "Alice".into(), "30".into() ] )
///   .build_view();
///
/// let formatter = JsonFormatter::new();
/// let json = formatter.format( &view ).unwrap();
/// assert!( json.contains( "\"Name\"" ) );
/// assert!( json.contains( "\"Alice\"" ) );
/// # }
/// ```
#[ derive( Debug, Clone ) ]
pub struct JsonFormatter
{
  /// Pretty-print output with indentation
  pub pretty : bool,
}

impl JsonFormatter
{
  /// Create new JSON formatter with pretty printing enabled
  pub fn new() -> Self
  {
    Self { pretty : true }
  }

  /// Create compact JSON formatter (no pretty printing)
  pub fn compact() -> Self
  {
    Self { pretty : false }
  }

  /// Set pretty printing
  #[ must_use ]
  pub fn with_pretty( mut self, pretty : bool ) -> Self
  {
    self.pretty = pretty;
    self
  }
}

impl Default for JsonFormatter
{
  fn default() -> Self
  {
    Self::new()
  }
}

impl Format for JsonFormatter
{
  /// Format `TableView` as JSON array of row objects.
  ///
  /// ## Output Format Strategy
  ///
  /// This method intentionally **flattens** the `TableView` structure to produce
  /// user-friendly JSON output. Instead of preserving the internal structure
  /// (metadata + rows), it transforms it into an array of objects where each
  /// row becomes a self-describing object with column names as keys.
  ///
  /// **Input** (`TableView` structure):
  /// ```rust,ignore
  /// TableView {
  ///   metadata: TableMetadata {
  ///     column_names: ["Name", "Age", "City"]
  ///   },
  ///   rows: [
  ///     ["Alice", "30", "NYC"],
  ///     ["Bob", "25", "LA"]
  ///   ]
  /// }
  /// ```
  ///
  /// **Output** (flat array):
  /// ```json
  /// [
  ///   {"Name": "Alice", "Age": "30", "City": "NYC"},
  ///   {"Name": "Bob", "Age": "25", "City": "LA"}
  /// ]
  /// ```
  ///
  /// ## Why Flatten?
  ///
  /// 1. **User Friendliness**: Consumers dont need to separately track column
  ///    names and row indices - each object is self-describing
  /// 2. **Standard JSON Pattern**: Array of objects is the most common JSON
  ///    format for tabular data (used by REST APIs, databases, etc.)
  /// 3. **Easier Processing**: Downstream tools can directly access fields by
  ///    name without maintaining separate column metadata
  ///
  /// ## Not Preserving `TableView` Structure
  ///
  /// If you need the complete `TableView` structure (with metadata), serialize
  /// the `TableView` directly using serde instead of using this formatter:
  /// ```rust,ignore
  /// let json = serde_json::to_string(&table_view)?;
  /// ```
  ///
  /// ## Test Coverage
  ///
  /// See `will_crates/tests/crate_formats_test.rs::test_unified_format_interface()`
  /// for comprehensive documentation of this behavior and why the flat format
  /// was chosen over preserving internal structure.
  fn format( &self, data : &TableView ) -> Result< String, FormatError >
  {
    // Convert `TableView` to Vec<HashMap<String, String>>
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

    if self.pretty
    {
      serde_json::to_string_pretty( &rows )
    }
    else
    {
      serde_json::to_string( &rows )
    }
    .map_err( | e | FormatError::Serialization( e.to_string() ) )
  }
}

#[ cfg( test ) ]
mod tests
{
  use super::*;
  use crate::{ RowBuilder, TableMetadata };

  #[ test ]
  fn test_json_formatter_pretty()
  {
    let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
      .add_row( vec![ "Alice".into(), "30".into() ] )
      .add_row( vec![ "Bob".into(), "25".into() ] )
      .build_view();

    let formatter = JsonFormatter::new();
    let json = formatter.format( &view ).unwrap();

    // Should be pretty-printed (contains newlines)
    assert!( json.contains( '\n' ) );

    // Should be array of objects format
    assert!( json.starts_with( '[' ) );
    assert!( json.trim().ends_with( ']' ) );

    // Should contain column names as keys
    assert!( json.contains( "\"Name\"" ) );
    assert!( json.contains( "\"Age\"" ) );

    // Should contain data
    assert!( json.contains( "\"Alice\"" ) );
    assert!( json.contains( "\"30\"" ) );
    assert!( json.contains( "\"Bob\"" ) );
    assert!( json.contains( "\"25\"" ) );
  }

  #[ test ]
  fn test_json_formatter_compact()
  {
    let view = RowBuilder::new( vec![ "A".into() ] )
      .add_row( vec![ "1".into() ] )
      .build_view();

    let formatter = JsonFormatter::compact();
    let json = formatter.format( &view ).unwrap();

    // Compact format should have minimal whitespace (array format)
    let newline_count = json.chars().filter( | c | *c == '\n' ).count();
    assert!( newline_count == 0, "Compact format should have no newlines" );

    // Should be array format: [{"A":"1"}]
    assert!( json.starts_with( '[' ) );
    assert!( json.ends_with( ']' ) );
    assert!( json.contains( "\"A\"" ) );
    assert!( json.contains( "\"1\"" ) );
  }

  #[ test ]
  fn test_json_formatter_empty_table()
  {
    let view = TableView::new(
      TableMetadata::new( vec![ "Column".into() ] ),
      vec![]
    );

    let formatter = JsonFormatter::new();
    let json = formatter.format( &view ).unwrap();

    // Empty table should produce empty array
    assert_eq!( json.trim(), "[]" );
  }

  #[ test ]
  fn test_json_formatter_builder_pattern()
  {
    let formatter = JsonFormatter::new()
      .with_pretty( false );

    assert!( !formatter.pretty );
  }

  #[ test ]
  fn test_json_formatter_output_structure()
  {
    // Verify the output is actually parseable as Vec<HashMap<String, String>>
    let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
      .add_row( vec![ "Alice".into(), "30".into() ] )
      .add_row( vec![ "Bob".into(), "25".into() ] )
      .build_view();

    let formatter = JsonFormatter::new();
    let json_str = formatter.format( &view ).unwrap();

    // Parse back to verify structure
    let parsed : Vec< HashMap< String, String > > = serde_json::from_str( &json_str )
      .expect( "Output should be valid JSON array of objects" );

    assert_eq!( parsed.len(), 2 );
    assert_eq!( parsed[ 0 ][ "Name" ], "Alice" );
    assert_eq!( parsed[ 0 ][ "Age" ], "30" );
    assert_eq!( parsed[ 1 ][ "Name" ], "Bob" );
    assert_eq!( parsed[ 1 ][ "Age" ], "25" );
  }
}
