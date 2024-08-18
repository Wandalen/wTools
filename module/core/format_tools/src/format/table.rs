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

  pub trait Key
  where
    Self : fmt::Debug + std::cmp::Eq + std::hash::Hash
  {
  }

  impl< T > Key for T
  where
    Self : fmt::Debug + std::cmp::Eq + std::hash::Hash
  {
  }

  // =

  /// A trait for iterating over all cells of a row.
  pub trait Cells< CellKey, CellFormat >
  where
    CellFormat : Copy + 'static,
    CellKey : ?Sized,
  {
    /// Returns an iterator over all cells of the row.
    fn cells< 'a, 'b >( &'a self ) -> impl IteratorTrait< Item = ( &'b CellKey, MaybeAs< 'b, str, CellFormat > ) >
    where
      'a : 'b,
      CellKey : 'b,
    ;
  }

  impl< Row, CellKey, CellFormat > Cells< CellKey, CellFormat >
  for Row
  where
    CellKey : ?Sized,
    for< 'k, 'v >
    Row : Fields
    <
      &'k CellKey,
      MaybeAs< 'v, str, CellFormat >,
      Key< 'k > = &'k CellKey,
      Val< 'v > = MaybeAs< 'v, str, CellFormat >,
    > + 'k + 'v,
    for< 'v > MaybeAs< 'v, str, CellFormat > : From
    <
      MaybeAs< 'v, str, CellFormat >,
      // <
      //   // Row as Fields< &'b CellKey, MaybeAs< 'b, str, CellFormat > >
      //   Row as Fields
      //   <
      //     &'b CellKey,
      //     MaybeAs< 'b, str, CellFormat >,
      //     Key< 'b > = &'b CellKey,
      //     Val< 'b > = MaybeAs< 'b, str, CellFormat >,
      //   >,
      // >::Val< 'b >
    >,
    CellFormat : Copy + 'static,
    // for< 'b > Row : 'b,
  {

    fn cells< 'a, 'b >( &'a self ) -> impl IteratorTrait< Item = ( &'b CellKey, MaybeAs< 'b, str, CellFormat > ) >
    where
      'a : 'b,
      CellKey : 'b,
    {
      self.fields().map
      (
        move | ( key, cell ) |
        {
          ( key, cell )
          // ( key.clone(), cell.clone() )
          // ( key, cell.into() )
        }
      )
    }

  }

  // =

  /// A trait for iterating over all rows of a table.
  pub trait TableRows<>
  where
    Self::Row : Clone + Cells< Self::CellKey, Self::CellFormat >,
    Self::CellFormat : Copy + 'static,
    // Self::CellKey : fmt::Debug + std::cmp::Eq + std::hash::Hash + ?Sized,
    // &'table CellKey : Clone,
  {
    type RowKey;
    type Row;
    type CellKey : fmt::Debug + std::cmp::Eq + std::hash::Hash + ?Sized;
    type CellFormat;

    /// Returns an iterator over all rows of the table.
    fn rows< 'a >( &'a self ) -> impl IteratorTrait< Item = &'a Self::Row >
    where Self::Row : 'a;
    // fn rows( &'data self ) -> impl IteratorTrait< Item = &'data Self::Row >
    // where Self::Row : 'data;
  }

  impl< T, RowKey, Row, CellKey, CellFormat >
  TableRows<>
  for AsTable< '_, T, RowKey, Row, CellKey, CellFormat >
  where

    // for< 'a > T : Fields< RowKey, &'a Row, Key< 'a > = RowKey, Val< 'a > = &'a Row  >,

    for< 'k, 'v > T : Fields
    <
      RowKey,
      &'v Row,
      Key< 'k > = RowKey,
      Val< 'v > = &'v Row,
    > + 'k + 'v,

    Row : Clone + Cells< CellKey, CellFormat >,
    CellKey : fmt::Debug + std::cmp::Eq + std::hash::Hash + ?Sized,
    // &'table CellKey : Clone,
    CellFormat : Copy + 'static,
  {
    type RowKey = RowKey;
    type Row = Row;
    type CellKey = CellKey;
    type CellFormat = CellFormat;

    fn rows< 'a >( &'a self ) -> impl IteratorTrait< Item = &'a Self::Row >
    where Self::Row : 'a
    // fn rows( &'data self ) -> impl IteratorTrait< Item = &'data Self::Row >
    //   where Self::Row : 'data
    {
      self.as_ref().fields()
      .filter_map( move | ( _k, e ) |
      {
        Some( e )
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
    Self : TableRows< RowKey = RowKey, Row = Row, CellKey = CellKey, CellFormat = CellFormat >,
    Row : Clone + Cells< CellKey, CellFormat >,
    CellKey : fmt::Debug + std::cmp::Eq + std::hash::Hash + ?Sized,
    // &'table CellKey : Clone,
    CellFormat : Copy + 'static,
  {
    fn mcells( &self ) -> [ usize ; 2 ]
    {
      let rows = self.rows();
      let nrows = rows.len();
      let row = rows.clone().next();
      if let Some( row2 ) = row
      {
        let cit = row2.cells().clone();
        let mcells = cit.len();
        [ mcells, nrows ]
      }
      else
      {
        [ 0, 0 ]
      }
    }
  }

  // =

  /// Trait returning headers of a table if any.
  pub trait TableHeader
  where
    // Self::CellKey : fmt::Debug + std::cmp::Eq + std::hash::Hash + ?Sized,
    // &'table CellKey : Clone,
  {
    type CellKey : fmt::Debug + std::cmp::Eq + std::hash::Hash + ?Sized;
    /// Returns an iterator over all fields of the specified type within the entity.
    fn header( &self ) -> Option< impl IteratorTrait< Item = ( &Self::CellKey, &'_ str ) > >;
    // fn header( &self ) -> Option< impl IteratorTrait< Item = ( Self::CellKey, Cow< '_, str > ) > >;
  }

  impl< T, RowKey, Row, CellKey, CellFormat > TableHeader
  for AsTable< '_, T, RowKey, Row, CellKey, CellFormat >
  where
    Self : TableRows< RowKey = RowKey, Row = Row, CellKey = CellKey, CellFormat = CellFormat >,
    Row : Clone + Cells< CellKey, CellFormat >,
    CellKey : fmt::Debug + std::cmp::Eq + std::hash::Hash + ?Sized,
    // &'table CellKey : Clone,
    CellKey : fmt::Display,
    CellKey : AsRef< str >,
    CellFormat : Copy + 'static,
  {
    type CellKey = CellKey;

    // fn header( &self ) -> Option< impl IteratorTrait< Item = ( Self::CellKey, Cow< '_, str > ) > >
    fn header( &self ) -> Option< impl IteratorTrait< Item = ( &Self::CellKey, &'_ str ) > >
    {
      let mut rows = self.rows();
      let row = rows.next();
      if let Some( row ) = row
      {
        Some
        (
          row
          .cells()
          // .map( | ( key, _title ) | ( key.clone(), Cow::Owned( format!( "{}", key ) ) ) )
          .map( | ( key, _title ) | ( key, key.as_ref() ) )
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

  #[ doc( inline ) ]
  pub use private::
  {
    Key,
  };

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
  pub use super::super::table;

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
