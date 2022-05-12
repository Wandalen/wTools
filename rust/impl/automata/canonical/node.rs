/// Internal namespace.
pub mod internal
{
  use crate::prelude::*;
  use std::collections::HashSet;
  use core::fmt::Debug;
  // use core::hash::{ Hash };
  use std::fmt;

  ///
  /// Nodes iterator for canonical node.
  ///

  #[ derive( Debug ) ]
  pub struct NodesIterator< 'a, Kind >
  where
    Kind : NodeKindInterface,
  {
    /// Node.
    pub node : &'a Node< Kind >,
  }

  impl< 'a, Kind > NodesIterator< 'a, Kind >
  where
    Kind : NodeKindInterface,
  {
    /// Make a new iterator of nodes.
    pub fn make( node : &'a Node< Kind > ) -> Self
    {
      Self
      {
        node,
      }
    }
  }

  impl< 'a, Kind > Iterator
  for NodesIterator< 'a, Kind >
  where
    Kind : NodeKindInterface,
  {
    type Item = Node< Kind >;
    fn next( &mut self ) -> Option< Self::Item >
    {
      None
    }
  }

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

  impl< Kind > HasId
  for Node< Kind >
  where
    Kind : NodeKindInterface,
  {

    type Id = crate::IdentityByName;

    fn id( &self ) -> Self::Id
    {
      self.name
    }

  }

  impl< Kind > NodeBasicInterface
  for Node< Kind >
  where
    Kind : NodeKindInterface,
  {

    fn out_nodes( &self ) -> Box< dyn Iterator< Item = < Self as HasId >::Id > + '_ >
    // fn out_nodes< 'a >( &'a self ) -> Box< dyn Iterator< Item = < Self as HasId >::Id > + 'a >
    {
      Box::new( self.out_nodes.iter().cloned() )
    }

  }

//   /* zzz : macro? */
//
//   impl< Kind > PartialEq
//   for Node< Kind >
//   where
//     Kind : NodeKindInterface,
//   {
//     fn eq( &self, other : &Self ) -> bool
//     {
//       self.id() == other.id()
//     }
//   }
//
//   impl< Kind > Eq
//   for Node< Kind >
//   where
//     Kind : NodeKindInterface,
//   {}
//
//   impl< Kind > Hash
//   for Node< Kind >
//   where
//     Kind : NodeKindInterface,
//   {
//     fn hash< H >( &self, state : &mut H )
//     where
//       H : Hasher,
//     {
//       self.id().hash( state );
//     }
//   }

  //

  impl Extend< crate::IdentityByName >
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

  // --

}

/// Own namespace of the module.
pub mod own
{
  // use super::internal as i;
  pub use super::parented::*;
}

pub use own::*;

/// Parented namespace of the module.
pub mod parented
{
  use super::internal as i;
  pub use super::exposed::*;
  pub use i::NodesIterator;
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
