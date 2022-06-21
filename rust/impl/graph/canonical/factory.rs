/// Internal namespace.
pub( crate ) mod private
{
  use crate::prelude::*;
  // use crate::canonical::*;
  use crate::canonical;
  use wtools::prelude::*;
  use core::fmt;
  use indexmap::IndexMap;
  use core::ops::Deref;

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

  pub struct NodeFactory< 'it, NodeId = crate::IdentityWithInt, EdgeId = crate::IdentityWithInt, Kind = crate::NodeKindless >
  where
    NodeId : IdentityInterface,
    EdgeId : IdentityInterface + IdentityGenerableInterface,
    Kind : NodeKindInterface,
    NodeFactory< 'it, NodeId, EdgeId, Kind > : crate::NodeFactoryInterface,
  {
    /// Map id to node.
    pub id_to_node_map : IndexMap< NodeId, crate::canonical::Node< NodeId, EdgeId, Kind > >,
    /// Map id to edge.
    pub id_to_edge_map : IndexMap< EdgeId, crate::canonical::Edge< EdgeId, NodeId, Kind > >,
    /// Generator of edge ids.
    pub _current_edge_id : EdgeId,
    /// Phantom.
    _p : core::marker::PhantomData< &'it i32 >,
  }

  impl< 'it, NodeId, EdgeId, Kind > NodeFactory< 'it, NodeId, EdgeId, Kind >
  where
    NodeId : IdentityInterface,
    EdgeId : IdentityInterface + IdentityGenerableInterface,
    Kind : NodeKindInterface,
  {
  }

  //

  impl< 'it, NodeId, EdgeId, Kind >
  AsRef< NodeFactory< 'it, NodeId, EdgeId, Kind > >
  for NodeFactory< 'it, NodeId, EdgeId, Kind >
  where
    NodeId : IdentityInterface,
    EdgeId : IdentityInterface + IdentityGenerableInterface,
    Kind : NodeKindInterface,
  {
    fn as_ref( &self ) -> &Self
    {
      self
    }
  }

  //

  impl< 'it, NodeId, EdgeId, Kind > GraphNodesNominalInterface
  for NodeFactory< 'it, NodeId, EdgeId, Kind >
  where
    NodeId : IdentityInterface,
    EdgeId : IdentityInterface + IdentityGenerableInterface,
    Kind : NodeKindInterface,
    // Self : 'it,
  {
    type NodeHandle = crate::canonical::Node< NodeId, EdgeId, Kind >;
    index!
    {
      node,
      out_nodes_ids,
    }

    // type NodeId = NODE_ID!();
    // // type OutNodesIdsIterator = core::slice::Iter< 'it, Self::NodeId >;
    // type OutNodesIdsIterator = core::iter::Cloned< indexmap::set::Iter< 'it, Self::NodeId > >;
    // fn out_nodes_ids_2< Id >( &self, node_id : Id ) -> Self::OutNodesIdsIterator
    // where
    //   Id : Into< NODE_ID!() >,
    //   // Self : 'it,
    // {
    //   let node = self.node( node_id );
    //   let iterator = node.out_nodes.iter().cloned();
    //   // let iterator = node.out_nodes.iter();
    //   iterator
    // }

    // type NodeId = NODE_ID!();
    // type OutNodesIdsIterator = core::slice::Iter< 'it, NODE_ID!() >;
    // fn out_nodes_ids( self ) -> Self::OutNodesIdsIterator
    // {
    //   self.map.iter()
    // }

  }

  impl< 'it, NodeId, EdgeId, Kind >
  GraphNodesNominalInterface2< NodeFactory< 'it, NodeId, EdgeId, Kind > >
  for &'it NodeFactory< 'it, NodeId, EdgeId, Kind >
  where
    NodeId : IdentityInterface,
    EdgeId : IdentityInterface + IdentityGenerableInterface,
    Kind : NodeKindInterface,
    Self : Deref< Target = NodeFactory< 'it, NodeId, EdgeId, Kind > >,
  {

    type OutNodesIdsIterator = core::iter::Cloned< indexmap::set::Iter< 'it, NodeId > >;
    fn out_nodes_ids_2< Id >( self, node_id : Id ) -> Self::OutNodesIdsIterator
    where
      Id : Into< NodeId >
    {
      let node = self.node( node_id.into() );
      let iterator = node.out_nodes.iter().cloned();
      iterator
    }

    type RefNode = < NodeFactory< 'it, NodeId, EdgeId, Kind > as GraphNodesNominalInterface >::NodeHandle;

    type OutNodesIterator = core::iter::Map
    <
      indexmap::set::Iter< 'it, NodeId >,
      Box< dyn FnMut ( &NodeId ) -> ( NodeId, Self::RefNode ) >,
    >;

//     fn out_nodes_2< Id >( self, node_id : Id )
//     ->
//     Self::OutNodesIterator
//     where
//       Self : Sized,
//       Id : Into< NodeId >
//     {
//       self.out_nodes_ids_2( node_id ).map( | id |
//       {
//         13_i32
//         // ( id, self.node( id ) )
//       })
//     }

  }

  //

  impl< 'it, NodeId, EdgeId, Kind > GraphEdgesNominalInterface
  for NodeFactory< 'it, NodeId, EdgeId, Kind >
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

  impl< 'it, NodeId, EdgeId, Kind > GraphNodesEnumerableInterface
  for NodeFactory< 'it, NodeId, EdgeId, Kind >
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

  impl< 'it, NodeId, EdgeId, Kind > GraphEdgesEnumerableInterface
  for NodeFactory< 'it, NodeId, EdgeId, Kind >
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

  impl< 'it, NodeId, EdgeId, Kind > GraphNodesExtendableInterface
  for NodeFactory< 'it, NodeId, EdgeId, Kind >
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

  impl< 'it, NodeId, EdgeId, Kind > GraphEdgesExtendableInterface
  for NodeFactory< 'it, NodeId, EdgeId, Kind >
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

  impl< 'it, NodeId, EdgeId, Kind > NodeFactoryInterface
  for NodeFactory< 'it, NodeId, EdgeId, Kind >
  where
    NodeId : IdentityInterface,
    EdgeId : IdentityInterface + IdentityGenerableInterface,
    Kind : NodeKindInterface,
  {
    // type NodeHandle = crate::canonical::Node< NodeId, EdgeId, Kind >; /* xxx2 : remove? */
  }

  //

  impl< 'it, NodeId, EdgeId, Kind > fmt::Debug
  for NodeFactory< 'it, NodeId, EdgeId, Kind >
  where
    NodeId : IdentityInterface,
    EdgeId : IdentityInterface + IdentityGenerableInterface,
    Kind : NodeKindInterface,
  {
    index!( fmt );
  }

  //

  impl< 'it, NodeId, EdgeId, Kind > Make0
  for NodeFactory< 'it, NodeId, EdgeId, Kind >
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
