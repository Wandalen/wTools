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
use std::borrow::{ Cow, Borrow };
use core::
{
  fmt,
};
use std::sync::OnceLock;

use format::wrap_text::wrap_text;

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

    let col_names : Vec< Cow< 'data, str > > = x.header().collect();
    let key_width = x.header().fold( 0, | acc, cell | acc.max( cell.len() ) );

    write!( c.buf, "{}", self.table_prefix )?;

    let mut first = true;

    for ( irow, row ) in x.rows().enumerate()
    {
      if first
      {
        first = false;
      }
      else
      {
        write!( c.buf, "{}", self.table_separator )?;
      }

      writeln!( c.buf, " = {}", irow + 1 )?;

      let value_width = row.iter().fold( 0, | acc, cell | acc.max( cell.len() ) );

      for ( icol, col ) in row.iter().enumerate()
      {
        let key = col_names.get(icol).map( Cow::borrow ).unwrap_or( "" );

        if icol > 0
        {
          write!( c.buf, "{}", self.row_separator )?;
        }
        
        write!( c.buf, "{}", self.row_prefix )?;

        write!( c.buf, "{}", self.cell_prefix )?;
        write!( c.buf, "{:<key_width$}", key )?;
        write!( c.buf, "{}", self.cell_postfix )?;
        write!( c.buf, "{}", self.cell_separator )?;
        write!( c.buf, "{}", self.cell_prefix )?;
        write!( c.buf, "{:<value_width$}", col )?;
        write!( c.buf, "{}", self.cell_postfix )?;

        write!( c.buf, "{}", self.row_postfix )?;
      }
    }

    write!( c.buf, "{}", self.table_postfix )?;

    Ok( () )
  }

}
