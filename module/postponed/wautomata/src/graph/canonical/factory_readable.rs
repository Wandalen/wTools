/// Internal namespace.
mod private
{
  use crate::prelude::*;
  // use crate::canonical::*;
  // use crate::canonical;
  use wtools::prelude::*;
  use core::fmt;
  use indexmap::IndexMap;
  // use std::default::Default;
  // use core::ops::Deref;

  include!( "./factory_impl.rs" );

  ///
  /// Radable node factory.
  ///

  pub struct ReadableNodeFactory< NodeId = crate::IdentityWithInt, EdgeId = crate::IdentityWithInt >
  where
    NodeId : IdentityInterface,
    EdgeId : IdentityInterface,
    ReadableNodeFactory< NodeId, EdgeId > : crate::GraphNodesNominalInterface,
  {
    /// Map id to node.
    pub id_to_node_map : IndexMap< NodeId, crate::canonical::Node< NodeId, EdgeId > >,
    /// Map id to edge.
    pub id_to_edge_map : IndexMap< EdgeId, crate::canonical::Edge< EdgeId, NodeId > >,
  }

  //

  impl< NodeId, EdgeId > GraphNodesNominalInterface
  for ReadableNodeFactory< NodeId, EdgeId >
  where
    NodeId : IdentityInterface,
    EdgeId : IdentityInterface,
  {
    type NodeHandle = crate::canonical::Node< NodeId, EdgeId >;
    index!
    {
      node,
      out_nodes_ids,
    }

  }

  //

  impl< NodeId, EdgeId > GraphEdgesNominalInterface
  for ReadableNodeFactory< NodeId, EdgeId >
  where
    EdgeId : IdentityInterface,
    NodeId : IdentityInterface,

  {
    type EdgeHandle = crate::canonical::Edge< EdgeId, NodeId >;
    index!
    {
      edge,
      out_edges_ids,
    }
  }

  //

  impl< NodeId, EdgeId > GraphNodesEnumerableInterface
  for ReadableNodeFactory< NodeId, EdgeId >
  where
    NodeId : IdentityInterface,
    EdgeId : IdentityInterface,

  {
    index!
    {
      nodes,
      nnodes,
    }

  }

  //

  impl< NodeId, EdgeId > GraphEdgesEnumerableInterface
  for ReadableNodeFactory< NodeId, EdgeId >
  where
    EdgeId : IdentityInterface,
    NodeId : IdentityInterface,

  {
    index!
    {
      edges,
      nedges,
    }
  }

  //

//   impl< NodeId, EdgeId > GraphNodesNominalInterface
//   for ReadableNodeFactory< NodeId, EdgeId >
//   where
//     NodeId : IdentityInterface,
//     EdgeId : IdentityInterface,
//   {
//   }
//
//   //
//
//   impl< NodeId, EdgeId > GraphNodesNominalInterface
//   for GenerativeNodeFactory< NodeId, EdgeId >
//   where
//     NodeId : IdentityInterface + HasIdGenerator< NodeId >,
//     EdgeId : IdentityInterface + HasIdGenerator< EdgeId >,
//   {
//   }

  //

  impl< NodeId, EdgeId > fmt::Debug
  for ReadableNodeFactory< NodeId, EdgeId >
  where
    NodeId : IdentityInterface,
    EdgeId : IdentityInterface,
  {
    index!( fmt );
  }

  //

  impl< NodeId, EdgeId > From_0
  for ReadableNodeFactory< NodeId, EdgeId >
  where
    NodeId : IdentityInterface,
    EdgeId : IdentityInterface,
  {
    index!
    {
      // from_0,
    }

    fn from_0() -> Self
    {
      let id_to_node_map = IndexMap::new();
      let id_to_edge_map = IndexMap::new();
      Self
      {
        id_to_node_map,
        id_to_edge_map,
      }
    }

  }

}

//

crate::mod_interface!
{
  orphan use ReadableNodeFactory;
}
