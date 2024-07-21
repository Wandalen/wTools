//!
//! Table interface.
//!

/// Internal namespace.
pub( crate ) mod private
{

  use crate::*;
  use core::fmt;
  use std::borrow::Cow;
  use reflect_tools::
  {
    IteratorTrait,
    Fields,
  };

  // =

  /// A trait for iterating over all cells of a row.
  pub trait Cells< 'row, 'cell, CellKey, Cell, CellKind >
  where
    'row : 'cell,
    Cell : std::borrow::ToOwned + ?Sized + 'cell,
    CellKind : Copy + 'static,
  {
    /// Returns an iterator over all cells of the row.
    fn cells( &'row self ) -> impl IteratorTrait< Item = ( CellKey, MaybeAs< 'cell, Cell, CellKind > ) >
    ;
  }

  impl< 'row, 'cell, CellKind, Row, CellKey, Cell > Cells< 'row, 'cell, CellKey, Cell, CellKind >
  for Row
  where
    'row : 'cell,
    Row : Fields< 'cell, CellKey, MaybeAs< 'cell, Cell, CellKind > >,
    Cell : std::borrow::ToOwned + ?Sized + 'cell,
    CellKind : Copy + 'static,
  {

    fn cells( &'row self ) -> impl IteratorTrait< Item = ( CellKey, MaybeAs< 'cell, Cell, CellKind > ) >
    {
      self.fields().map
      (
        move | ( key, cell ) |
        {
          ( key, cell.into() )
        }
      )
    }

  }

  // =

  /// A trait for iterating over all rows of a table.
  pub trait TableRows< 'table, 'row, 'cell, RowKey, Row, CellKey, Cell, CellKind >
  where
    'table : 'row,
    'row : 'cell,
    Row : Clone + Cells< 'row, 'cell, CellKey, Cell, CellKind >,
    Cell : std::borrow::ToOwned + ?Sized + 'cell + 'row,
    CellKind : Copy + 'static,
    CellKey : fmt::Debug + Clone + std::cmp::Eq + std::hash::Hash,
  {
    /// Returns an iterator over all rows of the table.
    fn rows( &'table self ) -> impl IteratorTrait< Item = Row >;
  }

  impl< 'table, 'row, 'cell, T, RowKey, Row, CellKey, Cell, CellKind, Title > TableRows< 'table, 'row, 'cell, RowKey, Row, CellKey, Cell, CellKind >
  for AsTable< 'table, 'row, 'cell, T, RowKey, Row, CellKey, Cell, CellKind, Title >
  where
    'table : 'row,
    'row : 'cell,
    T : Fields< 'row, RowKey, Option< Cow< 'row, Row > > >,
    Row : Clone + Cells< 'row, 'cell, CellKey, Cell, CellKind > + 'row,
    Title : fmt::Display,
    Cell : fmt::Display,
    Cell : std::borrow::ToOwned + ?Sized + 'cell + 'row,
    CellKey : fmt::Debug + Clone + std::cmp::Eq + std::hash::Hash,
    CellKind : Copy + 'static,
  {

    fn rows( &'table self ) -> impl IteratorTrait< Item = Row >
    {
      self.as_ref().fields()
      .filter_map( move | ( _k, e ) |
      {
        match e
        {
          Some( e ) => Some( e.into_owned() ),
          None => None,
        }
      })
      .collect::< Vec< _ > >().into_iter()
    }

  }

  // =

  /// A trait for iterating over all rows of a table.
  pub trait TableSize< 'table, 'row, 'cell >
  where
    'table : 'row,
    'row : 'cell,
  {
    /// Returns size of a table.
    fn table_size( &'table self ) -> [ usize ; 2 ]
    ;
  }

  impl< 'table, 'row, 'cell, T, RowKey, Row, CellKey, Cell, CellKind, Title > TableSize< 'table, 'row, 'cell >
  for AsTable< 'table, 'row, 'cell, T, RowKey, Row, CellKey, Cell, CellKind, Title >
  where
    'table : 'row,
    'row : 'cell,
    Self : TableRows< 'table, 'row, 'cell, RowKey, Row, CellKey, Cell, CellKind >,
    Row : Clone + Cells< 'row, 'cell, CellKey, Cell, CellKind > + 'row,
    Title : fmt::Display,
    Cell : fmt::Display,
    Cell : std::borrow::ToOwned + ?Sized + 'cell + 'row,
    CellKey : fmt::Debug + Clone + std::cmp::Eq + std::hash::Hash,
    CellKind : Copy + 'static,
  {
    fn table_size( &'table self ) -> [ usize ; 2 ]
    {
      let mut rows = self.rows();
      let nrows = rows.len();
      let row = rows.clone().next();
      if let Some( row2 ) = row
      {
        let cit = row2.cells().clone();
        // let ncells = cit.len();
        // [ nrows, ncells ]
        [ 0, 0 ]
      }
      else
      {
        [ 0, 0 ]
      }
    }
  }

//   // =
//
//   /// Trait returning headers of a table if any.
//   pub trait TableHeader< 'table, CellKey, Title >
//   where
//     // Title : fmt::Debug,
//   {
//     /// Returns an iterator over all fields of the specified type within the entity.
//     fn header( &'table self ) -> Option< impl IteratorTrait< Item = ( CellKey, Title ) > >;
//   }
//
//   impl< 'table, T, RowKey, Row, CellKey, Cell, CellKind, Title > TableHeader< 'table, CellKey, CellKey >
//   for AsTable< 'table, T, RowKey, Row, CellKey, Cell, CellKind, Title >
//   where
//     Self : TableRows< 'table, RowKey, Row, CellKey, Cell, CellKind >,
//     Row : Clone + Cells< 'table, CellKey, Cell, CellKind > + 'table,
//     // CellKey : Clone,
//     CellKey : fmt::Debug + Clone + std::cmp::Eq + std::hash::Hash,
//     Title : fmt::Display,
//     Cell : fmt::Display,
//     // Cell : fmt::Debug + 'table,
//     Cell : std::borrow::ToOwned + ?Sized,
//     CellKind : Copy + 'static,
//   {
//
//     fn header( &'table self ) -> Option< impl IteratorTrait< Item = ( CellKey, CellKey ) > >
//     {
//       let mut rows = self.rows();
//       let row = rows.next();
//       if let Some( row ) = row
//       {
//         Some
//         (
//           row
//           .cells()
//           .map( | ( key, _title ) | ( key.clone(), key ) )
//           .collect::< Vec< _ > >()
//           .into_iter()
//         )
//       }
//       else
//       {
//         None
//       }
//     }
//
//   }

  // =

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
    Cells,
    TableRows,
    TableSize,
    // TableHeader,
  };

}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;
}
