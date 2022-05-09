/// Internal namespace.
pub mod internal
{
  use crate::prelude::*;
  // use derive_tools::Default;
  // use std::collections::HashSet;

  ///
  /// No kind for edges.
  ///

  #[ derive( Debug, PartialEq, Copy, Clone ) ]
  pub enum EdgeKindNo
  {
    /// No kind.
    No,
  }

  impl Default for EdgeKindNo
  {
    fn default() -> Self
    {
      Self::No
    }
  }

  ///
  /// Canonical implementation of edge.
  ///

  #[ derive( Debug, PartialEq, Copy, Clone ) ]
  pub struct Edge< 'a, Node, Kind = EdgeKindNo >
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
pub mod own
{
  // use super::internal as i;
  pub use super::parented::*;
}

pub use own::*;

/// Parented namespace of the module.
pub mod parented
{
  use super::internal as i;
  pub use super::exposed::*;
  pub use i::EdgeKindNo;
  pub use i::Edge;
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
  // use super::internal as i;
}
