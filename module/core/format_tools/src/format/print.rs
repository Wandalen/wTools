//!
//! Nice print.
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
      let row_postfix = "".to_string();
      Styles { cell_separator, row_prefix, row_postfix }
    }
  }

  /// Struct for formatting tables.
  pub struct Context< 'a >
  {
    buf : &'a mut dyn fmt::Write,
    styles : Styles,
  }

  impl< 'a > Context< 'a >
  {
    /// Just constructr.
    pub fn new( buf : &'a mut dyn fmt::Write, styles : Styles ) -> Self
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
  pub trait TableToString< 'a >
  {
    /// Converts the table to a string representation.
    ///
    /// # Returns
    ///
    /// A `String` containing the formatted table.
    fn table_to_string( &'a self ) -> String;
  }

  impl< 'a, T > TableToString< 'a > for T
  where
    T : TableFormatter< 'a >
  {
    fn table_to_string( &'a self ) -> String
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
    fn fmt< 'a >( &'b self, f : &mut Context< 'a > ) -> fmt::Result;
  }

  /// A trait for formatting tables.
  impl< 'a, T, RowKey, Row, CellKey, CellFormat > TableFormatter< 'a >
  for AsTable< 'a, T, RowKey, Row, CellKey, CellFormat >
  where
    Self : TableRows< RowKey, Row, CellKey, CellFormat >,
    Self : TableHeader< CellKey >,
    Self : TableSize,
    Row : Clone + Cells< CellKey, CellFormat >,
    CellKey : fmt::Debug + Clone + std::cmp::Eq + std::hash::Hash + 'static,
    CellFormat : Copy + 'static,
  {
    fn fmt( &'a self, f : &mut Context< '_ > ) -> fmt::Result
    {

      let mut x = FormatExtract::extract( self );
      // x.extract_slices();

//       let FormatExtract
//       {
//         mcells,
//         col_order,
//         col_descriptors,
//         data,
//         ..
//       } = x;

      let cell_separator = &f.styles.cell_separator;
      let row_prefix = &f.styles.row_prefix;
      let row_postfix = &f.styles.row_postfix;

      // Write head with proper alignment
      if let Some( header ) = self.header()
      {
        // xxx : rid of vector
        let mut formatted_row : Vec< String > = Vec::with_capacity( x.col_order.len() );
        for k in &x.col_order
        {
          let descriptor = &x.col_descriptors[ &k ];
          let width = descriptor.1;
          let cell = descriptor.0.as_ref().unwrap_or( &Cow::Borrowed( "" ) );
          formatted_row.push( format!( "{:^width$}", cell, width = width ) );
        }
        writeln!( f.buf, "{}{}{}", row_prefix, formatted_row.join( cell_separator ), row_postfix )?;
      }

      // Write rows with proper alignment
      for row in &x.data
      {
        // xxx : rid of vector
        let height = row.iter().fold( 1, | acc, ( _k, e ) | acc.max( e.1[ 1 ] ) );
        // println!( "height : {height}" );

        let mut formatted_row : Vec< String > = Vec::with_capacity( x.col_order.len() );
        for k in &x.col_order
        {
          let cell = &row[ &k ];
          let descriptor = &x.col_descriptors[ &k ];
          let width = descriptor.1;
          println!( "width : {width:?}" );
          formatted_row.push( format!( "{:^width$}", cell.0.as_ref(), width = width ) );
        }
        writeln!( f.buf, "{}{}{}", row_prefix, formatted_row.join( cell_separator ), row_postfix )?;
      }

      Ok(())
    }
  }

  pub struct FormatExtract< 'a, 'b, CellKey >
  where
    CellKey : fmt::Debug + Clone + std::cmp::Eq + std::hash::Hash + 'static, // xxx
    'a : 'b,
    // 'b : 'a,
  {

    /// Multidimensional size in number of columns per table and number of rows per table.
    pub mcells : [ usize ; 2 ],

    /// Order of columns must be as stable as possible.
    pub col_order : Vec< CellKey >,

    //                             key        string,                   width, index
    pub col_descriptors : HashMap< CellKey, ( Option< Cow< 'a, str > >, usize, usize ) >,

    //                           height
    pub row_descriptors : Vec< ( usize, ) >,

    /// Either slices or strings extracted for further processsing.
    //                           key, string,           size,
    pub data : Vec< HashMap< CellKey, ( Cow< 'a, str >, [ usize ; 2 ] ) > >,

    /// Multidimensional size in number of subrows per row, number of columns per table and number of rows per table.
    /// Use it to retrive corresponding slice from multi-matrix of slices.
    pub slices_dim : [ usize ; 3 ],

    /// Either slices or strings extracted for further processsing.
    pub slices : Vec< &'b str >,

    // Does table have the header.
    pub has_header : bool,

  }

  impl< 'a, 'b, CellKey > FormatExtract< 'a, 'b, CellKey >
  where
    CellKey : fmt::Debug + Clone + std::cmp::Eq + std::hash::Hash + 'static,
    'a : 'b,
    // 'b : 'a,
  {

    pub fn extract_slices< 'c : 'b >( &'c mut self )
    {
      use md_math::MdOffset;

      let mut slices : Vec< &str > = vec![];
      std::mem::swap( &mut self.slices, &mut slices );

      let col : &( Option< Cow< '_, str > >, usize, usize ) = &self.col_descriptors[ &self.col_order[ 0 ] ];
      slices[ 0 ] = col.0.as_ref().unwrap();

      let mut irow : isize = -1;
      if self.has_header
      {

        irow += 1;
        for ( icol, k ) in self.col_order.iter().enumerate()
        {
          let col : &( _, _, _ ) = &self.col_descriptors[ k ];
          let cell = &col.0;

          if let Some( cell ) = cell
          {

            string::lines( cell )
            .enumerate()
            .for_each( | ( layer, s ) | slices[ [ layer, icol, irow as usize ].md_offset( self.slices_dim ) ] = s )
            ;

          }

        }

      }

      for row_data in self.data.iter()
      {

        irow += 1;
        let row = &self.row_descriptors[ irow as usize ];

        for ( icol, k ) in self.col_order.iter().enumerate()
        {
          let cell = &row_data[ &k ];
          string::lines( cell.0.as_ref() )
          .enumerate()
          .for_each( | ( layer, s ) | slices[ [ layer, icol, irow as usize ].md_offset( self.slices_dim ) ] = s )
          ;
        }

      }

      std::mem::swap( &mut self.slices, &mut slices );
    }

    pub fn extract< Table, RowKey, Row, CellFormat >( table : &'a Table ) -> Self
    where
      Table : TableRows< RowKey, Row, CellKey, CellFormat >,
      Table : TableHeader< CellKey >,
      Table : TableSize,
      Row : Clone + Cells< CellKey, CellFormat > + 'a,
      CellFormat : Copy + 'static,
    {

      let mcells = table.mcells();
      //                                 key        string,                   width, index
      let mut col_descriptors : HashMap< CellKey, ( Option< Cow< '_, str > >, usize, usize ) > = HashMap::new();
      //                             height
      let mut row_descriptors : Vec< ( usize, ) > = Vec::with_capacity( mcells[ 1 ] );

      let mut col_order : Vec< CellKey > = Vec::new();
      let mut has_header = false;

      // process header first

      let mut row_number : isize = -1;
      if let Some( header ) = table.header()
      {
        assert!( header.len() <= usize::MAX, "Header of a table has too many cells" );
        has_header = true;
        row_number = 0;
        row_descriptors.push( ( 1, ) );

        for ( key, title ) in header
        {
          let title_str : Cow< '_, str > = Cow::Owned( format!( "{}", title ) );
          let l = col_descriptors.len();
          let sz = string::size( &title_str );

          col_descriptors
          .entry( key.clone() )
          .and_modify( | col |
          {
            col.1 = col.1.max( sz[ 1 ] );
          })
          .or_insert_with( ||
          {
            col_order.push( key.clone() );
            ( Some( title_str ), sz[ 1 ], l + 1 )
          });

          row_descriptors[ row_number as usize ] = ( row_descriptors[ row_number as usize ].0.max( sz[ 0 ] ), );
          debug_assert!( row_descriptors.len() == ( row_number as usize ) + 1 );

        }
      }

      // Collect rows
      //                           key,       string,         size,
      let mut data : Vec< HashMap< CellKey, ( Cow< '_, str >, [ usize ; 2 ] ) > > = Vec::new();
      assert!( table.rows().len() <= usize::MAX, "Table has too many rows" );
      for row in table.rows()
      {
        assert!( row.cells().len() <= usize::MAX, "Row of a table has too many cells" );

        row_number += 1;
        row_descriptors.push( ( 1, ) );

        let fields : HashMap< CellKey, ( Cow< '_, str >, [ usize ; 2 ] ) > = row
        .cells()
        .map
        (
          | ( key, cell ) |
          {
            let r = match cell.0
            {
              Some( cell ) =>
              {
                ( key, cell )
              }
              None =>
              {
                ( key, Cow::Borrowed( "" ) )
              }
            };

            let sz = string::size( &r.1 );
            let l = col_descriptors.len();
            row_descriptors[ row_number as usize ] = ( row_descriptors[ row_number as usize ].0.max( sz[ 0 ] ), );

            col_descriptors
            .entry( r.0.clone() )
            .and_modify( | col |
            {
              col.1 = col.1.max( sz[ 1 ] );
            })
            .or_insert_with( ||
            {
              col_order.push( r.0.clone() );
              ( None, sz[ 1 ], l + 1 )
            });

            return ( r.0, ( r.1, sz ) );
          }
        )
        .collect();
        data.push( fields );
      }

      // cook slices multi-matrix

      let mut slices_dim = [ 1, mcells[ 0 ], mcells[ 1 ] + ( if has_header { 1 } else { 0 } ) ];
      slices_dim[ 0 ] = row_descriptors
      .iter()
      .fold( 0, | acc : usize, e | acc.max( e.0 ) )
      ;

      let slices_len = slices_dim[ 0 ] * slices_dim[ 1 ] * slices_dim[ 2 ];
      let mut slices : Vec< &str > = vec![ "" ; slices_len ];

      //

      let mut r = Self
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

      r

//       use md_math::MdOffset;
//       let mut slices : Vec< &str > = vec![];
//       std::mem::swap( &mut r.slices, &mut slices );
//
//       let col : &( Option< Cow< '_, str > >, usize, usize ) = &r.col_descriptors[ &r.col_order[ 0 ] ];
//       slices[ 0 ] = col.0.as_ref().unwrap();
//
//       let mut irow : isize = -1;
//       if r.has_header
//       {
//
//         irow += 1;
//         for ( icol, k ) in r.col_order.iter().enumerate()
//         {
//           let col : &( _, _, _ ) = &r.col_descriptors[ k ];
//           let cell = &col.0;
//
//           if let Some( cell ) = cell
//           {
//
//             string::lines( cell )
//             .enumerate()
//             .for_each( | ( layer, s ) | slices[ [ layer, icol, irow as usize ].md_offset( r.slices_dim ) ] = s )
//             ;
//
//           }
//
//         }
//
//       }
//
//       r
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
