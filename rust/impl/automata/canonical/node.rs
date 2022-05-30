/// Internal namespace.
pub( crate ) mod private
{
  use crate::prelude::*;
  use wtools::prelude::*;
  use indexmap::IndexSet;
  use std::fmt;

  ///
  /// Canonical implementation of node.
  ///

  pub struct Node< Id = crate::IdentityWithInt, Kind = crate::NodeKindless >
  where
    Id : IdentityInterface,
    Kind : NodeKindInterface,
  {
    /// Input node.
    pub out_nodes : IndexSet< Id >,
    /// Input node.
    pub out_edges : IndexSet< Id >,
    /// Kind of the node.
    pub kind : Kind,
    /// Identifier.
    pub id : Id,
  }

  //

  impl< Id, Kind > Node< Id, Kind >
  where
    Id : IdentityInterface,
    Kind : NodeKindInterface,
  {

    /// Construct an instance of the node with id.
    pub fn make_with_id< Name >( id : Name ) ->Self
    where
      Name : Into< < Self as HasId >::Id >,
    {
      let out_nodes = IndexSet::new();
      let out_edges = IndexSet::new();
      let kind = Default::default();
      Self
      {
        out_nodes,
        out_edges,
        kind,
        id : id.into(),
      }
    }

  }

  //

  impl< Id, Kind, IntoId > Make1< IntoId >
  for Node< Id, Kind >
  where
    Id : IdentityInterface,
    Kind : NodeKindInterface,
    IntoId : Into< < Self as HasId >::Id >,
  {
    fn make_1( id : IntoId ) -> Self
    {
      Self::make_with_id( id )
    }
  }

  //

  impl< Id, Kind > HasId
  for Node< Id, Kind >
  where
    Id : IdentityInterface,
    Kind : NodeKindInterface,
  {
    type Id = Id;
    fn id( &self ) -> Self::Id
    {
      self.id
    }
  }

  //

  impl< Id, Kind > NodeBasicInterface
  for Node< Id, Kind >
  where
    Id : IdentityInterface,
    Kind : NodeKindInterface,
  {
  }

  //

  impl< Id, Kind > Extend< < Self as HasId >::Id >
  for Node< Id, Kind >
  where
    Id : IdentityInterface,
    Kind : NodeKindInterface,
  {
    fn extend< Iter >( &mut self, iter : Iter )
    where
      Iter : IntoIterator< Item = < Self as HasId >::Id >
    {
      for node in iter
      {
        self.out_nodes.insert( node );
      }
    }
  }

  //

  impl< Id, Kind > fmt::Debug
  for Node< Id, Kind >
  where
    Id : IdentityInterface,
    Kind : NodeKindInterface,
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

  impl< Id, Kind > PartialEq
  for Node< Id, Kind >
  where
    Id : IdentityInterface,
    Kind : NodeKindInterface,
  {
    fn eq( &self, other : &Self ) -> bool
    {
      self.id() == other.id()
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
  pub use super::private::{ Node };
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
