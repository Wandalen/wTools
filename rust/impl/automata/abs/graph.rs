/// Internal namespace.
pub mod internal
{
  use crate::prelude::*;
  // use std::fmt;
  // use core::fmt::Debug;
  // use core::hash::Hash;
  // use core::cell::RefCell;
  // use std::sync::Arc;
  // use core::ops::Deref;

  ///
  /// Graph which know how to iterate output nodes of a given node.
  ///

  pub trait GraphBasicInterface
  {

    /// It's not always possible to operate a node directly, for example it it has to be wrapped by cell ref. For that use NodeHandle.
    /// Otherwise NodeHandle could be &Node.
    type NodeHandle : NodeBasicInterface;

    /// Iterate output nodes of the node.
    fn out_nodes< 'a, 'b >( &'a self, node_id : < Self::NodeHandle as HasId >::Id )
    ->
    Box< dyn Iterator< Item = < Self::NodeHandle as HasId >::Id > + 'b >
    where
      'a : 'b,
    ;

    /// Get node with id.
    fn node( &self, id : < Self::NodeHandle as HasId >::Id ) -> &Self::NodeHandle;

  }

  ///
  /// Graph which know how to extend set of out nodes of a given node.
  ///

  pub trait GraphExtendableInterface
  where
    Self :
      GraphBasicInterface +
      Extend< < < Self as GraphBasicInterface >::NodeHandle as HasId >::Id > +
    ,
  {
  }

  impl< T > GraphExtendableInterface for T
  where
    T :
      GraphBasicInterface +
      Extend< < < Self as GraphBasicInterface >::NodeHandle as HasId >::Id > +
    ,
  {
  }

  ///
  /// Graph nodes of which has a kind.
  ///

  pub trait GraphKindGetterInterface
  where
    Self : GraphBasicInterface,
  {
    /// Enumerate kinds of the node.
    type NodeKind : crate::NodeKindInterface;
    // type NodeKind : crate::NodeKindInterface = crate::NodeKindless;
    /// Get kind of the node.
    fn node_kind( &self, node_id : < < Self as GraphBasicInterface >::NodeHandle as HasId >::Id ) -> Self::NodeKind;
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
  // use super::internal as i;
  pub use super::prelude::*;
}

pub use exposed::*;

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  use super::internal as i;
  pub use i::GraphBasicInterface;
  pub use i::GraphExtendableInterface;
  pub use i::GraphKindGetterInterface;
}
