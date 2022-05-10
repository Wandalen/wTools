/// Internal namespace.
pub mod internal
{
  // use crate::prelude::*;
  use core::fmt::Debug;
  use std::cmp::{ PartialEq, Eq };

  ///
  /// Interface to identify an instance of somthging, for exampel a node.
  ///

  pub trait IdentityInterface
  where
    Self :
      'static +
      Copy +
      Debug +
      PartialEq +
      Eq +
  {
  }

  impl< T > IdentityInterface for T
  where
    T :
      'static +
      Copy +
      Debug +
      PartialEq +
      Eq +
    ,
  {
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


  ///
  /// Identify an instance by its location in memory.
  ///

  #[ derive( Debug, PartialEq, Eq, Copy, Clone, Hash ) ]
  pub struct IdentityByPointer( usize );

  impl IdentityByPointer
  {

    /// Construct from an arbitrary reference.
    #[ inline ]
    pub fn make< T >( src : &T ) -> Self
    {
      // Safety : it differentiate different instances.
      let ptr = unsafe
      {
        std::mem::transmute::< _, usize >( src )
      };
      Self( ptr )
    }

  }

  #[ test ]
  fn identity_implemented_for_identity_by_pointer()
  {
    let src = IdentityByPointer::make( &13 );
    check( src );
    fn check< T : IdentityInterface >( _ : T ){}
  }

}

/// Own namespace of the module.
pub mod own
{
  // use super::internal as i;
  pub use super::parented::*;
}

pub use own::*;

/// Parented namespace of the module.
pub mod parented
{
  // use super::internal as i;
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
  use super::internal as i;
  pub use i::IdentityByPointer;
  pub use super::prelude::*;
}

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  use super::internal as i;
  pub use i::IdentityInterface;
  pub use i::HasId;
  // pub use i::EqById;
}
