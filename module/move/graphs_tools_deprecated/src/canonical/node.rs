/// Define a private namespace for all its items.
mod private
{
  use crate::prelude::*;
  // use wtools::prelude::*;
  use indexmap::IndexSet;
  use core::fmt;

  ///
  /// Canonical implementation of node.
  ///

  pub struct Node< NodeId = crate::IdentityWithInt, EdgeId = crate::IdentityWithInt >
  where
    NodeId : IdentityInterface,
    EdgeId : IdentityInterface,
  {
    /// Input node.
    pub out_nodes : IndexSet< NodeId >,
    /// Input node.
    pub out_edges : IndexSet< EdgeId >,
    // /// Kind of the node.
    // pub kind : Kind,
    /// Identifier.
    pub id : NodeId,
  }

  //

//   impl< NodeId, EdgeId > Node< NodeId, EdgeId >
//   where
//     NodeId : IdentityInterface,
//     EdgeId : IdentityInterface,
//     //
//   {
//
//     /// Construct an instance of the node with id.
//     pub fn make_with_id< Name >( id : Name ) ->Self
//     where
//       Name : Into< < Self as HasId >::Id >,
//     {
//       let out_nodes = IndexSet::new();
//       let out_edges = IndexSet::new();
//       Self
//       {
//         out_nodes,
//         out_edges,
//         id : id.into(),
//       }
//     }
//
//   }

  //

  impl< NodeId, EdgeId > Node< NodeId, EdgeId >
  where
    NodeId : IdentityInterface,
    EdgeId : IdentityInterface,
  {
    /// Construct canonical node using id.
    pub fn _make_with_id< IntoId >( id : IntoId ) -> Self
    where
      IntoId : Into< < Self as HasId >::Id >,
    {
      let out_nodes = Default::default();
      let out_edges = Default::default();
      Node { out_nodes, out_edges, id : id.into() }
      // Self::make_with_id( id )
    }
  }

//   impl< NodeId, EdgeId, IntoId > From_1< IntoId >
//   for Node< NodeId, EdgeId >
//   where
//     NodeId : IdentityInterface,
//     EdgeId : IdentityInterface,
//
//     IntoId : Into< < Self as HasId >::Id >,
//   {
//     fn from_1( id : IntoId ) -> Self
//     {
//       let out_nodes = Default::default();
//       let in_nodes = Default::default();
//       Node { out_nodes, in_nodes, id }
//       // Self::make_with_id( id )
//     }
//   }

  //

  impl< NodeId, EdgeId > HasId
  for Node< NodeId, EdgeId >
  where
    NodeId : IdentityInterface,
    EdgeId : IdentityInterface,
  {
    type Id = NodeId;
    fn id( &self ) -> Self::Id
    {
      self.id
    }
  }

  //

  impl< NodeId, EdgeId > NodeBasicInterface
  for Node< NodeId, EdgeId >
  where
    NodeId : IdentityInterface,
    EdgeId : IdentityInterface,
  {
  }

  //

  // impl< NodeId, EdgeId > Extend< < Self as HasId >::Id >
  // for Node< NodeId, EdgeId >
  // where
  //   NodeId : IdentityInterface,
  //   EdgeId : IdentityInterface,
  //
  // {
  //   fn extend< Iter >( &mut self, iter : Iter )
  //   where
  //     Iter : IntoIterator< Item = < Self as HasId >::Id >
  //   {
  //     for node_id in iter
  //     {
  //       self.out_nodes.insert( node_id );
  //     }
  //   }
  // }

  //

  impl< NodeId, EdgeId > fmt::Debug
  for Node< NodeId, EdgeId >
  where
    NodeId : IdentityInterface,
    EdgeId : IdentityInterface,

  {
    fn fmt( &self, f : &mut fmt::Formatter<'_> ) -> fmt::Result
    {
      f.write_fmt( format_args!( "node::{:?}", self.id() ) )?;
      for e in &self.out_nodes
      {
        f.write_fmt( format_args!( "\n - {:?}", e ) )?;
      }
      f.write_fmt( format_args!( "" ) )
    }
  }

  //

  impl< NodeId, EdgeId > PartialEq
  for Node< NodeId, EdgeId >
  where
    NodeId : IdentityInterface,
    EdgeId : IdentityInterface,
  {
    fn eq( &self, other : &Self ) -> bool
    {
      self.id() == other.id()
    }
  }

  impl< NodeId, EdgeId > Eq
  for Node< NodeId, EdgeId >
  where
    NodeId : IdentityInterface,
    EdgeId : IdentityInterface,
  {}

}

//

crate::mod_interface!
{
  orphan use Node;
}

