//!
//! Nice print's wrapper.
//!

/// Internal namespace.
pub( crate ) mod private
{

  use crate::*;
  use core::ops::{ Deref };
  use core::marker::PhantomData;
  use core::fmt;

  /// Transparent wrapper for table-like structures.
  #[ repr( transparent ) ]
  #[ derive( Clone, Copy ) ]
  pub struct AsTable< 'a, T, RowKey, Row, CellKey, Cell, Title >
  (
    &'a T,
    ::core::marker::PhantomData< ( &'a (), fn () -> ( RowKey, Row, CellKey, Cell, Title ) ) >,
  )
  where
    Row : Clone + for< 'cell > Cells< 'cell, CellKey, Cell > + 'a,
    Title : fmt::Debug,
    Cell : fmt::Debug + Clone + 'a,
    CellKey : fmt::Debug + Clone,
  ;

  impl< 'a, T, RowKey, Row, CellKey, Cell, Title > AsTable< 'a, T, RowKey, Row, CellKey, Cell, Title >
  where
    Row : Clone + for< 'cell > Cells< 'cell, CellKey, Cell > + 'a,
    Title : fmt::Debug,
    Cell : fmt::Debug + Clone + 'a,
    CellKey : fmt::Debug + Clone,
  {
    /// Just a constructor.
    pub fn new( src : &'a T ) -> Self
    {
      Self( src, Default::default() )
    }
  }

  impl< 'a, T, RowKey, Row, CellKey, Cell, Title > AsRef< T > for AsTable< 'a, T, RowKey, Row, CellKey, Cell, Title >
  where
    Row : Clone + for< 'cell > Cells< 'cell, CellKey, Cell > + 'a,
    Title : fmt::Debug,
    Cell : fmt::Debug + Clone + 'a,
    CellKey : fmt::Debug + Clone,
  {
    fn as_ref( &self ) -> &T
    {
      &self.0
    }
  }

  impl< 'a, T, RowKey, Row, CellKey, Cell, Title > Deref for AsTable< 'a, T, RowKey, Row, CellKey, Cell, Title >
  where
    Row : Clone + for< 'cell > Cells< 'cell, CellKey, Cell > + 'a,
    Title : fmt::Debug,
    Cell : fmt::Debug + Clone + 'a,
    CellKey : fmt::Debug + Clone,
  {
    type Target = T;

    fn deref( &self ) -> &Self::Target
    {
      &self.0
    }
  }

  impl< 'a, T, RowKey, Row, CellKey, Cell, Title > From< &'a T >
  for AsTable< 'a, T, RowKey, Row, CellKey, Cell, Title >
  where
    Row : Clone + for< 'cell > Cells< 'cell, CellKey, Cell > + 'a,
    Title : fmt::Debug,
    Cell : fmt::Debug + Clone + 'a,
    CellKey : fmt::Debug + Clone,
  {
    fn from( table : &'a T ) -> Self
    {
      AsTable( table, PhantomData )
    }
  }

  impl< 'a, T, RowKey, Row, CellKey, Cell, Title > fmt::Debug for AsTable< 'a, T, RowKey, Row, CellKey, Cell, Title >
  where
    T : fmt::Debug,
    Row : Clone + for< 'cell > Cells< 'cell, CellKey, Cell > + 'a,
    Title : fmt::Debug,
    Cell : fmt::Debug + Clone + 'a,
    CellKey : fmt::Debug + Clone,
  {
    fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
    {
      f.debug_struct( "AsTable" )
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
