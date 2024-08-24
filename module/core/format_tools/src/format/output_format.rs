//! Customizable format of printing table.
//!
//! # Example of ordinary format
//!
//! ```text
//!  sid | sname | gap
//! -----+-------+-----
//!    3 | Alice |   5
//!    6 | Joe   |   1
//!   10 | Boris |   5
//! ```
//!
//! # Example of list of rows format.
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

/// Internal namespace.
pub( crate ) mod private
{

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

  //=

  /// Trait for converting table extracts into string representations.
  ///
  /// `TableOutputFormat` defines the method for formatting table data
  /// and writing it into a specified buffer, providing flexibility in
  /// output style and format.
  ///
  pub trait TableOutputFormat
  {
    /// Formats the table extract and writes it into the destination buffer.
    ///
    /// # Parameters
    /// - `x`: The `InputExtract` containing table data to be formatted.
    /// - `c`: The `Context` holding the buffer and styles for formatting.
    ///
    /// # Returns
    /// A `fmt::Result` indicating success or failure of the write operation.
    fn extract_write< 'buf, 'data >
    (
      &self,
      x : &InputExtract< 'data >,
      c : &mut Context< 'buf >,
    ) -> fmt::Result;
  }

  /// A struct representing the classic table output format.
  ///
  /// `Ordinary` provides a standard implementation for table formatting,
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
  #[derive( Debug )]
  pub struct Ordinary
  {
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

  impl Default for Ordinary
  {
    fn default() -> Self
    {

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

  impl Default for &'static Ordinary
  {
    fn default() -> Self
    {
      // qqq : find a better solution
      static STYLES : OnceLock< Ordinary > = OnceLock::new();
      STYLES.get_or_init( ||
      {
        Ordinary::default()
      })
    }
  }

  impl Ordinary
  {

    /// Returns a reference to a static instance of `Ordinary`.
    ///
    /// This method provides access to a single shared instance of `Ordinary`,
    /// ensuring efficient reuse of the classic table output format.
    pub fn instance() -> & 'static dyn TableOutputFormat
    {

      static INSTANCE : OnceLock< Ordinary > = OnceLock::new();
      INSTANCE.get_or_init( ||
      {
        Self::default()
      })

    }
  }

  impl Default for &'static dyn TableOutputFormat
  {
    #[ inline( always ) ]
    fn default() -> Self
    {
      Ordinary::instance()
    }
  }

  impl TableOutputFormat for Ordinary
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

      let mut prev_typ : Option< LineType > = None;

      // dbg!( x.row_descriptors.len() );

      for ( irow, row ) in x.row_descriptors.iter().enumerate()
      {
        let height = row.height;

        if let Some( prev_typ ) = prev_typ
        {
          if prev_typ == LineType::Header && row.typ == LineType::Regular
          {
            // write!( c.buf, "{}", row_separator )?;
            // write!( c.buf, "{}", "---" )?;
          }
        }
        prev_typ = Some( row.typ );

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

}

#[ allow( unused_imports ) ]
pub use own::*;

/// Own namespace of the module.
#[ allow( unused_imports ) ]
pub mod own
{
  use super::*;
  #[ doc( inline ) ]
  pub use orphan::*;

  #[ doc( inline ) ]
  pub use private::
  {
    // Ordinary,
    Ordinary,
  };

}

/// Orphan namespace of the module.
#[ allow( unused_imports ) ]
pub mod orphan
{
  use super::*;
  #[ doc( inline ) ]
  pub use exposed::*;
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;
  pub use super::super::output_format;

  #[ doc( inline ) ]
  pub use private::
  {
    TableOutputFormat,
  };

}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;
}
