
use super::*;
use core::fmt;
use std::borrow::Cow;
use former::Former;

// ==

/// A trait for iterating over all rows of a table.
pub trait TableSize
{
  /// Returns size of a table.
  fn size( &self ) -> [ usize ; 2 ];
}

/// A trait for iterating over all rows of a table.
pub trait TableRows< 'a, Row, Key, Cell >
where
  Row : Clone + Cells< 'a, Key, Cell >,
  Cell : fmt::Debug + Clone + 'static,
{
  /// Returns an iterator over all rows of the table.
  fn rows( &'a self ) -> impl IteratorTrait< Item = Row >;
}

/// Trait returning headers of a table if any.
pub trait TableHeader< 'a, Key, Title >
where
  Title : fmt::Debug,
  // Title : 'a,
  // Self : 'a,
{
  /// Returns an iterator over all fields of the specified type within the entity.
  fn header( &'a self ) -> Option< impl IteratorTrait< Item = ( Key, Title ) > >;
}

/// A trait for iterating over all cells of a row.
pub trait Cells< 'a, Key, Cell >
where
  Cell : fmt::Debug + Clone + 'static,
{
  /// Returns an iterator over all cells of the row.
  fn cells( &'a self ) -> impl IteratorTrait< Item = ( Key, Cell ) >
  // where
  //   Self : 'a,
  //   Cell : 'a,
  //   Key : 'static,
  ;
}

// ==

// impl< 'a, T, Row, Key, Cell, Title > TableSize
// for AsTable< 'a, T, Row, Key, Cell, Title >
// where
//   T : TableRows< 'a, Row, Key, Cell >,
//   T : TableHeader< 'a, Key, Title >,
//   T : TableSize,
//   Row : Clone + Cells< 'a, Key, Cell >,
//   Title : fmt::Debug,
//   Cell : fmt::Debug + Clone,
// {
//   fn size( &self ) -> [ usize ; 2 ]
//   {
//     let mut rows = self.rows();
//     let nrows = rows.len();
//     let row = rows.next();
//     if let Some( row ) = row
//     {
//       let ncells = row.cells().len();
//       [ nrows, ncells ]
//     }
//     else
//     {
//       [ 0, 0 ]
//     }
//   }
// }
//
// impl< 'a, T, Row, Key, Cell, Title > TableRows< 'a, Row, Key, Cell >
// for AsTable< 'a, T, Row, Key, Cell, Title >
// where
//   Self : 'a,
//   // Self : 'static,
//   T : TableRows< 'a, Row, Key, Cell >,
//   T : TableHeader< 'a, Key, Title >,
//   T : TableSize,
//   T : Fields< 'a, Key, Row >,
//   Row : Clone + Cells< 'a, Key, Cell >,
//   Row : 'static,
//   Title : fmt::Debug,
//   Cell : fmt::Debug + Clone,
// {
//
//   fn rows( &self ) -> impl IteratorTrait< Item = Row >
//   where
//     // Self : 'a,
//     // Self : 'static,
//   {
//     self.fields().map( move | ( _k, e ) | e.into_owned() )
//   }
//
// }

impl< 'a, T, Row, Key, Cell, Title > TableHeader< 'a, Key, Title >
for AsTable< 'a, T, Row, Key, Cell, Title >
where
  T : TableRows< 'a, Row, Key, Cell >,
  T : TableHeader< 'a, Key, Title >,
  T : TableSize,
  Row : Clone + for< 'cell > Cells< 'cell, Key, Cell >,
  Row : for< 'cell > Fields< 'cell, Key, Title >,
  Key : Clone + 'static,
  Title : fmt::Debug + Clone + 'static,
  Cell : fmt::Debug + Clone + 'static,
{

  fn header( &'a self ) -> Option< impl IteratorTrait< Item = ( Key, Title ) > >
  {
    let mut rows = self.rows();
    let row = rows.next();
    if let Some( row ) = row
    {
      Some
      (
        row
        .fields()
        .map( | ( key, title ) | ( key, title.into_owned() ) )
        .collect::< Vec< _ > >()
        .into_iter()
      )
    }
    else
    {
      None
    }
  }

}

impl< 'a, Row, Key, Cell > Cells< 'a, Key, Cell >
for Row
where
  Row : Fields< 'a, Key, Cell >,
  Cell : fmt::Debug + Clone + 'static,
{

  fn cells( &'a self ) -> impl IteratorTrait< Item = ( Key, Cell ) >
  {
    self.fields().map( move | ( key, cell ) | ( key, cell.into_owned() ) )
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
  T : for< 'b > TableFormatter< 'b >
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

pub trait TableFormatter< 'b >
{
  /// Formats the table and writes the result to the given formatter.
  fn fmt( &'b self, f : &'b mut Formatter< '_ > ) -> fmt::Result;
}

/// A trait for formatting tables.
impl< 'a, T, Row, Key, Cell, Title > TableFormatter< 'a >
for AsTable< 'a, T, Row, Key, Cell, Title >
where
  T : TableRows< 'a, Row, Key, Cell >,
  T : TableHeader< 'a, Key, Title >,
  T : TableSize,
  Row : Clone + for< 'cell > Cells< 'cell, Key, Cell >,
  Title : fmt::Debug,
  Cell : fmt::Debug + Clone + 'static,
  // 'b : 'a,
{
  fn fmt( &'a self, f : &'a mut Formatter< '_ > ) -> fmt::Result
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
      .map( | ( _key, cell ) | format!( "{:?}", &cell ) )
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
