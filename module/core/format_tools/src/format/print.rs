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
    /// Delimiter for separating table columns.
    pub cell_separator : String,

    /// Delimiter for adding prefix to a row.
    pub row_prefix : String,

    /// Delimiter for adding postfix to a row.
    pub row_postfix : String,
  }

  impl Default for Styles
  {
    fn default() -> Self
    {
      let cell_separator = " ".to_string();
      let row_prefix = "".to_string();
      let row_postfix = "\n".to_string();
      Styles { cell_separator, row_prefix, row_postfix }
    }
  }

  /// Struct for formatting tables.
  pub struct Context< 'data >
  {
    buf : &'data mut dyn fmt::Write,
    styles : Styles,
  }

  impl< 'data > Context< 'data >
  {
    /// Just constructr.
    pub fn new( buf : &'data mut dyn fmt::Write, styles : Styles ) -> Self
    {
      Self { buf, styles }
    }
  }

  impl fmt::Debug for Context< '_ >
  {
    fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
    {
      f
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

  /// A trait for formatting tables.
  ///
  /// This trait defines a method for formatting tables, allowing implementations
  /// to specify how a table should be formatted and displayed.
  ///

  pub trait TableFormatter< 'b >
  {
    /// Formats the table and writes the result to the given formatter.
    fn fmt< 'data >( &'b self, f : &mut Context< 'data > ) -> fmt::Result;
  }

  /// A trait for formatting tables.
  impl< 'data, T, RowKey, Row, CellKey, CellFormat > TableFormatter< 'data >
  for AsTable< 'data, T, RowKey, Row, CellKey, CellFormat >
  where
    Self : TableRows< RowKey, Row, CellKey, CellFormat >,
    Self : TableHeader< CellKey >,
    Self : TableSize,
    Row : Clone + Cells< CellKey, CellFormat >,
    CellKey : fmt::Debug + Clone + std::cmp::Eq + std::hash::Hash,
    CellFormat : Copy + 'static,
  {
    fn fmt( &'data self, f : &mut Context< '_ > ) -> fmt::Result
    {
      use md_math::MdOffset;

      FormatExtract::extract
      (
        self,
        | x |
        {

          // println!( "{:?}", x.slices );

          let cell_separator = &f.styles.cell_separator;
          let row_prefix = &f.styles.row_prefix;
          let row_postfix = &f.styles.row_postfix;

          for ( irow, row ) in x.row_descriptors.iter().enumerate()
          {
            let height = row.0;

            for islice in 0..height
            {
              write!( f.buf, "{}", row_prefix )?;

              for k in &x.col_order
              {
                let col = &x.col_descriptors[ &k ];
                // let cell_width = x.data[ irow ][ &k ].1[0];
                let width = col.1;
                let icol = col.2;
                // println!( "col : {:?}", col );
                let md_index = [ islice, icol, irow as usize ];
                let slice = x.slices[ x.slices_dim.md_offset( md_index ) ];

                // println!( "md_index : {md_index:?} | md_offset : {} | slice : {slice}", x.slices_dim.md_offset( md_index ) );

                if icol > 0
                {
                  write!( f.buf, "{}", cell_separator )?;
                }

                write!( f.buf, "{:^width$}", slice, width = width )?;
              }

              write!( f.buf, "{}", row_postfix )?;
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
  pub struct FormatExtract< 'data, CellKey >
  where
    CellKey : fmt::Debug + Clone + std::cmp::Eq + std::hash::Hash, // xxx
  {

    /// Multidimensional size in number of columns per table and number of rows per table.
    pub mcells : [ usize ; 2 ],

    /// Order of columns must be as stable as possible.
    pub col_order : Vec< CellKey >,

    /// Descriptors for each column, including optional title, width, and index.
    //                             key        string,                      width, index
    pub col_descriptors : HashMap< CellKey, ( Option< Cow< 'data, str > >, usize, usize ) >,

    /// Descriptors for each row, including height.
    //                           height
    pub row_descriptors : Vec< ( usize, ) >,

    /// Extracted data for each cell, including string content and size.
    //                        key,      string,              size,
    pub data : Vec< HashMap< CellKey, ( Cow< 'data, str >, [ usize ; 2 ] ) > >,

    /// Dimensions of slices for retrieving data from multi-matrix.
    pub slices_dim : [ usize ; 3 ],

    /// Extracted slices or strings for further processing.
    pub slices : Vec< & 'data str >,

    /// Indicates if the table has a header.
    pub has_header : bool,

  }

  //

  impl< 'data, CellKey > FormatExtract< 'data, CellKey >
  where
    CellKey : fmt::Debug + Clone + std::cmp::Eq + std::hash::Hash,
  {

    pub fn extract< 't, Table, RowKey, Row, CellFormat > // xxx : RowKey?
    (
      table : &'t Table,
      callback : impl for< 'a2 > FnOnce( &'a2 FormatExtract< 'a2, CellKey > ) -> fmt::Result,
    )
    -> fmt::Result
    where
      't : 'data,
      Table : TableRows< RowKey, Row, CellKey, CellFormat >,
      Table : TableHeader< CellKey >,
      Table : TableSize,
      Row : Clone + Cells< CellKey, CellFormat > + 'data,
      CellFormat : Copy + 'static,
    {
      use md_math::MdOffset;

      let mcells = table.mcells();
      //                                 key        string,                   width, index
      let mut col_descriptors : HashMap< CellKey, ( Option< Cow< '_, str > >, usize, usize ) > = HashMap::new();
      //                               height
      let mut row_descriptors : Vec< ( usize, ) > = Vec::with_capacity( mcells[ 1 ] );

      let mut col_order : Vec< CellKey > = Vec::new();
      let mut has_header = false;

      let mut data : Vec< HashMap< CellKey, ( Cow< '_, str >, [ usize ; 2 ] ) > > = Vec::new();
      let rows = table.rows();
      let mut irow : isize = -1;

      let mut row_add = | row : &mut dyn _IteratorTrait< Item = ( CellKey, Cow< 'data, str > ) > |
      {

        irow += 1;
        row_descriptors.push( ( 1, ) );

        let fields : HashMap< CellKey, ( Cow< '_, str >, [ usize ; 2 ] ) > = row
        // .cells()
        .map
        (
          | ( key, val ) |
          {

            let sz = string::size( &val );
            let l = col_descriptors.len();
            row_descriptors[ irow as usize ] = ( row_descriptors[ irow as usize ].0.max( sz[ 1 ] ), );

            col_descriptors
            .entry( key.clone() )
            .and_modify( | col |
            {
              col.1 = col.1.max( sz[ 0 ] );
            })
            .or_insert_with( ||
            {
              col_order.push( key.clone() );
              ( None, sz[ 0 ], l )
            });

            return ( key, ( val, sz ) );
          }
        )
        .collect();
        data.push( fields );


      };

      // process header first

      if let Some( header ) = table.header()
      {
        rows.len().checked_add( 1 ).expect( "Table has too many rows" );
        // assert!( header.len() <= usize::MAX, "Header of a table has too many cells" );
        has_header = true;

        let mut row2 =  header.map( | ( key, title ) |
        {
          let title_str : Cow< '_, str > = Cow::Owned( format!( "{}", title ) );

          // let l = col_descriptors.len();
          // let sz = string::size( &title_str );
          // col_descriptors
          // .entry( key.clone() )
          // .and_modify( | col |
          // {
          //   col.1 = col.1.max( sz[ 0 ] );
          // })
          // .or_insert_with( ||
          // {
          //   col_order.push( key.clone() );
          //   ( Some( title_str ), sz[ 0 ], l )
          // });

          ( key, title )
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

      println!( "row_descriptors : {row_descriptors:?}" );
      println!( "slices_dim : {slices_dim:?}" );
      println!( "slices_len : {slices_len:?}" );

      // println!( "{:?}", self.slices );

      let mut x = FormatExtract::< '_, CellKey >
      {
        mcells,
        col_order,
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

//       println!( "col_order : {:?}", x.col_order );
//       println!( "col_descriptors : {:?}", x.col_descriptors.keys() );
//
//       let col : &( Option< Cow< '_, str > >, usize, usize ) = &x.col_descriptors[ &x.col_order[ 0 ] ];
//       println!( "col : {:?}", col );
//       slices[ 0 ] = col.0.as_ref().unwrap();
//
//       let mut irow : isize = -1;
//       if x.has_header
//       {
//
//         irow += 1;
//         for ( icol, k ) in x.col_order.iter().enumerate()
//         {
//           let col : &( _, _, _ ) = &x.col_descriptors[ k ];
//           let cell = &col.0;
//
//           if let Some( cell ) = cell
//           {
//
//             string::lines( cell )
//             .enumerate()
//             .for_each( | ( layer, s ) |
//             {
//               let md_index = [ layer, icol, irow as usize ];
//               // println!( "s : {s} | md_index : {md_index:?}" );
//               slices[ x.slices_dim.md_offset( md_index ) ] = s;
//             })
//             ;
//
//           }
//         }
//
//       }

      for row_data in x.data.iter()
      {

        irow += 1;

        for ( icol, k ) in x.col_order.iter().enumerate()
        {
          let cell = &row_data[ &k ];
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
