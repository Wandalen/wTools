/// Internal namespace.
pub( crate ) mod private
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

/// Protected namespace of the module.
pub mod protected
{
  pub use super::orphan::*;
}

pub use protected::*;

/// Parented namespace of the module.
pub mod orphan
{
  // // use super::private as i;
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
  // // use super::private as i;
  pub use super::prelude::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  // // use super::private as i;
}
