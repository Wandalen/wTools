/// Internal namespace.
pub( crate ) mod private
{
  use crate::protected::*;
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
      From_0 +
      fmt::Debug +
    ,
  {
    /// Type of changer of the context.
    type Changer : ChangerInterface;
    /// Get changer of the context.
    fn changer( &mut self ) -> Self::Changer;
  }

}

crate::mod_interface!
{

  prelude use ContextInterface;

}
