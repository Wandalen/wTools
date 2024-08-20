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
  // xxx : enable
  // #[ derive( Debug, Former ) ]
  #[ derive( Debug ) ]
  pub struct Styles< 'callback >
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

    /// Convert extract into a string, writing it into destination buffer.
    pub output_format : &'static dyn TableOutputFormat,
    /// Filter out columns.
    pub filter_col : &'callback dyn FilterCol,

  }

  impl< 'callback > Default for Styles< 'callback >
  {
    fn default() -> Self
    {
      let cell_prefix = "".to_string();
      let cell_postfix = "".to_string();
      let cell_separator = " │ ".to_string();
      let row_prefix = "│ ".to_string();
      let row_postfix = " │".to_string();
      let row_separator = "\n".to_string();
      let output_format = Default::default();
      let filter_col = Default::default();
      Self
      {
        cell_prefix,
        cell_postfix,
        cell_separator,
        row_prefix,
        row_postfix,
        row_separator,
        output_format,
        filter_col,
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
    /// An instance of `Styles` that defines the formatting
    ///   options, such as delimiters and prefixes.
    pub styles : Styles< 'context >,
  }

  impl< 'context > Context< 'context >
  {
    /// Just constructr.
    pub fn new( buf : &'context mut dyn fmt::Write, styles : Styles< 'context > ) -> Self
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

      InputExtract::extract
      (
        self,
        c.styles.filter_col,
        | x |
        {
          c.styles.output_format.extract_write( x, c )
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

    /// Multidimensional size in number of columns per table and number of rows per table.
    pub mcells : [ usize ; 2 ],

    /// Descriptors for each column, including optional title, width, and index.
    //                             key        width, index
    pub col_descriptors : Vec< ( usize, usize ) >,

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
    fn filter_col( &self, key : &str ) -> bool;
  }

  /// Filter passing all elements.
  #[ derive( Debug, Default, PartialEq, Clone, Copy ) ]
  pub struct All;

  impl All
  {
    /// Returns a reference to a static instance of `Ordinary`.
    ///
    /// This method provides access to a single shared instance of `Ordinary`,
    /// ensuring efficient reuse of the classic table output format.
    pub fn instance() -> & 'static dyn FilterCol
    {
      static INSTANCE : All = All;
      &INSTANCE
    }
  }

  impl Default for &'static dyn FilterCol
  {
    #[ inline( always ) ]
    fn default() -> Self
    {
      All::instance()
    }
  }

  impl FilterCol for All
  {
    #[ inline( always ) ]
    fn filter_col( &self, _key : &str ) -> bool
    {
      true
    }
  }

  /// Filter skipping all elements.
  #[ derive( Debug, Default, PartialEq, Clone, Copy ) ]
  pub struct No;
  impl FilterCol for No
  {
    #[ inline( always ) ]
    fn filter_col( &self, _key : &str ) -> bool
    {
      false
    }
  }

  impl No
  {
    /// Returns a reference to a static instance of `Ordinary`.
    ///
    /// This method provides access to a single shared instance of `Ordinary`,
    /// ensuring efficient reuse of the classic table output format.
    pub fn instance() -> & 'static dyn FilterCol
    {
      static INSTANCE : All = All;
      &INSTANCE
    }
  }

  impl< F : Fn( &str ) -> bool + fmt::Debug > FilterCol for F
  {
    #[ inline( always ) ]
    fn filter_col( &self, key : &str ) -> bool
    {
      self( key )
    }
  }

  //

  impl< 'data > InputExtract< 'data >
  {

    /// Extract input data from and collect it in a format consumable by output formatter.
    pub fn extract< 't, 'context, Table, RowKey, Row, CellKey, CellRepr >
    (
      table : &'t Table,
      filter_col : &'context dyn FilterCol,
      callback : impl for< 'a2 > FnOnce( &'a2 InputExtract< 'a2 > ) -> fmt::Result,
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

      //                               width, index
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

            if !filter_col.filter_col( key.borrow() )
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
              ikey = *_ikey;
            })
            .or_insert_with( ||
            {
              let _ikey = l;
              ikey = _ikey;
              col_descriptors.push( ( sz[ 0 ], _ikey ) );
              _ikey
            });

            row_descriptors[ irow as usize ] = ( row_descriptors[ irow as usize ].0.max( sz[ 1 ] ), );

            return Some( ( val, sz ) );
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

      let mut x = InputExtract::< '_ >
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
    Context,
    Styles,
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
    TableFormatter,
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
    TableToString,
  };

}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;
}
