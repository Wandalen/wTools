
//!
//! Tools for testing.
//!

/// Internal namespace.
pub( crate ) mod private
{
}

// #[ cfg( not( feature = "no_std" ) ) ]
crate::mod_interface!
{
  layer compiletime;
  layer helper;
  layer smoke_test;
  layer version;
}
