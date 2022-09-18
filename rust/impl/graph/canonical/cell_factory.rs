/// Internal namespace.
pub( crate ) mod private
{
  use crate::prelude::*;
  use crate::canonical;
  use wtools::prelude::*;
  use indexmap::IndexMap;
  use core::fmt;

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

      let mut in_node = self.node( in_node_id ).borrow_mut();;

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
      let node = self.node( node_id ).borrow();
      let collected : Vec< NODE_ID!() > = node.out_nodes.iter().cloned().collect();
      let iterator : Box< dyn Iterator< Item = NODE_ID!() > > = Box::new( collected.into_iter() );
      iterator
    }

    //

    fn out_edges_ids< 'a, 'b, IntoId >( &'a self, node_id : IntoId )
    ->
    Box< dyn Iterator< Item = EDGE_ID!() > + 'b >
    where
      IntoId : Into< NODE_ID!() >,
      'a : 'b,
    {
      let node = self.node( node_id ).borrow();
      let collected : Vec< EDGE_ID!() > = node.out_edges.iter().cloned().collect();
      let iterator : Box< dyn Iterator< Item = EDGE_ID!() > > = Box::new( collected.into_iter() );
      iterator
    }

    //

  }

  ///
  /// Node factory.
  ///

  pub struct CellNodeFactory< NodeId = crate::IdentityWithInt, EdgeId = crate::IdentityWithInt, Kind = crate::NodeKindless >
  where
    NodeId : IdentityInterface,
    EdgeId : IdentityInterface + IdentityGenerableInterface,
    Kind : NodeKindInterface,
    CellNodeFactory< NodeId, EdgeId, Kind > : crate::NodeFactoryInterface,
  {
    /// Map id to node.
    pub id_to_node_map : IndexMap< NodeId, crate::NodeCell< canonical::Node< NodeId, EdgeId, Kind > > >,
    /// Map id to edge.
    pub id_to_edge_map : IndexMap< EdgeId, canonical::Edge< EdgeId, NodeId, Kind > >,
    /// Generator of edge ids.
    pub _current_edge_id : EdgeId,
  }

  //

  impl< NodeId, EdgeId, Kind > CellNodeFactory< NodeId, EdgeId, Kind >
  where
    NodeId : IdentityInterface,
    EdgeId : IdentityInterface + IdentityGenerableInterface,
    Kind : NodeKindInterface,
  {
  }

  //

  impl< NodeId, EdgeId, Kind > GraphNodesNominalInterface
  for CellNodeFactory< NodeId, EdgeId, Kind >
  where
    NodeId : IdentityInterface,
    EdgeId : IdentityInterface + IdentityGenerableInterface,
    Kind : NodeKindInterface,
  {
    type NodeHandle = crate::NodeCell< crate::canonical::Node< NodeId, EdgeId, Kind > >;
    index!
    {
      node,
      out_nodes_ids,
    }
  }

  //

  impl< NodeId, EdgeId, Kind > GraphEdgesNominalInterface
  for CellNodeFactory< NodeId, EdgeId, Kind >
  where
    NodeId : IdentityInterface,
    EdgeId : IdentityInterface + IdentityGenerableInterface,
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
  for CellNodeFactory< NodeId, EdgeId, Kind >
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

    //

    // type NodesIteratorItem = ( &'it NODE_ID!(), &'it < Self as GraphNodesNominalInterface >::NodeHandle );
    // type NodesIterator = std::collections::hash_map::Iter< 'it, NODE_ID!(), < Self as GraphNodesNominalInterface >::NodeHandle >;
    // fn nodes( self ) -> Self::NodesIterator
    // {
    //   self.map.iter()
    // }

  }

  //

  impl< NodeId, EdgeId, Kind > GraphEdgesEnumerableInterface
  for CellNodeFactory< NodeId, EdgeId, Kind >
  where
    NodeId : IdentityInterface,
    EdgeId : IdentityInterface + IdentityGenerableInterface,
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
  for CellNodeFactory< NodeId, EdgeId, Kind >
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
  for CellNodeFactory< NodeId, EdgeId, Kind >
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
  for CellNodeFactory< NodeId, EdgeId, Kind >
  where
    NodeId : IdentityInterface,
    EdgeId : IdentityInterface + IdentityGenerableInterface,
    Kind : NodeKindInterface,
  {
  }

  //

  impl< NodeId, EdgeId, Kind > fmt::Debug
  for CellNodeFactory< NodeId, EdgeId, Kind >
  where
    NodeId : IdentityInterface,
    EdgeId : IdentityInterface + IdentityGenerableInterface,
    Kind : NodeKindInterface,
  {
    index!( fmt );
  }

  //

  impl< NodeId, EdgeId, Kind > Make0
  for CellNodeFactory< NodeId, EdgeId, Kind >
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

//

crate::mod_interface!
{
}
