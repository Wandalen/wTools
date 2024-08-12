//!
//! Table interface.
//!

/// Internal namespace.
pub( crate ) mod private
{

  use crate::*;
  use core::
  {
    fmt,
    // borrow::Borrow,
  };
  use std::borrow::Cow;
  use reflect_tools::
  {
    IteratorTrait,
    Fields,
  };

  // =

  /// A trait for iterating over all cells of a row.
  pub trait Cells< CellKey, CellFormat >
  where
    // Cell : std::borrow::ToOwned + ?Sized,
    CellFormat : Copy + 'static,
  {
    /// Returns an iterator over all cells of the row.
    // fn cells( &self ) -> impl IteratorTrait< Item = ( CellKey, MaybeAs< '_, str, CellFormat > ) >
    // fn cells< 'a >( &'a self ) -> impl IteratorTrait< Item = ( CellKey, MaybeAs< 'a, str, CellFormat > ) >
    fn cells< 'a, 'b >( &'a self ) -> impl IteratorTrait< Item = ( CellKey, MaybeAs< 'b, str, CellFormat > ) >
    where
      'a : 'b,
      // Cell : 'a,
    ;
  }

  impl< Row, CellKey, CellFormat > Cells< CellKey, CellFormat >
  for Row
  where
    for< 'b > Row : Fields< CellKey, MaybeAs< 'b, str, CellFormat > >,
    CellFormat : Copy + 'static,
    for< 'b > MaybeAs< 'b, str, CellFormat > : From< < Row as Fields< CellKey, MaybeAs< 'b, str, CellFormat > > >::Value< 'b > >,
  {

    // fn cells( &self ) -> impl IteratorTrait< Item = ( CellKey, MaybeAs< '_, str, CellFormat > ) >
    // fn cells< 'a >( &'a self ) -> impl IteratorTrait< Item = ( CellKey, MaybeAs< 'a, str, CellFormat > ) >
    fn cells< 'a, 'b >( &'a self ) -> impl IteratorTrait< Item = ( CellKey, MaybeAs< 'b, str, CellFormat > ) >
    where
      'a : 'b,
    // where
      // Cell : 'a,
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
  pub trait TableRows< RowKey, Row, CellKey, CellFormat >
  where
    Row : Clone + Cells< CellKey, CellFormat >,
    // Cell : std::borrow::ToOwned + ?Sized,
    CellFormat : Copy + 'static,
    CellKey : fmt::Debug + Clone + std::cmp::Eq + std::hash::Hash,
  {
    /// Returns an iterator over all rows of the table.
    fn rows< 'a >( &'a self ) -> impl IteratorTrait< Item = &'a Row >
    where Row : 'a;
  }

  impl< T, RowKey, Row, CellKey, CellFormat >
  TableRows< RowKey, Row, CellKey, CellFormat >
  for AsTable< '_, T, RowKey, Row, CellKey, CellFormat >
  where
    for< 'a > T : Fields< RowKey, &'a Row, Value< 'a > = &'a Row > + 'a,
    Row : Clone + Cells< CellKey, CellFormat >,
    // Title : fmt::Display,
    // Cell : fmt::Display,
    // Cell : std::borrow::ToOwned + ?Sized,
    CellKey : fmt::Debug + Clone + std::cmp::Eq + std::hash::Hash,
    CellFormat : Copy + 'static,
  {

    fn rows< 'a >( &'a self ) -> impl IteratorTrait< Item = &'a Row >
    where Row : 'a
    {
      self.as_ref().fields()
      .filter_map( move | ( _k, e ) |
      {
        Some( e )
        // match e.0
        // {
        //   Some( e ) => Some( e.into_owned() ),
        //   None => None,
        // }
      })
      .collect::< Vec< _ > >().into_iter()
    }

  }

  // =

  /// A trait for iterating over all rows of a table.
  pub trait TableSize
  {
    /// Returns size of a table.
    fn mcells( &self ) -> [ usize ; 2 ];
  }

  impl< T, RowKey, Row, CellKey, CellFormat > TableSize
  for AsTable< '_, T, RowKey, Row, CellKey, CellFormat >
  where
    Self : TableRows< RowKey, Row, CellKey, CellFormat >,
    Row : Clone + Cells< CellKey, CellFormat >,
    // Title : fmt::Display,
    // Cell : fmt::Display,
    // Cell : std::borrow::ToOwned + ?Sized,
    CellKey : fmt::Debug + Clone + std::cmp::Eq + std::hash::Hash,
    CellFormat : Copy + 'static,
  {
    fn mcells( &self ) -> [ usize ; 2 ]
    {
      let mut rows = self.rows();
      let nrows = rows.len();
      let row = rows.clone().next();
      if let Some( row2 ) = row
      {
        let cit = row2.cells().clone();
        let mcells = cit.len();
        [ nrows, mcells ]
        // [ 0, 0 ]
      }
      else
      {
        [ 0, 0 ]
      }
    }
  }

  // =

  /// Trait returning headers of a table if any.
  pub trait TableHeader< CellKey >
  {
    /// Returns an iterator over all fields of the specified type within the entity.
    fn header( &self ) -> Option< impl IteratorTrait< Item = ( CellKey, Cow< '_, str > ) > >;
  }

  impl< T, RowKey, Row, CellKey, CellFormat > TableHeader< CellKey >
  for AsTable< '_, T, RowKey, Row, CellKey, CellFormat >
  where
    Self : TableRows< RowKey, Row, CellKey, CellFormat >,
    Row : Clone + Cells< CellKey, CellFormat >,
    CellKey : fmt::Debug + Clone + std::cmp::Eq + std::hash::Hash,
    CellKey : fmt::Display,
    CellKey : AsRef< str >,
    CellFormat : Copy + 'static,
  {

    fn header( &self ) -> Option< impl IteratorTrait< Item = ( CellKey, Cow< '_, str > ) > >
    {
      let mut rows = self.rows();
      let row = rows.next();
      if let Some( row ) = row
      {
        Some
        (
          row
          .cells()
          .map( | ( key, _title ) | ( key.clone(), Cow::Owned( format!( "{}", key ) ) ) )
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
    TableHeader,
  };

}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;
}
