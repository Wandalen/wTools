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
    cmp::Ordering,
  };

  /// Transparent wrapper for table-like structures.
  #[ repr( transparent ) ]
  #[ derive( Clone, Copy ) ]
  pub struct AsTable< 'a, T, RowKey, Row, CellKey, CellFormat, Title > // xxx : remove CellWrap or CellFormat?
  (
    &'a T,
    ::core::marker::PhantomData< ( &'a (), fn () -> ( RowKey, Row, CellKey, CellFormat, Title ) ) >,
  )
  where
    Row : Clone + Cells< CellKey, CellFormat >,
    Title : fmt::Display,
    // Cell : fmt::Display,
    // Cell : std::borrow::ToOwned + ?Sized,
    CellKey : fmt::Debug + Clone + std::cmp::Eq + std::hash::Hash,
    CellFormat : Copy + 'static,
  ;

  impl< 'table, T, RowKey, Row, CellKey, CellFormat, Title >
  AsTable< 'table, T, RowKey, Row, CellKey, CellFormat, Title >
  where
    Row : Clone + Cells< CellKey, CellFormat >,
    Title : fmt::Display,
    // Cell : fmt::Display,
    // Cell : std::borrow::ToOwned + ?Sized,
    CellKey : fmt::Debug + Clone + std::cmp::Eq + std::hash::Hash,
    CellFormat : Copy + 'static,
  {
    /// Just a constructor.
    pub fn new( src : &'table T ) -> Self
    {
      Self( src, Default::default() )
    }
  }

  impl< 'table, T, RowKey, Row, CellKey, CellFormat, Title > AsRef< T >
  for AsTable< 'table, T, RowKey, Row, CellKey, CellFormat, Title >
  where
    // 'table : 'row,
    // 'row : 'cell,
    Row : Clone + Cells< CellKey, CellFormat >,
    Title : fmt::Display,
    // Cell : fmt::Display,
    // Cell : std::borrow::ToOwned + ?Sized,
    CellKey : fmt::Debug + Clone + std::cmp::Eq + std::hash::Hash,
    CellFormat : Copy + 'static,
  {
    fn as_ref( &self ) -> &T
    {
      &self.0
    }
  }

  impl< 'table, T, RowKey, Row, CellKey, CellFormat, Title > Deref
  for AsTable< 'table, T, RowKey, Row, CellKey, CellFormat, Title >
  where
    // 'table : 'row,
    // 'row : 'cell,
    Row : Clone + Cells< CellKey, CellFormat >,
    Title : fmt::Display,
    // Cell : fmt::Display,
    // Cell : std::borrow::ToOwned + ?Sized,
    CellKey : fmt::Debug + Clone + std::cmp::Eq + std::hash::Hash,
    CellFormat : Copy + 'static,
  {
    type Target = T;

    fn deref( &self ) -> &Self::Target
    {
      &self.0
    }
  }

  impl< 'table, T, RowKey, Row, CellKey, CellFormat, Title > From< &'table T >
  for AsTable< 'table, T, RowKey, Row, CellKey, CellFormat, Title >
  where
    // 'table : 'row,
    // 'row : 'cell,
    Row : Clone + Cells< CellKey, CellFormat >,
    Title : fmt::Display,
    // Cell : fmt::Display,
    // Cell : std::borrow::ToOwned + ?Sized,
    CellKey : fmt::Debug + Clone + std::cmp::Eq + std::hash::Hash,
    CellFormat : Copy + 'static,
  {
    fn from( table : &'table T ) -> Self
    {
      AsTable( table, PhantomData )
    }
  }

  impl< 'table, T, RowKey, Row, CellKey, CellFormat, Title > fmt::Debug
  for AsTable< 'table, T, RowKey, Row, CellKey, CellFormat, Title >
  where
    // 'table : 'row,
    // 'row : 'cell,
    T : fmt::Debug,
    Row : Clone + Cells< CellKey, CellFormat >,
    Title : fmt::Display,
    // Cell : std::borrow::ToOwned + ?Sized,
    // Cell : fmt::Display,
    CellKey : fmt::Debug + Clone + std::cmp::Eq + std::hash::Hash,
    CellFormat : Copy + 'static,
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

  pub struct CellKeyWrap< CellKey >( pub CellKey, pub usize )
  where
    CellKey : fmt::Debug + Clone + std::cmp::Eq + std::hash::Hash,
  ;

  impl< CellKey > CellKeyWrap< CellKey >
  where
    CellKey : fmt::Debug + Clone + std::cmp::Eq + std::hash::Hash,
  {
    /// Just a constructor.
    pub fn new( key : CellKey, index : usize ) -> Self
    {
      Self( key, index )
    }
  }

  impl< CellKey > Clone for CellKeyWrap< CellKey >
  where
    CellKey : fmt::Debug + Clone + std::cmp::Eq + std::hash::Hash,
  {
    fn clone( &self ) -> Self
    {
      Self( self.0.clone(), self.1 )
    }
  }

  impl< CellKey > AsRef< CellKey > for CellKeyWrap< CellKey >
  where
    CellKey : fmt::Debug + Clone + std::cmp::Eq + std::hash::Hash,
  {
    fn as_ref( &self ) -> &CellKey
    {
      &self.0
    }
  }

  impl< CellKey > Deref for CellKeyWrap< CellKey >
  where
    CellKey : fmt::Debug + Clone + std::cmp::Eq + std::hash::Hash,
  {
    type Target = CellKey;
    fn deref( &self ) -> &CellKey
    {
      &self.0
    }
  }

  impl< CellKey > From< ( CellKey, usize ) >
  for CellKeyWrap< CellKey >
  where
    CellKey : fmt::Debug + Clone + std::cmp::Eq + std::hash::Hash,
  {
    fn from( src : ( CellKey, usize ) ) -> Self
    {
      CellKeyWrap::new( src.0, src.1 )
    }
  }

  impl< CellKey > fmt::Debug for CellKeyWrap< CellKey >
  where
    CellKey : fmt::Debug + Clone + std::cmp::Eq + std::hash::Hash,
  {
    fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
    {
      f.debug_struct( "CellKey" )
      .field( "0", &self.0 )
      .field( "1", &self.1 )
      .finish()
    }
  }

  impl< CellKey > PartialEq for CellKeyWrap< CellKey >
  where
    CellKey : fmt::Debug + Clone + std::cmp::Eq + std::hash::Hash, // xxx : there should be std::cmp::PartialEq, probably
  {
    fn eq( &self, other : &Self ) -> bool
    {
      self.1 == other.1
      // self.as_ref() == other.as_ref()
    }
  }

  impl< CellKey > Eq for CellKeyWrap< CellKey >
  where
    CellKey : fmt::Debug + Clone + std::cmp::Eq + std::hash::Hash,
  {
  }

  // impl< CellKey > Ord for CellKeyWrap< CellKey >
  // {
  //   fn cmp( &self, other : &Self ) -> Ordering
  //   {
  //     self.1.cmp( &other.1 ).then_with( || self.0.cmp( &other.0 ) )
  //   }
  // }

  impl< CellKey > PartialOrd for CellKeyWrap< CellKey >
  where
    CellKey : fmt::Debug + Clone + std::cmp::Eq + std::hash::Hash,
  {
    fn partial_cmp( &self, other : &Self ) -> Option< Ordering >
    {
      Some( self.1.cmp( &other.1 ) )
    }
  }

  impl< CellKey > Ord for CellKeyWrap< CellKey >
  where
    CellKey : fmt::Debug + Clone + std::cmp::Eq + std::hash::Hash,
  {
    fn cmp( &self, other : &Self ) -> Ordering
    {
      self.1.cmp( &other.1 )
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
    CellKeyWrap,
  };

}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;
}
