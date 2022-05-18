/// Internal namespace.
mod internal
{
  use crate::prelude::*;
  use std::fmt;
  use core::cell::RefCell;
  use std::sync::Arc;
  use core::ops::Deref;

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

  impl< Node > NodeHandleInterface
  for NodeCell< Node >
  where
    Node : NodeBasicInterface,
  {
    type Node = Node;
  }

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

    // fn out_nodes( &self ) -> Box< dyn Iterator< Item = < Self as HasId >::Id > + '_ >
    // {
    //   let node = self.borrow();
    //   let iterator : Box< dyn Iterator< Item = < Self as HasId >::Id > > = node.out_nodes();
    //   // safety : developer should make sure graph is not edited during the iterating
    //   let result = unsafe
    //   {
    //     std::mem::transmute::< _, _ >( iterator )
    //   };
    //   result
    // }

  }

  //

//   impl< Node > Extend
//   for NodeCell< Node >
//   where
//     Node : NodeBasicInterface + NodeExtendableInterface,
//   {
//
//     fn extend< Iter >( &mut self, iter : Iter )
//     where
//       Iter : IntoIterator< Item = < Self as HasId >::Id >
//     {
//       let node = self.borrow_mut();
//       node.extend( iter );
//       // for node in iter
//       // {
//       //   self.out_nodes.insert( node );
//       // }
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

  impl< Node > From< Node >
  for NodeCell< Node >
  where
    Node : NodeBasicInterface,
  {
    fn from( src : Node ) -> Self
    {
      Self( Arc::new( RefCell::new( src ) ) )
    }
  }

}

/// Own namespace of the module.
pub mod protected
{
  pub use super::orphan::*;
}

pub use protected::*;

/// Parented namespace of the module.
pub mod orphan
{
  // use super::internal as i;
  pub use super::exposed::*;
}

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
