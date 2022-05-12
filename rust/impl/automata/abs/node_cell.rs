/// Internal namespace.
pub mod internal
{
  use crate::prelude::*;
  use std::fmt;
  use core::fmt::Debug;
  use core::hash::Hash;
  use core::cell::RefCell;
  use std::sync::Arc;
  use core::ops::Deref;
  use core::hash::Hasher;

//   ///
//   /// Interface of node cell.
//   ///
//
//   pub trait NodeCellInterface
//   {
//
//     /// Node itself.
//     type Node : NodeBasicInterface;
//     /// Node itself.
//     type BorrowedNode;
//
//     /// Constructor.
//     fn make( src : Self::Node ) -> Self;
//     /// Borrow.
//     fn borrow( &self ) -> Self::BorrowedNode;
//
//   }

  ///
  /// Node in RefCell in Rc.
  ///

  #[ repr( transparent ) ]
  pub struct NodeCell< Node >( Arc< RefCell< Node > > )
  where
    Node : NodeBasicInterface,
  ;

  impl< Node > NodeCell< Node >
  where
    Node : NodeBasicInterface,
  {
    /// Constructor.
    #[ inline ]
    pub fn make( src : Node ) -> Self
    {
      Self( Arc::new( RefCell::new( src ) ) )
    }
  }

//   impl< Kind > NodeCellInterface
//   for NodeCell
//   {
//
//     /// Node itself.
//     type Node = Node< Kind >;
//     /// Node itself.
//     type BorrowedNode;
//
//     /// Constructor.
//     #[ inline ]
//     pub fn make( src : Node ) -> Self
//     {
//       Self( Arc::new( RefCell::new( src ) ) )
//     }
//     /// Constructor.
//     #[ inline ]
//     fn borrow( &self ) -> Self::BorrowedNode
//     {
//       Self( Arc::new( RefCell::new( src ) ) )
//     }
//   }

  impl< Node > HasId
  for NodeCell< Node >
  where
    Node : NodeBasicInterface,
  {

    type Id = Node::Id;

    fn id( &self ) -> Self::Id
    {
      self.borrow().id()
    }

  }

  impl< Node > NodeBasicInterface
  for NodeCell< Node >
  where
    Node : NodeBasicInterface,
  {

    fn out_nodes< 'a >( &'a self ) -> Box< dyn Iterator< Item = < Self as HasId >::Id > + 'a >
    {
      self.borrow().out_nodes()
    }

  }

//   impl< Node > PartialEq
//   for NodeCell< Node >
//   where
//     Node : NodeBasicInterface,
//   {
//     fn eq( &self, other : &Self ) -> bool
//     {
//       self.id() == other.id()
//     }
//   }
//
//   impl< Node > Eq
//   for NodeCell< Node >
//   where
//     Node : NodeBasicInterface,
//   {}
//
//   impl< Node > Hash
//   for NodeCell< Node >
//   where
//     Node : NodeBasicInterface,
//   {
//     fn hash< H >( &self, state : &mut H )
//     where
//       H : Hasher,
//     {
//       self.id().hash( state );
//     }
//   }

//   //
//
//   impl< Node > Extend< Node::Id >
//   for NodeCell< Node >
//   where
//     Node : NodeBasicInterface,
//   {
//
//     fn extend< Iter >( &mut self, iter : Iter )
//     where
//       Iter : IntoIterator< Item = < Self as HasId >::Id >
//     {
//       for node in iter
//       {
//         self.out_nodes.insert( node );
//       }
//     }
//   }

  impl< Node > fmt::Debug
  for NodeCell< Node >
  where
    Node : NodeBasicInterface + fmt::Debug,
  {
    fn fmt( &self, f : &mut fmt::Formatter<'_> ) -> fmt::Result
    {
      f.write_fmt( format_args!( "{:?}", self.0.borrow() ) )
    }
  }

  impl< Node > Deref
  for NodeCell< Node >
  where
    Node : NodeBasicInterface,
  {
    type Target = Arc< RefCell< Node > >;
    fn deref( &self ) -> &Self::Target
    {
      &self.0
    }
  }

  impl< Node > From< Arc< RefCell< Node > > >
  for NodeCell< Node >
  where
    Node : NodeBasicInterface,
  {
    fn from( src : Arc< RefCell< Node > > ) -> Self
    {
      Self( src )
    }
  }

}

/// Parented namespace of the module.
pub mod parented
{
  // use super::internal as i;
  pub use super::exposed::*;
}

pub use parented::*;

/// Exposed namespace of the module.
pub mod exposed
{
  use super::internal as i;
  pub use super::prelude::*;
  pub use i::NodeCell;
}

pub use exposed::*;

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  // use super::internal as i;
}
