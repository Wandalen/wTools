/// Internal namespace.
pub mod internal
{
  // use crate::prelude::*;
  // use std::collections::HashSet;
  // use std::collections::HashMap;
  use core::fmt::Debug;
  use std::cmp::{ PartialEq, Eq };
  // use core::hash::{ Hash, Hasher };

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
  // use super::internal as i;
  pub use super::prelude::*;
}

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  use super::internal as i;
  pub use i::IdentityInterface;
}
