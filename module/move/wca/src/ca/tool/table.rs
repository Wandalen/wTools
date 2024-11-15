mod private
{
  use crate::*;

  // use wtools::error::{ Result, err };
  // use error::err;

  /// Represents a table composed of multiple rows.
  ///
  /// The `Table` struct is a simple container that holds multiple `Row` objects.
  #[ derive( Debug ) ]
  pub struct Table( Vec< Row > );

  impl< T, R > From< T > for Table
  where
    T : IntoIterator< Item = R >,
    R : Into< Row >,
  {
    fn from( value : T ) -> Self
    {
      Self( value.into_iter().map( Into::into ).collect() )
    }
  }

  impl Table
  {
    /// Validates the structure of the given `self` object.
    ///
    /// It checks if all the rows have the same length as the first row of the object.
    /// If all the rows have the same length, it returns `true`, otherwise it returns `false`.
    ///
    /// # Returns
    ///
    /// - `true` if all the rows have the same length
    /// - `false` if at least one row has a different length
    pub fn validate( &self ) -> bool
    {
      let mut row_iter = self.0.iter();
      let Some( first_row ) = row_iter.next() else { return true };
      let first_row_length = first_row.0.len();
      for row in row_iter
      {
        if row.0.len() != first_row_length
        {
          return false;
        }
      }

      true
    }
  }

  /// Represents a row in a table.
  ///
  /// The `Row` struct is a container that holds multiple `String` objects representing the values in a table row.
  #[ derive( Debug ) ]
  pub struct Row( Vec< String > );

  impl< R, V > From< R > for Row
  where
    R : IntoIterator< Item = V >,
    V : Into< String >,
  {
    fn from( value : R ) -> Self
    {
      Self( value.into_iter().map( Into::into ).collect() )
    }
  }

  fn max_column_lengths( table : &Table ) -> Vec< usize >
  {
    let num_columns = table.0.get( 0 ).map_or( 0, | row | row.0.len() );
    ( 0 .. num_columns )
    .map( | column_index |
    {
      table.0.iter()
      .map( | row | row.0[ column_index ].len() )
      .max()
      .unwrap_or( 0 )
    })
    .collect()
  }

  #[ derive( Debug, error::typed::Error ) ]
  #[ error( "Invalid table" ) ]
  pub struct FormatTableError;

  /// Formats a table into a readable string representation.
  ///
  /// # Arguments
  ///
  /// * `table` - The table to be formatted.
  ///
  /// # Returns
  ///
  /// * `error::untyped::Result<String, Error>` - A `error::untyped::Result` containing the formatted table as a `String`, or an `Error` if the table is invalid.
  // aaa : use typed error
  // aaa : done
  pub fn format_table< IntoTable >( table : IntoTable ) -> Result< String, FormatTableError >
  where
    IntoTable : Into< Table >,
  {
    let table = table.into();
    if !table.validate()
    {
      return Err( FormatTableError );
    }

    let max_lengths = max_column_lengths( &table );

    let mut formatted_table = String::new();
    for row in table.0
    {
      for ( i, cell ) in row.0.iter().enumerate()
      {
        formatted_table.push_str( &format!( "{:width$}", cell, width = max_lengths[ i ] ) );
        formatted_table.push( ' ' );
      }
      formatted_table.pop(); // trailing space
      formatted_table.push( '\n' );
    }
    formatted_table.pop(); // trailing end of line

    Ok( formatted_table )
  }
}

//

crate::mod_interface!
{
  own use format_table;
}
