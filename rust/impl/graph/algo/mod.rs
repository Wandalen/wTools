/// Internal namespace.
pub( crate ) mod private
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
  // // use super::private as i;
  pub use super::prelude::*;

  pub use super::dfs::*;

}

pub use exposed::*;

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  // // use super::private as i;
  pub use super::dfs::*;

  // pub use super::private::GraphInterface;
}
