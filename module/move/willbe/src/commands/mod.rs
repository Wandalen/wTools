
crate::mod_interface!
{
  /// Init aggregator commands.
  prelude mod init;

  protected use super::init::protected::*;
}

// qqq : for Dima : remove. that could be inside mod_interface /* aaa : Dmytro : done */
// #[ cfg( not( feature = "no_std" ) ) ]
// pub use init::*;
