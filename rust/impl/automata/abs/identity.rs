/// Internal namespace.
pub( crate ) mod private
{
  // use crate::prelude::*;
  // use wtools::{ From2, Into2 };
  use core::fmt;
  use core::hash::Hash;
  use std::cmp::{ PartialEq, Eq };
  // use std::fmt;

  ///
  /// Interface to identify an instance of somthing, for exampel a node.
  ///

  pub trait IdentityInterface
  where
    Self :
      'static +
      Copy +
      Hash +
      fmt::Debug +
      PartialEq +
      Eq +
      Hash +
  {
  }

  impl< T > IdentityInterface for T
  where
    T :
      'static +
      Copy +
      Hash +
      fmt::Debug +
      PartialEq +
      Eq +
      Hash +
    ,
  {
  }

  ///
  /// Interface to identify an instance of somthing with ability to increase it to generate a new one.
  ///

  pub trait IdentityGenerableInterface
  where
    Self : IdentityInterface + Default,
  {
    /// Generate a new identity based on the current increasing it.
    fn next( &self ) -> Self;
    /// Generate the first identity.
    fn first() -> Self
    {
      Default::default()
    }
    /// Check is the identity valid.
    fn is_valid( &self ) -> bool;
  }

  ///
  /// Instance has an id.
  ///

  pub trait HasId
  {
    /// Id of the node.
    type Id : IdentityInterface;
    /// Get id.
    fn id( &self ) -> Self::Id;
  }

  // xxx : move to type_constructor

  // impl< Into1, Into2, Id > From< ( Into1, Into2 ) >
  // for ( Id, Id )
  // where
  //   Id : IdentityInterface,
  //   Into1 : Into< Id >,
  //   Into2 : Into< Id >,
  // {
  //   fn from( src : ( Into1, Into2 ) ) -> Self
  //   {
  //     ( src.0.into(), src.1.into() )
  //   }
  // }

//   pub trait IdsVectorizer< IdentityInterface, Dst >
//   {
//     fn into_ids( src : Src ) -> Dst;
//   }
//
//   impl IdsVectorizer<  >
//   {
//   }

//   /// Convert argument into edges ids.
//   #[ allow( non_snake_case ) ]
//   #[ inline ]
//   fn EdgesIds< In, Out >( src : In< EDGE_ID!() > ) -> Out< EDGE_ID!() >
//   where
//     In : IdsFromCollection,
//     Out : IdsCollection,
//   {
//     src.into()
//   }
//
//   /// Get edges with ids.
//   #[ inline ]
//   fn edges_ids< In, Out >( &self, src : In< EDGE_ID!() > ) -> Out< EDGE_ID!() >
//   where
//     In : IdsFromCollection,
//     Out : IdsCollection,
//   {
//     Self::EdgesIds( src )
//   }

}

/// Protected namespace of the module.
pub mod protected
{
  pub use super::orphan::*;
}

pub use protected::*;

/// Parented namespace of the module.
pub mod orphan
{
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::prelude::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  pub use super::private::
  {
    IdentityInterface,
    IdentityGenerableInterface,
    HasId,

    // From2,
    // Into2,
  };
}
