
//!
//! Try building a program for negative testing.
//!

// use crate::*;

/// Internal namespace.
pub( crate ) mod private
{
  #[ doc( inline ) ]
  pub use ::trybuild::*;
}

//

crate::mod_interface!
{
  protected use
  {
    *
  };
}
