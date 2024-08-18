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
    borrow::Borrow,
  };
  // use std::borrow::Cow;
  use reflect_tools::
  {
    IteratorTrait,
    Fields,
  };

  // =

  /// Trait for types used as keys in table-like structures.
  ///
  /// The `Key` trait aggregates necessary bounds for keys, ensuring they support
  /// debugging, equality comparison, and hashing.
  ///

  pub trait Key
  where
    Self : fmt::Debug + std::cmp::Eq + std::hash::Hash + Borrow< str >,
  {
  }

  impl< T > Key for T
  where
    T : fmt::Debug + std::cmp::Eq + std::hash::Hash + Borrow< str > + ?Sized,
  {
  }

  /// Trait for types representing table cell content.
  ///
  /// `CellRepr` aggregates necessary bounds for types used as cell representations,
  /// ensuring they are copyable and have a static lifetime.
  ///

  pub trait CellRepr
  where
    Self : Copy + 'static,
  {
  }

  impl< T > CellRepr for T
  where
    T : Copy + 'static + ?Sized,
  {
  }

  // =

  /// A trait for iterating over all cells of a row.
  pub trait Cells< CellKey, CellRepr >
  where
    CellRepr : table::CellRepr,
    CellKey : ?Sized, // xxx : ?
  {
    /// Returns an iterator over all cells of the row.
    fn cells< 'a, 'b >( &'a self ) -> impl IteratorTrait< Item = ( &'b CellKey, MaybeAs< 'b, str, CellRepr > ) >
    where
      'a : 'b,
      CellKey : 'b,
    ;
  }

  impl< Row, CellKey, CellRepr > Cells< CellKey, CellRepr >
  for Row
  where
    CellKey : ?Sized,
    for< 'k, 'v >
    Row : Fields
    <
      &'k CellKey,
      MaybeAs< 'v, str, CellRepr >,
      Key< 'k > = &'k CellKey,
      Val< 'v > = MaybeAs< 'v, str, CellRepr >,
    > + 'k + 'v,
    // for< 'v > MaybeAs< 'v, str, CellRepr > : From
    // <
    //   MaybeAs< 'v, str, CellRepr >,
    // >,
    CellRepr : table::CellRepr,
  {

    fn cells< 'a, 'b >( &'a self ) -> impl IteratorTrait< Item = ( &'b CellKey, MaybeAs< 'b, str, CellRepr > ) >
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

  /// Trait for iterating over rows in a table.
  ///
  /// `TableRows` provides an interface to access all rows in a table,
  /// allowing iteration over the data structure.
  ///
  /// # Associated Types
  ///
  /// - `RowKey`: The type used to identify each row.
  ///
  /// - `Row`: The type representing a row, which must implement `Cells`
  ///   for the specified `CellKey` and `CellRepr`.
  ///
  /// - `CellKey`: The type used to identify cells within a row, requiring
  ///   implementation of the `Key` trait.
  ///
  /// - `CellRepr`: The type representing the content of a cell, requiring
  ///   implementation of the `CellRepr` trait.
  ///
  /// # Required Methods
  ///
  /// - `rows(&self) -> impl IteratorTrait<Item = &Self::Row>`:
  ///   Returns an iterator over all rows in the table.
  pub trait TableRows
  {
    ///
    /// The type used to identify each row.
    type RowKey;
    ///
    /// The type representing a row, which must implement `Cells`
    ///   for the specified `CellKey` and `CellRepr`.
    type Row : Cells< Self::CellKey, Self::CellRepr >;
    ///
    /// The type used to identify cells within a row, requiring
    ///   implementation of the `Key` trait.
    type CellKey : table::Key + ?Sized;
    ///
    /// The type representing the content of a cell, requiring
    ///   implementation of the `CellRepr` trait.
    type CellRepr : table::CellRepr;

    /// Returns an iterator over all rows of the table.
    fn rows< 'a >( & 'a self ) -> impl IteratorTrait< Item = & 'a Self::Row >
    where
      Self::Row : 'a;
  }

  impl< T, RowKey, Row, CellKey, CellRepr >
  TableRows<>
  for AsTable< '_, T, RowKey, Row, CellKey, CellRepr >
  where

    for< 'k, 'v > T : Fields
    <
      RowKey,
      &'v Row,
      Key< 'k > = RowKey,
      Val< 'v > = &'v Row,
    > + 'k + 'v,

    Row : Cells< CellKey, CellRepr >,
    CellKey : table::Key + ?Sized,
    CellRepr : table::CellRepr,
  {
    type RowKey = RowKey;
    type Row = Row;
    type CellKey = CellKey;
    type CellRepr = CellRepr;

    fn rows< 'a >( &'a self ) -> impl IteratorTrait< Item = &'a Self::Row >
    where Self::Row : 'a
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

  impl< T, RowKey, Row, CellKey, CellRepr > TableSize
  for AsTable< '_, T, RowKey, Row, CellKey, CellRepr >
  where
    Self : TableRows< RowKey = RowKey, Row = Row, CellKey = CellKey, CellRepr = CellRepr >,
    Row : Cells< CellKey, CellRepr >,
    CellKey : table::Key + ?Sized,
    CellRepr : table::CellRepr,
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
  {
    /// The type used to identify cells within a row, requiring
    ///   implementation of the `Key` trait.
    type CellKey : table::Key + ?Sized;
    /// Returns an iterator over all fields of the specified type within the entity.
    fn header( &self ) -> Option< impl IteratorTrait< Item = ( &Self::CellKey, &'_ str ) > >;
  }

  impl< T, RowKey, Row, CellKey, CellRepr > TableHeader
  for AsTable< '_, T, RowKey, Row, CellKey, CellRepr >
  where
    Self : TableRows< RowKey = RowKey, Row = Row, CellKey = CellKey, CellRepr = CellRepr >,
    Row : Cells< CellKey, CellRepr >,
    CellKey : table::Key + ?Sized,
    // CellKey : AsRef< str >,
    // CellKey : Borrow< str >,
    // xxx
    CellRepr : table::CellRepr,
  {
    type CellKey = CellKey;

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
          .map( | ( key, _title ) | ( key, key.borrow() ) )
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
    CellRepr,
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
