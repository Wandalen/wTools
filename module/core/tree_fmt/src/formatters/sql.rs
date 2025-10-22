//! SQL INSERT statement formatter
//!
//! ## Purpose
//!
//! Generate SQL INSERT statements from tabular data for:
//! - Database seeding and migrations
//! - ETL pipelines (extract-transform-load)
//! - Test data generation
//! - Quick database loading
//!
//! ## Output Format
//!
//! Multi-row INSERT statement:
//! ```sql
//! INSERT INTO table_name (col1, col2, col3) VALUES
//!   ('value1', 'value2', 'value3'),
//!   ('value4', 'value5', 'value6');
//! ```
//!
//! ## SQL Dialects
//!
//! **`ANSI`** (default) - Standard SQL compliant
//! **`PostgreSQL`** - PostgreSQL-specific features
//! **`MySQL`** - MySQL/MariaDB syntax (backtick identifiers)
//! **`SQLite`** - `SQLite3` syntax
//!
//! ## Escaping
//!
//! Properly escapes SQL special characters:
//! - Single quotes: `'` → `''` (doubled)
//! - Backslashes: `\` → `\\` (`MySQL` only)
//! - NULL handling: Empty strings can become NULL
//!
//! ## Examples
//!
//! ```
//! # use tree_fmt::{ RowBuilder, SqlFormatter, SqlVariant, Format };
//! let view = RowBuilder::new( vec![ "name".into(), "age".into() ] )
//!   .add_row( vec![ "Alice".into(), "30".into() ] )
//!   .add_row( vec![ "Bob".into(), "25".into() ] )
//!   .build_view();
//!
//! let formatter = SqlFormatter::new( "users" );
//! let sql = formatter.format( &view ).unwrap();
//! // INSERT INTO users (name, age) VALUES ('Alice', 30), ('Bob', 25);
//! ```

use crate::{ TableView, formatters::{ Format, FormatError } };

/// SQL dialect for identifier quoting and syntax
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub enum SqlVariant
{
  /// ANSI SQL standard (double quotes for identifiers)
  Ansi,
  /// `PostgreSQL` (double quotes, supports advanced features)
  PostgreSQL,
  /// MySQL/MariaDB (backticks for identifiers)
  MySQL,
  /// `SQLite3`
  SQLite,
}

/// SQL INSERT statement formatter
///
/// Generates multi-row INSERT statements from tabular data.
///
/// # Examples
///
/// ```
/// # use tree_fmt::{ RowBuilder, SqlFormatter, Format };
/// let view = RowBuilder::new( vec![ "product".into(), "price".into() ] )
///   .add_row( vec![ "Widget".into(), "10".into() ] )
///   .add_row( vec![ "Gadget".into(), "20".into() ] )
///   .build_view();
///
/// let formatter = SqlFormatter::new( "products" );
/// let sql = formatter.format( &view ).unwrap();
///
/// assert!( sql.contains( "INSERT INTO \"products\"" ) );
/// assert!( sql.contains( "VALUES" ) );
/// ```
#[ derive( Debug, Clone ) ]
pub struct SqlFormatter
{
  /// Table name for INSERT statement
  pub table_name : String,
  /// SQL variant for formatting
  pub variant : SqlVariant,
  /// Treat empty strings as NULL
  pub empty_as_null : bool,
}

impl SqlFormatter
{
  /// Create new SQL formatter with table name (ANSI variant)
  pub fn new( table_name : impl Into< String > ) -> Self
  {
    Self
    {
      table_name : table_name.into(),
      variant : SqlVariant::Ansi,
      empty_as_null : false,
    }
  }

  /// Create SQL formatter with specific variant
  pub fn with_variant( table_name : impl Into< String >, variant : SqlVariant ) -> Self
  {
    Self
    {
      table_name : table_name.into(),
      variant,
      empty_as_null : false,
    }
  }

  /// Enable/disable empty string to NULL conversion
  #[ must_use ]
  pub fn empty_as_null( mut self, enabled : bool ) -> Self
  {
    self.empty_as_null = enabled;
    self
  }

  /// Quote identifier (table/column name) according to variant
  fn quote_identifier( &self, name : &str ) -> String
  {
    match self.variant
    {
      SqlVariant::Ansi | SqlVariant::PostgreSQL | SqlVariant::SQLite =>
      {
        format!( "\"{}\"", name.replace( '"', "\"\"" ) )
      }
      SqlVariant::MySQL =>
      {
        format!( "`{}`", name.replace( '`', "``" ) )
      }
    }
  }

  /// Escape SQL string value
  fn escape_value( &self, value : &str ) -> String
  {
    // Check if value is numeric (no quotes needed)
    if value.parse::< f64 >().is_ok()
    {
      return value.to_string();
    }

    // Handle NULL
    if self.empty_as_null && value.is_empty()
    {
      return "NULL".to_string();
    }

    // Escape single quotes by doubling
    let escaped = value.replace( '\'', "''" );

    // MySQL: also escape backslashes
    let escaped = if matches!( self.variant, SqlVariant::MySQL )
    {
      escaped.replace( '\\', "\\\\" )
    }
    else
    {
      escaped
    };

    format!( "'{escaped}'" )
  }
}

impl Format for SqlFormatter
{
  fn format( &self, data : &TableView ) -> Result< String, FormatError >
  {
    let mut output = String::new();

    // INSERT INTO table_name
    output.push_str( "INSERT INTO " );
    output.push_str( &self.quote_identifier( &self.table_name ) );
    output.push( ' ' );

    // Column names
    if !data.metadata.column_names.is_empty()
    {
      output.push( '(' );

      for ( idx, col_name ) in data.metadata.column_names.iter().enumerate()
      {
        if idx > 0
        {
          output.push_str( ", " );
        }
        output.push_str( &self.quote_identifier( col_name ) );
      }

      output.push_str( ") " );
    }

    // VALUES clause
    output.push_str( "VALUES" );

    // Data rows
    for ( row_idx, row ) in data.rows.iter().enumerate()
    {
      if row_idx == 0
      {
        output.push_str( "\n  " );
      }
      else
      {
        output.push_str( ",\n  " );
      }

      output.push( '(' );

      for ( cell_idx, cell ) in row.iter().enumerate()
      {
        if cell_idx > 0
        {
          output.push_str( ", " );
        }
        output.push_str( &self.escape_value( cell ) );
      }

      output.push( ')' );
    }

    output.push( ';' );

    Ok( output )
  }
}

#[ cfg( test ) ]
mod tests
{
  use super::*;
  use crate::RowBuilder;

  #[ test ]
  fn test_sql_basic()
  {
    let view = RowBuilder::new( vec![ "name".into(), "age".into() ] )
      .add_row( vec![ "Alice".into(), "30".into() ] )
      .add_row( vec![ "Bob".into(), "25".into() ] )
      .build_view();

    let formatter = SqlFormatter::new( "users" );
    let sql = formatter.format( &view ).unwrap();

    assert!( sql.contains( "INSERT INTO \"users\"" ) );
    assert!( sql.contains( "(\"name\", \"age\")" ) );
    assert!( sql.contains( "VALUES" ) );
    assert!( sql.contains( "('Alice', 30)" ) );
    assert!( sql.contains( "('Bob', 25)" ) );
  }

  #[ test ]
  fn test_sql_quote_escaping()
  {
    let view = RowBuilder::new( vec![ "name".into() ] )
      .add_row( vec![ "O'Brien".into() ] )
      .add_row( vec![ "It's working".into() ] )
      .build_view();

    let formatter = SqlFormatter::new( "people" );
    let sql = formatter.format( &view ).unwrap();

    assert!( sql.contains( "('O''Brien')" ) );
    assert!( sql.contains( "('It''s working')" ) );
  }

  #[ test ]
  fn test_sql_mysql_variant()
  {
    let view = RowBuilder::new( vec![ "name".into() ] )
      .add_row( vec![ "Test".into() ] )
      .build_view();

    let formatter = SqlFormatter::with_variant( "users", SqlVariant::MySQL );
    let sql = formatter.format( &view ).unwrap();

    assert!( sql.contains( "INSERT INTO `users`" ) );
    assert!( sql.contains( "(`name`)" ) );
  }

  #[ test ]
  fn test_sql_numeric_values()
  {
    let view = RowBuilder::new( vec![ "name".into(), "age".into(), "score".into() ] )
      .add_row( vec![ "Alice".into(), "30".into(), "95.5".into() ] )
      .build_view();

    let formatter = SqlFormatter::new( "users" );
    let sql = formatter.format( &view ).unwrap();

    // Numeric values should not be quoted
    assert!( sql.contains( "('Alice', 30, 95.5)" ) );
  }

  #[ test ]
  fn test_sql_empty_as_null()
  {
    let view = RowBuilder::new( vec![ "name".into(), "email".into() ] )
      .add_row( vec![ "Alice".into(), String::new() ] )
      .build_view();

    let formatter = SqlFormatter::new( "users" ).empty_as_null( true );
    let sql = formatter.format( &view ).unwrap();

    assert!( sql.contains( "('Alice', NULL)" ) );
  }

  #[ test ]
  fn test_sql_postgresql_variant()
  {
    let view = RowBuilder::new( vec![ "id".into() ] )
      .add_row( vec![ "1".into() ] )
      .build_view();

    let formatter = SqlFormatter::with_variant( "items", SqlVariant::PostgreSQL );
    let sql = formatter.format( &view ).unwrap();

    assert!( sql.contains( "INSERT INTO \"items\"" ) );
  }

  #[ test ]
  fn test_sql_sqlite_variant()
  {
    let view = RowBuilder::new( vec![ "data".into() ] )
      .add_row( vec![ "test".into() ] )
      .build_view();

    let formatter = SqlFormatter::with_variant( "table", SqlVariant::SQLite );
    let sql = formatter.format( &view ).unwrap();

    assert!( sql.contains( "INSERT INTO \"table\"" ) );
  }
}
