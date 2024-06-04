
use super::*;
use core::fmt;
use former::Former;

/// Struct to hold options to print data as table.
#[ derive( Debug, Default, Former ) ]
pub struct TableOptions
{
  /// Optional header row for the table.
  pub header : Option< Vec< String > >,
  /// Rows with data for the table.
  /// Its length should be equal to header length.
  pub rows : Vec< String >,
  /// Optional delimiter for separating table columns.
  pub delimiter : Option< String >,
}

impl TableOptions
{
  /// Function to print a table based on the iterator of items implementing `Fields` trait.
  pub fn perform< I, F, K, E >( &self, iter : I )
  where
    I : Iterator< Item = F >,
    F : Fields< K, E >,
    K : fmt::Debug,
    E : fmt::Debug,
  {
    let delimiter = self.delimiter.clone().unwrap_or_else( || ",".to_string() );

    // Print the header if provided
    if let Some( header ) = &self.header
    {
      println!( "{}", header.join( &delimiter ) );
    }

    // Collect rows
    let mut all_rows : Vec< Vec< String > > = Vec::new();
    for item in iter
    {
      let fields : Vec< String > = item
        .fields()
        .map( | ( key, value ) | format!( "{:?}: {:?}", key, value ) )
        .collect();
      all_rows.push( fields );
    }

    // Find the maximum width for each column
    let mut col_widths : Vec< usize > = Vec::new();
    if let Some( header ) = &self.header
    {
      for col in header
      {
        col_widths.push( col.len() );
      }
    }

    for row in &all_rows
    {
      for ( i, col ) in row.iter().enumerate()
      {
        if col_widths.len() <= i
        {
          col_widths.push( col.len() );
        }
        else if col.len() > col_widths[ i ]
        {
          col_widths[ i ] = col.len();
        }
      }
    }

    // Print rows with proper alignment
    for row in all_rows
    {
      let formatted_row : Vec< String > = row
        .iter()
        .enumerate()
        .map( | ( i, col ) | format!( "{:width$}", col, width = col_widths[ i ] ) )
        .collect();
      println!( "{}", formatted_row.join( &delimiter ) );
    }
  }
}
