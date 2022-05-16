/// Internal namespace.
mod internal
{
  use crate::prelude::*;
  // use core::fmt::Debug;
  // use core::iter::Iterator;

  ///
  /// Implementation of depth-first search algorithm.
  ///

  pub trait DfsAlgorithm
  where
    Self : NodeBasicInterface,
  {
//
//     fn dfs( roots : Iterator< IdInterface > )
//     {
//
//     }
  }

}

/// Parented namespace of the module.
pub mod orphan
{
  // use super::internal as i;
  pub use super::exposed::*;
}

pub use orphan::*;

/// Exposed namespace of the module.
pub mod exposed
{
  // use super::internal as i;
  pub use super::prelude::*;
}

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  // use super::internal as i;
}
