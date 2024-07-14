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
    Row : Clone + for< 'cell > Cells< 'cell, CellKey, Cell, () > + 'a,
    Cell : fmt::Debug + 'a,
    Cell : std::borrow::ToOwned + ?Sized,
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
  pub trait Cells< 'a, CellKey, Cell, Kind >
  where
    Cell : fmt::Debug + 'a,
    // &'a Cell : Clone,
    Cell : std::borrow::ToOwned + ?Sized,
    Kind : Copy + 'static,
  {
    /// Returns an iterator over all cells of the row.
    fn cells( &'a self ) -> impl IteratorTrait< Item = ( CellKey, MaybeAs< 'a, Cell, Kind > ) >
    // fn cells( &'a self ) -> impl IteratorTrait< Item = ( CellKey, Option< Cell > ) >
    ;
  }

  // ==

  impl< 'a, T, RowKey, Row, CellKey, Cell, Title > TableSize< 'a >
  for AsTable< 'a, T, RowKey, Row, CellKey, Cell, Title >
  where
    Self : TableRows< 'a, RowKey, Row, CellKey, Cell >,
    Row : Clone + for< 'cell > Cells< 'cell, CellKey, Cell, () > + 'a,
    Title : fmt::Debug,
    Cell : fmt::Debug + 'a,
    Cell : std::borrow::ToOwned + ?Sized,
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
    T : Fields< 'a, RowKey, Option< Cow< 'a, Row > > >,
    Row : Clone + for< 'cell > Cells< 'cell, CellKey, Cell, () > + 'a,
    Title : fmt::Debug,
    Cell : fmt::Debug + 'a,
    Cell : std::borrow::ToOwned + ?Sized,
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

//   impl< 'a, T, RowKey, Row, CellKey, Cell > TableHeader< 'a, CellKey, CellKey >
//   for AsTable< 'a, T, RowKey, Row, CellKey, Cell, CellKey >
//   where
//     Self : TableRows< 'a, RowKey, Row, CellKey, Cell >,
//     Row : Clone + for< 'cell > Cells< 'cell, CellKey, Cell, () > + 'a,
//     CellKey : fmt::Debug + Clone,
//     Cell : fmt::Debug + 'a,
//     CellKey : fmt::Debug + Clone,
//   {
//
//     fn header( &'a self ) -> Option< impl IteratorTrait< Item = ( CellKey, CellKey ) > >
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

  impl< 'a, Kind, Row, CellKey, Cell > Cells< 'a, CellKey, Cell, Kind >
  for Row
  where
    Row : Fields< 'a, CellKey, MaybeAs< 'a, Cell, Kind > > + 'a,
    MaybeAs< 'a, Cell, Kind > : Clone,
    Cell : fmt::Debug + Clone + 'a,
    Kind : Copy + 'static,
  {

    fn cells( &'a self ) -> impl IteratorTrait< Item = ( CellKey, MaybeAs< 'a, Cell, Kind > ) >
    {
      self.fields().map
      (
        move | ( key, cell ) |
        {
          match cell.0
          {
            Some( cell ) => ( key, cell.into() ),
            None => ( key, MaybeAs::none() )
          }
        }
      )
    }

  }

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
    TableSize,
    TableRows,
    TableHeader,
    Cells,
  };

}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;
}
