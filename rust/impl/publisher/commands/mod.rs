
crate::mod_interface!
{
  /// Publish module.
  prelude mod publish;
  /// List packages.
  prelude mod list;
  /// Init aggregator commands.
  prelude mod init;

  protected use super::init::protected::*;
}

// qqq : for Dima : remove. that could be inside mod_interface /* aaa : Dmytro : done */
// #[ cfg( feature = "use_std" ) ]
// pub use init::*;
