//! Text formatter for `TableView` data with 6 distinct styles
//!
//! ## Available Styles
//!
//! ### Bullets
//! ```
//! # #[cfg(feature = "format_text")]
//! # {
//! # use tree_fmt::{ RowBuilder, TextFormatter, TextVariant, Format };
//! # let view = RowBuilder::new(vec!["Name".into()]).add_row(vec!["Alice".into()]).build_view();
//! let formatter = TextFormatter::new(TextVariant::Bullets);
//! // • Alice
//! # }
//! ```
//!
//! ### Numbered
//! ```
//! # #[cfg(feature = "format_text")]
//! # {
//! # use tree_fmt::{ RowBuilder, TextFormatter, TextVariant, Format };
//! # let view = RowBuilder::new(vec!["Name".into()]).add_row(vec!["Alice".into()]).build_view();
//! let formatter = TextFormatter::new(TextVariant::Numbered);
//! // 1. Alice
//! # }
//! ```
//!
//! ### Sections
//! ```
//! # #[cfg(feature = "format_text")]
//! # {
//! # use tree_fmt::{ RowBuilder, TextFormatter, TextVariant, Format };
//! # let view = RowBuilder::new(vec!["Name".into()]).add_row(vec!["Alice".into()]).build_view();
//! let formatter = TextFormatter::new(TextVariant::Sections);
//! // == Row 1 ==
//! // Alice
//! # }
//! ```
//!
//! ### `KeyValue`
//! ```
//! # #[cfg(feature = "format_text")]
//! # {
//! # use tree_fmt::{ RowBuilder, TextFormatter, TextVariant, Format };
//! # let view = RowBuilder::new(vec!["Name".into()]).add_row(vec!["Alice".into()]).build_view();
//! let formatter = TextFormatter::new(TextVariant::KeyValue);
//! // Name: Alice
//! # }
//! ```
//!
//! ### Compact
//! ```
//! # #[cfg(feature = "format_text")]
//! # {
//! # use tree_fmt::{ RowBuilder, TextFormatter, TextVariant, Format };
//! # let view = RowBuilder::new(vec!["Name".into()]).add_row(vec!["Alice".into()]).build_view();
//! let formatter = TextFormatter::new(TextVariant::Compact);
//! // Alice
//! # }
//! ```
//!
//! ### `CliHelp`
//! ```
//! # #[cfg(feature = "format_text")]
//! # {
//! # use tree_fmt::{ RowBuilder, TextFormatter, TextVariant, Format };
//! let view = RowBuilder::new(vec!["Term".into(), "Description".into()])
//!   .add_row(vec!["USAGE".into(), "".into()])
//!   .add_row(vec!["command [options]".into(), "".into()])
//!   .add_row(vec!["OPTIONS".into(), "".into()])
//!   .add_row(vec!["--verbose".into(), "Enable verbose output".into()])
//!   .build_view();
//! let formatter = TextFormatter::new(TextVariant::CliHelp);
//! let output = formatter.format(&view).unwrap();
//! // USAGE:
//! //   command [options]
//! //
//! // OPTIONS:
//! //   --verbose  Enable verbose output
//! # }
//! ```

use crate::{ TableView, formatters::{ Format, FormatError } };

/// Text output style
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub enum TextVariant
{
  /// Bullet list format: • item
  Bullets,
  /// Numbered list format: 1. item
  Numbered,
  /// Section format with headers
  Sections,
  /// Key-value pairs: key: value
  KeyValue,
  /// Compact comma-separated: item, item, item
  Compact,
  /// CLI help text format with section headers and aligned descriptions
  CliHelp,
}

/// Text output formatter with configurable style
///
/// Formats `TableView` data as human-readable text in various styles.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "format_text")]
/// # {
/// use tree_fmt::{ RowBuilder, TextFormatter, TextVariant, Format };
///
/// let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
///   .add_row( vec![ "Alice".into(), "30".into() ] )
///   .build_view();
///
/// let formatter = TextFormatter::bullets();
/// let text = formatter.format( &view ).unwrap();
/// assert!( text.contains( "•" ) );
/// assert!( text.contains( "Alice" ) );
/// # }
/// ```
#[ derive( Debug, Clone ) ]
pub struct TextFormatter
{
  /// Output variant
  pub variant : TextVariant,
  /// Indentation spaces
  pub indent : usize,
  /// Separator string
  pub separator : String,
}

impl TextFormatter
{
  /// Create new text formatter with specified variant
  pub fn new( variant : TextVariant ) -> Self
  {
    Self
    {
      variant,
      indent : 2,
      separator : "\n".to_string(),
    }
  }

  /// Create bullet list formatter
  pub fn bullets() -> Self
  {
    Self::new( TextVariant::Bullets )
  }

  /// Create numbered list formatter
  pub fn numbered() -> Self
  {
    Self::new( TextVariant::Numbered )
  }

  /// Create key-value formatter
  pub fn key_value() -> Self
  {
    Self::new( TextVariant::KeyValue )
  }

  /// Create section formatter
  pub fn sections() -> Self
  {
    Self::new( TextVariant::Sections )
  }

  /// Create compact formatter
  pub fn compact() -> Self
  {
    Self
    {
      variant : TextVariant::Compact,
      indent : 0,
      separator : ", ".to_string(),
    }
  }

  /// Create CLI help text formatter
  pub fn cli_help() -> Self
  {
    Self::new( TextVariant::CliHelp )
  }

  /// Set indentation
  #[ must_use ]
  pub fn with_indent( mut self, indent : usize ) -> Self
  {
    self.indent = indent;
    self
  }

  /// Set separator
  #[ must_use ]
  pub fn with_separator( mut self, separator : String ) -> Self
  {
    self.separator = separator;
    self
  }
}

impl Default for TextFormatter
{
  fn default() -> Self
  {
    Self::bullets()
  }
}

impl Format for TextFormatter
{
  fn format( &self, data : &TableView ) -> Result< String, FormatError >
  {
    let result = match self.variant
    {
      TextVariant::Bullets => format_bullets( data, self.indent ),
      TextVariant::Numbered => format_numbered( data, self.indent ),
      TextVariant::KeyValue => format_key_value( data ),
      TextVariant::Sections => format_sections( data, self.indent ),
      TextVariant::Compact => format_compact( data, &self.separator ),
      TextVariant::CliHelp => format_cli_help( data, self.indent ),
    };
    Ok( result )
  }
}

// Style implementations

fn format_bullets( data : &TableView, indent : usize ) -> String
{
  let mut output = String::new();
  let indent_str = " ".repeat( indent );

  for row in &data.rows
  {
    let row_text = row.join( " " );
    output.push_str( &format!( "{indent_str}• {row_text}\n" ) );
  }

  output
}

fn format_numbered( data : &TableView, indent : usize ) -> String
{
  let mut output = String::new();
  let indent_str = " ".repeat( indent );

  for ( idx, row ) in data.rows.iter().enumerate()
  {
    let row_text = row.join( " " );
    output.push_str( &format!( "{}{}. {}\n", indent_str, idx + 1, row_text ) );
  }

  output
}

fn format_key_value( data : &TableView ) -> String
{
  let mut output = String::new();

  for row in &data.rows
  {
    for ( col_idx, cell ) in row.iter().enumerate()
    {
      if col_idx < data.metadata.column_names.len()
      {
        output.push_str( &format!(
          "{}: {}\n",
          data.metadata.column_names[ col_idx ],
          cell
        ) );
      }
    }
    output.push( '\n' );
  }

  output
}

fn format_sections( data : &TableView, indent : usize ) -> String
{
  let mut output = String::new();
  let indent_str = " ".repeat( indent );

  if !data.metadata.column_names.is_empty()
  {
    output.push_str( &format!( "{}:\n", data.metadata.column_names.join( ", " ) ) );
  }

  for row in &data.rows
  {
    for cell in row
    {
      output.push_str( &format!( "{indent_str}{cell}\n" ) );
    }
  }

  output
}

fn format_compact( data : &TableView, separator : &str ) -> String
{
  let rows_text : Vec< String > = data.rows
    .iter()
    .map( | row | row.join( " " ) )
    .collect();

  rows_text.join( separator )
}

fn format_cli_help( data : &TableView, indent : usize ) -> String
{
  if data.rows.is_empty()
  {
    return String::new();
  }

  let mut output = String::new();
  let indent_str = " ".repeat( indent );

  // First pass: identify sections and calculate max key width for alignment
  let mut sections : Vec< ( usize, bool ) > = Vec::new(); // (row_index, is_section_header)
  let mut max_key_width = 0;

  for ( idx, row ) in data.rows.iter().enumerate()
  {
    let first_col = row.first().map_or( "", String::as_str );
    let second_col = row.get( 1 ).map_or( "", String::as_str );

    // Detect section header: first column is uppercase, second is empty
    let is_section = first_col.chars().all( | c | c.is_uppercase() || c.is_whitespace() || c == '_' )
      && !first_col.is_empty()
      && second_col.is_empty();

    sections.push( ( idx, is_section ) );

    // Calculate max key width for aligned rows (non-section rows with descriptions)
    if !is_section && !second_col.is_empty()
    {
      max_key_width = max_key_width.max( first_col.len() );
    }
  }

  // Second pass: format output
  for ( row_idx, row ) in data.rows.iter().enumerate()
  {
    let first_col = row.first().map_or( "", String::as_str );
    let second_col = row.get( 1 ).map_or( "", String::as_str );
    let is_section = sections[ row_idx ].1;

    if is_section
    {
      // Add blank line before section (except for first section)
      if row_idx > 0
      {
        output.push( '\n' );
      }

      // Format section header
      output.push_str( first_col );
      output.push_str( ":\n" );
    }
    else
    {
      // Content row
      if second_col.is_empty()
      {
        // Simple line: just indent
        output.push_str( &indent_str );
        output.push_str( first_col );
        output.push( '\n' );
      }
      else
      {
        // Key-description pair: align description
        output.push_str( &indent_str );
        output.push_str( first_col );

        // Padding to align descriptions
        let padding_needed = max_key_width - first_col.len() + 2; // +2 for spacing
        output.push_str( &" ".repeat( padding_needed ) );
        output.push_str( second_col );
        output.push( '\n' );
      }
    }
  }

  output
}

#[ cfg( test ) ]
mod tests
{
  use super::*;
  use crate::RowBuilder;

  #[ test ]
  fn test_text_formatter_bullets()
  {
    let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
      .add_row( vec![ "Alice".into(), "30".into() ] )
      .add_row( vec![ "Bob".into(), "25".into() ] )
      .build_view();

    let formatter = TextFormatter::bullets();
    let text = formatter.format( &view ).unwrap();

    assert!( text.contains( "•" ) );
    assert!( text.contains( "Alice 30" ) );
    assert!( text.contains( "Bob 25" ) );
  }

  #[ test ]
  fn test_text_formatter_numbered()
  {
    let view = RowBuilder::new( vec![ "Item".into() ] )
      .add_row( vec![ "First".into() ] )
      .add_row( vec![ "Second".into() ] )
      .build_view();

    let formatter = TextFormatter::numbered();
    let text = formatter.format( &view ).unwrap();

    assert!( text.contains( "1. First" ) );
    assert!( text.contains( "2. Second" ) );
  }

  #[ test ]
  fn test_text_formatter_key_value()
  {
    let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
      .add_row( vec![ "Alice".into(), "30".into() ] )
      .build_view();

    let formatter = TextFormatter::key_value();
    let text = formatter.format( &view ).unwrap();

    assert!( text.contains( "Name: Alice" ) );
    assert!( text.contains( "Age: 30" ) );
  }

  #[ test ]
  fn test_text_formatter_compact()
  {
    let view = RowBuilder::new( vec![ "Item".into() ] )
      .add_row( vec![ "A".into() ] )
      .add_row( vec![ "B".into() ] )
      .add_row( vec![ "C".into() ] )
      .build_view();

    let formatter = TextFormatter::compact();
    let text = formatter.format( &view ).unwrap();

    assert_eq!( text, "A, B, C" );
  }

  #[ test ]
  fn test_text_formatter_sections()
  {
    let view = RowBuilder::new( vec![ "Header1".into(), "Header2".into() ] )
      .add_row( vec![ "Value1".into(), "Value2".into() ] )
      .build_view();

    let formatter = TextFormatter::sections();
    let text = formatter.format( &view ).unwrap();

    assert!( text.contains( "Header1, Header2:" ) );
    assert!( text.contains( "Value1" ) );
  }

  #[ test ]
  fn test_text_formatter_builder_pattern()
  {
    let formatter = TextFormatter::bullets()
      .with_indent( 4 )
      .with_separator( ";\n".to_string() );

    assert_eq!( formatter.indent, 4 );
    assert_eq!( formatter.separator, ";\n" );
  }
}
