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

    /// Iterate output nodes of the node.
    fn out_nodes< 'a >( &'a self ) -> Box< dyn Iterator< Item = < Self as HasId >::Id > + 'a >;

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

  ///
  /// Node which has a kind.
  ///

  pub trait NodeKindGetterInterface< Kind >
  where
    Kind : NodeKindInterface,
    Self : NodeBasicInterface,
  {
    /// Get kind of the node.
    fn kind() -> Kind;
  }

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
