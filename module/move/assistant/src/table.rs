
use super::*;
use core::fmt;

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
  fn cells( &'a self ) -> impl IteratorTrait< Item = ( CellKey, Option< Cell > ) >
  ;
}

// ==

impl< 'a, T, RowKey, Row, CellKey, Cell, Title > TableSize< 'a >
for AsTable< 'a, T, RowKey, Row, CellKey, Cell, Title >
where
  Self : TableRows< 'a, RowKey, Row, CellKey, Cell >,
  Row : Clone + for< 'cell > Cells< 'cell, CellKey, Cell > + 'a,
  Title : fmt::Debug,
  Cell : fmt::Debug + Clone + 'a,
  CellKey : fmt::Debug + Clone,
{
  fn table_size( &'a self ) -> [ usize ; 2 ]
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

impl< 'a, T, RowKey, Row, CellKey, Cell, Title > TableRows< 'a, RowKey, Row, CellKey, Cell >
for AsTable< 'a, T, RowKey, Row, CellKey, Cell, Title >
where
  T : Fields< 'a, RowKey, Row >,
  Row : Clone + for< 'cell > Cells< 'cell, CellKey, Cell > + 'a,
  Title : fmt::Debug,
  Cell : fmt::Debug + Clone + 'a,
  CellKey : fmt::Debug + Clone,
{

  fn rows( &'a self ) -> impl IteratorTrait< Item = Row >
  {
    self.as_ref().fields().filter_map( move | ( _k, e ) |
    {
      match e
      {
        Some( e ) => Some( e.into_owned() ),
        None => None,
      }
    }).collect::< Vec< _ > >().into_iter()
  }

}

impl< 'a, T, RowKey, Row, CellKey, Cell > TableHeader< 'a, CellKey, CellKey >
for AsTable< 'a, T, RowKey, Row, CellKey, Cell, CellKey >
where
  Self : TableRows< 'a, RowKey, Row, CellKey, Cell >,
  Row : Clone + for< 'cell > Cells< 'cell, CellKey, Cell > + 'a,
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
  Row : Fields< 'a, CellKey, Cell > + 'a,
  Cell : fmt::Debug + Clone + 'a,
{

  fn cells( &'a self ) -> impl IteratorTrait< Item = ( CellKey, Option< Cell > ) >
  {
    self.fields().map
    (
      move | ( key, cell ) |
      {
        match cell
        {
          Some( cell ) => ( key, Some( cell.into_owned() ) ),
          None => ( key, None )
        }
      }
    )
  }

}

// ==