//! Logfmt structured logging formatter
//!
//! ## What is Logfmt?
//!
//! Logfmt is a simple text format for structured logging data, originally
//! popularized by Heroku and widely adopted in observability tools.
//!
//! Each table row becomes one line of space-separated `key=value` pairs,
//! making logs both human-readable and machine-parseable.
//!
//! ## Format Specification
//!
//! - **One line per row**: Each data row becomes a single logfmt line
//! - **Space-separated pairs**: Fields separated by single space
//! - **Key=value format**: Header names become keys, cell values become values
//! - **Automatic escaping**: Values with special characters are properly escaped
//!
//! ## Escaping Rules
//!
//! Values are escaped according to logfmt specification:
//! - Contains space/tab → wrap entire value in double quotes
//! - Contains double quote → escape with backslash: `\"`
//! - Contains newline → replace with literal `\n`
//! - Simple values (no special chars) → output as-is
//!
//! ## Use Cases
//!
//! - **Application logging**: Structured log output for observability
//! - **Log aggregation**: Format compatible with Loki, Elasticsearch, Splunk
//! - **Grep-friendly logs**: Easy to search with standard Unix tools
//! - **CLI tool output**: Machine-parseable command output
//! - **Metric export**: Export metrics in logfmt for ingestion
//!
//! ## Examples
//!
//! ```
//! # use data_fmt::{ RowBuilder, LogfmtFormatter, Format };
//! let view = RowBuilder::new( vec![ "timestamp".into(), "level".into(), "msg".into() ] )
//!   .add_row( vec![
//!     "2025-01-15T10:30:00Z".into(),
//!     "info".into(),
//!     "user login".into()
//!   ])
//!   .build_view();
//!
//! let formatter = LogfmtFormatter::new();
//! let output = formatter.format( &view ).unwrap();
//!
//! // Output: timestamp=2025-01-15T10:30:00Z level=info msg="user login"
//! assert!( output.contains( "timestamp=2025-01-15T10:30:00Z" ) );
//! assert!( output.contains( "level=info" ) );
//! ```
//!
//! ## Why Logfmt?
//!
//! **Simplicity**: No complex parsing - just split by space, then by `=`
//! **Human-readable**: Can be read in raw form unlike JSON/binary formats
//! **Tool-friendly**: Works with grep, awk, sed without special processing
//! **Streaming**: Can be parsed line-by-line with constant memory
//! **Standardized**: Widely supported by logging infrastructure

use crate::{ TableView, formatters::{ Format, FormatError } };

/// Formatter parameters for logfmt output
///
/// Formats table data as logfmt structured logging output where each row
/// becomes one line of space-separated key=value pairs.
///
/// # Examples
///
/// ```
/// # use data_fmt::{ RowBuilder, LogfmtFormatter, Format };
/// let view = RowBuilder::new( vec![ "name".into(), "status".into() ] )
///   .add_row( vec![ "server1".into(), "ok".into() ] )
///   .add_row( vec![ "server2".into(), "error".into() ] )
///   .build_view();
///
/// let formatter = LogfmtFormatter::new();
/// let output = formatter.format( &view ).unwrap();
///
/// assert!( output.contains( "name=server1 status=ok" ) );
/// assert!( output.contains( "name=server2 status=error" ) );
/// ```
#[ derive( Debug, Clone ) ]
pub struct LogfmtFormatter
{
  /// Separator between key and value (default: "=")
  pub key_value_separator : String,
  /// Separator between fields (default: " ")
  pub field_separator : String,
}

impl LogfmtFormatter
{
  /// Create new logfmt formatter with default formatter parameters
  ///
  /// Uses `=` for key-value separator and space for field separator.
  pub fn new() -> Self
  {
    Self
    {
      key_value_separator : "=".to_string(),
      field_separator : " ".to_string(),
    }
  }

  /// Create logfmt formatter with custom key-value separator
  ///
  /// # Examples
  ///
  /// ```
  /// # use data_fmt::{ RowBuilder, LogfmtFormatter, Format };
  /// let formatter = LogfmtFormatter::with_separator( ":" );
  /// ```
  pub fn with_separator( separator : impl Into< String > ) -> Self
  {
    Self
    {
      key_value_separator : separator.into(),
      field_separator : " ".to_string(),
    }
  }

  /// Escape value according to logfmt specification
  ///
  /// Escaping rules:
  /// - Values with space/tab → wrap in double quotes
  /// - Values with quotes → escape with backslash
  /// - Values with newlines → replace with literal \n
  fn escape_value( value : &str ) -> String
  {
    // Check if escaping is needed
    let needs_quotes = value.contains( ' ' ) || value.contains( '\t' );
    let has_quotes = value.contains( '"' );
    let has_newlines = value.contains( '\n' );

    // Fast path: no escaping needed
    if !needs_quotes && !has_quotes && !has_newlines
    {
      return value.to_string();
    }

    // Build escaped value
    let mut escaped = String::with_capacity( value.len() + 10 );

    // Escape quotes and wrap if needed
    if has_quotes || needs_quotes || has_newlines
    {
      escaped.push( '"' );
      for ch in value.chars()
      {
        if ch == '"'
        {
          escaped.push( '\\' );
          escaped.push( '"' );
        }
        else if ch == '\n'
        {
          escaped.push( '\\' );
          escaped.push( 'n' );
        }
        else
        {
          escaped.push( ch );
        }
      }
      escaped.push( '"' );
    }
    else
    {
      escaped.push_str( value );
    }

    escaped
  }
}

impl Default for LogfmtFormatter
{
  fn default() -> Self
  {
    Self::new()
  }
}

impl Format for LogfmtFormatter
{
  fn format( &self, data : &TableView ) -> Result< String, FormatError >
  {
    let mut output = String::new();

    for row in &data.rows
    {
      let mut fields = Vec::new();

      for ( col_idx, cell ) in row.iter().enumerate()
      {
        if col_idx < data.metadata.column_names.len()
        {
          let key = &data.metadata.column_names[ col_idx ];
          let value = Self::escape_value( &cell.text );
          fields.push( format!(
            "{}{}{}",
            key,
            self.key_value_separator,
            value
          ) );
        }
      }

      output.push_str( &fields.join( &self.field_separator ) );
      output.push( '\n' );
    }

    Ok( output )
  }
}
