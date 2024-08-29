/// Internal namespace.
#[ cfg( not( feature = "no_std" ) ) ]
mod private
{
  // use crate::own::*;
  use core::fmt;

  // use wtools::From_0;

  use crate::abs::{identity::private::HasIdInterface, changer::private::ChangerInterface};
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
      // From_0 +
      fmt::Debug +
    ,
  {
    /// Type of changer of the context.
    type Changer : ChangerInterface;
    /// Get changer of the context.
    fn changer( &mut self ) -> Self::Changer;
  }

}

#[ cfg( not( feature = "no_std" ) ) ]
::meta_tools::mod_interface!
{

  prelude use ContextInterface;

}
