
/// Define a private namespace for all its items.
mod private
{

  use std::
  {
    hash::Hash,
    fmt,
  };

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
      Eq
    ,
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
      Eq
    ,
  {
  }

  /// Uniquely identify the node.
  pub trait NodeId : IdentityInterface
  {
  }

}

crate::mod_interface!
{
  own use { IdentityInterface, NodeId };
}
