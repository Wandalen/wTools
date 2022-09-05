/// Internal namespace.
pub( crate ) mod private
{
  // use crate::prelude::*;
  use core::fmt;
  use core::hash::Hash;
  use core::cmp::{ PartialEq, Eq };

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

}

//

crate::mod_interface!
{
  prelude use super::private::
  {
    IdentityInterface,
    IdentityGenerableInterface,
    HasId,
  };
}
