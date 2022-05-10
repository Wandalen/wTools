/// Internal namespace.
pub mod internal
{
  use crate::prelude::*;
  use std::collections::HashSet;
  use core::fmt::Debug;
  use std::cmp::Eq;
  use core::hash::{ Hash, Hasher };

  ///
  /// No kind for nodes.
  ///

  #[ derive( Debug, PartialEq, Copy, Clone, Hash, Default ) ]
  pub struct NodeKindless();

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

  #[ derive( Debug ) ]
  pub struct Node< Kind = NodeKindless >
  where
    Kind : NodeKindInterface,
  {
    /// Input node.
    pub out_nodes : HashSet< < Self as HasId >::Id >,
    /// Kind of the node.
    pub kind : Kind,
    /// Label.
    pub label : String,
    // //// Lifetime.
    // _p : std::marker::PhantomData< &'a () >,
  }

  //

  impl Node
  {

    /// Construct a labeled instance of the node.
    pub fn make_labeled( label : String ) ->Self
    {
      let out_nodes = HashSet::new();
      let kind = Default::default();
      Self
      {
        out_nodes,
        kind,
        label,
      }
    }

  }

  //

  impl< Kind > HasId
  for Node< Kind >
  where
    Kind : NodeKindInterface,
  {

    type Id = crate::IdentityByPointer;

    fn id( &self ) -> Self::Id
    {
      Self::Id::make( &self )
    }

  }

  impl< Kind > NodeBasicInterface
  for Node< Kind >
  where
    Kind : NodeKindInterface,
  {

    fn out_nodes< 'a >( &'a self ) -> Box< dyn Iterator< Item = Self > + 'a >
    {
      Box::new( NodesIterator::make( &self ) )
    }

  }

  impl< Kind > NodeConstructableInterface
  for Node< Kind >
  where
    Kind : NodeKindInterface,
  {

    fn make() -> Self
    {
      let out_nodes = HashSet::new();
      let kind = Default::default();
      let label = Default::default();
      Self
      {
        out_nodes,
        kind,
        label
      }
    }

  }

  /* zzz : macro? */

  impl< Kind > PartialEq
  for Node< Kind >
  where
    Kind : NodeKindInterface,
  {
    fn eq( &self, other : &Self ) -> bool
    {
      self.id() == other.id()
    }
  }

  impl< Kind > Eq
  for Node< Kind >
  where
    Kind : NodeKindInterface,
  {}

  impl< Kind > Hash
  for Node< Kind >
  where
    Kind : NodeKindInterface,
  {
    fn hash< H >( &self, state : &mut H )
    where
      H : Hasher,
    {
      self.id().hash( state );
    }
  }

  //

  impl Extend< crate::IdentityByPointer > for Node
  {

    fn extend< Iter >( &mut self, iter : Iter )
    where
      Iter : IntoIterator< Item = < Self as HasId >::Id >
    {
      for node in iter
      {
        self.out_nodes.insert( node );
      }
      // ( self.out_nodes as HashSet< Self > ).extend( iter );
    }
  }

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
  pub use i::NodeKindless;
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
