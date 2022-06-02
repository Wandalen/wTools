/// Internal namespace.
pub( crate ) mod private
{
  use crate::prelude::*;
  use wtools::prelude::*;
  use core::fmt;
  use core::cell::RefCell;
  use core::ops::Deref;
  use std::sync::Arc;

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
    // /// Constructor.
    // #[ inline ]
    // pub fn make( src : Node ) -> Self
    // {
    //   Self( Arc::new( RefCell::new( src ) ) )
    // }
  }

  //

  impl< Node > Make1< Node >
  for NodeCell< Node >
  where
    Node : NodeBasicInterface,
  {
    fn make_1( src : Node ) -> Self
    {
      Self( Arc::new( RefCell::new( src ) ) )
    }
  }

  //

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
  }

  //

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

  //

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

  //

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

  //

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

  //

  impl< Node > PartialEq
  for NodeCell< Node >
  where
    Node : NodeBasicInterface,
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
}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::prelude::*;
  pub use super::private::NodeCell;
}

pub use exposed::*;

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
}
