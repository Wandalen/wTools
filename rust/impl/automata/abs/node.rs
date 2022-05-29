/// Internal namespace.
pub( crate ) mod private
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
  /// No kind for nodes.
  ///

  #[ derive( Debug, PartialEq, Copy, Clone, Hash, Default ) ]
  pub struct NodeKindless();

  ///
  /// Node of a graph.
  ///

  pub trait NodeBasicInterface
  where
    Self :
      HasId +
  {
  }

//   ///
//   /// Node handle.
//   ///
//
//   pub trait NodeHandleInterface : NodeBasicInterface + HasId
//   {
//     /// Node itself.
//     type Node : NodeBasicInterface + HasId< Id = Self::Id >;
//   }

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
  pub use super::private::NodeKindless;
}

pub use exposed::*;

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  pub use super::private::NodeKindInterface;
  pub use super::private::NodeBasicInterface;
  // pub use super::private::NodeHandleInterface;
}
