/// Internal namespace.
pub( crate ) mod private
{
  use crate::prelude::*;
  use crate::canonical::*;
  use wtools::prelude::*;
  use indexmap::IndexMap;
  use std::fmt;

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
      let out_nodes_iter2 = out_nodes_iter.into_iter()
      .map( | id |
      {
        let id = id.into();
        self.node( id );
        id
      });
      self.node( node_id.into() ).borrow_mut().extend( out_nodes_iter2 );
    }

    //

    fn out_nodes< 'a, 'b, IntoId >( &'a self, node_id : IntoId )
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

    fn out_edges< 'a, 'b, IntoId >( &'a self, node_id : IntoId )
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
    EdgeId : IdentityInterface + IdentityIncInterface,
    Kind : NodeKindInterface,
    CellNodeFactory< NodeId, EdgeId, Kind > : crate::NodeFactoryInterface,
  {
    /// Map id to node.
    pub id_to_node_map : IndexMap< NodeId, crate::NodeCell< Node< NodeId, EdgeId, Kind > > >,
    /// Map id to edge.
    pub id_to_edge_map : IndexMap< EdgeId, crate::canonical::Edge< EdgeId, NodeId, Kind > >,
  }

  //

  impl< NodeId, EdgeId, Kind > CellNodeFactory< NodeId, EdgeId, Kind >
  where
    NodeId : IdentityInterface,
    EdgeId : IdentityInterface + IdentityIncInterface,
    Kind : NodeKindInterface,
  {
  }

  //

  impl< NodeId, EdgeId, Kind > GraphNodesInterface
  for CellNodeFactory< NodeId, EdgeId, Kind >
  where
    NodeId : IdentityInterface,
    EdgeId : IdentityInterface + IdentityIncInterface,
    Kind : NodeKindInterface,
  {
    type NodeHandle = crate::NodeCell< crate::canonical::Node< NodeId, EdgeId, Kind > >;
    index!
    {
      node,
      nodes,
      out_nodes,
    }
  }

  //

  impl< NodeId, EdgeId, Kind > GraphEdgesInterface
  for CellNodeFactory< NodeId, EdgeId, Kind >
  where
    NodeId : IdentityInterface,
    EdgeId : IdentityInterface + IdentityIncInterface,
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
  for CellNodeFactory< NodeId, EdgeId, Kind >
  where
    NodeId : IdentityInterface,
    EdgeId : IdentityInterface + IdentityIncInterface,
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

//   impl< NodeId, EdgeId, Kind > GraphEditableInterface
//   for CellNodeFactory< NodeId, EdgeId, Kind >
//   where
//     NodeId : IdentityInterface,
//     EdgeId : IdentityInterface + IdentityIncInterface,
//     Kind : NodeKindInterface,
//   {
//
//     index!
//     {
//       node_mut,
//       node_add_out_nodes,
//     }
//
//   }

  //

  impl< NodeId, EdgeId, Kind > NodeFactoryInterface
  for CellNodeFactory< NodeId, EdgeId, Kind >
  where
    NodeId : IdentityInterface,
    EdgeId : IdentityInterface + IdentityIncInterface,
    Kind : NodeKindInterface,
  {
  }

  //

  impl< NodeId, EdgeId, Kind > fmt::Debug
  for CellNodeFactory< NodeId, EdgeId, Kind >
  where
    NodeId : IdentityInterface,
    EdgeId : IdentityInterface + IdentityIncInterface,
    Kind : NodeKindInterface,
  {
    index!( fmt );
  }

  //

  impl< NodeId, EdgeId, Kind > Make0
  for CellNodeFactory< NodeId, EdgeId, Kind >
  where
    NodeId : IdentityInterface,
    EdgeId : IdentityInterface + IdentityIncInterface,
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
  pub use super::private::CellNodeFactory;
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
