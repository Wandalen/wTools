
//!
//! Try building a program for negative testing.
//!

/// Internal namespace.
pub( crate ) mod private
{
  #[ doc( inline ) ]
  pub use ::trybuild::*;
}

//

crate::mod_interface!
{
  // xxx : make it working
  // exposed use super;
  exposed use super::super::compiletime;
  protected use
  {
    *
  };
}
