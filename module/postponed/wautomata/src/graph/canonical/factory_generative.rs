/// Internal namespace.
pub( crate ) mod private
{
  use crate::prelude::*;
  // use crate::canonical::*;
  use crate::canonical;
  use wtools::prelude::*;
  use core::fmt;
  use indexmap::IndexMap;
  use std::default::Default;
  // use core::ops::Deref;

  include!( "./factory_impl.rs" );

  ///
  /// Generative node factory.
  ///

  pub struct GenerativeNodeFactory< NodeId = crate::IdentityWithInt, EdgeId = crate::IdentityWithInt >
  where
    NodeId : IdentityInterface + HasIdGenerator< NodeId >,
    EdgeId : IdentityInterface + HasIdGenerator< EdgeId >,
    GenerativeNodeFactory< NodeId, EdgeId > : crate::GraphNodesNominalInterface,
  {
    /// Map id to node.
    pub id_to_node_map : IndexMap< NodeId, crate::canonical::Node< NodeId, EdgeId > >,
    /// Map id to edge.
    pub id_to_edge_map : IndexMap< EdgeId, crate::canonical::Edge< EdgeId, NodeId > >,
    /// Generator of node ids.
    pub _node_id_generator : NodeId::Generator,
    /// Generator of edge ids.
    pub _edge_id_generator : EdgeId::Generator,
  }

  // xxx : ?

  impl< NodeId, EdgeId >
  AsRef< GenerativeNodeFactory< NodeId, EdgeId > >
  for GenerativeNodeFactory< NodeId, EdgeId >
  where
    NodeId : IdentityInterface + HasIdGenerator< NodeId >,
    EdgeId : IdentityInterface + HasIdGenerator< EdgeId >,
  {
    fn as_ref( &self ) -> &Self
    {
      self
    }
  }

  //

  impl< NodeId, EdgeId > GraphNodesNominalInterface
  for GenerativeNodeFactory< NodeId, EdgeId >
  where
    NodeId : IdentityInterface + HasIdGenerator< NodeId >,
    EdgeId : IdentityInterface + HasIdGenerator< EdgeId >,
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
  for GenerativeNodeFactory< NodeId, EdgeId >
  where
    EdgeId : IdentityInterface + HasIdGenerator< EdgeId >,
    NodeId : IdentityInterface + HasIdGenerator< NodeId >,

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
  for GenerativeNodeFactory< NodeId, EdgeId >
  where
    NodeId : IdentityInterface + HasIdGenerator< NodeId >,
    EdgeId : IdentityInterface + HasIdGenerator< EdgeId >,

  {
    index!
    {
      nodes,
      nnodes,
    }

  }

  //

  impl< NodeId, EdgeId > GraphEdgesEnumerableInterface
  for GenerativeNodeFactory< NodeId, EdgeId >
  where
    EdgeId : IdentityInterface + HasIdGenerator< EdgeId >,
    NodeId : IdentityInterface + HasIdGenerator< NodeId >,

  {
    index!
    {
      edges,
      nedges,
    }
  }

  //

  impl< NodeId, EdgeId > GraphNodesExtendableInterface
  for GenerativeNodeFactory< NodeId, EdgeId >
  where
    NodeId : IdentityInterface + HasIdGenerator< NodeId >,
    EdgeId : IdentityInterface + HasIdGenerator< EdgeId >,

  {

    index!
    {
      node_mut,
      node_add_out_nodes,
      node_making,
    }

  }

  //

  impl< NodeId, EdgeId > GraphEdgesExtendableInterface
  for GenerativeNodeFactory< NodeId, EdgeId >
  where
    NodeId : IdentityInterface + HasIdGenerator< NodeId >,
    EdgeId : IdentityInterface + HasIdGenerator< EdgeId >,

  {

    index!
    {
      // _edge_id_generate,
      _edge_add,
    }

  }

  //

  impl< NodeId, EdgeId > fmt::Debug
  for GenerativeNodeFactory< NodeId, EdgeId >
  where
    NodeId : IdentityInterface + HasIdGenerator< NodeId >,
    EdgeId : IdentityInterface + HasIdGenerator< EdgeId >,
  {
    index!( fmt );
  }

  //

  impl< NodeId, EdgeId > From_0
  for GenerativeNodeFactory< NodeId, EdgeId >
  where
    NodeId : IdentityInterface + HasIdGenerator< NodeId >,
    EdgeId : IdentityInterface + HasIdGenerator< EdgeId >,
  {
    index!
    {
      // from_0,
    }
    fn from_0() -> Self
    {
      let id_to_node_map = IndexMap::new();
      let id_to_edge_map = IndexMap::new();
      let _node_id_generator = Default::default();
      let _edge_id_generator = Default::default();
      Self
      {
        id_to_node_map,
        id_to_edge_map,
        _node_id_generator,
        _edge_id_generator,
      }
    }
  }

}

//

crate::mod_interface!
{
  orphan use GenerativeNodeFactory;
}
