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

      let node_id = node_id.into();
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
        // self.edge_making_for_nodes( node_id, id )
      })
      ;

      self.node_mut( node_id ).extend( iter3 );
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

    //

    fn out_edges< 'a, 'b, IntoId >( &'a self, node_id : IntoId )
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
  }

  impl< NodeId, EdgeId, Kind > NodeFactory< NodeId, EdgeId, Kind >
  where
    NodeId : IdentityInterface,
    EdgeId : IdentityInterface + IdentityGenerableInterface,
    Kind : NodeKindInterface,
  {
  }

  //

  impl< NodeId, EdgeId, Kind > GraphNodesInterface
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
      out_nodes,
      nodes,
    }
  }

  //

  impl< NodeId, EdgeId, Kind > GraphEdgesInterface
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
      edges,
      out_edges,
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

//   impl< NodeId, EdgeId, Kind > GraphEdgesExtendableInterface
//   for NodeFactory< NodeId, EdgeId, Kind >
//   where
//     NodeId : IdentityInterface,
//     EdgeId : IdentityInterface + IdentityGenerableInterface,
//     Kind : NodeKindInterface,
//   {
//
//     fn _edge_id_make_for( &mut self, node1 : NODE_ID!(), node2 : NODE_ID!() ) -> EDGE_ID!()
//     {
//       x
//     }
//
//     //
//
//     fn _edge_add( &mut self, edge : EDGE_ID!(), node1 : NODE_ID!(), node2 : NODE_ID!() )
//     {
//
//     }
//
//   }

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
