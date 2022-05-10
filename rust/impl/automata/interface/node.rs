/// Internal namespace.
pub mod internal
{
  use crate::prelude::*;
  use core::fmt::Debug;
  use core::hash::Hash;

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

  ///
  /// Node which has constructor make.
  ///

  pub trait NodeConstructableInterface
  where
    Self :
      NodeBasicInterface +
    ,
  {
    /// Constructor without arguments.
    fn make() -> Self;
  }

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
  // pub use i::NodesIterator;
}

pub use exposed::*;

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  use super::internal as i;
  pub use i::NodeKindInterface;
  pub use i::NodeBasicInterface;
  pub use i::NodeConstructableInterface;
  pub use i::NodeKindGetterInterface;
}
