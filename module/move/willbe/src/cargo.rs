mod private
{
  use std::path::Path;
  use crate::process;
  use crate::process::CmdReport;
  use crate::wtools::error::Result;

  /// Upload a package to the registry
  ///
  /// # Args:
  /// - `path` - path to the package directory
  /// - `dry` - a flag that indicates whether to execute the command or not
  ///
  pub fn publish< P >( path : P, dry : bool ) -> Result< CmdReport >
  where
    P : AsRef< Path >
  {
    let command = "cargo publish";

    if dry
    {
      Ok
      (
        CmdReport
        {
          command : command.to_string(),
          path : path.as_ref().to_path_buf(),
          out : String::new(),
          err : String::new(),
        }
      )
    }
    else
    {
      // qqq : for Bohdan : process::start_sync is overkill. sh is not needed. introduce process::start2_sync
      process::start_sync( command, path.as_ref() )
    }
  }
}

//

crate::mod_interface!
{
  protected use publish;
}
