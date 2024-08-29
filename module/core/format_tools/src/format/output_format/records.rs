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
use md_math::MdOffset;
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

/// A struct representing the list of records( rows ) output format.
///
/// `Records` provides an implementation for table formatting that outputs
/// each row as a separate table with 2 columns, first is name of column in the original data and second is cell value itself.
#[derive( Debug )]
pub struct Records;

impl Records
{
  /// Returns a reference to a static instance of `Records`.
  pub fn instance() -> & 'static dyn TableOutputFormat
  {
    static INSTANCE : OnceLock< Records > = OnceLock::new();
    INSTANCE.get_or_init( || Records )
  }
}

impl Default for Records
{
  fn default() -> Self
  {
    Self
    {
    }
  }
}

impl TableOutputFormat for Records
{
  fn extract_write< 'buf, 'data >
  (
    & self,
    x : & InputExtract< 'data >,
    c : & mut Context< 'buf >,
  ) -> fmt::Result
  {
    for ( i, row ) in x.row_descriptors.iter().enumerate()
    {
      if !row.vis
      {
        continue;
      }
      writeln!( c.buf, "-[ RECORD {} ]", i + 1 )?;
      for ( icol, col ) in x.col_descriptors.iter().enumerate()
      {
        // let cell_width = x.data[ i ][ icol ].1[ 0 ];
        let md_index = [ 0, icol, i ];
        let slice = x.slices[ x.slices_dim.md_offset( md_index ) ];
        writeln!( c.buf, "{} | {}", col.width, slice )?;
      }
    }
    Ok(())
  }
}
