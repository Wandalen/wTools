//!
//! Print data as table.
//!

/// Define a private namespace for all its items.
mod private
{

  use crate::*;
  use std::borrow::Cow;
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

    /// Indicates if the table has a header.
    pub has_header : bool,

    /// Extracted data for each cell.
    /// If the table has a header, then the first row is treated as a header row with column names.
    pub data : Vec< Vec< Cow< 'data, str > > >,

  }

  //

  impl< 'data > InputExtract< 'data >
  {

    /// Returns an iterator over the row descriptors, skipping the header if present.
    ///
    /// This function provides an iterator that yields each row descriptor along with its index.
    /// If the table has a header, the first row is skipped, ensuring that iteration starts from
    /// the first data row.
    pub fn rows( & self ) -> impl _IteratorTrait< Item = &Vec< Cow< 'data, str > > >
    {
      self.data
        .iter()
        .skip( if self.has_header { 1 } else { 0 } )
    }

    /// Returns an iterator over the header cells, or a default value if no header is present.
    ///
    /// This function provides an iterator that yields each cell in the header row. If the table
    /// does not have a header, it returns an iterator over default values, which are empty strings.
    pub fn header( & self ) -> Box< dyn Iterator< Item = Cow< 'data, str > > + '_ >
    {
      if self.has_header && self.data.len() != 0
      {
        Box::new( self.data[ 0 ].iter().cloned() )
      }
      else
      {
        let col_count = if self.data.len() == 0 { 0 } else { self.data[0].len() };
        Box::new( std::iter::repeat( Cow::Borrowed( "" ) ).take( col_count ) )
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
      Row : Cells< CellKey > + 'data,
      CellKey : table::CellKey + ?Sized + 'data,
      // CellRepr : table::CellRepr,
    {
      let mut has_header = false;

      let mut data : Vec< Vec< Cow< 't, str > > > = Vec::new();
      let rows = table.rows();

      let mut row_add = | row_iter : &'_ mut dyn _IteratorTrait< Item = ( &'t CellKey, Cow< 't, str > ) > |
      {
        let fields : Vec< Cow< 't, str > > = row_iter
        .filter_map
        (
          | ( key, val ) |
          {
            if !filter_col.filter_col( key.borrow() )
            {
              return None;
            }

            return Some( val );
          }
        )
        .collect();

        if filter_row.filter_row( &fields )
        {
          data.push( fields );
        }
      };

      if let Some( header ) = table.header()
      {
        has_header = true;

        let mut row2 = header.map( | ( key, title ) |
        {
          ( key, Cow::Borrowed( title ) )
        });

        row_add( &mut row2 );
      }

      for row in rows
      {
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

        row_add( &mut row2 );
      }

      let x = InputExtract::< '_ >
      {
        data,
        has_header,
      };

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
