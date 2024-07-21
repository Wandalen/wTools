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
  pub trait Cells< CellKey, Cell, CellWrap, CellKind >
  where
    Cell : std::borrow::ToOwned + ?Sized,
    CellKind : Copy + 'static,
  {
    /// Returns an iterator over all cells of the row.
    fn cells< 'a >( &'a self ) -> impl IteratorTrait< Item = ( CellKey, MaybeAs< 'a, Cell, CellKind > ) >
    where
      Cell : 'a,
    ;
  }

  impl< CellKind, Row, CellKey, Cell, CellWrap > Cells< CellKey, Cell, CellWrap, CellKind >
  for Row
  where
    for< 'b > Row : Fields< CellKey, CellWrap >,
    Cell : std::borrow::ToOwned + ?Sized,
    CellKind : Copy + 'static,
    for< 'b > MaybeAs< 'b, Cell, CellKind > : From< < Row as Fields< CellKey, CellWrap > >::Value< 'b > >,
  {

    fn cells< 'a >( &'a self ) -> impl IteratorTrait< Item = ( CellKey, MaybeAs< 'a, Cell, CellKind > ) >
    where
      Cell : 'a,
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
  pub trait TableRows< RowKey, Row, CellKey, Cell, CellWrap, CellKind >
  where
    // 'table : 'row,
    // 'row : 'cell,
    Row : Clone + Cells< CellKey, Cell, CellWrap, CellKind >,
    Cell : std::borrow::ToOwned + ?Sized,
    CellKind : Copy + 'static,
    CellKey : fmt::Debug + Clone + std::cmp::Eq + std::hash::Hash,
  {
    /// Returns an iterator over all rows of the table.
    fn rows( &self ) -> impl IteratorTrait< Item = Row >;
  }

  impl< T, RowKey, Row, CellKey, Cell, CellWrap, CellKind, Title > TableRows< RowKey, Row, CellKey, Cell, CellWrap, CellKind >
  for AsTable< '_, T, RowKey, Row, CellKey, Cell, CellWrap, CellKind, Title >
  where
    // 'table : 'row,
    // 'row : 'cell,
    for< 'a > T : Fields< RowKey, CellWrap, Value< 'a > = Option< Cow< 'a, Row > > > + 'a,
    Row : Clone + Cells< CellKey, Cell, CellWrap, CellKind >,
    Title : fmt::Display,
    Cell : fmt::Display,
    Cell : std::borrow::ToOwned + ?Sized,
    CellKey : fmt::Debug + Clone + std::cmp::Eq + std::hash::Hash,
    CellKind : Copy + 'static,
  {

    fn rows( &self ) -> impl IteratorTrait< Item = Row >
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
  pub trait TableSize
  {
    /// Returns size of a table.
    fn table_size( &self ) -> [ usize ; 2 ]
    ;
  }

  impl< T, RowKey, Row, CellKey, Cell, CellWrap, CellKind, Title > TableSize
  for AsTable< '_, T, RowKey, Row, CellKey, Cell, CellWrap, CellKind, Title >
  where
    Self : TableRows< RowKey, Row, CellKey, Cell, CellWrap, CellKind >,
    Row : Clone + Cells< CellKey, Cell, CellWrap, CellKind >,
    Title : fmt::Display,
    Cell : fmt::Display,
    Cell : std::borrow::ToOwned + ?Sized,
    CellKey : fmt::Debug + Clone + std::cmp::Eq + std::hash::Hash,
    CellKind : Copy + 'static,
  {
    fn table_size( &self ) -> [ usize ; 2 ]
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
