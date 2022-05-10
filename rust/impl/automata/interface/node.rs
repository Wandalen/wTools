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

  ///
  /// Kind of a ode.
  ///

  pub trait NodeKindInterface
  where
    Self :
      'static +
      Copy +
      Debug +
      PartialEq +
      Hash  +
      Default +
    ,
  {
  }

  impl< T > NodeKindInterface for T
  where
    T :
      'static +
      Copy +
      Debug +
      PartialEq +
      Hash  +
      Default +
    ,
  {
  }

  ///
  /// Node of a graph.
  ///

  pub trait NodeBasicInterface
  where
    Self :
      Hash +
      HasId +
  {

    // /// Id of the node.
    // type Id : IdentityInterface;
//     /// Type which represents edge between nodes.
//     type Edge : EdgeInterface;
//
//     /// Iterate all edges of the node.
//     fn edges( &self ) -> crate::EdgesIterator< Self::Edge >;

    /// Iterate output nodes of the node.
    fn out_nodes< 'a >( &'a self ) -> Box< dyn Iterator< Item = Self > + 'a >;

  }

  ///
  /// Node which is extendable
  ///

  pub trait NodeExtendableInterface
  where
    Self :
      Sized +
      NodeBasicInterface +
      Extend< Self > +
    ,
  {
  }

  impl< T > NodeExtendableInterface for T
  where
    T :
      NodeBasicInterface +
      Extend< Self > +
    ,
  {
  }

//   ///
//   /// Node which has constructor make.
//   ///
//
//   pub trait NodeConstructableInterface
//   where
//     Self :
//       NodeBasicInterface +
//     ,
//   {
//     /// Constructor without arguments.
//     fn make() -> Self;
//   }

  ///
  /// Node wich has a kind.
  ///

  pub trait NodeKindGetterInterface< Kind >
  where
    Kind : NodeKindInterface,
    Self : NodeBasicInterface,
  {
    /// Get kind of the node.
    fn kind() -> Kind;
  }

  ///
  /// Node in RefCell in Rc.
  ///

  pub struct NodeCell< Node >( Arc< RefCell< Node > > )
  where
    Node : NodeBasicInterface,
  ;

  impl< Node > NodeCell< Node >
  where
    Node : NodeBasicInterface,
  {
    #[ inline ]
    pub fn make( src : Node ) -> Self
    {
      Self( Arc::new( RefCell::new( src ) ) )
    }
  }

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
  use super::internal as i;
  pub use i::NodeKindInterface;
  pub use i::NodeBasicInterface;
  pub use i::NodeKindGetterInterface;
}
