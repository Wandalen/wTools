
use super::*;
use core::fmt;
use former::Former;

// ==

/// A trait for iterating over all rows of a table.
pub trait TableSize
{
  /// Returns size of a table.
  fn size( &self ) -> [ usize ; 2 ];
}

/// A trait for iterating over all rows of a table.
pub trait TableRows< Row, Key, Cell >
where
  Row : Cells< Key, Cell >,
  Cell : fmt::Debug,
{
  /// Returns an iterator over all rows of the table.
  fn rows( &self ) -> impl IteratorTrait< Item = Row >;
}

/// Trait returning headers of a table if any.
pub trait TableHeader< Key, Title >
where
  Title : fmt::Debug,
{
  /// Returns an iterator over all fields of the specified type within the entity.
  fn header( &self ) -> Option< impl IteratorTrait< Item = ( Key, Title ) > >;
}

/// A trait for iterating over all cells of a row.
pub trait Cells< Key, Cell >
where
  Cell : fmt::Debug,
{
  /// Returns an iterator over all cells of the row.
  fn cells( &self ) -> impl IteratorTrait< Item = ( Key, Cell ) >;
}

// ==

impl< T, Row, Key, Cell, Title > TableSize
for AsTable< T, Row, Key, Cell, Title >
where
  T : TableRows< Row, Key, Cell >,
  T : TableHeader< Key, Title >,
  T : TableSize,
  Row : Cells< Key, Cell >,
  Title : fmt::Debug,
  Cell : fmt::Debug,
{
  fn size( &self ) -> [ usize ; 2 ]
  {
    let mut rows = self.rows();
    let nrows = rows.len();
    let row = rows.next();
    if let Some( row ) = row
    {
      let ncells = row.cells().len();
      [ nrows, ncells ]
    }
    else
    {
      [ 0, 0 ]
    }
  }
}

impl< T, Row, Key, Cell, Title > TableRows< Row, Key, Cell >
for AsTable< T, Row, Key, Cell, Title >
where
  T : TableRows< Row, Key, Cell >,
  T : TableHeader< Key, Title >,
  T : TableSize,
  T : Fields< Key, Row >,
  Row : Cells< Key, Cell >,
  Title : fmt::Debug,
  Cell : fmt::Debug,
{

  fn rows( &self ) -> impl IteratorTrait< Item = Row >
  {
    self.fields().map( | ( _k, e ) | e )
  }

}

impl< T, Row, Key, Cell, Title > TableHeader< Key, Title >
for AsTable< T, Row, Key, Cell, Title >
where
  T : TableRows< Row, Key, Cell >,
  T : TableHeader< Key, Title >,
  T : TableSize,
  Row : Cells< Key, Cell >,
  Row : Fields< Key, Title >,
  Key : Clone,
  Title : fmt::Debug + Clone,
  Cell : fmt::Debug,
{

  fn header( &self ) -> Option< impl IteratorTrait< Item = ( Key, Title ) > >
  {
    let mut rows = self.rows();
    let row = rows.next();
    if let Some( row ) = row
    {
      Some( row.fields().collect::< Vec< _ > >().into_iter() )
    }
    else
    {
      None
    }
  }

}

// /// A trait for iterating over all cells of a row.
// pub trait Cells2< Key, Cell >
// where
//   Cell : fmt::Debug,
// {
//   /// Returns an iterator over all cells of the row.
//   fn cells( &self ) -> impl IteratorTrait< Item = ( Key, Cell ) >;
// }

impl< Row, Key, Cell > Cells< Key, Cell >
for Row
where
  // Row : Cells< Key, Cell >,
  Row : Fields< Key, Cell >,
  Cell : fmt::Debug,
{

  fn cells( &self ) -> impl IteratorTrait< Item = ( Key, Cell ) >
  {
    self.fields()
  }

}

// ==

/// Struct to hold options to print data as table.
#[ derive( Debug, Default, Former ) ]
pub struct Styles
{
  /// Delimiter for separating table columns.
  pub separator : String,
}

/// Struct for formatting tables.
pub struct Formatter< 'a >
{
  buf : &'a mut dyn fmt::Write,
  styles : Styles,
}

impl fmt::Debug for Formatter< '_ >
{
  fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
  {
    f
    .debug_struct( "Formatter" )
    .field( "buf", &"dyn fmt::Write" )
    .field( "styles", &self.styles )
    .finish()
  }
}

impl< 'a > Formatter< 'a >
{
  /// Creates a new `Formatter` with the given buffer and separator.
  pub fn new( buf : &'a mut dyn fmt::Write, styles : Styles ) -> Self
  {
    Formatter { buf, styles }
  }
}

/// A trait for converting tables to a string representation.
pub trait TableToString
{
  /// Converts the table to a string representation.
  ///
  /// # Returns
  ///
  /// A `String` containing the formatted table.
  fn table_to_string( &self ) -> String;
}

impl< T > TableToString for T
where
  T : TableFormatter
{
  fn table_to_string( &self ) -> String
  {
    let mut output = String::new();
    let mut formatter = Formatter
    {
      buf : &mut output,
      styles : Styles::default(),
    };
    T::fmt( self, &mut formatter ).expect( "Formatting failed" );
    output
  }
}

/// A trait for formatting tables.
///
/// This trait defines a method for formatting tables, allowing implementations
/// to specify how a table should be formatted and displayed.
///

pub trait TableFormatter
{
  /// Formats the table and writes the result to the given formatter.
  fn fmt( &self, f : &mut Formatter< '_ > ) -> fmt::Result;
}

/// A trait for formatting tables.
impl< T, Row, Key, Cell, Title > TableFormatter for AsTable< T, Row, Key, Cell, Title >
where
  T : TableRows< Row, Key, Cell >,
  T : TableHeader< Key, Title >,
  T : TableSize,
  Row : Cells< Key, Cell >,
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
      for ( _key, title ) in header
      {
        if !first
        {
          write!( f.buf, "{}", separator )?;
        }
        col_widths[ i ] = format!( "{:?}", title ).len();
        // zzz : avoid extra allocation of memory
        write!( f.buf, "{:?}", title )?;
        first = false;
        i += 1;
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
      .map( | ( _key, cell ) | format!( "{:?}", cell ) )
      .collect();
      all_rows.push( fields );
    }

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
      .map( | ( i, cell ) | format!( "{:width$}", cell, width = col_widths[ i ] ) )
      .collect();
      writeln!( f.buf, "{}", formatted_row.join( separator ) )?;
    }

    Ok(())
  }
}
