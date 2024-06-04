
use super::*;
use core::fmt;
use former::Former;

/// Struct to hold options to print data as table.
#[ derive( Debug, Default, Former ) ]
pub struct Styles
{
  /// Optional delimiter for separating table columns.
  pub delimiter : Option< String >,
}

/// Struct to hold options to print data as table.
#[ derive( Debug, Default, Former ) ]
pub struct TableOptions
{
  /// Optional header row for the table.
  pub header : Option< Vec< String > >,
  /// Rows with data for the table.
  /// Its length should be equal to header length.
  pub rows : Vec< String >,
  /// Styles.
  pub styles : Styles,
}

/// A trait for iterating over all fields of a specified type within an entity to print it.
pub trait TableRows< Row, Cell >
where
  Row : RowFields< Cell >,
  Cell : fmt::Debug,
{
  /// Returns an iterator over all fields of the specified type within the entity.
  fn rows( &self ) -> impl Iterator< Item = Row > + Clone;
}

/// A trait for iterating over all fields of a specified type within an entity to print it.
pub trait RowFields< Cell >
where
  Cell : fmt::Debug,
{
  /// Returns an iterator over all fields of the specified type within the entity.
  fn row_fields( &self ) -> impl Iterator< Item = Cell > + Clone;
}

/// Trait returning headers of a table if any.
pub trait TableHeader< Title >
where
  Title : fmt::Debug,
{
  /// Returns an iterator over all fields of the specified type within the entity.
  fn header( &self ) -> Option< impl Iterator< Item = Title > + Clone >;
}

// impl TableHeader for I
// where
//   I : Fields< K, E >,
// {
// }

/// Struct for formatting tables.
pub struct Formatter< 'a >
{
  buf : &'a mut dyn fmt::Write,
  styles : Styles,
}

impl< 'a > Formatter< 'a >
{
  /// Creates a new `Formatter` with the given buffer and delimiter.
  pub fn new( buf : &'a mut dyn fmt::Write, styles : Styles ) -> Self
  {
    Formatter { buf, styles }
  }
}

/// A trait for formatting tables.
pub trait TableFormatter
{
  fn fmt< I, F, K, E >( &self, f : &mut Formatter<'_>, iter : I ) -> fmt::Result
  where
    I : Iterator< Item = F >,
    F : Fields< K, E >,
    K : fmt::Debug,
    E : fmt::Debug;
}

impl TableFormatter for TableOptions
{
  fn fmt< I, F, K, E >( &self, f : &mut Formatter<'_>, iter : I ) -> fmt::Result
  where
    I : Iterator< Item = F >,
    F : Fields< K, E >,
    K : fmt::Debug,
    E : fmt::Debug,
  {
    let delimiter = &self.styles.delimiter.clone().unwrap_or_else( || ",".to_string() );

    // Write the header if provided
    if let Some( header ) = &self.header
    {
      writeln!( f.buf, "{}", header.join( delimiter ) )?;
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

    // Write rows with proper alignment
    for row in all_rows
    {
      let formatted_row : Vec< String > = row
        .iter()
        .enumerate()
        .map( | ( i, col ) | format!( "{:width$}", col, width = col_widths[ i ] ) )
        .collect();
      writeln!( f.buf, "{}", formatted_row.join( delimiter ) )?;
    }

    Ok(())
  }
}