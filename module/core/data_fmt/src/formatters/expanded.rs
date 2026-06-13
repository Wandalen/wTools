//! `ExpandedFormatter` for vertical record display with 2 distinct styles
//!
//! ## Available Styles
//!
//! ### `PostgreSQL` Style (default)
//! ```
//! # use data_fmt::{ RowBuilder, ExpandedFormatter, Format };
//! # let view = RowBuilder::new(vec!["Name".into(), "Age".into()])
//! #   .add_row(vec!["Alice".into(), "30".into()])
//! #   .add_row(vec!["Bob".into(), "25".into()])
//! #   .build_view();
//! let formatter = ExpandedFormatter::new();
//! # let _ = formatter.format( &view );
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
//! # use data_fmt::{ RowBuilder, ExpandedFormatter, ExpandedConfig, Format };
//! # let view = RowBuilder::new(vec!["Name".into(), "Age".into()])
//! #   .add_row(vec!["Alice".into(), "30".into()])
//! #   .add_row(vec!["Bob".into(), "25".into()])
//! #   .build_view();
//! let formatter = ExpandedFormatter::with_config( ExpandedConfig::property_style() );
//! # let _ = formatter.format( &view );
//! // Name: Alice
//! // Age:  30
//! //
//! // Name: Bob
//! // Age:  25
//! ```

use crate::{ TreeNode, ExpandedConfig, TableView };
use crate::ansi_str::visual_len;
use color_tools::DecoratedText;

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
/// use data_fmt::{ RowBuilder, ExpandedFormatter, Format };
///
/// let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
///   .add_row( vec![ "Alice".into(), "30".into() ] )
///   .add_row( vec![ "Bob".into(), "25".into() ] )
///   .build_view();
///
/// let formatter = ExpandedFormatter::new();
/// let output = formatter.format( &view ).unwrap_or_default();
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

  /// Format hierarchical tree as vertical records (flattened)
  ///
  /// Flattens hierarchical tree to table with columns: path, name, depth, data.
  /// Then formats as vertical records.
  ///
  /// # Examples
  ///
  /// ```
  /// use data_fmt::{ TreeBuilder, ExpandedFormatter };
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
    super::Format::format( self, &flattened ).unwrap_or_default()
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
  /// use data_fmt::{ RowBuilder, ExpandedFormatter };
  /// use std::io::Cursor;
  ///
  /// let view = RowBuilder::new( vec![ "Key".into() ] )
  ///   .add_row( vec![ "Value".into() ] )
  ///   .build_view();
  ///
  /// let formatter = ExpandedFormatter::new();
  /// let mut buffer = Cursor::new( Vec::new() );
  /// formatter.write_to( &view, &mut buffer ).unwrap();
  ///
  /// let output = String::from_utf8( buffer.into_inner() ).unwrap();
  /// assert!( output.contains( "Value" ) );
  /// ```
  pub fn write_to< W : std::io::Write >(
    &self,
    data : &TableView,
    writer : &mut W
  )
  -> std::io::Result< () >
  {
    let output = self.format_view( data );
    writer.write_all( output.as_bytes() )
  }

  fn format_view( &self, data : &TableView ) -> String
  {
    let headers = &data.metadata.column_names;
    if headers.is_empty()
    {
      return String::new();
    }

    let max_key_width = headers.iter()
      .map( | h | visual_len( h ) )
      .max()
      .unwrap_or( 0 );

    let mut output = String::with_capacity( INITIAL_CAPACITY );

    for ( idx, row ) in data.rows.iter().enumerate()
    {
      let record_name = ( idx + 1 ).to_string();

      if !self.config.record_separator.is_empty()
      {
        let record_label = if self.config.show_record_numbers { record_name.as_str() } else { "" };
        output.push_str( &self.config.record_separator.replace( "{}", record_label ) );
        output.push( '\n' );
      }
      else if idx > 0
      {
        output.push( '\n' );
      }

      for ( key, cell ) in headers.iter().zip( row.iter() )
      {
        output.push_str( &self.config.indent_prefix );
        // Fix(BUG-012): call .render() to get color+text+RESET, not raw .data access.
        // Root cause: .data.as_ref().map_or("", ..) returned bare text, bypassing ANSI
        //   color wrapping and emitting plain strings for colored DecoratedText cells.
        // Pitfall: never access .data directly for rendering — .render() is the only
        //   correct output path as it handles both plain and colored cell values.
        let value = cell.render();
        let key_width = visual_len( key );

        match self.config.padding_side
        {
          crate::PaddingSide::BeforeSeparator =>
          {
            let padding_needed = max_key_width - key_width;

            if self.config.colorize_keys && !self.config.key_color.is_empty()
            {
              let key_padded = format!( "{}{}", key, " ".repeat( padding_needed ) );
              output.push_str( &DecoratedText::from( key_padded ).with_color( self.config.key_color.clone() ).render() );
            }
            else
            {
              output.push_str( key );
              output.push_str( &" ".repeat( padding_needed ) );
            }

            output.push_str( &self.config.key_value_separator );
            output.push_str( &value );
          }

          crate::PaddingSide::AfterSeparator =>
          {
            let padding_needed = max_key_width - key_width;
            let sep_trimmed = self.config.key_value_separator.trim_end();
            let sep_trailing_spaces = self.config.key_value_separator.len() - sep_trimmed.len();

            if self.config.colorize_keys && !self.config.key_color.is_empty()
            {
              let key_with_sep = format!( "{key}{sep_trimmed}" );
              output.push_str( &DecoratedText::from( key_with_sep ).with_color( self.config.key_color.clone() ).render() );
            }
            else
            {
              output.push_str( key );
              output.push_str( sep_trimmed );
            }

            output.push_str( &" ".repeat( sep_trailing_spaces + padding_needed ) );
            output.push_str( &value );
          }
        }

        output.push( '\n' );
      }
    }

    output
  }
}

impl Default for ExpandedFormatter
{
  fn default() -> Self
  {
    Self::new()
  }
}

impl super::Format for ExpandedFormatter
{
  fn format( &self, data : &TableView ) -> Result< String, super::FormatError >
  {
    Ok( self.format_view( data ) )
  }
}

