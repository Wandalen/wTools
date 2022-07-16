
//!
//! Library of utility to operate packages from a command line.
//!

#[ cfg( feature = "use_std" ) ]
wtools::meta::mod_interface!
{
  prelude mod bool;
  prelude mod files;
  prelude mod http;
  prelude mod manifest;
  prelude mod process;
  prelude mod digest;
}
#[ cfg( not( feature = "use_std" ) ) ]
wtools::meta::mod_interface!
{
}
