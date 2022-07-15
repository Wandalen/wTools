
//!
//! Library of utility to work with commands.
//!

#[ cfg( feature = "use_std" ) ]
wtools::meta::mod_interface!
{
  /// Publish module.
  #[ cfg( feature = "use_std" ) ]
  prelude mod publish;
  /// List packages.
  #[ cfg( feature = "use_std" ) ]
  prelude mod list;
  /// Init aggregator commands.
  #[ cfg( feature = "use_std" ) ]
  prelude mod init;
}
#[ cfg( feature = "use_std" ) ]
pub use init::*;
#[ cfg( not( feature = "use_std" ) ) ]
wtools::meta::mod_interface!
{
}
