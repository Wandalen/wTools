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
    pub separator : String,
  }

  impl Default for Styles
  {
    fn default() -> Self
    {
      let separator = " | ".to_string();
      Styles { separator }
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
      T::fmt( self, &mut context ).expect( "Formatting failed" );
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
  impl< 'a, T, RowKey, Row, CellKey, CellFormat, Title > TableFormatter< 'a >
  for AsTable< 'a, T, RowKey, Row, CellKey, CellFormat, Title >
  where
    Self : TableRows< RowKey, Row, CellKey, CellFormat >,
    Self : TableHeader< CellKey, Title >,
    Self : TableSize,
    Row : Clone + Cells< CellKey, CellFormat >,
    Title : fmt::Display,
    CellKey : fmt::Debug + Clone + std::cmp::Eq + std::hash::Hash,
    // Cell : fmt::Display,
    // Cell : std::borrow::ToOwned + ?Sized,
    // Cell : AsRef< str >,
    CellFormat : Copy + 'static,
  {
    fn fmt( &'a self, f : &mut Context< '_ > ) -> fmt::Result
    {

      let table_size = self.table_size();
      // let mut widths : Vec< usize > = vec![ 0 ; table_size[ 1 ] ];
      // let mut widths : BTreeMap< CellKeyWrap< CellKey >, usize > = BTreeMap::new();
      // let mut widths = HashMap::< CellKey, usize >::new();
      let mut key_to_col : HashMap< CellKey, ( usize, usize, Option< Cow< '_, str > > ) > = HashMap::new();
      let mut cols : Vec< CellKey > = Vec::new();

      let separator = &f.styles.separator;

      // dbg!( &widths );

      // process header first

      if let Some( header ) = self.header()
      {
        // let mut i = 0;
        for ( key, title ) in header
        {
          let title_str : Cow< '_, str > = Cow::Owned( format!( "{}", title ) );
          let l = key_to_col.len();
          key_to_col
          .entry( key.clone() )
          .and_modify( | e | { e.1 = e.1.max( title_str.len() ) } )
          .or_insert( ( l + 1, title_str.len(), Some( title_str ) ) );
          // widths[ &( key, i ).into() ] = format!( "{}", title ).len();
          // i += 1;
        }
        writeln!( f.buf )?;
      }

      // Collect rows
      // let mut data : Vec< BTreeMap< CellKeyWrap< CellKey >, Cow< '_, str > > > = Vec::new();
      let mut data : Vec< HashMap< CellKey, Cow< '_, str > > > = Vec::new();
      for row in self.rows()
      {
        let mut i = 0;
        // let fields : Vec< String > = row
        // let fields : BTreeMap< CellKeyWrap< CellKey >, Cow< '_, str > > = row
        let fields : HashMap< CellKey, Cow< '_, str > > = row
        .cells()
        .map
        (
          | ( key, cell ) |
          {
            let r = match cell.0
            {
              // Some( cell ) => ( ( key, 0 ).into(), format!( "{}", < Cow< '_, Cell > as Borrow< Cell > >::borrow( &cell ) ) ),
              // None => ( ( key, 0 ).into(), "".to_string() ),
              Some( cell ) =>
              {
                let l = key_to_col.len();
                key_to_col
                .entry( key.clone() )
                .and_modify( | e | { e.1 = e.1.max( cell.len() ) } )
                .or_insert( ( l + 1, cell.len(), None ) );
                ( key, cell )
              }
              // None => ( key, Cow::Borrowed( "" ) ),
              None =>
              {
                let l = key_to_col.len();
                key_to_col
                .entry( key.clone() )
                .or_insert( ( l + 1, 0, None ) );
                ( key, Cow::Borrowed( "" ) )
              }
            };
            i += 1;
            return r;
          }
        )
        // .map
        // (
        //   | ( _key, cell ) |
        //   {
        //     match cell.0
        //     {
        //       // Some( cell ) => format!( "{}", cell.borrow() ),
        //       Some( cell ) => format!( "{}", < Cow< '_, Cell > as Borrow< Cell > >::borrow( &cell ) ),
        //       None => "".to_string(),
        //       // Some( cell ) => < Cow< '_, Cell > as Borrow< Cell > >::borrow( &cell ).as_ref(),
        //       // None => "",
        //     }
        //   }
        // )
        .collect();
        data.push( fields );
      }

      // for row in &data
      // {
      //   for ( i, cell ) in row.iter().enumerate()
      //   {
      //     if widths.len() <= i
      //     {
      //       widths.push( cell.data.len() );
      //     }
      //     else if cell.len() > widths[ i ]
      //     {
      //       widths[ i ] = cell.data.len();
      //     }
      //   }
      // }

//       // Write the header if provided
//       if let Some( header ) = self.header()
//       {
//         let mut i = 0;
//         for ( _key, title ) in header
//         {
//           if i > 0
//           {
//             write!( f.buf, "{}", separator )?;
//           }
//           write!( f.buf, "{:^width$}", format!( "{}", title ), width = widths[ i ] )?;
//           // write!( f.buf, "{:?}", title )?;
//           i += 1;
//         }
//         writeln!( f.buf )?;
//       }
//
//       // dbg!( &widths );
//
//       // Write rows with proper alignment
//       for row in &data
//       {
//         let mut i = 0;
//         for cell in row
//         {
//           if i > 0
//           {
//             write!( f.buf, "{}", separator )?;
//           }
//           write!( f.buf, "{:^width$}", cell, width = widths[ i ] )?;
//           i += 1;
//         }
//         writeln!( f.buf )?;
//       }

      // // Write rows with proper alignment
      // for row in data
      // {
      //   let formatted_row : Vec< String > = row
      //   .iter()
      //   .enumerate()
      //   .map( | ( i, cell ) | format!( "{:?^width$}", cell, width = widths[ i ] ) )
      //   .collect();
      //   writeln!( f.buf, "{}", formatted_row.join( separator ) )?;
      // }

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
