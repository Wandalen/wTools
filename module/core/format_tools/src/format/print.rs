//!
//! Print data as table.
//!

/// Define a private namespace for all its items.
mod private
{

  use crate::*;
  use std::
  {
    borrow::{ Cow, Borrow },
    collections::HashMap,
  };
  use core::
  {
    fmt,
  };
  // use former::Former;

  //=

  /// A struct to configure options for printing data as a table.
  ///
  /// The `Printer` struct provides customizable delimiters for formatting table data. It allows
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

  // xxx : enable
  // #[ derive( Debug, Former ) ]
  // #[ derive( Debug ) ]
  pub struct Printer< 'callback >
  {

    /// Convert extract into a string, writing it into destination buffer.
    pub output_format : &'callback dyn TableOutputFormat,
    /// Filter out columns.
    pub filter_col : &'callback ( dyn FilterCol + 'callback ),
    /// Filter out rows.
    pub filter_row : &'callback ( dyn FilterRow + 'callback ),

  }

  impl< 'callback > Printer< 'callback >
  {
    /// Constructor accepting styles/foramt.
    pub fn with_format( output_format : &'callback dyn TableOutputFormat ) -> Self
    {
      let filter_col = Default::default();
      let filter_row = Default::default();
      Self
      {
        output_format,
        filter_col,
        filter_row
      }
    }
  }

  impl< 'callback > fmt::Debug for Printer< 'callback >
  {
    fn fmt( & self, f : & mut fmt::Formatter< '_ > ) -> fmt::Result
    {
      f.debug_struct( "Printer" )
      // .field( "cell_prefix", & self.cell_prefix )
      // .field( "cell_postfix", & self.cell_postfix )
      // .field( "cell_separator", & self.cell_separator )
      // .field( "row_prefix", & self.row_prefix )
      // .field( "row_postfix", & self.row_postfix )
      // .field( "row_separator", & self.row_separator )
      // .field( "output_format", & format_args!( "{:?}", self.output_format ) )
      // .field( "filter_col", & format_args!( "{:?}", self.filter_col ) )
      .finish()
    }
  }

  impl< 'callback > Default for Printer< 'callback >
  {
    fn default() -> Self
    {
      let output_format = Default::default();
      let filter_col = Default::default();
      let filter_row = Default::default();
      Self
      {
        output_format,
        filter_col,
        filter_row
      }
    }
  }

  /// Struct for managing table formatting context.
  ///
  /// `Context` holds the buffer and styling options used during table
  /// formatting, facilitating the writing of formatted table data.
  ///
  pub struct Context< 'context >
  {
    ///
    /// A mutable reference to a buffer implementing `fmt::Write`,
    ///   used to collect the formatted output.
    pub buf : &'context mut dyn fmt::Write,
    ///
    /// An instance of `Printer` that defines the formatting
    ///   options, such as delimiters and prefixes.
    pub printer : Printer< 'context >,
  }

  impl< 'context > Context< 'context >
  {
    /// Just constructr.
    pub fn new( buf : &'context mut dyn fmt::Write, printer : Printer< 'context > ) -> Self
    {
      Self { buf, printer }
    }
  }

  impl fmt::Debug for Context< '_ >
  {
    fn fmt( &self, c : &mut fmt::Formatter< '_ > ) -> fmt::Result
    {
      c
      .debug_struct( "Context" )
      .field( "buf", &"dyn fmt::Write" )
      .field( "printer", &self.printer )
      .finish()
    }
  }

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
    fn fmt< 'context >( &'data self, c : & mut Context< 'context > ) -> fmt::Result;

    /// Converts the table to a string representation.
    ///
    /// # Returns
    ///
    /// A `String` containing the formatted table.
    fn table_to_string( &'data self ) -> String
    {
      self.table_to_string_with_format( &output_format::Table::default() )
    }

    /// Converts the table to a string representation specifying printer.
    ///
    /// # Returns
    ///
    /// A `String` containing the formatted table.
    fn table_to_string_with_format< 'context, Styles >( &'data self, styles : &'context Styles ) -> String
    where
      Styles : TableOutputFormat,
    {
      let mut output = String::new();
      let printer = Printer
      {
        output_format : styles,
        filter_col : Default::default(),
        filter_row : Default::default(),
      };
      let mut context = Context
      {
        buf : &mut output,
        printer,
      };
      Self::fmt( self, &mut context ).expect( "Table formatting failed" );
      output
    }

  }

  /// A trait for formatting tables.
  impl< 'data, T, RowKey, Row, CellKey> TableFormatter< 'data >
  for AsTable< 'data, T, RowKey, Row, CellKey>
  where
    Self : TableRows< CellKey = CellKey, RowKey = RowKey, Row = Row >,
    Self : TableHeader< CellKey = CellKey >,
    RowKey : table::RowKey,
    Row : Cells< CellKey>,
    CellKey : table::CellKey + ?Sized,
    // CellRepr : table::CellRepr,
  {

    fn fmt< 'a >( &'data self, c : &mut Context< 'a > ) -> fmt::Result
    {

      InputExtract::extract
      (
        self,
        c.printer.filter_col,
        c.printer.filter_row,
        | x |
        {
          c.printer.output_format.extract_write( x, c )
        }
      )
    }

  }

  /// A struct for extracting and organizing row of table data for formatting.

  #[ derive( Debug, Default ) ]
  pub struct RowDescriptor
  {

    
    /// Index of the row.
    pub irow : usize,
    /// Height of the row.
    pub height : usize,
    /// Type of the line: header or regular.
    pub typ : LineType,
    /// Visibility of the row.
    pub vis : bool,
  }

  /// A struct for extracting and organizing row of table data for formatting.

  #[ derive( Debug, Default ) ]
  pub struct ColDescriptor< 'label >
  {
    /// Index of the column.
    pub icol : usize,
    /// Column width.
    pub width : usize,
    /// Label of the column.
    pub label : &'label str,
  }

  /// A struct for extracting and organizing table data for formatting.
  ///
  /// `InputExtract` holds metadata and content necessary for formatting tables,
  /// including dimensions, column order, and data slices. It facilitates the
  /// transformation of raw table data into a structured format suitable for
  /// rendering as a table.
  ///

  #[ allow( dead_code ) ]
  #[ derive( Debug ) ]
  pub struct InputExtract< 'data >
  {

    /// Multidimensional size in number of columns per table and number of rows per table.
    pub mcells : [ usize ; 2 ],

    /// Multidimensional size in number of visible columns per table and number of visible rows per table.
    pub mcells_vis : [ usize ; 2 ],

    /// Multidimensional size in number of character without taking into account grids.
    pub mchars : [ usize ; 2 ],

    /// Indicates if the table has a header.
    pub has_header : bool,

    /// Descriptors for each column, including optional title, width, and index.
    //                           width, index
    pub col_descriptors : Vec< ColDescriptor< 'data > >,

    /// Descriptors for each row, including height.
    pub row_descriptors : Vec< RowDescriptor >,

    /// Extracted data for each cell, including string content and size.
    //                      string,              size,
    pub data : Vec< Vec< ( Cow< 'data, str >, [ usize ; 2 ] ) > >, // xxx : use maybe flat vector

  }

  //

  impl< 'data > InputExtract< 'data >
  {

    /// Returns an iterator over the row descriptors, skipping the header if present.
    ///
    /// This function provides an iterator that yields each row descriptor along with its index.
    /// If the table has a header, the first row is skipped, ensuring that iteration starts from
    /// the first data row.
    ///
    /// # Returns
    ///
    /// An iterator over tuples containing:
    /// - `usize`: The index of the row.
    /// - `&RowDescriptor`: A reference to the row descriptor.
    ///
    pub fn rows( & self ) -> impl _IteratorTrait< Item = ( usize, &RowDescriptor ) >
    {
      self.row_descriptors
        .iter()
        .enumerate()
        .skip( if self.has_header { 1 } else { 0 } )
    }

    /// Returns an iterator over the header cells, or a default value if no header is present.
    ///
    /// This function provides an iterator that yields each cell in the header row. If the table
    /// does not have a header, it returns an iterator over default values, which are empty strings
    /// with a size of `[0, 1]`.
    ///
    /// # Returns
    ///
    /// A boxed iterator yielding tuples containing:
    /// - `Cow<'data, str>`: A clone-on-write string representing the cell content.
    /// - `[usize; 2]`: An array representing the size of the cell.
    ///
    pub fn header( & self ) -> Box< dyn Iterator< Item = ( Cow< 'data, str >, [ usize ; 2 ] ) > + '_ >
    {
      if self.has_header
      {
        Box::new( self.data[ 0 ].iter().cloned() )
      }
      else
      {
        Box::new( std::iter::repeat( ( Cow::Borrowed( "" ), [ 0, 1 ] ) ).take( self.mcells[ 0 ] ) )
      }
    }

    /// Returns a slice from the header, or an empty string if no header is present.
    ///
    /// # Arguments
    ///
    /// - `icol`: The column index within the header row.
    ///
    /// # Returns
    ///
    /// A string slice representing the header content.
    ///
    pub fn header_slice( & self, icol : usize ) -> & str
    {
      if self.has_header
      {
        self.data[ 0 ][ icol ].0.borrow()
      }
      else
      {
        ""
      }
    }


    /// Extract input data from and collect it in a format consumable by output formatter.
    pub fn extract< 'context, Table, RowKey, Row, CellKey>
    (
      table : &'data Table,
      filter_col : &'context ( dyn FilterCol + 'context ),
      filter_row : &'context ( dyn FilterRow + 'context ),
      callback : impl for< 'a2 > FnOnce( &'a2 InputExtract< 'a2 > ) -> fmt::Result,
    )
    -> fmt::Result
    where
      Table : TableRows< RowKey = RowKey, Row = Row, CellKey = CellKey >,
      Table : TableHeader< CellKey = CellKey >,
      RowKey : table::RowKey,
      Row : Cells< CellKey > + 'data,
      Row : Cells< CellKey > + 'data,
      CellKey : table::CellKey + ?Sized + 'data,
      // CellRepr : table::CellRepr,
    {
      let mut key_to_ikey : HashMap< Cow< 'data, str >, usize > = HashMap::new();
      let mut keys_count = 0;

      let rows = table.rows().map( | r |
      {
        let mut unsorted : Vec< ( usize, Cow< 'data, str > ) > = r.cells().map( | ( key, c ) | 
        {
          if !key_to_ikey.contains_key( key.borrow() )
          {
            key_to_ikey.insert( key.borrow().into(), keys_count );
            keys_count += 1;
          }

          ( key_to_ikey[ key.borrow() ], c.unwrap_or( Cow::from( "" ) ) )
        } ).collect();

        unsorted.sort_by( | ( i1, _ ), ( i2, _ ) | i1.cmp(i2) );

        unsorted.into_iter().map( | ( _, c ) | c).collect()
      } ).collect();

      let has_header = table.header().is_some();

      let column_names = match table.header()
      {
        Some( header ) => header.map( | ( k, _ ) | Cow::from( k.borrow() ) ).collect(),

        None => match table.rows().next()
        {
          Some( r ) => r.cells().map( | ( k, _ ) | Cow::from( k.borrow() ) ).collect(),
          None => Vec::new()
        }
      };

      Self::extract_from_raw_table
      (
        column_names,
        has_header,
        rows,
        filter_col,
        filter_row,
        callback,
      )
    }

    /// Extract input data from a table that is constructed with vectors and `Cow`s and collect
    /// it in a format consumable by output formatter.
    ///
    /// `rows` should not contain header of the table, it will be automatically added if `has_header`
    /// is true.
    pub fn extract_from_raw_table< 'context >
    (
      column_names : Vec< Cow< 'data, str > >,
      has_header : bool,
      rows : Vec< Vec< Cow< 'data, str > > >,
      filter_col : &'context ( dyn FilterCol + 'context ),
      filter_row : &'context ( dyn FilterRow + 'context ),
      callback : impl for< 'a2 > FnOnce( &'a2 InputExtract< 'a2 > ) -> fmt::Result,
    ) -> fmt::Result
    {
      // let mcells = table.mcells();
      let mut mcells_vis = [ 0 ; 2 ];
      let mut mcells = [ 0 ; 2 ];
      let mut mchars = [ 0 ; 2 ];

      //                                 key        width, index
      let mut key_to_ikey : HashMap< Cow< 'data, str >, usize > = HashMap::new();

      let mut col_descriptors : Vec< ColDescriptor< '_ > > = Vec::with_capacity( mcells[ 0 ] );
      let mut row_descriptors : Vec< RowDescriptor > = Vec::with_capacity( mcells[ 1 ] );

      let mut data : Vec< Vec< ( Cow< 'data, str >, [ usize ; 2 ] ) > > = Vec::new();
      let mut irow : usize = 0;
      let filter_col_need_args = filter_col.need_args();
      // let filter_row_need_args = filter_row.need_args();

      let mut row_add = | row_data : Vec< Cow< 'data, str > >, typ : LineType |
      {

        irow = row_descriptors.len();
        let vis = true;
        let height = 1;
        let mut row = RowDescriptor { height, typ, vis, irow };
        let mut ncol = 0;
        let mut ncol_vis = 0;

        let fields : Vec< ( Cow< 'data, str >, [ usize ; 2 ] ) > = row_data
        .into_iter()
        .enumerate()
        .filter_map
        (
          | ( ikey, val ) |
          {
            let key = &column_names[ ikey ];
            let l = col_descriptors.len();

            ncol += 1;

            if filter_col_need_args
            {
              if !filter_col.filter_col( key.as_ref() )
              {
                return None;
              }
            }
            else
            {
              if !filter_col.filter_col( "" )
              {
                return None;
              }
            }

            ncol_vis += 1;

            let sz = string::size( &val );

            key_to_ikey
            .entry( key.clone() )
            .and_modify( | icol |
            {
              let col = &mut col_descriptors[ *icol ];
              col.width = col.width.max( sz[ 0 ] );
              col.label = "";
            })
            .or_insert_with( ||
            {
              let icol = l;
              let width = sz[ 0 ];
              let col = ColDescriptor { width, icol, label : "" };
              col_descriptors.push( col );
              icol
            });

            row.height = row.height.max( sz[ 1 ] );
            return Some( ( val, sz ) );
          }
        )
        .collect();

        mcells[ 0 ] = mcells[ 0 ].max( ncol );
        mcells_vis[ 0 ] = mcells_vis[ 0 ].max( ncol_vis );

        row.vis = filter_row.filter_row( typ, irow, &fields );
        if row.vis
        {
          mcells_vis[ 1 ] += 1;
        }
        mcells[ 1 ] += 1;

        row_descriptors.push( row );
        data.push( fields );

      };

      // process header first

      if has_header
      {
        row_add( column_names.clone(), LineType::Header );
      }

      // Collect rows
      //                           key,       string,           size,
      for row in rows
      {
        // assert!( row.cells().len() <= usize::MAX, "Row of a table has too many cells" );

        row_add( row, LineType::Regular );
      }

      // calculate size in chars

      mchars[ 0 ] = col_descriptors.iter().fold( 0, | acc, col | acc + col.width );
      mchars[ 1 ] = row_descriptors.iter().fold( 0, | acc, row | acc + if row.vis { row.height } else { 0 } );
      
      let mut x = InputExtract::< '_ >
      {
        mcells,
        mcells_vis,
        mchars,
        col_descriptors,
        row_descriptors,
        data,
        has_header,
      };

      if x.data.len() > 0
      {
        for icol in 0 .. x.col_descriptors.len()
        {
          x.col_descriptors[ icol ].label = x.data[ 0 ][ icol ].0.as_ref();
        }
      }

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
    Context,
    Printer,
    InputExtract,
    RowDescriptor,
    ColDescriptor,
  };

}

/// Orphan namespace of the module.
#[ allow( unused_imports ) ]
pub mod orphan
{
  use super::*;
  #[ doc( inline ) ]
  pub use exposed::*;

  #[ doc( inline ) ]
  pub use private::
  {
  };

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
    TableFormatter,
  };

}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;
}

//
