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
    // type Id : crate::IdentityInterface;
    type Node : crate::HasId + crate::NodeBasicInterface;
    /// Iterate output nodes of the node.
    fn out_nodes< 'a >( &'a self, node_id : Self::Id ) -> Box< dyn Iterator< Item = Self::Node::Id > + 'a >;

    /// Get node with id.
    fn node< Self::Id >( &self, id : Self::Id )
    -> &Node
    // -> &crate::NodeCell< Node >
    {
    }

  }

  ///
  /// Graph which know how to extend set of out nodes of a given node.
  ///

  pub trait GraphExtendableInterface
  where
    Self :
      GraphBasicInterface +
      Extend< < Self as GraphBasicInterface >::Id > +
    ,
  {
  }

  impl< T > GraphExtendableInterface for T
  where
    T :
      GraphBasicInterface +
      Extend< < Self as GraphBasicInterface >::Id > +
    ,
  {
  }

  ///
  /// Graph nodes of which has a kind.
  ///

  pub trait GraphKindGetterInterface
    Self : GraphBasicInterface,
  {
    type NodeKind : crate::NodeKindIterface = crate::NodeKindless;
    /// Get kind of the node.
    fn node_kind( &self, node_id : < Self as GraphBasicInterface >::Id ) -> Self::Kind;
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
