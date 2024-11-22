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
    use format::text_wrap::{ text_wrap, width_calculate };

    if self.max_width != 0 && self.max_width < self.min_width()
    {
      return Err( fmt::Error );
    }

    // 2 because there are only 2 columns: key and value.
    let columns_max_width = if self.max_width == 0 { 0 } else { self.max_width - self.min_width() + 2 };

    let keys : Vec< ( Cow< 'data, str >, [ usize; 2 ] ) > = x.header().collect();
    let keys_width = width_calculate( &keys );

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

      let values = &x.data[ itable_descriptor ];
      let values_width = width_calculate( &values );

      let table_for_wrapping : Vec< Vec< ( Cow< 'data, str >, [ usize; 2] ) > > =
      keys.iter().enumerate().map( | ( ikey, key ) |
      {
        vec![ key.clone(), values[ ikey ].clone() ]
      }).collect();

      let wrapped_text = text_wrap
      (
        table_for_wrapping.iter(),
        &[ keys_width, values_width ],
        columns_max_width,
        keys_width + values_width,
      );

      for ( irow, cols ) in wrapped_text.data.into_iter().enumerate()
      {
        if irow != 0
        {
          write!( c.buf, "{}", self.row_separator )?;
        }

        let key = &cols[ 0 ];
        let value = &cols[ 1 ];

        let key_width = wrapped_text.column_widthes[ 0 ];
        let value_width = wrapped_text.column_widthes[ 1 ];

        write!( c.buf, "{}", self.row_prefix )?;

        write!( c.buf, "{}", self.cell_prefix )?;
        write!( c.buf, "{:<key_width$}", key.content )?;
        write!( c.buf, "{}", self.cell_postfix )?;
        write!( c.buf, "{}", self.cell_separator )?;
        write!( c.buf, "{}", self.cell_prefix )?;
        // No need to use `wrap_width` of `WrappedCell`, as `output_format::Records`
        // does not center values in cells (they are always left aligned).
        write!( c.buf, "{:<value_width$}", value.content )?;
        write!( c.buf, "{}", self.cell_postfix )?;

        write!( c.buf, "{}", self.row_postfix )?;
      }
    }

    write!( c.buf, "{}", self.table_postfix )?;

    Ok( () )
  }

}
