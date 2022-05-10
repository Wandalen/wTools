/// Internal namespace.
pub mod internal
{
  use crate::prelude::*;
  // use std::collections::HashSet;
  use std::collections::HashMap;
  // use core::fmt::Debug;
  // use std::cmp::Eq;
  // use core::hash::{ Hash, Hasher };

  ///
  /// Node factory.
  ///

  #[ derive( Debug ) ]
  pub struct NodeFactory< Node, Id >
  where
    Node : NodeConstructableInterface< Id = Id >,
    Id : IdentityInterface,
  {
    /// Map id to node.
    pub id_to_node_map : HashMap< Id, Node >,
    // /// Map name to node.
    // pub name_to_node_map : HashMap< Name, Node >,
  }

  impl< Node, Id > NodeFactory< Node, Id >
  where
    Node : NodeConstructableInterface< Id = Id >,
    Id : IdentityInterface,
  {

    /// Constructor.
    pub fn make() -> Self
    {
      let id_to_node_map = HashMap::new();
      // let name_to_node_map = HshMap::new();
      Self
      {
        id_to_node_map,
        // name_to_node_map
      }
    }

    /// New node.
    pub fn node_make() -> Node
    {
      Node::make()
    }

    // /// New node with name.
    // pub fn node_make_wiwth_name( name : Name ) -> Node;

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
  pub use i::NodeFactory;
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
