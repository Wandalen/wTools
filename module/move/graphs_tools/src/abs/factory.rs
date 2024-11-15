/// Define a private namespace for all its items.
mod private
{
  use crate::prelude::*;
  // use core::ops::Deref;

  macro_rules! NODE_ID
  {
    () => { < < Self as GraphNodesNominalInterface >::NodeHandle as HasId >::Id };
  }

  macro_rules! EDGE_ID
  {
    () => { < < Self as GraphEdgesNominalInterface >::EdgeHandle as HasId >::Id };
  }

  ///
  /// Graph which know how to iterate neighbourhood of a node and capable to convert id of a node into a node.
  ///

  pub trait GraphNodesNominalInterface
  {

    /// Handle of a node - entity representing a node or the node itself.
    /// It's not always possible to operate a node directly, for example it it has to be wrapped by cell ref. For that use NodeHandle.
    /// Otherwise NodeHandle could be &Node.
    type NodeHandle : NodeBasicInterface;

    // /// Convert argument into node id.
    // #[ allow( non_snake_case ) ]
    // #[ inline ]
    // fn NodeId< Id >( id : Id ) -> NODE_ID!()
    // where
    //   Id : Into< NODE_ID!() >
    // {
    //   id.into()
    // }

    /// Convert argument into node id.
    #[ inline ]
    fn node_id< Id >( &self, id : Id ) -> NODE_ID!()
    where
      Id : Into< NODE_ID!() >
    {
      id.into()
    }

    /// Get node with id.
    fn node< Id >( &self, id : Id ) -> &Self::NodeHandle
    where
      Id : Into< NODE_ID!() >
    ;

    // type NodeId;
    // // type OutNodesIdsIterator : Iterator< Item = ( &'it < Graph::NodeHandle as HasId >::Id, &'it Graph::NodeHandle ) >;
    // type OutNodesIdsIterator : Iterator< Item = Self::NodeId >;
    // /// Iterate over all nodes.
    // fn out_nodes_ids< Id >( &self, node_id : Id ) -> Self::OutNodesIdsIterator
    // where
    //   Id : Into< NODE_ID!() >
    // ;

    // type NodeId;
    // type OutNodesIdsIterator : Iterator< Item = Self::NodeId >;
    // /// Iterate over all nodes.
    // fn out_nodes_ids_2< Id >( &self, node_id : Id ) -> Self::OutNodesIdsIterator
    // where
    //   Id : Into< NODE_ID!() >
    // ;

    /// Iterate over neighbourhood of the node. Callback gets ids of nodes in neighbourhood of a picked node.
    fn out_nodes_ids< 'a, 'b, Id >( &'a self, node_id : Id )
    ->
    Box< dyn Iterator< Item = NODE_ID!() > + 'b >
    where
      Id : Into< NODE_ID!() >,
      'a : 'b,
    ;

    /// Iterate over neighbourhood of the node. Callback gets ids and reference on itself of nodes in neighbourhood of a picked node.
    fn out_nodes< 'a, 'b, Id >( &'a self, node_id : Id )
    ->
    Box< dyn Iterator< Item = ( NODE_ID!(), &< Self as GraphNodesNominalInterface >::NodeHandle ) > + 'b >
    where
      Id : Into< NODE_ID!() >,
      'a : 'b,
    {
      Box::new( self.out_nodes_ids( node_id ).map( | id |
      {
        ( id, self.node( id ) )
      }))
    }

  }

//   ///
//   /// Graph which know how to iterate neighbourhood of a node and capable to convert id of a node into a node.
//   ///
//
//   pub trait GraphNodesNominalInterface2< T >
//   where
//     Self : Deref< Target = T >,
//     T : GraphNodesNominalInterface,
//   {
//
//     /// Iterator to iterate ids of nodes.
//     type OutNodesIdsIterator : Iterator< Item = < < T as GraphNodesNominalInterface >::NodeHandle as HasId >::Id >;
//     /// Iterate over all nodes.
//     fn out_nodes_ids_2< Id >( self, node_id : Id ) -> Self::OutNodesIdsIterator
//     where
//       Id : Into< < < T as GraphNodesNominalInterface >::NodeHandle as HasId >::Id >
//     ;
//
//     /// Reference on a node handle.
//     type RefNode;
//     /// Iterator to iterate pairs id - node
//     type OutNodesIterator : Iterator< Item = ( < < T as GraphNodesNominalInterface >::NodeHandle as HasId >::Id, Self::RefNode ) >;
//
//     // /// Iterate over neighbourhood of the node. Callback gets ids and reference on itself of nodes in neighbourhood of a picked node.
//     // fn out_nodes_2< Id >( self, node_id : Id )
//     // ->
//     // Self::OutNodesIdsIterator
//     // where
//     //   Self : Sized,
//     //   Id : Into< < < T as GraphNodesNominalInterface >::NodeHandle as HasId >::Id >
//     // ;
//
//   }

  ///
  /// Graph which know how to iterate neighbourhood of a node and capable to convert id of a node into a node.
  ///

  pub trait GraphEdgesNominalInterface
  where
    Self : GraphNodesNominalInterface,
  {

    /// Handle of an edge - entity representing an edge or the edge itself.
    /// It's not always possible to operate an edge directly, for example it it has to be wrapped by cell ref. For that use NodeHandle.
    /// Otherwise EdgeHandle could be &Node.
    type EdgeHandle : EdgeBasicInterface;

    // /// Convert argument into edge id.
    // #[ allow( non_snake_case ) ]
    // #[ inline ]
    // fn EdgeId< Id >( id : Id ) -> EDGE_ID!()
    // where
    //   Id : Into< EDGE_ID!() >
    // {
    //   id.into()
    // }

    /// Convert argument into edge id.
    #[ inline ]
    fn edge_id< Id >( &self, id : Id ) -> EDGE_ID!()
    where
      Id : Into< EDGE_ID!() >
    {
      id.into()
      // Self::EdgeId( id )
    }

    /// Get edge with id.
    fn edge< Id >( &self, id : Id ) -> &Self::EdgeHandle
    where
      Id : Into< EDGE_ID!() >
    ;

    /// Iterate over output edges of the node. Callback gets ids of nodes in neighbourhood of a picked node.
    fn out_edges_ids< 'a, 'b, IntoId >( &'a self, node_id : IntoId )
    ->
    Box< dyn Iterator< Item = EDGE_ID!() > + 'b >
    where
      IntoId : Into< NODE_ID!() >,
      'a : 'b,
    ;

    /// Iterate over output edges of the node. Callback gets ids and references of edges in neighbourhood of a picked node.
    fn out_edges< 'a, 'b, IntoId >( &'a self, node_id : IntoId )
    ->
    Box< dyn Iterator< Item = ( EDGE_ID!(), &< Self as GraphEdgesNominalInterface >::EdgeHandle ) > + 'b >
    where
      IntoId : Into< NODE_ID!() >,
      'a : 'b,
    {
      Box::new( self.out_edges_ids( node_id ).map( | id |
      {
        ( id, self.edge( id ) )
      }))
    }

  }

//   /// Into iterator of nodes.
//
//   pub trait IntoIteratorOfNodes
//   {
//     type NodesIteratorItem;
//     type NodesIterator : Iterator< Item = Self::NodesIteratorItem >;
//     // /// Iterate over all nodes.
//     // fn nodes( self ) -> Self::NodesIterator;
//   }
//
//   //
//
//   impl< 'it, Graph > IntoIteratorOfNodes
//   for &'it Graph
//   where
//     Graph : GraphNodesNominalInterface,
//   {
//     type NodesIteratorItem = ( &'it < Graph::NodeHandle as HasId >::Id, &'it Graph::NodeHandle );
//     type NodesIterator = std::collections::hash_map::Iter< 'it, < Graph::NodeHandle as HasId >::Id, Graph::NodeHandle >;
//     // fn nodes( self ) -> Self::NodesIterator
//     // {
//     //   self.map.iter()
//     // }
//   }

  ///
  /// Graph nodes of which is possible to enumerate.
  ///

  // pub trait GraphNodesEnumerableInterface< 'it, 'it2, It >
  pub trait GraphNodesEnumerableInterface
  where
    Self : GraphNodesNominalInterface,
    // It : Iterator< Item = &'it2 ( NODE_ID!(), &'it < Self as GraphNodesNominalInterface >::NodeHandle ) >,
    // < Self as GraphNodesNominalInterface >::NodeHandle : 'it,
    // 'it : 'it2,
  {

    // type NodesIteratorItem;
    // // type NodesIterator : Iterator< Item = ( &'it < Graph::NodeHandle as HasId >::Id, &'it Graph::NodeHandle ) >;
    // type NodesIterator : Iterator< Item = Self::NodesIteratorItem >;
    // /// Iterate over all nodes.
    // fn nodes( self ) -> Self::NodesIterator;

    /// Iterate over all nodes.
    fn nodes< 'a, 'b >( &'a self )
    ->
    Box< dyn Iterator< Item = ( NODE_ID!(), &< Self as GraphNodesNominalInterface >::NodeHandle ) > + 'b >
    where
      'a : 'b,
    ;

    /// Number of nodes. Order of the graph.
    fn nnodes( &self ) -> usize
    {
      self.nodes().count()
    }

  }

  ///
  /// Graph edges of which is possible to enumerate.
  ///

  pub trait GraphEdgesEnumerableInterface
  where
    Self :
      GraphNodesNominalInterface +
      GraphEdgesNominalInterface +
    ,
  {

    /// Iterate over all edges.
    fn edges< 'a, 'b >( &'a self )
    ->
    Box< dyn Iterator< Item = ( EDGE_ID!(), &< Self as GraphEdgesNominalInterface >::EdgeHandle ) > + 'b >
    where
      'a : 'b,
    ;

    /// Number of edges. Size of the graph.
    fn nedges( &self ) -> usize
    {
      self.edges().count()
    }

  }

  ///
  /// Graph interface which allow to add more nodes. Know nothing about edges.
  ///

  pub trait GraphNodesExtendableInterface
  where
    Self :
      GraphNodesNominalInterface +
    ,
  {

    /// Get node with id mutably.
    fn node_mut< Id >( &mut self, id : Id ) -> &mut Self::NodeHandle
    where
      Id : Into< NODE_ID!() >
    ;

    /// Add out nodes to the node.
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
    ;

    /// Add out edges to the node.
    fn node_add_out_node< IntoId1, IntoId2 >
    (
      &mut self,
      node_id : IntoId1,
      out_node_id : IntoId2,
    )
    where
      IntoId1 : Into< NODE_ID!() >,
      IntoId1 : Clone,
      IntoId2 : Into< NODE_ID!() >,
      IntoId2 : Clone,
    {
      self.node_add_out_nodes( node_id, core::iter::once( out_node_id ) );
    }

    /// Either make new or get existing node.
    fn node_making< Id >( &mut self, id : Id ) -> NODE_ID!()
    where
      Id : Into< NODE_ID!() >
    ;

    /// Make edges.
    fn make_with_edge_list< IntoIter, Id >( &mut self, into_iter : IntoIter )
    where
      Id : Into< NODE_ID!() >,
      IntoIter : IntoIterator< Item = Id >,
      IntoIter::IntoIter : core::iter::ExactSizeIterator< Item = Id >,
    {
      // use wtools::iter::prelude::*;
      use crate::iter::prelude::*;
      let iter = into_iter.into_iter();
      debug_assert_eq!( iter.len() % 2, 0 );
      for mut chunk in &iter.chunks( 2 )
      {
        let id1 = chunk.next().unwrap().into();
        let id2 = chunk.next().unwrap().into();
        self.node_making( id1 );
        self.node_making( id2 );
        self.node_add_out_node( id1, id2 );
      }

    }

  }

  ///
  /// Graph interface which allow to add more edges.
  ///

  pub trait GraphEdgesExtendableInterface
  where
    Self :
      GraphNodesNominalInterface +
      GraphEdgesNominalInterface +
      GraphNodesExtendableInterface +
    ,
  {

    // /// Either make new or get existing edge for specified nodes.
    // fn _edge_id_generate( &mut self, node1 : NODE_ID!(), node2 : NODE_ID!() ) -> EDGE_ID!();

    /// Either make new or get existing edge for specified nodes.
    fn _edge_add( &mut self, node1 : NODE_ID!(), node2 : NODE_ID!() ) -> EDGE_ID!();

    /// Either make new or get existing edge for specified nodes.
    #[ inline ]
    fn _edge_make_for_nodes< IntoNodeId1, IntoNodeId2 >( &mut self, node1 : IntoNodeId1, node2 : IntoNodeId2 ) -> EDGE_ID!()
    where
      IntoNodeId1 : Into< NODE_ID!() >,
      IntoNodeId2 : Into< NODE_ID!() >,
    {
      let node1 = node1.into();
      let node2 = node2.into();
      // let edge = self._edge_id_generate( node1, node2 );
      let edge = self._edge_add( node1, node2 );
      edge
    }

  }

//   ///
//   /// Graph nodes of which has a kind.
//   ///
//
//   pub trait GraphNodesKindGetterInterface
//   where
//     Self : GraphNodesNominalInterface,
//   {
//     /// Enumerate kinds of the node.
//     type NodeKind : crate::NodeKindInterface;
//     /// Get kind of the node.
//     fn node_kind( &self, node_id : NODE_ID!() ) -> Self::NodeKind;
//   }
//
//   ///
//   /// Graph nodes of which has a kind.
//   ///
//
//   pub trait GraphEdgesKindGetterInterface
//   where
//     Self :
//       GraphNodesNominalInterface +
//       GraphEdgesNominalInterface +
//     ,
//   {
//     /// Enumerate kinds of the node.
//     type EdgeKind : crate::EdgeKindInterface;
//     /// Get kind of the node.
//     fn edge_kind( &self, edge_id : EDGE_ID!() ) -> Self::EdgeKind;
//   }

}

//

crate::mod_interface!
{
  prelude use super::private::
  {
    GraphNodesNominalInterface,
    // GraphNodesNominalInterface2,
    GraphEdgesNominalInterface,
    GraphNodesEnumerableInterface,
    GraphEdgesEnumerableInterface,
    GraphNodesExtendableInterface,
    GraphEdgesExtendableInterface,
    // GraphNodesKindGetterInterface,
    // GraphEdgesKindGetterInterface,
  };
}
