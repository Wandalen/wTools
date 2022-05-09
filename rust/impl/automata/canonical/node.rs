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
    pub out_nodes : HashSet< Self >,
    /// Kind of the node.
    pub kind : Kind,
    /// Label.
    pub label : String,
    // //// Lifetime.
    // _p : std::marker::PhantomData< &'a () >,
  }

  //

  impl PartialEq for Node
  {
    fn eq( &self, other : &Self ) -> bool
    {
      // Different instances of node are different logical instances.
      unsafe
      {
        let ptr1 = std::mem::transmute::< _, *const () >( self );
        let ptr2 = std::mem::transmute::< _, *const () >( other );
        ptr1 == ptr2
      }
    }
  }

  impl Eq for Node {}

  impl< Kind > Hash for Node< Kind >
  where
    Kind : NodeKindInterface,
  {
    fn hash< H >( &self, state : &mut H )
    where
      H : Hasher,
    {
      // Just use the address of the instance.
      unsafe
      {
        let ptr1 = std::mem::transmute::< _, *const () >( self );
        ptr1.hash( state );
      }
    }
  }

  //

  impl< Kind > NodeBasicInterface
  for Node< Kind >
  where
    Kind : NodeKindInterface,
  {

    /// Iterate output nodes of the node.
    fn out_nodes< 'a >( &'a self ) -> Box< dyn Iterator< Item = Self > + 'a >
    {
      Box::new( NodesIterator::make( &self ) )
    }

  }

  //

  impl Extend< Node > for Node
  {

    fn extend< Iter >( &mut self, iter : Iter )
    where
      Iter : IntoIterator< Item = Node >
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
