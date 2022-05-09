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

/// Edge interface.
pub mod edge;
/// Interface to identify an instance of somthging, for exampel a node.
pub mod identity;
/// Node interface.
pub mod node;

/// Factory of nodes.
pub mod factory;

/// Exposed namespace of the module.
pub mod exposed
{
  // use super::internal as i;
  pub use super::prelude::*;

  pub use super::edge::*;
  pub use super::node::*;
  pub use super::identity::*;
  pub use super::factory::*;

}

pub use exposed::*;

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  // use super::internal as i;
  pub use super::edge::*;
  pub use super::node::*;
  pub use super::identity::*;
  pub use super::factory::*;

  // pub use i::GraphInterface;
}
