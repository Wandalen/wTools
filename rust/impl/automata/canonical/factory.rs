/// Internal namespace.
pub( crate ) mod private
{
  use crate::prelude::*;
  use crate::canonical::*;
  use wtools::prelude::*;
  use std::fmt;
  use indexmap::IndexMap;

  include!( "./factory_impl.rs" );

  impls!
  {

    ///
    /// Iterate output nodes of the node.
    ///

    fn node_add_out_nodes< IntoId1, IntoId2, Iter >
    (
      &mut self,
      node_id : IntoId1,
      out_nodes_iter : Iter,
    )
    where
      IntoId1 : Into< NODE_ID!() >,
      IntoId2 : Into< NODE_ID!() >,
      Iter : IntoIterator< Item = IntoId2 >,
      Iter::IntoIter : Clone,
    {

      let iter = out_nodes_iter.into_iter();
      let iter2 = iter.clone();

      #[ cfg( debug_assertions ) ]
      iter
      .map( | id |
      {
        let node = self.node( id );
      })
      .fold( (), | acc, e | () )
      ;

      let iter3 = iter2.into_iter()
      .map( | id |
      {
        let id = id.into();
        id
      })
      ;

      self.node_mut( node_id.into() ).extend( iter3 );
    }

    //

    fn out_nodes< 'a, 'b, IntoId >( &'a self, node_id : IntoId )
    ->
    Box< dyn Iterator< Item = NODE_ID!() > + 'b >
    where
      IntoId : Into< NODE_ID!() >,
      'a : 'b,
    {
      let node = self.node( node_id );
      let iterator
        : Box< dyn Iterator< Item = NODE_ID!() > >
        = Box::new( node.out_nodes.iter().cloned() );
      iterator
    }

  }

  ///
  /// Node factory.
  ///

  pub struct NodeFactory< NodeId = crate::IdentityWithInt, EdgeId = crate::IdentityWithInt, Kind = crate::NodeKindless >
  where
    NodeId : IdentityInterface,
    EdgeId : IdentityInterface,
    Kind : NodeKindInterface,
    NodeFactory< NodeId, EdgeId, Kind > : crate::NodeFactoryInterface,
  {
    /// Map id to node.
    pub id_to_node_map : IndexMap< NodeId, crate::canonical::Node< NodeId, Kind > >,
    /// Map id to edge.
    pub id_to_edge_map : IndexMap< EdgeId, crate::canonical::Edge< NodeId, EdgeId, Kind > >,
  }

  impl< NodeId, EdgeId, Kind > NodeFactory< NodeId, EdgeId, Kind >
  where
    NodeId : IdentityInterface,
    EdgeId : IdentityInterface,
    Kind : NodeKindInterface,
  {
  }

  //

  impl< NodeId, EdgeId, Kind > GraphNodesInterface
  for NodeFactory< NodeId, EdgeId, Kind >
  where
    NodeId : IdentityInterface,
    EdgeId : IdentityInterface,
    Kind : NodeKindInterface,
  {
    type NodeHandle = crate::canonical::Node< NodeId, Kind >;
    index!
    {
      node,
      out_nodes,
      nodes,
    }
  }

  //

  impl< NodeId, EdgeId, Kind > GraphEdgesInterface
  for NodeFactory< NodeId, EdgeId, Kind >
  where
    EdgeId : IdentityInterface,
    NodeId : IdentityInterface,
    Kind : NodeKindInterface,
  {
    type EdgeHandle = crate::canonical::Edge< EdgeId, NodeId, Kind >;
    index!
    {
      // edge,
      // out_edges,
      // edges,
    }
  }

  //

  impl< NodeId, EdgeId, Kind > GraphExtendableInterface
  for NodeFactory< NodeId, EdgeId, Kind >
  where
    NodeId : IdentityInterface,
    EdgeId : IdentityInterface,
    Kind : NodeKindInterface,
  {

    index!
    {
      node_making,
    }

  }

  //

  impl< NodeId, EdgeId, Kind > GraphEditableInterface
  for NodeFactory< NodeId, EdgeId, Kind >
  where
    NodeId : IdentityInterface,
    EdgeId : IdentityInterface,
    Kind : NodeKindInterface,
  {

    index!
    {
      node_mut,
      node_add_out_nodes,
    }

  }

  //

  impl< NodeId, EdgeId, Kind > NodeFactoryInterface
  for NodeFactory< NodeId, EdgeId, Kind >
  where
    NodeId : IdentityInterface,
    EdgeId : IdentityInterface,
    Kind : NodeKindInterface,
  {
    // type NodeHandle = crate::canonical::Node< NodeId, Kind >; /* xxx2 : remove? */
  }

  //

  impl< NodeId, EdgeId, Kind > fmt::Debug
  for NodeFactory< NodeId, EdgeId, Kind >
  where
    NodeId : IdentityInterface,
    EdgeId : IdentityInterface,
    Kind : NodeKindInterface,
  {
    index!( fmt );
  }

  //

  impl< NodeId, EdgeId, Kind > Make0
  for NodeFactory< NodeId, EdgeId, Kind >
  where
    NodeId : IdentityInterface,
    EdgeId : IdentityInterface,
    Kind : NodeKindInterface,
  {
    index!
    {
      make_0,
    }
  }

}

/// Protected namespace of the module.
pub mod protected
{
  pub use super::orphan::*;
}

pub use protected::*;

/// Parented namespace of the module.
pub mod orphan
{
  pub use super::exposed::*;
  pub use super::private::NodeFactory;
  // pub use super::private::NodeFactory2;
}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::prelude::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
}
