
//!
//! Tools for testing.
//!

// use super::*;

/// Internal namespace.
pub( crate ) mod private
{
}

//
#[ cfg( not( feature = "no_std" ) ) ]
crate::mod_interface!
{
  layer helper;
  layer smoke_test;
  layer compiletime;
}
