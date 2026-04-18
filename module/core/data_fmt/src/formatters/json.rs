//! JSON formatter for `TableView` data with 2 modes
//!
//! ## Available Modes
//!
//! ### Pretty (default)
//! ```
//! # #[cfg(feature = "format_json")]
//! # {
//! # use data_fmt::{ RowBuilder, JsonFormatter, Format };
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
//! # use data_fmt::{ RowBuilder, JsonFormatter, Format };
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
/// use data_fmt::{ RowBuilder, JsonFormatter, Format };
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
  /// ```text
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
  /// ```text
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
          .map( | ( name, value ) | ( name.clone(), value.text.clone() ) )
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
