//! Implement classic table output format.
//!
//! # Example
//!
//! ```text
//!  sid | sname | gap
//! -----+-------+-----
//!    3 | Alice |   5
//!    6 | Joe   |   1
//!   10 | Boris |   5
//! ```

use crate::*;
use print::
{
  InputExtract,
  Context,
};
use core::
{
  fmt,
};
use std::sync::OnceLock;

/// A struct representing the classic table output format.
///
/// `Table` provides a standard implementation for table formatting,
/// supporting a classic style with default settings.
///
/// # Example
///
/// ```text
///  sid | sname | gap
/// -----+-------+-----
///    3 | Alice |   5
///    6 | Joe   |   1
///   10 | Boris |   5
/// ```
#[ derive( Debug ) ]
pub struct Table
{
  /// Delimitting header with grid line or not.
  pub delimitting_header : bool,
  /// Prefix added to each cell.
  pub cell_prefix : String,
  /// Postfix added to each cell.
  pub cell_postfix : String,
  /// Separator used between table columns.
  pub cell_separator : String,
  /// Prefix added to each row.
  pub row_prefix : String,
  /// Postfix added to each row.
  pub row_postfix : String,
  /// Separator used between rows.
  pub row_separator : String,
  /// Horizontal line character.
  pub h : char,
  /// Vertical line character.
  pub v : char,
  /// Left T-junction character.
  pub t_l : char,
  /// Right T-junction character.
  pub t_r : char,
  /// Top T-junction character.
  pub t_t : char,
  /// Bottom T-junction character.
  pub t_b : char,
  /// Cross junction character.
  pub cross : char,
  /// Top-left corner character.
  pub corner_lt : char,
  /// Top-right corner character.
  pub corner_rt : char,
  /// Bottom-left corner character.
  pub corner_lb : char,
  /// Bottom-right corner character.
  pub corner_rb : char,
  /// Limit table size (0 - no limit).
  pub max_width : usize,
}

impl Default for Table
{
  fn default() -> Self
  {

    let delimitting_header = true;

    let cell_prefix = "".to_string();
    let cell_postfix = "".to_string();
    let cell_separator = " │ ".to_string();
    let row_prefix = "│ ".to_string();
    let row_postfix = " │".to_string();
    let row_separator = "\n".to_string();

    let h = '─';
    let v = '|';
    let t_l = '├';
    let t_r = '┤';
    let t_t = '┬';
    let t_b = '┴';
    let cross = '┼';
    let corner_lt = '┌';
    let corner_rt = '┐';
    let corner_lb = '└';
    let corner_rb = '┘';

    let max_width = 50;

    Self
    {
      delimitting_header,
      cell_prefix,
      cell_postfix,
      cell_separator,
      row_prefix,
      row_postfix,
      row_separator,
      h,
      v,
      t_l,
      t_r,
      t_t,
      t_b,
      cross,
      corner_lt,
      corner_rt,
      corner_lb,
      corner_rb,
      max_width,
    }
  }
}

impl Default for &'static Table
{
  fn default() -> Self
  {
    // qqq : find a better solution
    static STYLES : OnceLock< Table > = OnceLock::new();
    STYLES.get_or_init( ||
    {
      Table::default()
    })
  }
}

impl Table
{

  /// Returns a reference to a static instance of `Table`.
  ///
  /// This method provides access to a single shared instance of `Table`,
  /// ensuring efficient reuse of the classic table output format.
  pub fn instance() -> & 'static dyn TableOutputFormat
  {

    static INSTANCE : OnceLock< Table > = OnceLock::new();
    INSTANCE.get_or_init( ||
    {
      Self::default()
    })

  }
}

impl TableOutputFormat for Table
{
  fn extract_write< 'buf, 'data >( &self, x : &InputExtract< 'data >, c : &mut Context< 'buf > ) -> fmt::Result
  {
    let cell_prefix = &self.cell_prefix;
    let cell_postfix = &self.cell_postfix;
    let cell_separator = &self.cell_separator;
    let row_prefix = &self.row_prefix;
    let row_postfix = &self.row_postfix;
    let row_separator = &self.row_separator;
    let h = self.h.to_string();

    let column_count = x.header().count();

    let mut col_width : Vec< usize > = vec![ 0; column_count ];

    for ( _, row ) in x.data.iter().enumerate()
    {
      for ( icol, col ) in row.iter().enumerate()
      {
        col_width[ icol ] = col_width[ icol ].max( col.chars().count() );
      }
    }

    let max_row_width = col_width.iter().sum::<usize>()
    + self.row_prefix.chars().count()
    + self.row_postfix.chars().count()
    + column_count * ( self.cell_postfix.chars().count() + self.cell_prefix.chars().count() )
    + if column_count == 0 { 0 } else { ( column_count - 1 ) * self.cell_separator.chars().count() };

    for ( irow, row ) in x.data.iter().enumerate()
    {
      if irow == 1 && x.has_header && self.delimitting_header
      {
        write!( c.buf, "{}", row_separator )?;
        write!( c.buf, "{}", h.repeat( max_row_width ) )?;
      }
      
      if irow > 0
      {
        write!( c.buf, "{}", row_separator )?;
      }

      write!( c.buf, "{}", row_prefix )?;

      for ( icol, col ) in row.iter().enumerate()
      {
        let cell_width = col_width[ icol ];
        let width = col.len();

        if icol > 0
        {
          write!( c.buf, "{}", cell_separator )?;
        }

        write!( c.buf, "{}", cell_prefix )?;
        
        let lspaces = if cell_width >= width
        {
          ( cell_width - width ) / 2
        }
        else
        {
          0
        };

        let rspaces = if cell_width >= width
        {
          ( ( cell_width - width ) as f32 / 2 as f32 ).round() as usize
        }
        else
        {
          0
        };

        if lspaces > 0
        {
          write!( c.buf, "{:<width$}", " ", width = lspaces )?;
        }
        
        write!( c.buf, "{}", col )?;

        if rspaces > 0
        {
          write!( c.buf, "{:>width$}", " ", width = rspaces )?;
        }

        write!( c.buf, "{}", cell_postfix )?;
      }

      write!( c.buf, "{}", row_postfix )?;
    }

    Ok(())
  }
}
