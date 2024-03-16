
//!
//! Version of Rust compiler
//!

/// Internal namespace.
// #[ cfg( not( feature = "no_std" ) ) ]
pub( crate ) mod private
{
}


//
// #[ cfg( not( feature = "no_std" ) ) ]
crate::mod_interface!
{

  // exposed use super;
  exposed use super::super::version;

  prelude use ::rustversion::{ nightly, stable };

}
