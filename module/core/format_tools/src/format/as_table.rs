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
  };

  /// Transparent wrapper for table-like structures.
  #[ repr( transparent ) ]
  #[ derive( Clone, Copy ) ]
  pub struct AsTable< 'table, 'row, 'cell, T, RowKey, Row, CellKey, Cell, CellKind, Title >
  (
    &'table T,
    ::core::marker::PhantomData< ( &'table (), &'row (), &'cell (), fn () -> ( RowKey, Row, CellKey, Box< Cell >, CellKind, Title ) ) >,
  )
  where
    'table : 'row,
    'row : 'cell,
    Row : Clone + Cells< 'row, 'cell, CellKey, Cell, CellKind >,
    Title : fmt::Display,
    Cell : fmt::Display,
    Cell : std::borrow::ToOwned + ?Sized + 'cell,
    CellKey : fmt::Debug + Clone + std::cmp::Eq + std::hash::Hash,
    CellKind : Copy + 'static,
  ;

  impl< 'table, 'row, 'cell, T, RowKey, Row, CellKey, Cell, CellKind, Title > AsTable< 'table, 'row, 'cell, T, RowKey, Row, CellKey, Cell, CellKind, Title >
  where
    'table : 'row,
    'row : 'cell,
    Row : Clone + Cells< 'row, 'cell, CellKey, Cell, CellKind >,
    Title : fmt::Display,
    Cell : fmt::Display,
    Cell : std::borrow::ToOwned + ?Sized + 'cell,
    CellKey : fmt::Debug + Clone + std::cmp::Eq + std::hash::Hash,
    CellKind : Copy + 'static,
  {
    /// Just a constructor.
    pub fn new( src : &'table T ) -> Self
    {
      Self( src, Default::default() )
    }
  }

  impl< 'table, 'row, 'cell, T, RowKey, Row, CellKey, Cell, CellKind, Title > AsRef< T >
  for AsTable< 'table, 'row, 'cell, T, RowKey, Row, CellKey, Cell, CellKind, Title >
  where
    'table : 'row,
    'row : 'cell,
    Row : Clone + Cells< 'row, 'cell, CellKey, Cell, CellKind >,
    Title : fmt::Display,
    Cell : fmt::Display,
    Cell : std::borrow::ToOwned + ?Sized + 'cell,
    CellKey : fmt::Debug + Clone + std::cmp::Eq + std::hash::Hash,
    CellKind : Copy + 'static,
  {
    fn as_ref( &self ) -> &T
    {
      &self.0
    }
  }

  impl< 'table, 'row, 'cell, T, RowKey, Row, CellKey, Cell, CellKind, Title > Deref
  for AsTable< 'table, 'row, 'cell, T, RowKey, Row, CellKey, Cell, CellKind, Title >
  where
    'table : 'row,
    'row : 'cell,
    Row : Clone + Cells< 'row, 'cell, CellKey, Cell, CellKind >,
    Title : fmt::Display,
    Cell : fmt::Display,
    Cell : std::borrow::ToOwned + ?Sized + 'cell,
    CellKey : fmt::Debug + Clone + std::cmp::Eq + std::hash::Hash,
    CellKind : Copy + 'static,
  {
    type Target = T;

    fn deref( &self ) -> &Self::Target
    {
      &self.0
    }
  }

  impl< 'table, 'row, 'cell, T, RowKey, Row, CellKey, Cell, CellKind, Title > From< &'table T >
  for AsTable< 'table, 'row, 'cell, T, RowKey, Row, CellKey, Cell, CellKind, Title >
  where
    'table : 'row,
    'row : 'cell,
    Row : Clone + Cells< 'row, 'cell, CellKey, Cell, CellKind >,
    Title : fmt::Display,
    Cell : fmt::Display,
    Cell : std::borrow::ToOwned + ?Sized + 'cell,
    CellKey : fmt::Debug + Clone + std::cmp::Eq + std::hash::Hash,
    CellKind : Copy + 'static,
  {
    fn from( table : &'table T ) -> Self
    {
      AsTable( table, PhantomData )
    }
  }

  impl< 'table, 'row, 'cell, T, RowKey, Row, CellKey, Cell, CellKind, Title > fmt::Debug
  for AsTable< 'table, 'row, 'cell, T, RowKey, Row, CellKey, Cell, CellKind, Title >
  where
    'table : 'row,
    'row : 'cell,
    T : fmt::Debug,
    Row : Clone + Cells< 'row, 'cell, CellKey, Cell, CellKind >,
    Title : fmt::Display,
    Cell : std::borrow::ToOwned + ?Sized + 'cell,
    Cell : fmt::Display,
    CellKey : fmt::Debug + Clone + std::cmp::Eq + std::hash::Hash,
    CellKind : Copy + 'static,
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
  };

}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;
}
