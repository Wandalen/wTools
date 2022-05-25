/// Internal namespace.
mod internal
{
  use crate::prelude::*;

  ///
  /// Canonical implementation of edge.
  ///

  #[ derive( Debug, PartialEq, Copy, Clone ) ]
  pub struct Edge< 'a, Node, Kind = crate::EdgeKindless >
  where
    Node : NodeBasicInterface,
    Kind : EdgeKindInterface,
  {
    /// Input node.
    pub in_node : &'a Node,
    /// Output node.
    pub out_node : &'a Node,
    /// Kind of the edge.
    pub kind : Kind,
  }

  impl< 'a, Node, Kind > EdgeInterface
  for Edge< 'a, Node, Kind >
  where
    Node : NodeBasicInterface,
    Kind : EdgeKindInterface,
  {
  }

}

/// Own namespace of the module.
pub mod protected
{
  // use super::internal as i;
  pub use super::orphan::*;
}

pub use protected::*;

/// Parented namespace of the module.
pub mod orphan
{
  use super::internal as i;
  pub use super::exposed::*;
  pub use i::Edge;
}

/// Exposed namespace of the module.
pub mod exposed
{
  // use super::internal as i;
  pub use super::prelude::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  // use super::internal as i;
}
