/// Internal namespace.
pub( crate ) mod private
{
  use crate::prelude::*;
  // use crate::canonical::*;
  use crate::canonical;
  use wtools::prelude::*;
  use core::fmt;
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
      in_node_id : IntoId1,
      out_nodes_iter : Iter,
    )
    where
      IntoId1 : Into< NODE_ID!() >,
      IntoId2 : Into< NODE_ID!() >,
      Iter : IntoIterator< Item = IntoId2 >,
      Iter::IntoIter : Clone,
    {

      let in_node_id = in_node_id.into();
      let iter = out_nodes_iter.into_iter();

      let out_ids : Vec< _ > = iter
      .map( | out_node_id |
      {
        let out_node_id = out_node_id.into();
        #[ cfg( debug_assertions ) ]
        let node = self.node( out_node_id );
        let out_edge_id = self._edge_make_for_nodes( in_node_id, out_node_id );
        ( out_edge_id, out_node_id )
      })
      .collect()
      ;

      let in_node = self.node_mut( in_node_id );

      for out_id in out_ids
      {
        in_node.out_edges.insert( out_id.0 );
        in_node.out_nodes.insert( out_id.1 );
      }

    }

    //

    fn out_nodes_ids< 'a, 'b, IntoId >( &'a self, node_id : IntoId )
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

    //

    // fn out_edges< 'a, 'b, IntoId >( &'a self, node_id : IntoId )
    // ->
    // Box< dyn Iterator< Item = ( &NODE_ID!(), &< Self as GraphNodesNominalInterface >::NodeHandle ) > + 'b >
    // where
    //   IntoId : Into< NODE_ID!() >,
    //   'a : 'b,
    // {
    //   let node = self.node( node_id );
    //   let iterator
    //     : Box< dyn Iterator< Item = ( &NODE_ID!(), &< Self as GraphNodesNominalInterface >::NodeHandle ) > >
    //     = Box::new( node.out_edges.iter().map( | el |
    //     {
    //       self.node(  )
    //     }));
    //   iterator
    // }

    fn out_edges_ids< 'a, 'b, IntoId >( &'a self, node_id : IntoId )
    ->
    Box< dyn Iterator< Item = EDGE_ID!() > + 'b >
    where
      IntoId : Into< NODE_ID!() >,
      'a : 'b,
    {
      let node = self.node( node_id );
      let iterator
        : Box< dyn Iterator< Item = EDGE_ID!() > >
        = Box::new( node.out_edges.iter().cloned() );
      iterator
    }

  }

  ///
  /// Node factory.
  ///

  pub struct NodeFactory< NodeId = crate::IdentityWithInt, EdgeId = crate::IdentityWithInt, Kind = crate::NodeKindless >
  where
    NodeId : IdentityInterface,
    EdgeId : IdentityInterface + IdentityGenerableInterface,
    Kind : NodeKindInterface,
    NodeFactory< NodeId, EdgeId, Kind > : crate::NodeFactoryInterface,
  {
    /// Map id to node.
    pub id_to_node_map : IndexMap< NodeId, crate::canonical::Node< NodeId, EdgeId, Kind > >,
    /// Map id to edge.
    pub id_to_edge_map : IndexMap< EdgeId, crate::canonical::Edge< EdgeId, NodeId, Kind > >,
    /// Generator of edge ids.
    pub _current_edge_id : EdgeId,
  }

  impl< NodeId, EdgeId, Kind > NodeFactory< NodeId, EdgeId, Kind >
  where
    NodeId : IdentityInterface,
    EdgeId : IdentityInterface + IdentityGenerableInterface,
    Kind : NodeKindInterface,
  {
  }

  //

  impl< NodeId, EdgeId, Kind > GraphNodesNominalInterface
  for NodeFactory< NodeId, EdgeId, Kind >
  where
    NodeId : IdentityInterface,
    EdgeId : IdentityInterface + IdentityGenerableInterface,
    Kind : NodeKindInterface,
  {
    type NodeHandle = crate::canonical::Node< NodeId, EdgeId, Kind >;
    index!
    {
      node,
      out_nodes_ids,
    }
  }

  //

  impl< NodeId, EdgeId, Kind > GraphEdgesNominalInterface
  for NodeFactory< NodeId, EdgeId, Kind >
  where
    EdgeId : IdentityInterface + IdentityGenerableInterface,
    NodeId : IdentityInterface,
    Kind : NodeKindInterface,
  {
    type EdgeHandle = crate::canonical::Edge< EdgeId, NodeId, Kind >;
    index!
    {
      edge,
      out_edges_ids,
    }
  }

  //

  impl< NodeId, EdgeId, Kind > GraphNodesEnumerableInterface
  for NodeFactory< NodeId, EdgeId, Kind >
  where
    NodeId : IdentityInterface,
    EdgeId : IdentityInterface + IdentityGenerableInterface,
    Kind : NodeKindInterface,
  {
    index!
    {
      nodes,
      nnodes,
    }
  }

  //

  impl< NodeId, EdgeId, Kind > GraphEdgesEnumerableInterface
  for NodeFactory< NodeId, EdgeId, Kind >
  where
    EdgeId : IdentityInterface + IdentityGenerableInterface,
    NodeId : IdentityInterface,
    Kind : NodeKindInterface,
  {
    index!
    {
      edges,
      nedges,
    }
  }

  //

  impl< NodeId, EdgeId, Kind > GraphNodesExtendableInterface
  for NodeFactory< NodeId, EdgeId, Kind >
  where
    NodeId : IdentityInterface,
    EdgeId : IdentityInterface + IdentityGenerableInterface,
    Kind : NodeKindInterface,
  {

    index!
    {
      node_mut,
      node_add_out_nodes,
      node_making,
    }

  }

  //

  impl< NodeId, EdgeId, Kind > GraphEdgesExtendableInterface
  for NodeFactory< NodeId, EdgeId, Kind >
  where
    NodeId : IdentityInterface,
    EdgeId : IdentityInterface + IdentityGenerableInterface,
    Kind : NodeKindInterface,
  {

    index!
    {
      _edge_id_generate,
      _edge_add,
    }

  }

  //

  impl< NodeId, EdgeId, Kind > NodeFactoryInterface
  for NodeFactory< NodeId, EdgeId, Kind >
  where
    NodeId : IdentityInterface,
    EdgeId : IdentityInterface + IdentityGenerableInterface,
    Kind : NodeKindInterface,
  {
    // type NodeHandle = crate::canonical::Node< NodeId, EdgeId, Kind >; /* xxx2 : remove? */
  }

  //

  impl< NodeId, EdgeId, Kind > fmt::Debug
  for NodeFactory< NodeId, EdgeId, Kind >
  where
    NodeId : IdentityInterface,
    EdgeId : IdentityInterface + IdentityGenerableInterface,
    Kind : NodeKindInterface,
  {
    index!( fmt );
  }

  //

  impl< NodeId, EdgeId, Kind > Make0
  for NodeFactory< NodeId, EdgeId, Kind >
  where
    NodeId : IdentityInterface,
    EdgeId : IdentityInterface + IdentityGenerableInterface,
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
