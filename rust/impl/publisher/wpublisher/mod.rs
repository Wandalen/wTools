
//!
//! Library of utility to operate packages from a command line.
//!

#[ cfg( feature = "use_std" ) ]
crate::mod_interface!
{
  orphan mod
  {
    bool,
    files,
    http,
    manifest,
    process,
    digest,
  };
}
#[ cfg( not( feature = "use_std" ) ) ]
crate::mod_interface!
{
}
