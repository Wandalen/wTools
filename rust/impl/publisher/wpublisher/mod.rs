
//!
//! Library of utility to operate packages from a command line.
//!

crate::mod_interface!
{
  #[ cfg( feature = "use_std" ) ]
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
