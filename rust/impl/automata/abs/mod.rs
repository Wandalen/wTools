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
/// Node in a ref counted cell.
pub mod node_cell;

/// Factory of nodes.
pub mod factory;

/// Exposed namespace of the module.
pub mod exposed
{
  // use super::internal as i;
  pub use super::prelude::*;

  pub use super::edge::exposed::*;
  pub use super::node::exposed::*;
  pub use super::node_cell::exposed::*;
  pub use super::identity::exposed::*;
  pub use super::factory::exposed::*;

}

pub use exposed::*;

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  // use super::internal as i;
  pub use super::edge::prelude::*;
  pub use super::node::prelude::*;
  pub use super::node_cell::prelude::*;
  pub use super::identity::prelude::*;
  pub use super::factory::prelude::*;

  // pub use i::GraphInterface;
}
