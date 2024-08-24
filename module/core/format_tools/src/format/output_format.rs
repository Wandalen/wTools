//!
//! Print data as table.
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


  // Example of output formatting as table.
  //
  //  sid | sname | gap
  // -----+-------+-----
  //    3 | Alice |   5
  //    6 | Joe   |   1
  //   10 | Boris |   5
  // (3 rows)

  // Example of output formatting as list of rows.
  // -[ RECORD 1 ]
  // sid   | 3
  // sname | Alice
  // gap   | 5
  // -[ RECORD 2 ]
  // sid   | 6
  // sname | Joe
  // gap   | 1
  // -[ RECORD 3 ]
  // sid   | 10
  // sname | Boris
  // gap   | 5

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
    fn extract_write< 'buf, 'data >(
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
  /// # Traits
  ///
  /// - `Debug`: Allows the struct to be formatted using the `{:?}` formatter.
  /// - `Default`: Provides a default instance of `Ordinary`.
  /// - `Clone` and `Copy`: Enables copying of the `Ordinary` instance.

  #[derive( Debug, Default, Clone, Copy )]
  pub struct Ordinary;

  impl Ordinary
  {
    /// Returns a reference to a static instance of `Ordinary`.
    ///
    /// This method provides access to a single shared instance of `Ordinary`,
    /// ensuring efficient reuse of the classic table output format.
    pub fn instance() -> & 'static dyn TableOutputFormat
    {
      static INSTANCE : Ordinary = Ordinary;
      &INSTANCE
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

      let cell_prefix = &c.styles.cell_prefix;
      let cell_postfix = &c.styles.cell_postfix;
      let cell_separator = &c.styles.cell_separator;
      let row_prefix = &c.styles.row_prefix;
      let row_postfix = &c.styles.row_postfix;
      let row_separator = &c.styles.row_separator;

      // dbg!( x.row_descriptors.len() );

      for ( irow, row ) in x.row_descriptors.iter().enumerate()
      {
        let height = row.height;

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
