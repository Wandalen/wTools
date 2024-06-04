
use super::*;
use core::fmt;
use former::Former;

// ==

// pub type Iterator2< Item > = Iterator< Item = Item > + ExactSizeIterator + Clone;

/// A trait for iterating over all rows of a table.
pub trait TableSize
{
  /// Returns size of a table.
  fn size( &self ) -> [ usize ; 2 ];
}

/// A trait for iterating over all rows of a table.
pub trait TableRows< Row, Cell >
where
  Row : Cells< Cell >,
  Cell : fmt::Debug,
{
  /// Returns an iterator over all rows of the table.
  fn rows( &self ) -> impl Iterator< Item = Row > + ExactSizeIterator + Clone;
}

/// A trait for iterating over all cells of a row.
pub trait Cells< Cell >
where
  Cell : fmt::Debug,
{
  /// Returns an iterator over all cells of the row.
  fn cells( &self ) -> impl Iterator< Item = Cell > + ExactSizeIterator + Clone;
}

/// Trait returning headers of a table if any.
pub trait TableHeader< Title >
where
  Title : fmt::Debug,
{
  /// Returns an iterator over all fields of the specified type within the entity.
  fn header( &self ) -> Option< impl Iterator< Item = Title > + ExactSizeIterator + Clone >;
}

// ==

/// Struct to hold options to print data as table.
#[ derive( Debug, Default, Former ) ]
pub struct Styles
{
  /// Delimiter for separating table columns.
  pub separator : String,
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

/// Struct for formatting tables.
pub struct Formatter< 'a >
{
  buf : &'a mut dyn fmt::Write,
  styles : Styles,
}

impl< 'a > Formatter< 'a >
{
  /// Creates a new `Formatter` with the given buffer and separator.
  pub fn new( buf : &'a mut dyn fmt::Write, styles : Styles ) -> Self
  {
    Formatter { buf, styles }
  }
}

/// A trait for formatting tables.
pub trait TableFormatter< Row, Cell, Title >
where
  Self : TableRows< Row, Cell >,
  Self : TableHeader< Title >,
  Self : TableSize,
  Row : Cells< Cell >,
  Title : fmt::Debug,
  Cell : fmt::Debug,
{
  fn fmt( &self, f : &mut Formatter< '_ > ) -> fmt::Result
  {
    let size = self.size();
    let mut col_widths : Vec< usize > = vec![ 0 ; size[ 1 ] ];
    let separator = &f.styles.separator;

    // Write the header if provided
    if let Some( header ) = self.header()
    {
      let mut first = true;
      let mut i = 0;
      for title in header
      {
        if !first
        {
          write!( f.buf, "{}", separator )?;
        }
        col_widths[ i ] = format!( "{:?}", title ).len();
        // zzz : avoid extra allocation of memory
        write!( f.buf, "{:?}", title )?;
        first = false;
      }
      writeln!( f.buf )?;
    }

    // Collect rows
    let mut all_rows : Vec< Vec< String > > = Vec::new();
    for row in self.rows()
    {
      let fields : Vec< String > = row
      .cells()
      // .map( | ( key, value ) | format!( "{:?}: {:?}", key, value ) )
      .map( | e | format!( "{:?}", e ) )
      .collect();
      all_rows.push( fields );
    }

    // // Find the maximum width for each column
    // let mut col_widths : Vec< usize > = Vec::new();
    // if let Some( header ) = self.header()
    // {
    //   for col in header
    //   {
    //     col_widths.push( col.len() );
    //   }
    // }

    for row in &all_rows
    {
      for ( i, cell ) in row.iter().enumerate()
      {
        if col_widths.len() <= i
        {
          col_widths.push( cell.len() );
        }
        else if cell.len() > col_widths[ i ]
        {
          col_widths[ i ] = cell.len();
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
      writeln!( f.buf, "{}", formatted_row.join( separator ) )?;
    }

    Ok(())
  }
}