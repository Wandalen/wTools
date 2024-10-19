//!
//! Table interface.
//!

/// Internal namespace.
mod private
{

  use crate::*;
  use core::
  {
    // fmt,
    borrow::Borrow,
  };
  use std::borrow::Cow;
  use reflect_tools::
  {
    IteratorTrait,
    Fields,
  };

  // =

  /// Trait for types used as keys of rows in table-like structures.
  ///

  pub trait RowKey
  {
  }

  impl< T > RowKey for T
  where
    T : ?Sized,
  {
  }

  /// Trait for types used as keys of cells in table-like structures.
  ///
  /// The `CellKey` trait aggregates necessary bounds for keys, ensuring they support
  /// debugging, equality comparison, and hashing.
  ///

  pub trait CellKey
  where
    Self : core::cmp::Eq + std::hash::Hash + Borrow< str >,
  {
  }

  impl< T > CellKey for T
  where
    T : core::cmp::Eq + std::hash::Hash + Borrow< str > + ?Sized,
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
    T : Copy + 'static,
  {
  }

  // =

  /// Marker trait to tag structures for whcih table trait deducing should be done from trait Fields, which is reflection.
  pub trait TableWithFields {}

  // =

  /// A trait for iterating over all cells of a row.
  pub trait Cells< CellKey >
  where
    // CellRepr : table::CellRepr,
    CellKey : table::CellKey + ?Sized,
  {
    /// Returns an iterator over all cells of the row.
    // fn cells< 'a, 'b >( &'a self ) -> impl IteratorTrait< Item = ( &'b CellKey, OptionalCow< 'b, str > ) >
    fn cells< 'a, 'b >( &'a self ) -> impl IteratorTrait< Item = ( &'b CellKey, Option< Cow< 'b, str > > ) >
    where
      'a : 'b,
      CellKey : 'b,
    ;
  }

  impl< Row, CellKey > Cells< CellKey >
  for Row
  where
    CellKey : table::CellKey + ?Sized,
    for< 'ckv >
    Row : TableWithFields + Fields
    <
      &'ckv CellKey,
      // OptionalCow< 'ckv, str >,
      Option< Cow< 'ckv, str > >,
      Key< 'ckv > = &'ckv CellKey,
      // Val< 'ckv > = OptionalCow< 'ckv, str >,
      Val< 'ckv > = Option< Cow< 'ckv, str > >,
    > + 'ckv, // xxx
    // CellRepr : table::CellRepr,
  {

    // fn cells< 'a, 'b >( &'a self ) -> impl IteratorTrait< Item = ( &'b CellKey, OptionalCow< 'b, str > ) >
    fn cells< 'a, 'b >( &'a self ) -> impl IteratorTrait< Item = ( &'b CellKey, Option< Cow< 'b, str > > ) >
    where
      'a : 'b,
      CellKey : 'b,
    {
      self.fields().map
      (
        move | ( key, cell ) |
        {
          ( key, cell )
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
    type Row : Cells< Self::CellKey >;
    ///
    /// The type used to identify cells within a row, requiring
    ///   implementation of the `Key` trait.
    type CellKey : table::CellKey + ?Sized;
    ///
    // /// The type representing the content of a cell, requiring
    // ///   implementation of the `CellRepr` trait.
    // type // CellRepr : table::CellRepr;

    /// Returns an iterator over all rows of the table.
    fn rows( &self ) -> impl IteratorTrait< Item = &Self::Row >;
    // fn rows< 'a >( & 'a self ) -> impl IteratorTrait< Item = & 'a Self::Row >
    // where
    //   Self::Row : 'a;
  }

  impl< T, RowKey, Row, CellKey > TableRows<>
  for AsTable< '_, T, RowKey, Row, CellKey >
  where

    for< 'k, 'v > T : Fields
    <
      RowKey,
      &'k Row,
      // Key< 'k > = RowKey,
      Val< 'v > = &'v Row,
    > + 'k + 'v,

    RowKey : table::RowKey,
    Row : TableWithFields + Cells< CellKey >,
    CellKey : table::CellKey + ?Sized,
    // CellRepr : table::CellRepr,
  {
    type RowKey = RowKey;
    type Row = Row;
    type CellKey = CellKey;
    // type CellRepr = CellRepr;

    fn rows( &self ) -> impl IteratorTrait< Item = &Self::Row >
    // fn rows< 'a >( &'a self ) -> impl IteratorTrait< Item = &'a Self::Row >
    // where
      // Self::Row : 'a
    {
      self.as_ref().fields()
      .map( move | ( _k, e ) : ( _, &Row ) |
      {
        e
      })
    }

  }

  // =

//   /// A trait for iterating over all rows of a table.
//   pub trait TableSize
//   {
//     /// Returns multi-dimensional size of a table.
//     fn mcells( &self ) -> [ usize ; 2 ];
//   }
//
//   impl< T, RowKey, Row, CellKey > TableSize
//   for AsTable< '_, T, RowKey, Row, CellKey >
//   where
//     Self : TableRows< RowKey = RowKey, Row = Row, CellKey = CellKey >,
//     RowKey : table::RowKey,
//     Row : Cells< CellKey >,
//     CellKey : table::CellKey + ?Sized,
//     // CellRepr : table::CellRepr,
//   {
//     fn mcells( &self ) -> [ usize ; 2 ]
//     {
//       let rows = self.rows();
//       let nrows = rows.len();
//       let row = rows.clone().next();
//       if let Some( row2 ) = row
//       {
//         let cit = row2.cells().clone();
//         let mcells = cit.len();
//         [ mcells, nrows + 1 ]
//       }
//       else
//       {
//         [ 0, 0 ] // xxx : test
//       }
//     }
//   }

  // =

  /// Trait returning headers of a table if any.
  pub trait TableHeader
  {
    /// The type used to identify cells within a row, requiring
    ///   implementation of the `Key` trait.
    type CellKey : table::CellKey + ?Sized;
    /// Returns an iterator over all fields of the specified type within the entity.
    fn header( &self ) -> Option< impl IteratorTrait< Item = ( &Self::CellKey, &'_ str ) > >;
  }

  impl< T, RowKey, Row, CellKey > TableHeader
  for AsTable< '_, T, RowKey, Row, CellKey >
  where
    Self : TableRows< RowKey = RowKey, Row = Row, CellKey = CellKey >,
    RowKey : table::RowKey,
    Row : TableWithFields + Cells< CellKey >,
    CellKey : table::CellKey + ?Sized,
    // CellRepr : table::CellRepr,
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
          .map( | ( key, _title ) | ( key, key.borrow() ) )
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
    RowKey,
    CellKey,
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
    TableWithFields,
    Cells,
    TableRows,
    // TableSize,
    TableHeader,
  };

}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;
}
