//! `ExpandedFormatter` for vertical record display with 2 distinct styles
//!
//! ## Available Styles
//!
//! ### `PostgreSQL` Style (default)
//! ```
//! # use tree_fmt::{ RowBuilder, ExpandedFormatter };
//! # let tree = RowBuilder::new(vec!["Name".into(), "Age".into()])
//! #   .add_row(vec!["Alice".into(), "30".into()])
//! #   .add_row(vec!["Bob".into(), "25".into()])
//! #   .build();
//! let formatter = ExpandedFormatter::new();
//! // -[ RECORD 1 ]
//! // Name | Alice
//! // Age  | 30
//! // -[ RECORD 2 ]
//! // Name | Bob
//! // Age  | 25
//! ```
//!
//! ### Property List Style
//! ```
//! # use tree_fmt::{ RowBuilder, ExpandedFormatter, ExpandedConfig };
//! # let tree = RowBuilder::new(vec!["Name".into(), "Age".into()])
//! #   .add_row(vec!["Alice".into(), "30".into()])
//! #   .add_row(vec!["Bob".into(), "25".into()])
//! #   .build();
//! let formatter = ExpandedFormatter::with_config( ExpandedConfig::property_style() );
//! // Name: Alice
//! // Age:  30
//! //
//! // Name: Bob
//! // Age:  25
//! ```

use crate::{ TreeNode, ExpandedConfig };
use crate::data::TableShapedView;
use crate::helpers::visual_len;

/// Initial string capacity for expanded output
const INITIAL_CAPACITY : usize = 512;

/// Formatter for rendering data as vertical records (`PostgreSQL` `\x` mode)
///
/// Displays each row as a vertical block of key-value pairs.
/// Similar to `PostgreSQL`'s expanded display mode.
///
/// # Examples
///
/// ```
/// use tree_fmt::{ RowBuilder, ExpandedFormatter };
///
/// let tree = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
///   .add_row( vec![ "Alice".into(), "30".into() ] )
///   .add_row( vec![ "Bob".into(), "25".into() ] )
///   .build();
///
/// let formatter = ExpandedFormatter::new();
/// let output = formatter.format( &tree );
///
/// assert!( output.contains( "-[ RECORD 1 ]" ) );
/// assert!( output.contains( "Name | Alice" ) );
/// ```
#[ derive( Debug ) ]
pub struct ExpandedFormatter
{
  config : ExpandedConfig,
}

impl ExpandedFormatter
{
  /// Create a new expanded formatter with default formatter parameters
  pub fn new() -> Self
  {
    Self
    {
      config : ExpandedConfig::default(),
    }
  }

  /// Create an expanded formatter with custom formatter parameters
  pub const fn with_config( config : ExpandedConfig ) -> Self
  {
    Self
    {
      config,
    }
  }

  /// Format table-shaped tree as vertical records
  ///
  /// Each row is displayed as a separate record block with key-value pairs.
  /// Record names come from row node names in the tree.
  ///
  /// # Examples
  ///
  /// ```
  /// use tree_fmt::{ RowBuilder, ExpandedFormatter };
  ///
  /// let tree = RowBuilder::new( vec![ "ID".into(), "Name".into() ] )
  ///   .add_row( vec![ "1".into(), "Alice".into() ] )
  ///   .build();
  ///
  /// let formatter = ExpandedFormatter::new();
  /// let output = formatter.format( &tree );
  ///
  /// assert!( output.contains( "ID   | 1" ) );
  /// assert!( output.contains( "Name | Alice" ) );
  /// ```
  pub fn format( &self, tree : &TreeNode< String > ) -> String
  {
    let mut output = String::with_capacity( INITIAL_CAPACITY );

    let headers = tree.extract_headers().unwrap_or_default();
    if headers.is_empty()
    {
      return output;
    }

    // Calculate max key width for alignment
    let max_key_width = headers.iter()
      .map( | h | visual_len( h ) )
      .max()
      .unwrap_or( 0 );

    // Format each row as a record
    for ( idx, row_node ) in tree.children.iter().enumerate()
    {
      let record_name = &row_node.name;

      // Record separator (if configured)
      if !self.config.record_separator.is_empty()
      {
        output.push_str( &self.config.record_separator.replace( "{}", record_name ) );
        output.push( '\n' );
      }
      else if idx > 0
      {
        // Blank line between records when no separator header
        output.push( '\n' );
      }

      // Key-value pairs
      for cell in &row_node.children
      {
        let key = &cell.name;
        let value = cell.data.as_ref().map_or( "", String::as_str );
        let key_width = visual_len( key );

        match self.config.padding_side
        {
          crate::PaddingSide::BeforeSeparator =>
          {
            // PostgreSQL style: "Name   | Value"
            let padding_needed = max_key_width - key_width;

            // Apply color to key if enabled
            if self.config.colorize_keys
            {
              output.push_str( &self.config.key_color );
              output.push_str( key );
              output.push_str( &" ".repeat( padding_needed ) );
              output.push_str( "\x1b[0m" );  // Reset before separator
            }
            else
            {
              output.push_str( key );
              output.push_str( &" ".repeat( padding_needed ) );
            }

            output.push_str( &self.config.key_value_separator );
            output.push_str( value );
          }

          crate::PaddingSide::AfterSeparator =>
          {
            // Property style: "Name: Value"
            let padding_needed = max_key_width - key_width;

            // Separate separator into non-space and trailing space parts
            let sep_trimmed = self.config.key_value_separator.trim_end();
            let sep_trailing_spaces = self.config.key_value_separator.len() - sep_trimmed.len();

            // Apply color to key+separator (without trailing spaces) if enabled
            if self.config.colorize_keys
            {
              output.push_str( &self.config.key_color );
              output.push_str( key );
              output.push_str( sep_trimmed );
              output.push_str( "\x1b[0m" );  // Reset after separator
            }
            else
            {
              output.push_str( key );
              output.push_str( sep_trimmed );
            }

            // Add trailing separator spaces + alignment padding
            output.push_str( &" ".repeat( sep_trailing_spaces + padding_needed ) );
            output.push_str( value );
          }
        }

        output.push( '\n' );
      }
    }

    output
  }

  /// Format hierarchical tree as vertical records (flattened)
  ///
  /// Flattens hierarchical tree to table with columns: path, name, depth, data.
  /// Then formats as vertical records.
  ///
  /// # Examples
  ///
  /// ```
  /// use tree_fmt::{ TreeBuilder, ExpandedFormatter };
  ///
  /// let tree = TreeBuilder::new( "root" )
  ///   .insert( &[ "file.txt" ], 100 )
  ///   .build();
  ///
  /// let formatter = ExpandedFormatter::new();
  /// let output = formatter.format_tree( &tree );
  ///
  /// assert!( output.contains( "-[ RECORD" ) );
  /// ```
  pub fn format_tree< T : std::fmt::Display >( &self, tree : &TreeNode< T > ) -> String
  {
    let flattened = crate::conversions::flatten_to_table_tree( tree );
    self.format( &flattened )
  }

  /// Write formatted expanded view directly to a writer
  ///
  /// # Errors
  ///
  /// Returns an error if writing to the provided writer fails
  ///
  /// # Examples
  ///
  /// ```
  /// use tree_fmt::{ RowBuilder, ExpandedFormatter };
  /// use std::io::Cursor;
  ///
  /// let tree = RowBuilder::new( vec![ "Key".into() ] )
  ///   .add_row( vec![ "Value".into() ] )
  ///   .build();
  ///
  /// let formatter = ExpandedFormatter::new();
  /// let mut buffer = Cursor::new( Vec::new() );
  /// formatter.write_to( &tree, &mut buffer ).unwrap();
  ///
  /// let output = String::from_utf8( buffer.into_inner() ).unwrap();
  /// assert!( output.contains( "Value" ) );
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

impl Default for ExpandedFormatter
{
  fn default() -> Self
  {
    Self::new()
  }
}

impl super::TableShapedFormatter for ExpandedFormatter
{
  fn format( &self, tree : &TreeNode< String > ) -> String
  {
    self.format( tree )
  }
}
