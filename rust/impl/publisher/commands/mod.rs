
//!
//! Library of utility to work with commands.
//!

#[ cfg( feature = "use_std" ) ]
crate::mod_interface!
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

  #[ cfg( feature = "use_std" ) ]
  protected use super::init::*;
}
#[ cfg( not( feature = "use_std" ) ) ]
crate::mod_interface!
{
}

// qqq : for Dima : remove. that could be inside mod_interface
// #[ cfg( feature = "use_std" ) ]
// pub use init::*;
