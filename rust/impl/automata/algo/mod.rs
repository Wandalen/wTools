/// Internal namespace.
pub mod internal
{
  // use crate::prelude::*;

//   ///
//   /// Generic description of types of a graph.
//   ///
//
//   pub trait GraphInterface
//   {
//   }

}

/// Depth-first search.
pub mod dfs;

/// Exposed namespace of the module.
pub mod exposed
{
  // use super::internal as i;
  pub use super::prelude::*;

  pub use super::dfs::*;

}

pub use exposed::*;

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  // use super::internal as i;
  pub use super::dfs::*;

  // pub use i::GraphInterface;
}
