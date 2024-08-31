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

  fn extract_write< 'buf, 'data >(
    & self,
    x : & InputExtract< 'data >,
    c : & mut Context< 'buf >,
  ) -> fmt::Result
  {
    // Calculate max width for each column
    // let mut labels = vec![ "".to_string(); x.row_descriptors.len() ];
    let mut max_widths = vec![ 0; x.col_descriptors.len() ];

    for ( icol, _col ) in x.col_descriptors.iter().enumerate()
    {
      // labels[ icol ] = format!( " = {}\n", icol );
      // println!( "labels[ icol ] : {}", labels[ icol ].len() );

      // let label = &labels[ icol ];
      max_widths[ icol ] = 0;
      for row_data in &x.data
      {
        let sz = string::size( &row_data[ icol ].0 );
        if sz[ 0 ] > max_widths[ icol ]
        {
          max_widths[ icol ] = sz[ 0 ];
        }
      }

    }

    //

    // xxx : test with highest title

    let mut slices_dim = [ 1, x.mcells[ 0 ], x.mcells[ 1 ] ];
    slices_dim[ 0 ] = x.row_descriptors
    .iter()
    .fold( 0, | acc : usize, row | acc.max( row.height ) )
    ;

    let slices_len = slices_dim[ 0 ] * slices_dim[ 1 ] * slices_dim[ 2 ];
    let slices : Vec< &str > = vec![ "" ; slices_len ];
    let labels : Vec< &str > = vec![ "" ; slices_dim[ 0 ] ];

//     let mut irow : isize = -1;
//     for row_data in x.data.iter()
//     {
//
//       irow += 1;
//
//       for icol in 0 .. x.col_descriptors.len()
//       {
//         let cell = &row_data[ icol ];
//         string::lines( cell.0.as_ref() )
//         .enumerate()
//         .for_each( | ( layer, s ) |
//         {
//           let md_index = [ layer, icol, irow as usize ];
//           slices[ x.slices_dim.md_offset( md_index ) ] = s;
//         })
//         ;
//         x.col_descriptors[ icol ].label = cell.0.as_ref(); // xxx
//       }
//
//     }

    // Write each record
    for ( irow, row ) in x.row_descriptors.iter().enumerate()
    {
      if !row.vis
      {
        continue;
      }
      writeln!( c.buf, "{}", irow )?;
      for ( icol, _col ) in x.col_descriptors.iter().enumerate()
      {
        let md_index = [ 0, icol, irow ];
        let slice = x.slices[ x.slices_dim.md_offset( md_index ) ];
        let width = max_widths[ icol ];
        writeln!( c.buf, "{:<width$} | {}", "", slice, width = width )?;
      }
    }

    Ok(())
  }

}
