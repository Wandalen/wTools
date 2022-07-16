
wtools::meta::mod_interface!
{
  /// Perform smoke testing.
  #[ cfg( feature = "use_std" ) ]
  prelude mod smoke;
  /// Init aggregator commands.
  #[ cfg( feature = "use_std" ) ]
  prelude mod init;
}

#[ cfg( feature = "use_std" ) ]
pub use init::*;
