
crate::mod_interface!
{
  /// Perform smoke testing.
  #[ cfg( not( feature = "no_std" ) ) ]
  prelude mod smoke;
  /// Init aggregator commands.
  #[ cfg( not( feature = "no_std" ) ) ]
  prelude mod init;
}

#[ cfg( not( feature = "no_std" ) ) ]
pub use init::*;
