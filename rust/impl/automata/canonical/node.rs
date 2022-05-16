/// Internal namespace.
mod internal
{
  use crate::prelude::*;
  use std::collections::HashSet;
  use std::fmt;

  ///
  /// Canonical implementation of node.
  ///

  pub struct Node< Kind = crate::NodeKindless >
  where
    Kind : NodeKindInterface,
  {
    /// Input node.
    pub out_nodes : HashSet< < Self as HasId >::Id >,
    /// Kind of the node.
    pub kind : Kind,
    /// Name.
    pub name : < Self as HasId >::Id,
  }

  //

  impl Node
  {

    /// Construct a name instance of the node.
    pub fn make_named< Name >( name : Name ) ->Self
    where
      Name : Into< < Self as HasId >::Id >,
    {
      let out_nodes = HashSet::new();
      let kind = Default::default();
      Self
      {
        out_nodes,
        kind,
        name : name.into(),
      }
    }

  }

  //

//   impl< Kind > HasId
//   for Node< Kind >
//   where
//     Kind : NodeKindInterface,
//   {
//
//     type Id = crate::IdentityWithName;
//
//     fn id( &self ) -> Self::Id
//     {
//       self.name
//     }
//
//   }

  //

  impl< Kind > HasId
  for Node< Kind >
  where
    Kind : NodeKindInterface,
  {

    type Id = crate::IdentityWithInt;

    fn id( &self ) -> Self::Id
    {
      self.name
    }

  }

  //

  impl< Kind > NodeBasicInterface
  for Node< Kind >
  where
    Kind : NodeKindInterface,
  {
  }

  //

  impl Extend< < Self as HasId >::Id >
  for Node
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

  impl< Kind > fmt::Debug
  for Node< Kind >
  where
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

  impl< Kind > PartialEq for Node< Kind >
  where
    Kind : NodeKindInterface,
  {
    fn eq( &self, other : &Self ) -> bool
    {
      self.id() == other.id()
    }
  }

}

/// Own namespace of the module.
pub mod protected
{
  // use super::internal as i;
  pub use super::orphan::*;
}

pub use protected::*;

/// Parented namespace of the module.
pub mod orphan
{
  use super::internal as i;
  pub use super::exposed::*;
  // pub use i::NodesIterator;
  pub use i::Node;
}

/// Exposed namespace of the module.
pub mod exposed
{
  // use super::internal as i;
  pub use super::prelude::*;
}

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  // use super::internal as i;
}
