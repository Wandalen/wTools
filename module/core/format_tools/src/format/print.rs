//!
//! Print data as table.
//!

/// Internal namespace.
pub( crate ) mod private
{

  use crate::*;
  use std::
  {
    borrow::Cow,
    collections::HashMap,
  };
  use core::
  {
    fmt,
  };
  use former::Former;

  //=

  /// A struct to configure options for printing data as a table.
  ///
  /// The `Styles` struct provides customizable delimiters for formatting table data. It allows
  /// you to define how table data should be separated and formatted, making it adaptable to
  /// various needs.
  ///
  /// # Fields
  ///
  /// - `cell_separator`: A `String` that specifies the delimiter used to separate columns
  ///   within a table. This is the character or string that separates each column.
  ///
  /// - `row_prefix`: A `String` that specifies the prefix added to each row. This can be
  ///   used to add a consistent start to each row.
  ///
  /// - `row_postfix`: A `String` that specifies the postfix added to each row. This can be
  ///   used to add a consistent end to each row.
  ///
  /// - `row_postfix`: A `String` that specifies the postfix added to each row. This can be
  ///   used to add a consistent end to each row.
  ///
  /// ```
  #[ derive( Debug, Former ) ]
  pub struct Styles
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

    // /// Convert extract into a string, writing it into destination buffer.
    // pub writer : &'static dyn TableWriter,

  }

  impl Default for Styles
  {
    fn default() -> Self
    {
      let cell_prefix = "".to_string();
      let cell_postfix = "".to_string();
      let cell_separator = " │ ".to_string();
      let row_prefix = "│ ".to_string();
      let row_postfix = " │".to_string();
      let row_separator = "\n".to_string();
      // let filter_col = FilterColumnls::default();
      Self
      {
        cell_prefix,
        cell_postfix,
        cell_separator,
        row_prefix,
        row_postfix,
        row_separator,
        // filter_col,
      }
    }
  }

  /// Struct for formatting tables.
  pub struct Context< 'buf >
  {
    buf : &'buf mut dyn fmt::Write,
    styles : Styles,
  }

  impl< 'buf > Context< 'buf >
  {
    /// Just constructr.
    pub fn new( buf : &'buf mut dyn fmt::Write, styles : Styles ) -> Self
    {
      Self { buf, styles }
    }
  }

  impl fmt::Debug for Context< '_ >
  {
    fn fmt( &self, c : &mut fmt::Formatter< '_ > ) -> fmt::Result
    {
      c
      .debug_struct( "Context" )
      .field( "buf", &"dyn fmt::Write" )
      .field( "styles", &self.styles )
      .finish()
    }
  }

  /// A trait for converting tables to a string representation.
  pub trait TableToString< 'data >
  {
    /// Converts the table to a string representation.
    ///
    /// # Returns
    ///
    /// A `String` containing the formatted table.
    fn table_to_string( &'data self ) -> String;
  }

  impl< 'data, T > TableToString< 'data > for T
  where
    T : TableFormatter< 'data >
  {
    fn table_to_string( &'data self ) -> String
    {
      let mut output = String::new();
      let mut context = Context
      {
        buf : &mut output,
        styles : Styles::default(),
      };
      T::fmt( self, &mut context ).expect( "Table formatting failed" );
      output
    }
  }

  // xxx : improvde documentatiopn
  /// Convert extract into a string, writing it into destination buffer.
  pub trait TableWriter< CellKey >
  where
    CellKey : table::CellKey + ?Sized,
  {
    /// Convert extract into a string, writing it into destination buffer.
    fn extract_write< 'buf, 'data >( x : &FormatExtract< 'data >, c : &mut Context< 'buf > ) -> fmt::Result;
  }

  #[ derive( Debug, Default ) ]
  pub struct TableStringerDefault;

//   impl< CellKey > TableWriter< CellKey > for TableStringerDefault
//   where
//     CellKey : table::CellKey + ?Sized,
//   {
//     fn extract_write< 'buf, 'data >( x : &FormatExtract< 'data >, c : &mut Context< 'buf > ) -> fmt::Result
//     {
//       use md_math::MdOffset;
//
//       let cell_prefix = &c.styles.cell_prefix;
//       let cell_postfix = &c.styles.cell_postfix;
//       let cell_separator = &c.styles.cell_separator;
//       let row_prefix = &c.styles.row_prefix;
//       let row_postfix = &c.styles.row_postfix;
//       let row_separator = &c.styles.row_separator;
//
//       for ( irow, row ) in x.row_descriptors.iter().enumerate()
//       {
//         let height = row.0;
//
//         for islice in 0..height
//         {
//
//           if irow > 0
//           {
//             write!( c.buf, "{}", row_separator )?;
//           }
//
//           write!( c.buf, "{}", row_prefix )?;
//
//           // for k in &x.col_order
//           // for k in &x.col_order
//           for icol in 0 .. x.col_descriptors.len()
//           {
//             let col = &x.col_descriptors[ icol ];
//             let cell_width = x.data[ irow ][ icol ].1[0];
//             let width = col.0;
//             // let icol = col.1;
//             let md_index = [ islice, icol, irow as usize ];
//             let slice = x.slices[ x.slices_dim.md_offset( md_index ) ];
//
//             // println!( "md_index : {md_index:?} | md_offset : {} | slice : {slice}", x.slices_dim.md_offset( md_index ) );
//
//             if icol > 0
//             {
//               write!( c.buf, "{}", cell_separator )?;
//             }
//
//             write!( c.buf, "{}", cell_prefix )?;
//
//             let lspaces = ( width - cell_width ) / 2;
//             let rspaces = ( width - cell_width + 1 ) / 2 + cell_width - slice.len();
//             // println!( "icol : {icol} | irow : {irow} | width : {width} | cell_width : {cell_width} | lspaces : {lspaces} | rspaces : {rspaces}" );
//
//             if lspaces > 0
//             {
//               write!( c.buf, "{:<width$}", " ", width = lspaces )?;
//             }
//             write!( c.buf, "{}", slice )?;
//             if rspaces > 0
//             {
//               write!( c.buf, "{:>width$}", " ", width = rspaces )?;
//             }
//
//             write!( c.buf, "{}", cell_postfix )?;
//           }
//
//           write!( c.buf, "{}", row_postfix )?;
//         }
//
//       }
//
//       Ok(())
//     }
//   }

  /// Trait for defining table formatting logic.
  ///
  /// `TableFormatter` allows implementations to specify how tables are formatted
  /// and displayed, providing flexibility in presentation.
  ///
  /// # Type Parameters
  ///
  /// - `'data`: The lifetime of the data being formatted.
  ///
  pub trait TableFormatter< 'data >
  {
    /// Formats the table and writes the result to the provided context.
    fn fmt< 'buf >( &'data self, c : & mut Context< 'buf > ) -> fmt::Result;
  }

  /// A trait for formatting tables.
  impl< 'data, T, RowKey, Row, CellKey, CellRepr > TableFormatter< 'data >
  for AsTable< 'data, T, RowKey, Row, CellKey, CellRepr >
  where
    Self : TableRows< CellKey = CellKey, CellRepr = CellRepr, RowKey = RowKey, Row = Row >,
    Self : TableHeader< CellKey = CellKey >,
    Self : TableSize,
    RowKey : table::RowKey,
    Row : Cells< CellKey, CellRepr >,
    CellKey : table::CellKey + ?Sized,
    CellRepr : table::CellRepr,
  {
    fn fmt< 'a >( &'data self, c : &mut Context< 'a > ) -> fmt::Result
    {

      FormatExtract::extract
      (
        self,
        All,
        | x |
        {
          use md_math::MdOffset;

          let cell_prefix = &c.styles.cell_prefix;
          let cell_postfix = &c.styles.cell_postfix;
          let cell_separator = &c.styles.cell_separator;
          let row_prefix = &c.styles.row_prefix;
          let row_postfix = &c.styles.row_postfix;
          let row_separator = &c.styles.row_separator;

          for ( irow, row ) in x.row_descriptors.iter().enumerate()
          {
            let height = row.0;

            for islice in 0..height
            {

              if irow > 0
              {
                write!( c.buf, "{}", row_separator )?;
              }

              write!( c.buf, "{}", row_prefix )?;

              // for k in &x.col_order
              for icol in 0 .. x.col_descriptors.len()
              {
                let col = &x.col_descriptors[ icol ];
                let cell_width = x.data[ irow ][ icol ].1[0];
                let width = col.0;
                // let icol = col.1;
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
      )
    }
  }

  /// A struct for extracting and organizing table data for formatting.
  ///
  /// `FormatExtract` holds metadata and content necessary for formatting tables,
  /// including dimensions, column order, and data slices. It facilitates the
  /// transformation of raw table data into a structured format suitable for
  /// rendering as a table.
  ///

  #[ allow( dead_code ) ]
  #[ derive( Debug ) ]
  pub struct FormatExtract< 'data >
  // where
    // CellKey : table::CellKey + ?Sized,
  {

    /// Multidimensional size in number of columns per table and number of rows per table.
    pub mcells : [ usize ; 2 ],

    // /// Order of columns must be as stable as possible.
    // pub col_order : Vec< &'data CellKey >,

    /// Descriptors for each column, including optional title, width, and index.
    //                             key        width, index
    pub col_descriptors : Vec< ( usize, usize ) >,
    // pub col_descriptors : HashMap< usize, ( usize, usize ) >,

    /// Descriptors for each row, including height.
    //                           height
    pub row_descriptors : Vec< ( usize, ) >,

    /// Extracted data for each cell, including string content and size.
    //                        key,      string,              size,
    pub data : Vec< Vec< ( Cow< 'data, str >, [ usize ; 2 ] ) > >,

    /// Dimensions of slices for retrieving data from multi-matrix.
    pub slices_dim : [ usize ; 3 ],

    /// Extracted slices or strings for further processing.
    pub slices : Vec< & 'data str >,

    /// Indicates if the table has a header.
    pub has_header : bool,

  }

  /// Filter columns of a table to print it only partially.
  pub trait FilterCol : fmt::Debug
  {
    /// Filter columns of a table to print it only partially.
    fn filter_col< CellKey >( &self, key : &CellKey ) -> bool
    where
      CellKey : table::CellKey + ?Sized,
    ;
  }

  /// Filter passing all elements.
  #[ derive( Debug, Default, PartialEq ) ]
  pub struct All;
  impl FilterCol for All
  {
    #[ inline( always ) ]
    fn filter_col< CellKey >( &self, _key : &CellKey ) -> bool
    where
      CellKey : table::CellKey + ?Sized,
    {
      true
    }
  }

  /// Filter skipping all elements.
  #[ derive( Debug, Default, PartialEq ) ]
  pub struct No;
  impl FilterCol for No
  {
    #[ inline( always ) ]
    fn filter_col< CellKey >( &self, _key : &CellKey ) -> bool
    where
      CellKey : table::CellKey + ?Sized,
    {
      false
    }
  }

  //

  impl< 'data > FormatExtract< 'data >
  // where
    // CellKey : table::CellKey + ?Sized,
  {

    pub fn extract< 't, Table, RowKey, Row, CellKey, CellRepr > // xxx : RowKey?
    (
      table : &'t Table,
      filter_col : impl FilterCol,
      callback : impl for< 'a2 > FnOnce( &'a2 FormatExtract< 'a2 > ) -> fmt::Result,
    )
    -> fmt::Result
    where
      'data : 't,
      Table : TableRows< RowKey = RowKey, Row = Row, CellKey = CellKey, CellRepr = CellRepr >,
      Table : TableHeader< CellKey = CellKey >,
      Table : TableSize,
      RowKey : table::RowKey,
      Row : Cells< CellKey, CellRepr > + 'data,
      CellKey : table::CellKey + ?Sized + 'data,
      CellRepr : table::CellRepr,
    {
      use md_math::MdOffset;

      let mcells = table.mcells();

      //                                 key        width, index
      let mut key_to_ikey : HashMap< &'t CellKey, usize > = HashMap::new();
      // let mut col_order : Vec< &'t CellKey > = Vec::new();

      //                               width, index
      // let mut col_descriptors : Vec< ( usize, usize ) > = HashMap::new();
      let mut col_descriptors : Vec< ( usize, usize ) > = Vec::new();
      //                               height
      let mut row_descriptors : Vec< ( usize, ) > = Vec::with_capacity( mcells[ 1 ] );

      let mut has_header = false;

      let mut data : Vec< Vec< ( Cow< 't, str >, [ usize ; 2 ] ) > > = Vec::new();
      let rows = table.rows();
      let mut irow : isize = -1;

      let mut row_add = | row : &'_ mut dyn _IteratorTrait< Item = ( &'t CellKey, Cow< 't, str > ) > |
      {

        irow += 1;
        row_descriptors.push( ( 1, ) );

        let fields : Vec< ( Cow< 't, str >, [ usize ; 2 ] ) > = row
        .filter_map
        (
          | ( key, val ) |
          {

            if !filter_col.filter_col( key )
            {
              return None;
            }

            let sz = string::size( &val );
            let l = col_descriptors.len();
            let mut ikey = 0;

            key_to_ikey
            .entry( key )
            .and_modify( | _ikey |
            {
              let col = &mut col_descriptors[ *_ikey ];
              col.0 = col.0.max( sz[ 0 ] );
              // println!( "and_modify {col:?}" );
              ikey = *_ikey;
            })
            .or_insert_with( ||
            {
              let _ikey = l;
              // col_order.push( key );
              ikey = _ikey;
              col_descriptors.push( ( sz[ 0 ], _ikey ) );
              let mut col = col_descriptors[ _ikey ];
              // println!( "or_insert_with {col:?}" );
              _ikey
            });

            //   {
            //     let ikey = l;
            //     col_order.push( key );
            //     key_to_ikey.push( ( sz[ 0 ], ikey ) );
            //     ikey
            //   },
            //   Some( &ikey ) =>
            //   {
            //     let col = col_descriptors[ ikey ];
            //     col.0 = col.0.max( sz[ 0 ] );
            //     ikey
            //   },
//
//             };

            row_descriptors[ irow as usize ] = ( row_descriptors[ irow as usize ].0.max( sz[ 1 ] ), );

            // col_descriptors
            // .entry( l )
            // .and_modify( | col |
            // {
            //   col.0 = col.0.max( sz[ 0 ] );
            // })
            // .or_insert_with( ||
            // {
            //   col_order.push( key );
            //   ( sz[ 0 ], l )
            //   // let title = if is_title { Some( val.as_ref() ) } else { None };
            //   // ( title, sz[ 0 ], l )
            // });

            return Some( ( val, sz ) );
            // return Some( ( ikey, ( val, sz ) ) );
          }
        )
        .collect();
        data.push( fields );

      };

// xxx

      // process header first

      if let Some( header ) = table.header()
      {
        rows.len().checked_add( 1 ).expect( "Table has too many rows" );
        // assert!( header.len() <= usize::MAX, "Header of a table has too many cells" );
        has_header = true;

        let mut row2 =  header.map( | ( key, title ) |
        {
          // let title_str : Cow< '_, str > = Cow::Owned( format!( "{}", title ) );
          ( key, Cow::Borrowed( title ) )
        });

        row_add( &mut row2 );
      }

      // Collect rows
      //                           key,       string,           size,
      for row in rows
      {
        // assert!( row.cells().len() <= usize::MAX, "Row of a table has too many cells" );

        let mut row2 = row
        .cells()
        .map
        (
          | ( key, val ) |
          {

            let val = match val.0
            {
              Some( val ) =>
              {
                val
              }
              None =>
              {
                Cow::Borrowed( "" )
              }
            };

            return ( key, val );
          }
        );

        row_add( &mut row2 );

      }

      // cook slices multi-matrix

      let mut slices_dim = [ 1, mcells[ 0 ], mcells[ 1 ] + ( if has_header { 1 } else { 0 } ) ];
      slices_dim[ 0 ] = row_descriptors
      .iter()
      .fold( 0, | acc : usize, e | acc.max( e.0 ) )
      ;

      let slices_len = slices_dim[ 0 ] * slices_dim[ 1 ] * slices_dim[ 2 ];
      let slices : Vec< &str > = vec![ "" ; slices_len ];

      let mut x = FormatExtract::< '_ >
      {
        mcells,
        // col_order,
        col_descriptors,
        row_descriptors,
        data,
        has_header,
        slices_dim,
        slices,
      };

      // extract slices

      let mut slices : Vec< &str > = vec![];
      std::mem::swap( &mut x.slices, &mut slices );

      let mut irow : isize = -1;

      for row_data in x.data.iter()
      {

        irow += 1;

        // dbg!( x.col_descriptors.len() );
        // for ( icol, k ) in x.col_order.iter().enumerate()
        for icol in 0 .. x.col_descriptors.len()
        {
          // dbg!( &row_data, icol );
          let cell = &row_data[ icol ];
          string::lines( cell.0.as_ref() )
          .enumerate()
          .for_each( | ( layer, s ) |
          {
            let md_index = [ layer, icol, irow as usize ];
            slices[ x.slices_dim.md_offset( md_index ) ] = s;
          })
          ;
        }

      }

      std::mem::swap( &mut x.slices, &mut slices );

      return callback( &x );
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
    All,
    No,
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
  pub use super::super::print;

  #[ doc( inline ) ]
  pub use private::
  {
    Styles,
    Context,
    TableFormatter,
    TableToString,
  };

}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;
}
