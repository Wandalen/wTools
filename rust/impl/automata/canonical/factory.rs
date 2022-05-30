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

  pub struct NodeFactory< Id = crate::IdentityWithInt, Kind = crate::NodeKindless >
  where
    Id : IdentityInterface,
    Kind : NodeKindInterface,
    NodeFactory< Id, Kind > : crate::NodeFactoryInterface,
  {
    /// Map id to node.
    pub id_to_node_map : IndexMap< NODE_ID!(), crate::canonical::Node< Id, Kind > >,
  }

  impl< Id, Kind > NodeFactory< Id, Kind >
  where
    Id : IdentityInterface,
    Kind : NodeKindInterface,
  {
  }

  //

  impl< Id, Kind > GraphNodesInterface
  for NodeFactory< Id, Kind >
  where
    Id : IdentityInterface,
    Kind : NodeKindInterface,
  {
    type NodeHandle = crate::canonical::Node< Id, Kind >;
    index!
    {
      node,
      out_nodes,
      nodes,
    }
  }

  //

  // impl< Id, Kind > GraphEdgesInterface
  // for NodeFactory< Id, Kind >
  // where
  //   Id : IdentityInterface,
  //   Kind : EdgeKindInterface,
  // {
  //   type EdgeHandle = crate::canonical::Edge< Id, Kind >;
  //   index!
  //   {
  //     edge,
  //     out_edges,
  //     edges,
  //   }
  // }

  //

  impl< Id, Kind > GraphExtendableInterface
  for NodeFactory< Id, Kind >
  where
    Id : IdentityInterface,
    Kind : NodeKindInterface,
  {

    index!
    {
      node_making,
    }

  }

  //

  impl< Id, Kind > GraphEditableInterface
  for NodeFactory< Id, Kind >
  where
    Id : IdentityInterface,
    Kind : NodeKindInterface,
  {

    index!
    {
      node_mut,
      node_add_out_nodes,
    }

  }

  //

  impl< Id, Kind > NodeFactoryInterface
  for NodeFactory< Id, Kind >
  where
    Id : IdentityInterface,
    Kind : NodeKindInterface,
  {
    // type NodeHandle = crate::canonical::Node< Id, Kind >; /* xxx2 : remove? */
  }

  //

  impl< Id, Kind > fmt::Debug
  for NodeFactory< Id, Kind >
  where
    Id : IdentityInterface,
    Kind : NodeKindInterface,
  {
    index!( fmt );
  }

  //

  impl< Id, Kind > Make0
  for NodeFactory< Id, Kind >
  where
    Id : IdentityInterface,
    Kind : NodeKindInterface,
  {
    fn make_0() -> Self
    {
      let id_to_node_map = IndexMap::new();
      Self
      {
        id_to_node_map,
      }
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
