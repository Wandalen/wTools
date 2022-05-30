/// Internal namespace.
pub( crate ) mod private
{
  // use crate::prelude::*;
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

  pub trait IdentityIncInterface
  where
    Self : IdentityInterface,
  {
    /// Generate a new identity based on the current increasing it.
    fn inc( &self ) -> Self;
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
    IdentityIncInterface,
    HasId,
  };
}
