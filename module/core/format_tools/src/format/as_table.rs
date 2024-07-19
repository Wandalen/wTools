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
  pub struct AsTable< 'a, T, RowKey, Row, CellKey, Cell, Kind, Title >
  (
    &'a T,
    ::core::marker::PhantomData< ( &'a (), fn () -> ( RowKey, Row, CellKey, Box< Cell >, Kind, Title ) ) >,
  )
  where
    Row : Clone + for< 'cell > Cells< 'cell, CellKey, Cell, Kind > + 'a,
    Title : fmt::Debug,
    Cell : fmt::Debug + 'a,
    Cell : std::borrow::ToOwned + ?Sized,
    CellKey : fmt::Debug + Clone,
    Kind : Copy + 'static,
  ;

  impl< 'a, T, RowKey, Row, CellKey, Cell, Kind, Title > AsTable< 'a, T, RowKey, Row, CellKey, Cell, Kind, Title >
  where
    Row : Clone + for< 'cell > Cells< 'cell, CellKey, Cell, Kind > + 'a,
    Title : fmt::Debug,
    Cell : fmt::Debug + 'a,
    Cell : std::borrow::ToOwned + ?Sized,
    CellKey : fmt::Debug + Clone,
    Kind : Copy + 'static,
  {
    /// Just a constructor.
    pub fn new( src : &'a T ) -> Self
    {
      Self( src, Default::default() )
    }
  }

  impl< 'a, T, RowKey, Row, CellKey, Cell, Kind, Title > AsRef< T >
  for AsTable< 'a, T, RowKey, Row, CellKey, Cell, Kind, Title >
  where
    Row : Clone + for< 'cell > Cells< 'cell, CellKey, Cell, Kind > + 'a,
    Title : fmt::Debug,
    Cell : fmt::Debug + 'a,
    Cell : std::borrow::ToOwned + ?Sized,
    CellKey : fmt::Debug + Clone,
    Kind : Copy + 'static,
  {
    fn as_ref( &self ) -> &T
    {
      &self.0
    }
  }

  impl< 'a, T, RowKey, Row, CellKey, Cell, Kind, Title > Deref
  for AsTable< 'a, T, RowKey, Row, CellKey, Cell, Kind, Title >
  where
    Row : Clone + for< 'cell > Cells< 'cell, CellKey, Cell, Kind > + 'a,
    Title : fmt::Debug,
    Cell : fmt::Debug + 'a,
    Cell : std::borrow::ToOwned + ?Sized,
    CellKey : fmt::Debug + Clone,
    Kind : Copy + 'static,
  {
    type Target = T;

    fn deref( &self ) -> &Self::Target
    {
      &self.0
    }
  }

  impl< 'a, T, RowKey, Row, CellKey, Cell, Kind, Title > From< &'a T >
  for AsTable< 'a, T, RowKey, Row, CellKey, Cell, Kind, Title >
  where
    Row : Clone + for< 'cell > Cells< 'cell, CellKey, Cell, Kind > + 'a,
    Title : fmt::Debug,
    Cell : fmt::Debug + 'a,
    Cell : std::borrow::ToOwned + ?Sized,
    CellKey : fmt::Debug + Clone,
    Kind : Copy + 'static,
  {
    fn from( table : &'a T ) -> Self
    {
      AsTable( table, PhantomData )
    }
  }

  impl< 'a, T, RowKey, Row, CellKey, Cell, Kind, Title > fmt::Debug
  for AsTable< 'a, T, RowKey, Row, CellKey, Cell, Kind, Title >
  where
    T : fmt::Debug,
    Row : Clone + for< 'cell > Cells< 'cell, CellKey, Cell, Kind > + 'a,
    Title : fmt::Debug,
    Cell : std::borrow::ToOwned + ?Sized,
    Cell : fmt::Debug + 'a,
    CellKey : fmt::Debug + Clone,
    Kind : Copy + 'static,
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
