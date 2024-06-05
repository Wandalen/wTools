
use super::*;
use core::fmt;
// use std::borrow::Cow;
use former::Former;

// ==

/// A trait for iterating over all rows of a table.
pub trait TableSize< 'a >
{
  /// Returns size of a table.
  fn table_size( &'a self ) -> [ usize ; 2 ];
}

/// A trait for iterating over all rows of a table.
pub trait TableRows< 'a, RowKey, Row, CellKey, Cell >
where
  Row : Clone + for< 'cell > Cells< 'cell, CellKey, Cell > + 'a,
  Cell : fmt::Debug + Clone + 'a,
{
  /// Returns an iterator over all rows of the table.
  fn rows( &'a self ) -> impl IteratorTrait< Item = Row >;
}

/// Trait returning headers of a table if any.
pub trait TableHeader< 'a, CellKey, Title >
where
  Title : fmt::Debug,
  // Title : 'a,
  // Self : 'a,
{
  /// Returns an iterator over all fields of the specified type within the entity.
  fn header( &'a self ) -> Option< impl IteratorTrait< Item = ( CellKey, Title ) > >;
}

/// A trait for iterating over all cells of a row.
pub trait Cells< 'a, CellKey, Cell >
where
  Cell : fmt::Debug + Clone + 'a,
{
  /// Returns an iterator over all cells of the row.
  fn cells( &'a self ) -> impl IteratorTrait< Item = ( CellKey, Cell ) >
  // where
  //   Self : 'a,
  //   Cell : 'a,
  //   CellKey : 'static,
  ;
}

// ==

impl< 'a, T, RowKey, Row, CellKey, Cell, Title > TableSize< 'a >
for AsTable< 'a, T, RowKey, Row, CellKey, Cell, Title >
where
  Self : TableRows< 'a, RowKey, Row, CellKey, Cell >,
  // T : TableHeader< 'a, CellKey, Title >,
  // T : TableSize< 'a >,
  Row : Clone + for< 'cell > Cells< 'cell, CellKey, Cell > + 'a,
  Title : fmt::Debug,
  Cell : fmt::Debug + Clone + 'a,
  CellKey : fmt::Debug + Clone,
{
  fn table_size( &'a self ) -> [ usize ; 2 ]
  {
    // [ 0, 0 ]
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

impl< 'a, T, RowKey, Row, CellKey, Cell, Title > TableRows< 'a, RowKey, Row, CellKey, Cell >
for AsTable< 'a, T, RowKey, Row, CellKey, Cell, Title >
where
  // T : TableRows< 'a, RowKey, Row, CellKey, Cell >,
  T : Fields< 'a, RowKey, Row >,
  // T : TableHeader< 'a, CellKey, Title >,
  // T : TableSize< 'a >,
  Row : Clone + for< 'cell > Cells< 'cell, CellKey, Cell > + 'a,
  Title : fmt::Debug,
  Cell : fmt::Debug + Clone + 'a,
  CellKey : fmt::Debug + Clone,
{

  fn rows( &'a self ) -> impl IteratorTrait< Item = Row >
  {
    self.as_ref().fields().map( move | ( _k, e ) | e.into_owned() )
  }

}

impl< 'a, T, RowKey, Row, CellKey, Cell > TableHeader< 'a, CellKey, CellKey >
for AsTable< 'a, T, RowKey, Row, CellKey, Cell, CellKey >
where
  Self : TableRows< 'a, RowKey, Row, CellKey, Cell >,
  // T : TableHeader< 'a, CellKey, Title >,
  // T : TableSize< 'a >,
  Row : Clone + for< 'cell > Cells< 'cell, CellKey, Cell > + 'a,
  // Row : for< 'cell > Fields< 'cell, CellKey, Cell > + 'a,
  // CellKey : Clone,
  CellKey : fmt::Debug + Clone,
  Cell : fmt::Debug + Clone + 'a,
  CellKey : fmt::Debug + Clone,
{

  fn header( &'a self ) -> Option< impl IteratorTrait< Item = ( CellKey, CellKey ) > >
  {
    let mut rows = self.rows();
    let row = rows.next();
    if let Some( row ) = row
    {
      Some
      (
        row
        .cells()
        .map( | ( key, _title ) | ( key.clone(), key ) )
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

impl< 'a, Row, CellKey, Cell > Cells< 'a, CellKey, Cell >
for Row
where
  // Row : Clone + for< 'cell > Cells< 'cell, CellKey, Cell > + 'a,
  Row : Fields< 'a, CellKey, Cell > + 'a,
  Cell : fmt::Debug + Clone + 'a,
{

  fn cells( &'a self ) -> impl IteratorTrait< Item = ( CellKey, Cell ) >
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

impl< 'a > Formatter< 'a >
{
  /// Just constructr.
  pub fn new( buf : &'a mut dyn fmt::Write, styles : Styles ) -> Self
  {
    Self { buf, styles }
  }
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
impl< 'a, T, RowKey, Row, CellKey, Cell, Title > TableFormatter< 'a >
for AsTable< 'a, T, RowKey, Row, CellKey, Cell, Title >
where
  Self : TableRows< 'a, RowKey, Row, CellKey, Cell >,
  Self : TableHeader< 'a, CellKey, Title >,
  Self : TableSize< 'a >,
  Row : Clone + for< 'cell > Cells< 'cell, CellKey, Cell > + 'a,
  Title : fmt::Debug,
  Cell : fmt::Debug + Clone + 'a,
  CellKey : fmt::Debug + Clone,
{
  fn fmt( &'a self, f : &'a mut Formatter< '_ > ) -> fmt::Result
  {
    let table_size = self.table_size();
    let mut col_widths : Vec< usize > = vec![ 0 ; table_size[ 1 ] ];
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
