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

//

crate::mod_interface!
{
  prelude use NodeFactoryInterface;
}
