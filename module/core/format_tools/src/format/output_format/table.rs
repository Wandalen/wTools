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
    use md_math::MdOffset;

    let cell_prefix = &self.cell_prefix;
    let cell_postfix = &self.cell_postfix;
    let cell_separator = &self.cell_separator;
    let row_prefix = &self.row_prefix;
    let row_postfix = &self.row_postfix;
    let row_separator = &self.row_separator;
    let h = self.h.to_string();

    let mut delimitting_header = self.delimitting_header;
    let row_width = if delimitting_header
    {
      let mut grid_width = x.mcells_vis[ 0 ] * ( cell_prefix.chars().count() + cell_postfix.chars().count() );
      grid_width += row_prefix.chars().count() + row_postfix.chars().count();
      if x.mcells_vis[ 0 ] > 0
      {
        grid_width += ( x.mcells_vis[ 0 ] - 1 ) * ( cell_separator.chars().count() );
      }
      x.mchars[ 0 ] + grid_width
    }
    else
    {
      0
    };
    let mut prev_typ : Option< LineType > = None;

    // dbg!( x.row_descriptors.len() );

    for ( irow, row ) in x.row_descriptors.iter().enumerate()
    {
      let height = row.height;

      if delimitting_header
      {
        if let Some( prev_typ ) = prev_typ
        {
          if prev_typ == LineType::Header && row.typ == LineType::Regular
          {
            write!( c.buf, "{}", row_separator )?;
            write!( c.buf, "{}", h.repeat( row_width ) )?;
            delimitting_header = false
          }
        }
        if row.vis
        {
          prev_typ = Some( row.typ );
        }
      }

      if !row.vis
      {
        continue;
      }

      // dbg!( row.height );

      for islice in 0..height
      {

        if irow > 0
        {
          write!( c.buf, "{}", row_separator )?;
        }

        write!( c.buf, "{}", row_prefix )?;

        for icol in 0 .. x.col_descriptors.len()
        {
          let col = &x.col_descriptors[ icol ];
          let cell_width = x.data[ irow ][ icol ].1[0];
          let width = col.width;
          let md_index = [ islice, icol, irow as usize ];
          let slice = x.slices[ x.slices_dim.md_offset( md_index ) ];

          // println!( "md_index : {md_index:?} | md_offset : {} | slice : {slice}", x.slices_dim.md_offset( md_index ) );

          if icol > 0
          {
            write!( c.buf, "{}", cell_separator )?;
          }

          write!( c.buf, "{}", cell_prefix )?;

          // println!( "icol : {icol} | irow : {irow} | width : {width} | cell_width : {cell_width}" );
          let lspaces = ( width - cell_width ) / 2;
          let rspaces = ( width - cell_width + 1 ) / 2 + cell_width - slice.len();
          // println!( "icol : {icol} | irow : {irow} | width : {width} | cell_width : {cell_width} | lspaces : {lspaces} | rspaces : {rspaces}" );

          if lspaces > 0
          {
            write!( c.buf, "{:<width$}", " ", width = lspaces )?;
          }
          write!( c.buf, "{}", slice )?;
          if rspaces > 0
          {
            write!( c.buf, "{:>width$}", " ", width = rspaces )?;
          }

          write!( c.buf, "{}", cell_postfix )?;
        }

        write!( c.buf, "{}", row_postfix )?;
      }

    }

    Ok(())
  }
}
