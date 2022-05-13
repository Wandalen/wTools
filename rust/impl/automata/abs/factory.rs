/// Internal namespace.
pub mod internal
{
  use crate::prelude::*;
  use core::fmt::Debug;

  ///
  /// Interface of a type responsible for constructing nodes.
  ///

  pub trait NodeFactoryInterface
  where
    Self : Debug,
  {
    // /// Node of the graph.
    // type Node : NodeBasicInterface;
    /// It's not always possible to operate a node directly, for example it it has to be wrapped by cell ref. For that use NodeHandle.
    /// Otherwise NodeHandle is the same as Node.
    type NodeHandle : NodeBasicInterface;
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
  // use super::internal as i;
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::prelude::*;
  // use super::internal as i;
}

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  use super::internal as i;
  pub use i::NodeFactoryInterface;
}
