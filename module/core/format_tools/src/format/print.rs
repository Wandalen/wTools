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
    collections::BTreeMap,
  };
  use core::
  {
    fmt,
    borrow::Borrow,
  };
  use former::Former;

  //=

  /// Struct to hold options to print data as table.
  #[ derive( Debug, Former ) ]
  pub struct Styles
  {
    /// Delimiter for separating table columns.
    pub cell_separator : String,
    pub row_prefix : String,
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
    CellKey : fmt::Debug + Clone + std::cmp::Eq + std::hash::Hash,
    CellFormat : Copy + 'static,
  {
    fn fmt( &'a self, f : &mut Context< '_ > ) -> fmt::Result
    {

      let table_size = self.table_size();
      //                                 key        string,                   size,          index
      let mut col_descriptors : HashMap< CellKey, ( Option< Cow< '_, str > >, [ usize ; 2 ], usize ) > = HashMap::new();
      let mut col_order : Vec< CellKey > = Vec::new();
      let cell_separator = &f.styles.cell_separator;
      let row_prefix = &f.styles.row_prefix;
      let row_postfix = &f.styles.row_postfix;

      // process header first

      if let Some( header ) = self.header()
      {
        for ( key, title ) in header
        {
          let title_str : Cow< '_, str > = Cow::Owned( format!( "{}", title ) );
          let l = col_descriptors.len();
          col_descriptors
          .entry( key.clone() )
          .and_modify( | e |
          {
            let sz = string::size( &title_str );
            e.1[ 0 ] = e.1[ 0 ].max( sz[ 0 ] );
            e.1[ 1 ] = e.1[ 1 ].max( sz[ 1 ] );
          })
          .or_insert_with( ||
          {
            col_order.push( key.clone() );
            let sz = string::size( &title_str );
            ( Some( title_str ), sz, l + 1 )
          });
        }
      }

      // Collect rows
      //                           key,       string,         size,

      let mut data : Vec< HashMap< CellKey, ( Cow< '_, str >, [ usize ; 2 ] ) > > = Vec::new(); // xxx : continue
      for row in self.rows()
      {
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

            col_descriptors
            .entry( r.0.clone() )
            .and_modify( | e |
            {
              e.1[ 0 ] = e.1[ 0 ].max( sz[ 0 ] );
              e.1[ 1 ] = e.1[ 1 ].max( sz[ 1 ] );
            })
            .or_insert_with( ||
            {
              col_order.push( r.0.clone() );
              ( None, sz, l + 1 )
            });

            return ( r.0, ( r.1, sz ) );
          }
        )
        .collect();
        data.push( fields );
      }

      // Write head with proper alignment
      if let Some( header ) = self.header()
      {
        // xxx : rid of vector
        let mut formatted_row : Vec< String > = Vec::with_capacity( col_order.len() );
        for k in &col_order
        {
          let descriptor = &col_descriptors[ &k ];
          let sz = descriptor.1;
          let cell = descriptor.0.as_ref().unwrap_or( &Cow::Borrowed( "" ) );
          formatted_row.push( format!( "{:^width$}", cell, width = sz[ 0 ] ) );
        }
        writeln!( f.buf, "{}{}{}", row_prefix, formatted_row.join( cell_separator ), row_postfix )?;
      }

      // Write rows with proper alignment
      for row in data
      {
        // xxx : rid of vector
        let height = row.iter().fold( 1, | acc, ( k, e ) | acc.max( e.1[ 1 ] ) );
        // println!( "height : {height}" );

        let mut formatted_row : Vec< String > = Vec::with_capacity( col_order.len() );
        for k in &col_order
        {
          let cell = &row[ &k ];
          let descriptor = &col_descriptors[ &k ];
          let sz = descriptor.1;
          println!( "sz : {sz:?}" );
          formatted_row.push( format!( "{:^.width$}", cell.0.as_ref(), width = sz[ 0 ] ) );
        }
        writeln!( f.buf, "{}{}{}", row_prefix, formatted_row.join( cell_separator ), row_postfix )?;
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
