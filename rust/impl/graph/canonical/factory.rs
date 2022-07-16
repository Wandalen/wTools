/// Internal namespace.
pub( crate ) mod private
{
  use crate::prelude::*;
  // use crate::canonical::*;
  use crate::canonical;
  use wtools::prelude::*;
  use core::fmt;
  use indexmap::IndexMap;
  // use core::ops::Deref;

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
    // /// Phantom.
    // _p : core::marker::PhantomData< &'it i32 >,
  }

  impl< NodeId, EdgeId, Kind > NodeFactory< NodeId, EdgeId, Kind >
  where
    NodeId : IdentityInterface,
    EdgeId : IdentityInterface + IdentityGenerableInterface,
    Kind : NodeKindInterface,
  {
  }

  //

  impl< NodeId, EdgeId, Kind >
  AsRef< NodeFactory< NodeId, EdgeId, Kind > >
  for NodeFactory< NodeId, EdgeId, Kind >
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

  impl< NodeId, EdgeId, Kind > GraphNodesNominalInterface
  for NodeFactory< NodeId, EdgeId, Kind >
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
    // // type OutNodesIdsIterator = core::slice::Iter< Self::NodeId >;
    // type OutNodesIdsIterator = core::iter::Cloned< indexmap::set::Iter< Self::NodeId > >;
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
    // type OutNodesIdsIterator = core::slice::Iter< NODE_ID!() >;
    // fn out_nodes_ids( self ) -> Self::OutNodesIdsIterator
    // {
    //   self.map.iter()
    // }

  }

  //

//   struct _IterateGraphIdsAndNodes< NodeId, EdgeId, Kind >
//   where
//     NodeId : IdentityInterface,
//     EdgeId : IdentityInterface + IdentityGenerableInterface,
//     Kind : NodeKindInterface,
//   {
//     container : &'it mut NodeFactory< NodeId, EdgeId, Kind >,
//     node_id : NodeId,
//     // it : dyn Iterator< Item = ( NodeId, < NodeFactory< NodeId, EdgeId, Kind > as GraphNodesNominalInterface >::NodeHandle ) >,
//     _it : [ i32 ; 6 ],
//   }
//
//   impl< NodeId, EdgeId, Kind >
//   _IterateGraphIdsAndNodes< NodeId, EdgeId, Kind >
//   where
//     NodeId : IdentityInterface,
//     EdgeId : IdentityInterface + IdentityGenerableInterface,
//     Kind : NodeKindInterface,
//   {
//     /// Constructor.
//     pub fn new
//     (
//       container : &'it mut NodeFactory< NodeId, EdgeId, Kind >,
//       node_id : NodeId,
//     )
//     -> Self
//     {
//       let it = container.out_nodes_ids_2( node_id ).map( | id |
//       {
//         ( id, container.node( id ) )
//       });
//       let _it = unsafe
//       {
//         core::mem::transmute::< _, _ >( it )
//       };
//       Self
//       {
//         container,
//         node_id,
//         _it,
//       }
//     }
//
//     /// Iterator.
//     pub fn it( &mut self ) ->
//     &mut impl Iterator< Item = ( NodeId, &< NodeFactory< NodeId, EdgeId, Kind > as GraphNodesNominalInterface >::NodeHandle ) >
//     {
//       let result;
//       let mut _r;
//       #[ allow( unreachable_code ) ]
//       #[ allow( unused_assignments ) ]
//       if false
//       {
//         unreachable!();
//         _r = self.container.out_nodes_ids_2( self.node_id ).map( | id |
//         {
//           ( id, self.container.node( id ) )
//         });
//         result = &mut _r;
//       };
//       result = unsafe
//       {
//         core::mem::transmute::< _, _ >( &mut self._it )
//       };
//       return result;
//       // self.container.out_nodes_ids_2( self.node_id ).map( | id |
//       // {
//       //   // 13_i32
//       //   ( id, self.container.node( id ) )
//       // })
//     }
//
//   }
//
//   impl< NodeId, EdgeId, Kind >
//   Iterator
//   for _IterateGraphIdsAndNodes< NodeId, EdgeId, Kind >
//   where
//     NodeId : IdentityInterface,
//     EdgeId : IdentityInterface + IdentityGenerableInterface,
//     Kind : NodeKindInterface,
//   {
//     type Item = ( NodeId, &'it < NodeFactory< NodeId, EdgeId, Kind > as GraphNodesNominalInterface >::NodeHandle );
//     fn next( &mut self ) -> Option< Self::Item >
//     {
//       // self.container.out_nodes_ids_2( self.node_id ).map( | id |
//       // {
//       //   // 13_i32
//       //   ( id, self.container.node( id ) )
//       // })
//       self.it().next()
//     }
//   }

  //

//   impl< NodeId, EdgeId, Kind >
//   GraphNodesNominalInterface2< NodeFactory< NodeId, EdgeId, Kind > >
//   for &'it NodeFactory< NodeId, EdgeId, Kind >
//   where
//     NodeId : IdentityInterface,
//     EdgeId : IdentityInterface + IdentityGenerableInterface,
//     Kind : NodeKindInterface,
//     Self : Deref< Target = NodeFactory< NodeId, EdgeId, Kind > >,
//   {
//
//     type OutNodesIdsIterator = core::iter::Cloned< indexmap::set::Iter< NodeId > >;
//     fn out_nodes_ids_2< Id >( self, node_id : Id ) -> Self::OutNodesIdsIterator
//     where
//       Id : Into< NodeId >,
//     {
//       let node = self.node( node_id.into() );
//       let iterator = node.out_nodes.iter().cloned();
//       iterator
//     }
//
//     // type RefNode = < NodeFactory< NodeId, EdgeId, Kind > as GraphNodesNominalInterface >::NodeHandle;
//     // // type OutNodesIterator = _IterateGraphIdsAndNodes< NodeId, EdgeId, Kind >;
//     // type OutNodesIterator = core::iter::Map
//     // <
//     //   indexmap::set::Iter< NodeId >,
//     //   Box< dyn FnMut ( &NodeId ) -> ( NodeId, Self::RefNode ) >,
//     // >;
//
//     // fn out_nodes_2< Id >( self, node_id : Id )
//     // ->
//     // Self::OutNodesIterator
//     // where
//     //   Self : Sized,
//     //   Id : Into< NodeId >
//     // {
//     //   // fn map< NodeId >( id : NodeId ) -> ( NodeId, < NodeFactory< NodeId, EdgeId, Kind > as GraphNodesNominalInterface >::NodeHandle )
//     //   // where
//     //   //   NodeId : IdentityInterface,
//     //   // {
//     //   //   ( id, self.node( id ) )
//     //   // }
//     //   // return self.out_nodes_ids_2( node_id ).map( map );
//     //   _IterateGraphIdsAndNodes::new
//     //   (
//     //     &self,
//     //     node_id,
//     //   )
//     // }
//
//   }

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

    //

    // type NodesIteratorItem = ( &'it NODE_ID!(), &'it < Self as GraphNodesNominalInterface >::NodeHandle );
    // type NodesIterator = std::collections::hash_map::Iter< NODE_ID!(), < Self as GraphNodesNominalInterface >::NodeHandle >;
    // fn nodes( self ) -> Self::NodesIterator
    // {
    //   self.map.iter()
    // }

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
