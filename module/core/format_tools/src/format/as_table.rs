//!
//! Nice print's wrapper.
//!

/// Internal namespace.
mod private
{

  use crate::*;
  use core::
  {
    ops::{ Deref },
    marker::PhantomData,
    fmt,
  };

  /// Transparent wrapper for interpreting data as a table.
  ///
  /// `AsTable` provides a reference-based wrapper for table-like structures,
  /// encapsulating type information needed to interpret data as a table.
  ///
  #[ repr( transparent ) ]
  #[ derive( Clone, Copy ) ]
  pub struct AsTable< 'table, Table, RowKey, Row, CellKey>
  (
    &'table Table,
    ::core::marker::PhantomData
    <(
      &'table (),
      fn() -> ( &'table RowKey, Row, &'table CellKey ),
    )>,
  )
  where
    RowKey : table::RowKey,
    Row : Cells< CellKey>,
    CellKey : table::CellKey + ?Sized,
    // CellRepr : table::CellRepr
  ;

  impl< 'table, Table, RowKey, Row, CellKey>
  AsTable< 'table, Table, RowKey, Row, CellKey>
  where
    RowKey : table::RowKey,
    Row : Cells< CellKey>,
    CellKey : table::CellKey + ?Sized,
    // CellRepr : table::CellRepr,
  {
    /// Just a constructor.
    pub fn new( src : &'table Table ) -> Self
    {
      Self( src, Default::default() )
    }
  }

  impl< 'table, Table, RowKey, Row, CellKey> AsRef< Table >
  for AsTable< 'table, Table, RowKey, Row, CellKey>
  where
    RowKey : table::RowKey,
    Row : Cells< CellKey>,
    CellKey : table::CellKey + ?Sized,
    // CellRepr : table::CellRepr,
  {
    fn as_ref( &self ) -> &Table
    {
      &self.0
    }
  }

  impl< 'table, Table, RowKey, Row, CellKey> Deref
  for AsTable< 'table, Table, RowKey, Row, CellKey>
  where
    RowKey : table::RowKey,
    Row : Cells< CellKey>,
    CellKey : table::CellKey + ?Sized,
    // CellRepr : table::CellRepr,
  {
    type Target = Table;

    fn deref( &self ) -> &Self::Target
    {
      &self.0
    }
  }

  impl< 'table, Table, RowKey, Row, CellKey> From< &'table Table >
  for AsTable< 'table, Table, RowKey, Row, CellKey>
  where
    RowKey : table::RowKey,
    Row : Cells< CellKey>,
    CellKey : table::CellKey + ?Sized,
    // CellRepr : table::CellRepr,
  {
    fn from( table : &'table Table ) -> Self
    {
      AsTable( table, PhantomData )
    }
  }

  impl< 'table, Table, RowKey, Row, CellKey> fmt::Debug
  for AsTable< 'table, Table, RowKey, Row, CellKey>
  where
    Table : fmt::Debug,
    RowKey : table::RowKey,
    Row : Cells< CellKey>,
    CellKey : table::CellKey + ?Sized,
    // CellRepr : table::CellRepr,
  {
    fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
    {
      f
      .debug_struct( "AsTable" )
      .field( "0", &self.0 )
      .finish()
    }
  }

  // =

  /// Trait for converting data references into `AsTable` references.
  ///
  /// `IntoAsTable` provides a way to interpret data as a table, encapsulating
  /// the necessary type information for handling table-like structures.
  ///
  pub trait IntoAsTable
  {
    /// The type representing the table.
    type Table;

    /// The type used to identify each row.
    type RowKey : table::RowKey;

    /// The type representing a row, must implement `Cells`.
    type Row : Cells< Self::CellKey >;

    /// The type used to identify cells within a row, must implement `Key` and can be unsized.
    type CellKey : table::CellKey + ?Sized;

    // /// The type representing the content of a cell, must implement `CellRepr`.
    // type // CellRepr : table::CellRepr;

    /// Converts the data reference into an `AsTable` reference.
    fn as_table( &self ) -> AsTable< '_, Self::Table, Self::RowKey, Self::Row, Self::CellKey >;
  }

  impl< 'table, Table, RowKey, Row, CellKey> IntoAsTable
  for AsTable< 'table, Table, RowKey, Row, CellKey>
  where
    RowKey : table::RowKey,
    Row : Cells< CellKey>,
    CellKey : table::CellKey + ?Sized,
    // CellRepr : table::CellRepr,
    Self : Copy,
  {

    type Table = Table;
    type RowKey = RowKey;
    type Row = Row;
    type CellKey = CellKey;
    // type CellRepr = CellRepr;

    fn as_table( &self ) -> AsTable< '_, Self::Table, Self::RowKey, Self::Row, Self::CellKey >
    {
      *self
    }

  }

//   impl< Row > IntoAsTable
//   for Vec< Row >
//   where
//     Row : Cells< Self::CellKey >,
//     // CellKey : table::CellKey + ?Sized,
//     // // CellRepr : table::CellRepr,
//   {
//
//     type Table = Self;
//     type RowKey = usize;
//     type Row = Row;
//     type CellKey = str;
//     type CellRepr = WithRef;
//
//     fn as_table( &self ) -> AsTable< '_, Self::Table, Self::RowKey, Self::Row, Self::CellKey >
//     {
//       AsTable::from( self )
//     }
//
//   }

  // pub struct AsTable< 'table, Table, RowKey, Row, CellKey>

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
    AsTable,
    IntoAsTable,
  };

}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;
}
