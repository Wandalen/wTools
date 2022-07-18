/// Internal namespace.
pub( crate ) mod private
{
  use crate::prelude::*;
  use core::fmt;
  use core::hash::Hash;

  ///
  /// Kind of a node.
  ///

  pub trait NodeKindInterface
  where
    Self :
      'static +
      Copy +
      fmt::Debug +
      PartialEq +
      // Eq +
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
      fmt::Debug +
      PartialEq +
      // Eq +
      Hash  +
      Default +
    ,
  {
  }

  ///
  /// No kind for nodes.
  ///

  #[ derive( Debug, PartialEq, Eq, Copy, Clone, Hash, Default ) ]
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

}

/// Protected namespace of the module.
pub mod protected
{
  #[ doc( inline ) ]
  pub use super::orphan::*;
}

#[ doc( inline ) ]
pub use protected::*;

/// Parented namespace of the module.
pub mod orphan
{
  #[ doc( inline ) ]
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
  #[ doc( inline ) ]
  pub use super::prelude::*;
  #[ doc( inline ) ]
  pub use super::private::NodeKindless;
}

#[ doc( inline ) ]
pub use exposed::*;

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  #[ doc( inline ) ]
  pub use super::private::
  {
    NodeKindInterface,
    NodeBasicInterface,
  };
}
