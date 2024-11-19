//! Implement list of records ( rows ) output format.
//!
//! Implementation for table formatting that outputs
//! each row as a separate table with 2 columns, first is name of column in the original data and second is cell value itself.
//!
//! # Example
//!
//! ```text
//! -[ RECORD 1 ]
//! sid   | 3
//! sname | Alice
//! gap   | 5
//! -[ RECORD 2 ]
//! sid   | 6
//! sname | Joe
//! gap   | 1
//! -[ RECORD 3 ]
//! sid   | 10
//! sname | Boris
//! gap   | 5
//! ```
//!

use crate::*;
use print::
{
  InputExtract,
  Context,
};
use std::borrow::Cow;
use core::
{
  fmt,
};
use std::sync::OnceLock;

/// A struct representing the list of records( rows ) output format.
///
/// `Records` provides an implementation for table formatting that outputs
/// each row as a separate table with 2 columns, first is name of column in the original data and second is cell value itself.
#[derive( Debug )]
pub struct Records
{
  /// Prefix added to each row.
  pub table_prefix : String,
  /// Postfix added to each row.
  pub table_postfix : String,
  /// Separator used between rows.
  pub table_separator : String,
  /// Prefix added to each row.
  pub row_prefix : String,
  /// Postfix added to each row.
  pub row_postfix : String,
  /// Separator used between rows.
  pub row_separator : String,
  /// Prefix added to each cell.
  pub cell_prefix : String,
  /// Postfix added to each cell.
  pub cell_postfix : String,
  /// Separator used between table columns.
  pub cell_separator : String,
  /// Limit table width. If the value is zero, then no limitation.
  pub max_width: usize,
  // /// Horizontal line character.
  // pub h : char,
  // /// Vertical line character.
  // pub v : char,
  // /// Left T-junction character.
  // pub t_l : char,
  // /// Right T-junction character.
  // pub t_r : char,
  // /// Top T-junction character.
  // pub t_t : char,
  // /// Bottom T-junction character.
  // pub t_b : char,
  // /// Cross junction character.
  // pub cross : char,
  // /// Top-left corner character.
  // pub corner_lt : char,
  // /// Top-right corner character.
  // pub corner_rt : char,
  // /// Bottom-left corner character.
  // pub corner_lb : char,
  // /// Bottom-right corner character.
  // pub corner_rb : char,
}

impl Records
{
  /// Returns a reference to a static instance of `Records`.
  pub fn instance() -> & 'static dyn TableOutputFormat
  {
    static INSTANCE : OnceLock< Records > = OnceLock::new();
    INSTANCE.get_or_init( || Records::default() )
  }

  /// Calculate how much space is minimally needed in order to generate an output with this output formatter.
  /// It will be impossible to render tables smaller than the result of `min_width()`.
  ///
  /// This function is similar to `output_format::Table::min_width`, but it does not contain a
  /// `column_count` as it always equal to 2, and it aslo uses the `output_format::Records` 
  /// style parameters.
  pub fn min_width
  (
    &self,
  ) -> usize
  {
    // 2 is used here, because `Records` displays 2 columns: keys and values.
    self.row_prefix.chars().count()
    + self.row_postfix.chars().count()
    + 2 * ( self.cell_postfix.chars().count() + self.cell_prefix.chars().count() )
    + self.cell_separator.chars().count()
    + 2
  }
}

impl Default for Records
{
  fn default() -> Self
  {

    let cell_prefix = "".to_string();
    let cell_postfix = "".to_string();
    let cell_separator = " │ ".to_string();
    let row_prefix = "│ ".to_string();
    let row_postfix = " │".to_string();
    let row_separator = "\n".to_string();
    let table_prefix = "".to_string();
    let table_postfix = "".to_string();
    let table_separator = "\n".to_string();

    let max_width = 0;

    // let h = '─';
    // let v = '|';
    // let t_l = '├';
    // let t_r = '┤';
    // let t_t = '┬';
    // let t_b = '┴';
    // let cross = '┼';
    // let corner_lt = '┌';
    // let corner_rt = '┐';
    // let corner_lb = '└';
    // let corner_rb = '┘';

    Self
    {
      table_prefix,
      table_postfix,
      table_separator,
      row_prefix,
      row_postfix,
      row_separator,
      cell_prefix,
      cell_postfix,
      cell_separator,
      max_width,
      // h,
      // v,
      // t_l,
      // t_r,
      // t_t,
      // t_b,
      // cross,
      // corner_lt,
      // corner_rt,
      // corner_lb,
      // corner_rb,
    }
  }
}

impl TableOutputFormat for Records
{

  fn extract_write< 'buf, 'data >(
    & self,
    x : & InputExtract< 'data >,
    c : & mut Context< 'buf >,
  ) -> fmt::Result
  {
    if self.max_width != 0 && self.max_width < self.min_width()
    {
      return Err( fmt::Error );
    }

    // 2 because there are only 2 columns: key and value.
    let columns_max_width = if self.max_width == 0 { 0 } else { self.max_width - self.min_width() + 2 };

    let field_names : Vec< ( Cow< 'data, str >, [ usize; 2 ] ) > = x.header().collect();

    write!( c.buf, "{}", self.table_prefix )?;

    let mut printed_tables_count = 0;

    for ( itable_descriptor, table_descriptor ) in x.row_descriptors.iter().enumerate()
    {
      if !table_descriptor.vis || ( x.has_header && itable_descriptor == 0 )
      {
        continue;
      }

      if printed_tables_count > 0
      {
        write!( c.buf, "{}", self.table_separator )?;
      }

      printed_tables_count += 1;

      writeln!( c.buf, " = {}", table_descriptor.irow )?;

      let wrapped_text = text_wrap( &field_names, &x.data[ itable_descriptor ], columns_max_width );

      for ( irow, ( key, value ) ) in wrapped_text.data.iter().enumerate()
      {
        if irow != 0
        {
          write!( c.buf, "{}", self.row_separator )?;
        }

        let key_width = wrapped_text.key_width;
        let value_width = wrapped_text.value_width;

        write!( c.buf, "{}", self.row_prefix )?;

        write!( c.buf, "{}", self.cell_prefix )?;
        write!( c.buf, "{:<key_width$}", key )?;
        write!( c.buf, "{}", self.cell_postfix )?;
        write!( c.buf, "{}", self.cell_separator )?;
        write!( c.buf, "{}", self.cell_prefix )?;
        write!( c.buf, "{:<value_width$}", value )?;
        write!( c.buf, "{}", self.cell_postfix )?;

        write!( c.buf, "{}", self.row_postfix )?;
      }
    }

    write!( c.buf, "{}", self.table_postfix )?;

    Ok( () )
  }

}

/// Struct that represents a wrapped tabular data. It is similar to `InputExtract`,
/// but we cannot use it as it does not wrap the text and it contains wrong column
/// widthes and height (as they are dependent on wrapping, too).
///
/// This struct is similar to `output_format::Table::WrappedInputExtract` (which is
/// private, too), but it is made only for 2 columns, as tables in `Records` contain
/// only key and value columns.
#[ derive( Debug ) ]
struct WrappedInputExtract< 'data >
{
  /// Tabular data for display. Because `Records` only show 2 columns, we used a tuple 
  /// here instead of a vector.
  data : Vec< ( &'data str, &'data str ) >,

  /// Width of key column.
  key_width : usize,

  /// Width of value column.
  value_width : usize,
}

/// Wrap cells in `InputExtract`.
///
/// `InputExtract` contains cells with full content, so it represents the logical
/// structure of the table. `WrappedInputExtract` wraps original cells to smaller 
/// cells. The resulting data is more low-level and corresponds to the table that
/// will be actually printed to the console (or other output type).
///
/// Wrapping is controlled by `columns_max_width` parameter.
/// `columns_max_width` is the size space that is allowed to be occupied by columns.
/// It equals to maximum table width minus lengthes of visual elements (prefixes,
/// postfixes, separators, etc.).
///
/// The function will perform wrapping and shrink the columns so that they occupy not
/// more than `columns_max_width`.
///
/// If `columns_max_width` is equal to 0, then no wrapping will be performed. 
fn text_wrap<'data>
(
  keys : &'data Vec< ( Cow< 'data, str >, [ usize; 2 ] ) >,
  values : &'data Vec< ( Cow< 'data, str >, [ usize; 2 ] ) >,
  columns_max_width : usize,
)
-> WrappedInputExtract< 'data >
{
  let mut data = Vec::new();
  let mut key_width = width_calculate( keys );
  let mut value_width = width_calculate( values );

  let orig_columns_width = key_width + value_width;

  if columns_max_width != 0 && orig_columns_width > columns_max_width 
  {
    let factor = ( columns_max_width as f32 ) / ( orig_columns_width as f32 );
    key_width = ( ( key_width as f32 ) * factor ).round() as usize;
    value_width = columns_max_width - key_width;
  }

  for i in 0..values.len()
  {
    let key = &keys[ i ];
    let value = &values[ i ];

    let key_wrapped : Vec< &'data str > = string::lines_with_limit( key.0.as_ref(), key_width ).collect();
    let value_wrapped : Vec< &'data str > = string::lines_with_limit( value.0.as_ref(), value_width ).collect();

    for j in 0..( key_wrapped.len().max( value_wrapped.len() ) )
    {
      let key = key_wrapped.get( j ).copied().unwrap_or( "" );
      let value = value_wrapped.get( j ).copied().unwrap_or( "" );

      data.push( ( key, value ) );
    }
  }

  WrappedInputExtract
  {
    data,
    key_width,
    value_width,
  }
}

/// Calculate width of the column without wrapping.
fn width_calculate< 'data >
( 
  column : &'data Vec< ( Cow< 'data, str >, [ usize; 2 ] ) >
)
-> usize
{
  column.iter().map( |k| 
  {
    string::lines( k.0.as_ref() ).map( |l| l.chars().count() ).max().unwrap_or( 0 )
  } ).max().unwrap_or( 0 )
}