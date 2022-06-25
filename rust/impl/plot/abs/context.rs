/// Internal namespace.
pub( crate ) mod private
{
  use crate::*;
  // use crate::abs::*;
  // use once_cell::sync::Lazy;
  // use std::sync::Mutex;
  // use dashmap::DashMap;
  // use std::sync::Arc;

  /// Registry of contexts.
  pub trait ContextInterface
  where
    Self :
      HasIdInterface +
      Make0 +
      fmt::Debug +
    ,
  {
    /// Type of changer of the context.
    type Changer : ChangerInterface;
    /// Get changer of the context.
    fn changer( &mut self ) -> Self::Changer;
  }

}

/// Protected namespace of the module.
pub mod protected
{
  pub use super::
  {
    orphan::*,
  };
}

pub use protected::*;

/// Parented namespace of the module.
pub mod orphan
{
  pub use super::
  {
    exposed::*,
    private::ContextInterface,
  };
}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::
  {
    prelude::*,
  };
}

pub use exposed::*;

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  pub use super::private::
  {
  };
}
