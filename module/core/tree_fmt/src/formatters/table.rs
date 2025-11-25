//! `TableFormatter` for rendering tabular data with 9 distinct styles
//!
//! ## Available Styles
//!
//! ### Plain (default)
//! ```
//! # use tree_fmt::{ RowBuilder, TableFormatter };
//! # let view = RowBuilder::new(vec!["Name".into(), "Age".into()])
//! #   .add_row(vec!["Alice".into(), "30".into()])
//! #   .build_view();
//! let formatter = TableFormatter::new();
//! // Name   Age
//! // ----   ---
//! // Alice  30
//! ```
//!
//! ### Minimal (no separator)
//! ```
//! # use tree_fmt::{ RowBuilder, TableFormatter, TableConfig };
//! # let view = RowBuilder::new(vec!["Name".into()]).add_row(vec!["Alice".into()]).build_view();
//! let formatter = TableFormatter::with_config( TableConfig::minimal() );
//! // Name
//! // Alice
//! ```
//!
//! ### Bordered
//! ```
//! # use tree_fmt::{ RowBuilder, TableFormatter, TableConfig };
//! # let view = RowBuilder::new(vec!["Name".into()]).add_row(vec!["Alice".into()]).build_view();
//! let formatter = TableFormatter::with_config( TableConfig::bordered() );
//! //  Name  | Age
//! // -------+-----
//! //  Alice |  30
//! ```
//!
//! ### Markdown
//! ```
//! # use tree_fmt::{ RowBuilder, TableFormatter, TableConfig };
//! # let view = RowBuilder::new(vec!["Name".into()]).add_row(vec!["Alice".into()]).build_view();
//! let formatter = TableFormatter::with_config( TableConfig::markdown() );
//! // | Name  | Age |
//! // |-------|-----|
//! // | Alice | 30  |
//! ```
//!
//! ### Grid (ASCII box)
//! ```
//! # use tree_fmt::{ RowBuilder, TableFormatter, TableConfig };
//! # let view = RowBuilder::new(vec!["Name".into()]).add_row(vec!["Alice".into()]).build_view();
//! let formatter = TableFormatter::with_config( TableConfig::grid() );
//! // +-------+-----+
//! // | Name  | Age |
//! // +-------+-----+
//! // | Alice | 30  |
//! // +-------+-----+
//! ```
//!
//! ### Unicode Box
//! ```
//! # use tree_fmt::{ RowBuilder, TableFormatter, TableConfig };
//! # let view = RowBuilder::new(vec!["Name".into()]).add_row(vec!["Alice".into()]).build_view();
//! let formatter = TableFormatter::with_config( TableConfig::unicode_box() );
//! // ┌───────┬─────┐
//! // │ Name  │ Age │
//! // ├───────┼─────┤
//! // │ Alice │ 30  │
//! // └───────┴─────┘
//! ```
//!
//! ### CSV
//! ```
//! # use tree_fmt::{ RowBuilder, TableFormatter, TableConfig };
//! # let view = RowBuilder::new(vec!["Name".into()]).add_row(vec!["Alice".into()]).build_view();
//! let formatter = TableFormatter::with_config( TableConfig::csv() );
//! // Name,Age
//! // Alice,30
//! ```
//!
//! ### TSV
//! ```
//! # use tree_fmt::{ RowBuilder, TableFormatter, TableConfig };
//! # let view = RowBuilder::new(vec!["Name".into()]).add_row(vec!["Alice".into()]).build_view();
//! let formatter = TableFormatter::with_config( TableConfig::tsv() );
//! // Name    Age
//! // Alice    30
//! ```
//!
//! ### Compact
//! ```
//! # use tree_fmt::{ RowBuilder, TableFormatter, TableConfig };
//! # let view = RowBuilder::new(vec!["Name".into()]).add_row(vec!["Alice".into()]).build_view();
//! let formatter = TableFormatter::with_config( TableConfig::compact() );
//! // Name  Age
//! // Alice  30
//! ```

use crate::{ TreeNode, TableConfig };
use crate::data::TableShapedView;
use crate::helpers::{ visual_len, pad_to_width };

/// Initial string capacity for table output
const INITIAL_CAPACITY : usize = 512;

/// Formatter for rendering tabular data as strings
///
/// Provides table rendering with configurable borders, column widths,
/// and alignment. Automatically handles ANSI color codes for proper alignment.
#[ derive( Debug ) ]
pub struct TableFormatter
{
  config : TableConfig,
}

impl Default for TableFormatter
{
  fn default() -> Self
  {
    Self::new()
  }
}

impl TableFormatter
{
  /// Create a new table formatter with default formatter parameters
  ///
  /// # Examples
  ///
  /// ```
  /// use tree_fmt::TableFormatter;
  ///
  /// let formatter = TableFormatter::new();
  /// ```
  pub fn new() -> Self
  {
    Self
    {
      config : TableConfig::default(),
    }
  }

  /// Create a table formatter with custom formatter parameters
  pub const fn with_config( config : TableConfig ) -> Self
  {
    Self
    {
      config,
    }
  }

  /// Format table-shaped tree
  ///
  /// Extracts headers and rows from tree using `TableView` trait.
  ///
  /// # Examples
  ///
  /// ```
  /// use tree_fmt::{ RowBuilder, TableFormatter };
  ///
  /// let tree = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
  ///   .add_row( vec![ "Alice".into(), "30".into() ] )
  ///   .build();
  ///
  /// let formatter = TableFormatter::new();
  /// let output = formatter.format( &tree );
  ///
  /// assert!( output.contains( "Name" ) );
  /// assert!( output.contains( "Alice" ) );
  /// ```
  pub fn format( &self, tree : &TreeNode< String > ) -> String
  {
    let headers = tree.extract_headers().unwrap_or_default();
    let rows = tree.to_rows();
    self.format_internal( &headers, &rows )
  }

  /// Format hierarchical tree as table (flattened)
  ///
  /// Flattens hierarchical tree to table with columns: path, name, depth, data.
  ///
  /// # Examples
  ///
  /// ```
  /// use tree_fmt::{ TreeBuilder, TableFormatter };
  ///
  /// let tree = TreeBuilder::new( "root" )
  ///   .insert( &[ "file.txt" ], 100 )
  ///   .build();
  ///
  /// let formatter = TableFormatter::new();
  /// let output = formatter.format_tree( &tree );
  ///
  /// assert!( output.contains( "path" ) );
  /// assert!( output.contains( "file.txt" ) );
  /// ```
  pub fn format_tree< T : std::fmt::Display >( &self, tree : &TreeNode< T > ) -> String
  {
    let flattened = crate::conversions::flatten_to_table_tree( tree );
    self.format( &flattened )
  }

  /// Internal implementation of table formatting
  fn format_internal( &self, headers : &[ String ], rows : &[ Vec< String > ] ) -> String
  {
    let mut output = String::with_capacity( INITIAL_CAPACITY );

    // Calculate column widths
    let column_widths = self.calculate_column_widths_for_rows( headers, rows );

    // Format header row
    self.format_row( &mut output, headers, &column_widths, true );

    // Format header separator
    self.format_header_separator( &mut output, &column_widths );

    // Format data rows
    for row in rows
    {
      self.format_row( &mut output, row, &column_widths, false );
    }

    output
  }

  /// Format a single row (header or data)
  ///
  /// ## Border Consistency with Header Separator
  ///
  /// This function ensures row formatting is consistent with the header separator style.
  /// When `header_separator` is `AsciiGrid` or `Markdown`, the separator line includes
  /// leading and trailing pipes (`|...|`). To maintain visual consistency, data rows
  /// MUST also have leading and trailing pipes.
  ///
  /// **Bug History**: Previously, rows only had column separators between cells, while
  /// the header separator had border pipes. This created misaligned output:
  /// ```text
  ///  Crate   |Type   |Name        (no leading/trailing pipes)
  /// |---------|--------|---------  (has pipes!)
  ///  mindful |Binary |mindful     (no leading/trailing pipes)
  /// ```
  ///
  /// **Fix**: Check `header_separator` style and add border pipes when needed:
  /// ```text
  /// | Crate   |Type   |Name       |  (has pipes)
  /// |---------|--------|---------  |  (has pipes)
  /// | mindful |Binary |mindful    |  (has pipes)
  /// ```
  ///
  /// This ensures consistency across all table styles without breaking CSV/TSV formats.
  fn format_row(
    &self,
    output : &mut String,
    cells : &[ String ],
    column_widths : &[ usize ],
    _is_header : bool
  )
  {
    use crate::config::HeaderSeparatorVariant;

    // CSV/TSV formats should NOT pad cells for alignment
    let should_pad = !matches!(
      &self.config.column_separator,
      crate::config::ColumnSeparator::Character( ',' | '\t' )
    );

    // Add leading border pipe if header separator variant uses pipes.
    // This maintains consistency with header separator formatting - if the separator
    // line has pipes, all rows must also have pipes for proper alignment.
    let needs_border_pipes = matches!(
      self.config.header_separator_variant,
      HeaderSeparatorVariant::AsciiGrid | HeaderSeparatorVariant::Markdown
    );

    if needs_border_pipes
    {
      output.push( '|' );
    }

    for ( idx, cell ) in cells.iter().enumerate()
    {
      let width = column_widths.get( idx ).copied().unwrap_or( 10 );
      let align_right = self.config.align_right.get( idx ).copied().unwrap_or( false );

      // Add padding before cell if outer_padding enabled (skip for CSV/TSV)
      if idx == 0 && self.config.outer_padding && should_pad
      {
        output.push_str( &" ".repeat( self.config.inner_padding ) );
      }

      // Render cell content (padded for aligned formats, raw for CSV/TSV)
      if should_pad
      {
        output.push_str( &pad_to_width( cell, width, align_right ) );
      }
      else
      {
        output.push_str( cell );
      }

      // Add column separator (except after last column)
      if idx < cells.len() - 1
      {
        self.append_column_separator( output );
      }

      // Add padding after last cell if outer_padding enabled (skip for CSV/TSV)
      if idx == cells.len() - 1 && self.config.outer_padding && should_pad
      {
        output.push_str( &" ".repeat( self.config.inner_padding ) );
      }
    }

    // Add trailing border pipe if header separator style uses pipes
    if needs_border_pipes
    {
      output.push( '|' );
    }

    output.push( '\n' );
  }

  /// Append column separator based on formatter parameters
  fn append_column_separator( &self, output : &mut String )
  {
    match &self.config.column_separator
    {
      crate::config::ColumnSeparator::Spaces( n ) =>
      {
        output.push_str( &" ".repeat( *n ) );
      }
      crate::config::ColumnSeparator::Character( ch ) =>
      {
        output.push( *ch );
      }
      crate::config::ColumnSeparator::String( s ) =>
      {
        output.push_str( s );
      }
    }
  }

  /// Format header separator line
  fn format_header_separator( &self, output : &mut String, column_widths : &[ usize ] )
  {
    use crate::config::HeaderSeparatorVariant;

    match self.config.header_separator_variant
    {
      HeaderSeparatorVariant::None =>
      {
        // No separator
      }
      HeaderSeparatorVariant::Dash =>
      {
        // Plain dashes under each column
        for ( idx, &width ) in column_widths.iter().enumerate()
        {
          if idx == 0 && self.config.outer_padding
          {
            output.push_str( &" ".repeat( self.config.inner_padding ) );
          }

          output.push_str( &"-".repeat( width ) );

          if idx < column_widths.len() - 1
          {
            self.append_column_separator( output );
          }

          if idx == column_widths.len() - 1 && self.config.outer_padding
          {
            output.push_str( &" ".repeat( self.config.inner_padding ) );
          }
        }
        output.push( '\n' );
      }
      HeaderSeparatorVariant::AsciiGrid =>
      {
        // Pipe-separated dashes matching cell format: |-----|-----|
        output.push( '|' );
        for ( idx, &width ) in column_widths.iter().enumerate()
        {
          // Leading padding for first column
          if idx == 0 && self.config.outer_padding
          {
            output.push_str( &"-".repeat( self.config.inner_padding ) );
          }

          // Dashes for content width
          output.push_str( &"-".repeat( width ) );

          // Trailing padding for last column (before the pipe!)
          if idx == column_widths.len() - 1 && self.config.outer_padding
          {
            output.push_str( &"-".repeat( self.config.inner_padding ) );
          }

          // Column separator as pipe (after all content)
          output.push( '|' );
        }
        output.push( '\n' );
      }
      HeaderSeparatorVariant::Unicode =>
      {
        // Unicode box separator: ├─────┼─────┤
        output.push( '├' );
        for ( idx, &width ) in column_widths.iter().enumerate()
        {
          output.push_str( &"─".repeat( width + 2 ) );
          if idx < column_widths.len() - 1
          {
            output.push( '┼' );
          }
        }
        output.push( '┤' );
        output.push( '\n' );
      }
      HeaderSeparatorVariant::Markdown =>
      {
        // Markdown separator: |-----|-----|
        output.push( '|' );
        for &width in column_widths
        {
          output.push_str( &"-".repeat( width + 2 ) );
          output.push( '|' );
        }
        output.push( '\n' );
      }
    }
  }

  /// Calculate column widths based on content
  ///
  /// Uses `visual_len()` to properly handle ANSI color codes in cell content.
  fn calculate_column_widths_for_rows(
    &self,
    headers : &[ String ],
    rows : &[ Vec< String > ]
  )
  -> Vec< usize >
  {
    // Use provided widths if available
    if !self.config.column_widths.is_empty()
    {
      return self.config.column_widths.clone();
    }

    // Auto-calculate based on content
    let mut widths = vec![ 0; headers.len() ];

    // Consider header widths (using visual length for ANSI-aware calculation)
    for ( idx, header ) in headers.iter().enumerate()
    {
      widths[ idx ] = visual_len( header );
    }

    // Consider row widths (using visual length for ANSI-aware calculation)
    for row in rows
    {
      for ( idx, cell ) in row.iter().enumerate()
      {
        if idx < widths.len()
        {
          widths[ idx ] = widths[ idx ].max( visual_len( cell ) );
        }
      }
    }

    widths
  }

  /// Write formatted table directly to a writer
  ///
  /// # Errors
  ///
  /// Returns an error if writing to the provided writer fails
  ///
  /// # Examples
  ///
  /// ```
  /// use tree_fmt::{ RowBuilder, TableFormatter };
  /// use std::io::Cursor;
  ///
  /// let tree = RowBuilder::new( vec![ "Name".into() ] )
  ///   .add_row( vec![ "Alice".into() ] )
  ///   .build();
  ///
  /// let formatter = TableFormatter::new();
  /// let mut buffer = Cursor::new( Vec::new() );
  /// formatter.write_to( &tree, &mut buffer ).unwrap();
  ///
  /// let output = String::from_utf8( buffer.into_inner() ).unwrap();
  /// assert!( output.contains( "Alice" ) );
  /// ```
  pub fn write_to< W : std::io::Write >(
    &self,
    tree : &TreeNode< String >,
    writer : &mut W
  )
  -> std::io::Result< () >
  {
    let output = self.format( tree );
    writer.write_all( output.as_bytes() )
  }
}

impl super::TableShapedFormatter for TableFormatter
{
  fn format( &self, tree : &TreeNode< String > ) -> String
  {
    self.format( tree )
  }
}

// Implement unified Format trait for TableView
impl super::Format for TableFormatter
{
  fn format( &self, data : &crate::TableView ) -> Result< String, super::FormatError >
  {
    Ok( self.format_internal( &data.metadata.column_names, &data.rows ) )
  }
}
