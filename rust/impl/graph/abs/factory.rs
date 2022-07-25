/// Internal namespace.
pub( crate ) mod private
{
  // use crate::prelude::*;
  // use core::fmt;

  ///
  /// Interface of a type responsible for constructing nodes.
  ///

  pub trait NodeFactoryInterface
  where
    Self : crate::GraphNodesNominalInterface,
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
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  #[ doc( inline ) ]
  pub use super::private::NodeFactoryInterface;
}
