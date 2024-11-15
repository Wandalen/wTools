//!
//! Print data as table.
//!

/// Define a private namespace for all its items.
mod private
{

  use crate::*;
  use md_math::MdOffset;
  use std::
  {
    borrow::Cow,
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
    pub irow : usize,
    pub height : usize,
    pub typ : LineType,
    pub vis : bool,
  }

  /// A struct for extracting and organizing row of table data for formatting.

  #[ derive( Debug, Default ) ]
  pub struct ColDescriptor< 'label >
  {
    pub icol : usize,
    pub width : usize,
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

    /// Dimensions of slices for retrieving data from multi-matrix.
    pub slices_dim : [ usize ; 3 ],
    /// Extracted slices or strings for further processing.
    pub slices : Vec< &'data str >,

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
    /// This function retrieves a specific slice from the header row based on the provided indices.
    /// If the table does not have a header, it returns an empty string.
    ///
    /// # Arguments
    ///
    /// - `islice`: The slice index within the header cell.
    /// - `icol`: The column index within the header row.
    ///
    /// # Returns
    ///
    /// A string slice representing the header content at the specified indices.
    ///
    pub fn header_slice( & self, islice : usize, icol : usize ) -> & str
    {
      if self.has_header
      {
        let md_index = [ islice, icol, 0 ];
        self.slices[ self.slices_dim.md_offset( md_index ) ]
      }
      else
      {
        ""
      }
    }
    /// Extract input data from and collect it in a format consumable by output formatter.
    pub fn extract< 't, 'context, Table, RowKey, Row, CellKey>
    (
      table : &'t Table,
      filter_col : &'context ( dyn FilterCol + 'context ),
      filter_row : &'context ( dyn FilterRow + 'context ),
      callback : impl for< 'a2 > FnOnce( &'a2 InputExtract< 'a2 > ) -> fmt::Result,
    )
    -> fmt::Result
    where
      'data : 't,
      // 't : 'data,
      Table : TableRows< RowKey = RowKey, Row = Row, CellKey = CellKey >,
      Table : TableHeader< CellKey = CellKey >,
      RowKey : table::RowKey,
      Row : Cells< CellKey> + 'data,
      CellKey : table::CellKey + ?Sized + 'data,
      // CellRepr : table::CellRepr,
    {
      use md_math::MdOffset;

      // let mcells = table.mcells();
      let mut mcells_vis = [ 0 ; 2 ];
      let mut mcells = [ 0 ; 2 ];
      let mut mchars = [ 0 ; 2 ];

      //                                 key        width, index
      let mut key_to_ikey : HashMap< &'t CellKey, usize > = HashMap::new();

      let mut col_descriptors : Vec< ColDescriptor< '_ > > = Vec::with_capacity( mcells[ 0 ] );
      let mut row_descriptors : Vec< RowDescriptor > = Vec::with_capacity( mcells[ 1 ] );
      let mut has_header = false;

      let mut data : Vec< Vec< ( Cow< 't, str >, [ usize ; 2 ] ) > > = Vec::new();
      let rows = table.rows();
      let mut irow : usize = 0;
      let filter_col_need_args = filter_col.need_args();
      // let filter_row_need_args = filter_row.need_args();

      let mut row_add = | row_iter : &'_ mut dyn _IteratorTrait< Item = ( &'t CellKey, Cow< 't, str > ) >, typ : LineType |
      {

        irow = row_descriptors.len();
        let vis = true;
        let height = 1;
        let mut row = RowDescriptor { height, typ, vis, irow };
        let mut ncol = 0;
        let mut ncol_vis = 0;

        let fields : Vec< ( Cow< 't, str >, [ usize ; 2 ] ) > = row_iter
        .filter_map
        (
          | ( key, val ) |
          {
            let l = col_descriptors.len();

            ncol += 1;

            if filter_col_need_args
            {
              if !filter_col.filter_col( key.borrow() )
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
            .entry( key )
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

      if let Some( header ) = table.header()
      {
        rows.len().checked_add( 1 ).expect( "Table has too many rows" );
        // assert!( header.len() <= usize::MAX, "Header of a table has too many cells" );
        has_header = true;

        let mut row2 =  header.map( | ( key, title ) |
        {
          ( key, Cow::Borrowed( title ) )
        });

        row_add( &mut row2, LineType::Header );
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

            let val = match val
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

        row_add( &mut row2, LineType::Regular );
      }

      // calculate size in chars

      mchars[ 0 ] = col_descriptors.iter().fold( 0, | acc, col | acc + col.width );
      mchars[ 1 ] = row_descriptors.iter().fold( 0, | acc, row | acc + if row.vis { row.height } else { 0 } );

      // cook slices multi-matrix

      let mut slices_dim = [ 1, mcells[ 0 ], mcells[ 1 ] ];
      slices_dim[ 0 ] = row_descriptors
      .iter()
      .fold( 0, | acc : usize, row | acc.max( row.height ) )
      ;

      let slices_len = slices_dim[ 0 ] * slices_dim[ 1 ] * slices_dim[ 2 ];
      let slices : Vec< &str > = vec![ "" ; slices_len ];

  //     assert_eq!( mcells, mcells, r#"Incorrect multidimensional size of table
  // mcells <> mcells
  // {mcells:?} <> {mcells:?}"# );
  //     println!( "mcells : {mcells:?} | mcells : {mcells:?} | mcells_vis : {mcells_vis:?}" );

      let mut x = InputExtract::< '_ >
      {
        mcells,
        mcells_vis,
        mchars,
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

        for icol in 0 .. x.col_descriptors.len()
        {
          let cell = &row_data[ icol ];
          string::lines( cell.0.as_ref() )
          .enumerate()
          .for_each( | ( layer, s ) |
          {
            let md_index = [ layer, icol, irow as usize ];
            slices[ x.slices_dim.md_offset( md_index ) ] = s;
          })
          ;
          if irow == 0
          {
            x.col_descriptors[ icol ].label = cell.0.as_ref();
          }
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
