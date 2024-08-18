//!
//! Nice print's wrapper.
//!

/// Internal namespace.
pub( crate ) mod private
{

  use crate::*;
  use core::
  {
    ops::{ Deref },
    marker::PhantomData,
    fmt,
    // cmp::Ordering,
  };

  // let as_table : AsTable< '_, Vec< TestObject >, usize, TestObject, str, WithRef > = AsTable::new( &test_objects );

  /// Transparent wrapper for table-like structures.
  #[ repr( transparent ) ]
  #[ derive( Clone, Copy ) ]
  pub struct AsTable< 'table, Table, RowKey, Row, CellKey, CellRepr >
  (
    &'table Table,
    ::core::marker::PhantomData< ( &'table (), fn () -> ( RowKey, Row, &'table CellKey, CellRepr ) ) >,
  )
  where
    Row : Clone + Cells< CellKey, CellRepr >,
    CellKey : table::Key + ?Sized,
    CellRepr : Copy + 'static,
  ;

  impl< 'table, Table, RowKey, Row, CellKey, CellRepr >
  AsTable< 'table, Table, RowKey, Row, CellKey, CellRepr >
  where
    Row : Clone + Cells< CellKey, CellRepr >,
    CellKey : table::Key + ?Sized,
    CellRepr : Copy + 'static,
  {
    /// Just a constructor.
    pub fn new( src : &'table Table ) -> Self
    {
      Self( src, Default::default() )
    }
  }

  impl< 'table, Table, RowKey, Row, CellKey, CellRepr > AsRef< Table >
  for AsTable< 'table, Table, RowKey, Row, CellKey, CellRepr >
  where
    Row : Clone + Cells< CellKey, CellRepr >,
    CellKey : table::Key + ?Sized,
    CellRepr : Copy + 'static,
  {
    fn as_ref( &self ) -> &Table
    {
      &self.0
    }
  }

  impl< 'table, Table, RowKey, Row, CellKey, CellRepr > Deref
  for AsTable< 'table, Table, RowKey, Row, CellKey, CellRepr >
  where
    Row : Clone + Cells< CellKey, CellRepr >,
    CellKey : table::Key + ?Sized,
    CellRepr : Copy + 'static,
  {
    type Target = Table;

    fn deref( &self ) -> &Self::Target
    {
      &self.0
    }
  }

  impl< 'table, Table, RowKey, Row, CellKey, CellRepr > From< &'table Table >
  for AsTable< 'table, Table, RowKey, Row, CellKey, CellRepr >
  where
    Row : Clone + Cells< CellKey, CellRepr >,
    CellKey : table::Key + ?Sized,
    CellRepr : Copy + 'static,
  {
    fn from( table : &'table Table ) -> Self
    {
      AsTable( table, PhantomData )
    }
  }

  impl< 'table, Table, RowKey, Row, CellKey, CellRepr > fmt::Debug
  for AsTable< 'table, Table, RowKey, Row, CellKey, CellRepr >
  where
    Table : fmt::Debug,
    Row : Clone + Cells< CellKey, CellRepr >,
    CellKey : table::Key + ?Sized,
    CellRepr : Copy + 'static, // xxx : maybe special trait?
  {
    fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
    {
      f
      .debug_struct( "AsTable" )
      .field( "0", &self.0 )
      .finish()
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
    AsTable,
    // CellKeyWrap,
  };

}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;
}
