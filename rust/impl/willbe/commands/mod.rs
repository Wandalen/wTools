wtools::mod_interface!
{
  /// Init aggregator commands.
  prelude mod init;

  /// Information about package
  prelude mod info;

  /// Iterate over subject
  prelude mod each;
  
  protected use super::init::protected::*;
}
