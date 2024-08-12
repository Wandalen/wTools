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

  /// Transparent wrapper for table-like structures.
  #[ repr( transparent ) ]
  #[ derive( Clone, Copy ) ]
  pub struct AsTable< 'table, T, RowKey, Row, CellKey, CellFormat >
  (
    &'table T,
    ::core::marker::PhantomData< ( &'table (), fn () -> ( RowKey, Row, CellKey, CellFormat ) ) >,
  )
  where
    Row : Clone + Cells< CellKey, CellFormat >,
    CellKey : fmt::Debug + Clone + std::cmp::Eq + std::hash::Hash,
    CellFormat : Copy + 'static,
  ;

  impl< 'table, T, RowKey, Row, CellKey, CellFormat >
  AsTable< 'table, T, RowKey, Row, CellKey, CellFormat >
  where
    Row : Clone + Cells< CellKey, CellFormat >,
    CellKey : fmt::Debug + Clone + std::cmp::Eq + std::hash::Hash,
    CellFormat : Copy + 'static,
  {
    /// Just a constructor.
    pub fn new( src : &'table T ) -> Self
    {
      Self( src, Default::default() )
    }
  }

  impl< 'table, T, RowKey, Row, CellKey, CellFormat > AsRef< T >
  for AsTable< 'table, T, RowKey, Row, CellKey, CellFormat >
  where
    Row : Clone + Cells< CellKey, CellFormat >,
    CellKey : fmt::Debug + Clone + std::cmp::Eq + std::hash::Hash,
    CellFormat : Copy + 'static,
  {
    fn as_ref( &self ) -> &T
    {
      &self.0
    }
  }

  impl< 'table, T, RowKey, Row, CellKey, CellFormat > Deref
  for AsTable< 'table, T, RowKey, Row, CellKey, CellFormat >
  where
    Row : Clone + Cells< CellKey, CellFormat >,
    CellKey : fmt::Debug + Clone + std::cmp::Eq + std::hash::Hash,
    CellFormat : Copy + 'static,
  {
    type Target = T;

    fn deref( &self ) -> &Self::Target
    {
      &self.0
    }
  }

  impl< 'table, T, RowKey, Row, CellKey, CellFormat > From< &'table T >
  for AsTable< 'table, T, RowKey, Row, CellKey, CellFormat >
  where
    Row : Clone + Cells< CellKey, CellFormat >,
    CellKey : fmt::Debug + Clone + std::cmp::Eq + std::hash::Hash,
    CellFormat : Copy + 'static,
  {
    fn from( table : &'table T ) -> Self
    {
      AsTable( table, PhantomData )
    }
  }

  impl< 'table, T, RowKey, Row, CellKey, CellFormat > fmt::Debug
  for AsTable< 'table, T, RowKey, Row, CellKey, CellFormat >
  where
    T : fmt::Debug,
    Row : Clone + Cells< CellKey, CellFormat >,
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

//   // =
//
//   pub struct CellKeyWrap< CellKey >
//   where
//     CellKey : fmt::Debug + Clone + std::cmp::Eq + std::hash::Hash,
//   {
//     pub data : CellKey,
//     pub index : usize,
//   }
//
//   impl< CellKey > CellKeyWrap< CellKey >
//   where
//     CellKey : fmt::Debug + Clone + std::cmp::Eq + std::hash::Hash,
//   {
//     /// Just a constructor.
//     pub fn new( data : CellKey, index : usize ) -> Self
//     {
//       Self { data, index }
//     }
//   }
//
//   impl< CellKey > Clone for CellKeyWrap< CellKey >
//   where
//     CellKey : fmt::Debug + Clone + std::cmp::Eq + std::hash::Hash,
//   {
//     fn clone( &self ) -> Self
//     {
//       Self::new( self.data.clone(), self.index )
//     }
//   }
//
//   impl< CellKey > AsRef< CellKey > for CellKeyWrap< CellKey >
//   where
//     CellKey : fmt::Debug + Clone + std::cmp::Eq + std::hash::Hash,
//   {
//     fn as_ref( &self ) -> &CellKey
//     {
//       &self.data
//     }
//   }
//
//   impl< CellKey > Deref for CellKeyWrap< CellKey >
//   where
//     CellKey : fmt::Debug + Clone + std::cmp::Eq + std::hash::Hash,
//   {
//     type Target = CellKey;
//     fn deref( &self ) -> &CellKey
//     {
//       &self.data
//     }
//   }
//
//   impl< CellKey > From< ( CellKey, usize ) >
//   for CellKeyWrap< CellKey >
//   where
//     CellKey : fmt::Debug + Clone + std::cmp::Eq + std::hash::Hash,
//   {
//     fn from( src : ( CellKey, usize ) ) -> Self
//     {
//       CellKeyWrap::new( src.0, src.1 )
//     }
//   }
//
//   impl< CellKey > fmt::Debug for CellKeyWrap< CellKey >
//   where
//     CellKey : fmt::Debug + Clone + std::cmp::Eq + std::hash::Hash,
//   {
//     fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
//     {
//       f.debug_struct( "CellKey" )
//       .field( "data", &self.data )
//       .field( "index", &self.index )
//       .finish()
//     }
//   }
//
//   impl< CellKey > PartialEq for CellKeyWrap< CellKey >
//   where
//     CellKey : fmt::Debug + Clone + std::cmp::Eq + std::hash::Hash, // xxx : there should be std::cmp::PartialEq, probably
//   {
//     fn eq( &self, other : &Self ) -> bool
//     {
//       self.index == other.index
//       // self.as_ref() == other.as_ref()
//     }
//   }
//
//   impl< CellKey > Eq for CellKeyWrap< CellKey >
//   where
//     CellKey : fmt::Debug + Clone + std::cmp::Eq + std::hash::Hash,
//   {
//   }
//
//   impl< CellKey > PartialOrd for CellKeyWrap< CellKey >
//   where
//     CellKey : fmt::Debug + Clone + std::cmp::Eq + std::hash::Hash,
//   {
//     fn partial_cmp( &self, other : &Self ) -> Option< Ordering >
//     {
//       Some( self.index.cmp( &other.index ) )
//     }
//   }
//
//   impl< CellKey > Ord for CellKeyWrap< CellKey >
//   where
//     CellKey : fmt::Debug + Clone + std::cmp::Eq + std::hash::Hash,
//   {
//     fn cmp( &self, other : &Self ) -> Ordering
//     {
//       self.index.cmp( &other.index )
//     }
//   }

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
