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
    // sync::OnceLock,
  };
  use std::sync::OnceLock;

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
  #[ derive( Debug ) ]
  pub struct OrdinaryStyles
  {

    /// Delimiter for adding prefix to a cell.
    pub cell_prefix : String,
    /// Delimiter for adding postfix to a cell.
    pub cell_postfix : String,
    /// Delimiter for separating table columns.
    pub cell_separator : String,

    /// Delimiter for adding prefix to a row.
    pub row_prefix : String,
    /// Delimiter for adding postfix to a row.
    pub row_postfix : String,
    /// Delimiter for adding in between of rows.
    pub row_separator : String,

  }

  impl Default for OrdinaryStyles
  {
    fn default() -> Self
    {
      let cell_prefix = "".to_string();
      let cell_postfix = "".to_string();
      let cell_separator = " │ ".to_string();
      let row_prefix = "│ ".to_string();
      let row_postfix = " │".to_string();
      let row_separator = "\n".to_string();
      Self
      {
        cell_prefix,
        cell_postfix,
        cell_separator,
        row_prefix,
        row_postfix,
        row_separator,
      }
    }
  }

  impl Default for &'static OrdinaryStyles
  {
    fn default() -> Self
    {
      // qqq : find a better solution
      static STYLES : OnceLock< OrdinaryStyles > = OnceLock::new();
      STYLES.get_or_init( ||
      {
        OrdinaryStyles::default()
      })
    }
  }

// // xxx : implement
//
//   /// Convert styles into format
//   pub trait IntoFormat
//   {
//     type Format : TableOutputFormat;
//     fn into_format( self ) -> Self::Format;
//   }
//
//   impl< 'context > IntoFormat for &'context OrdinaryStyles
//   {
//     type Format = Ordinary< 'context >;
//
//     fn into_format( self ) -> Self::Format
//     {
//       let format = Ordinary( self );
//       return format
//     }
//
//   }
//
//   // impl< 'context > From< &'context OrdinaryStyles > for &'context dyn TableOutputFormat
//   impl< 'context > From< &'context OrdinaryStyles > for Ordinary< 'context >
//   {
//
//     fn from( src : &'context OrdinaryStyles ) -> Self
//     {
//       let format = Ordinary( src );
//       return format
//       // let result : &'context dyn TableOutputFormat = &format;
//       // &Ordinary( src )
//       // result
//     }
//
//   }

  // &'context Styles : Into< &'context dyn TableOutputFormat >,

//   /// A struct representing the classic table output format.
//   ///
//   /// `Ordinary` provides a standard implementation for table formatting,
//   /// supporting a classic style with default settings.
//   ///
//   /// # Traits
//   ///
//   /// - `Debug`: Allows the struct to be formatted using the `{:?}` formatter.
//   /// - `Default`: Provides a default instance of `Ordinary`.
//   /// - `Clone` and `Copy`: Enables copying of the `Ordinary` instance.
//
//   #[derive( Debug, Default, Clone, Copy )]
//   pub struct Ordinary< 'a >( &'a OrdinaryStyles );

  impl OrdinaryStyles
  {

    // /// Constructor accepting styles.
    // pub fn with_styles( styles : &'a OrdinaryStyles ) -> Self
    // {
    //   Self( styles )
    // }

    /// Returns a reference to a static instance of `Ordinary`.
    ///
    /// This method provides access to a single shared instance of `Ordinary`,
    /// ensuring efficient reuse of the classic table output format.
    pub fn instance() -> & 'static dyn TableOutputFormat
    {

      // static STYLES : OnceLock< OrdinaryStyles > = OnceLock::new();
      // let styles : &OrdinaryStyles = STYLES.get_or_init( ||
      // {
      //   OrdinaryStyles::Default()
      // });

      static INSTANCE : OnceLock< OrdinaryStyles > = OnceLock::new();
      INSTANCE.get_or_init( ||
      {
        Self::default()
        // let styles : &'static OrdinaryStyles = Default::default();
        // Ordinary( styles )
      })

      // static INSTANCE: OnceLock< Ordinary< 'static > > = OnceLock::new( || Ordinary( STYLES.get().unwrap() ) );
      // INSTANCE.get().unwrap()

      // static STYLES : OrdinaryStyles = OrdinaryStyles::default();
      // static INSTANCE : Ordinary< 'static > = Ordinary( &STYLES );
      // &INSTANCE
    }
  }

  impl Default for &'static dyn TableOutputFormat
  {
    #[ inline( always ) ]
    fn default() -> Self
    {
      OrdinaryStyles::instance()
    }
  }

  impl TableOutputFormat for OrdinaryStyles
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
    // Ordinary,
    OrdinaryStyles,
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
