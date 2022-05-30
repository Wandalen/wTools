/// Internal namespace.
pub( crate ) mod private
{
  // use crate::prelude::*;

}

/// Edge interface.
pub mod edge;
/// Factory of nodes.
pub mod factory;
/// Interface of a graph.
pub mod graph;
/// Interface to identify an instance of somthging, for exampel a node.
pub mod identity;
/// Node interface.
pub mod node;
/// Node in a ref counted cell.
pub mod node_cell;

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::prelude::*;
  pub use super::edge::exposed::*;
  pub use super::factory::exposed::*;
  pub use super::graph::exposed::*;
  pub use super::node::exposed::*;
  pub use super::node_cell::exposed::*;
  pub use super::identity::exposed::*;
}

pub use exposed::*;

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  pub use super::edge::prelude::*;
  pub use super::factory::prelude::*;
  pub use super::graph::prelude::*;
  pub use super::node::prelude::*;
  pub use super::node_cell::prelude::*;
  pub use super::identity::prelude::*;
}
