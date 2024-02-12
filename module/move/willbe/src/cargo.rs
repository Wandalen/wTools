mod private
{
  use crate::*;

  use std::path::Path;

  use process::CmdReport;
  use wtools::error::Result;

  ///
  /// Assemble the local package into a distributable tarball.
  ///
  /// # Args:
  /// - `path` - path to the package directory
  /// - `dry` - a flag that indicates whether to execute the command or not
  ///
  pub fn package< P >( path : P, dry : bool ) -> Result< CmdReport >
  where
    P : AsRef< Path >
  {
    let ( program, args ) = ( "cargo", [ "package" ] );

    if dry
    {
      Ok
      (
        CmdReport
        {
          command : format!( "{program} {}", args.join( " " ) ),
          path : path.as_ref().to_path_buf(),
          out : String::new(),
          err : String::new(),
        }
      )
    }
    else
    {
      process::start2_sync( program, args, path )
    }
  }
  
 /// Upload a package to the registry
  pub fn publish< P >( path : P, dry : bool ) -> Result< CmdReport >
  where
    P : AsRef< Path >
  {
    let ( program, args ) = ( "cargo", [ "publish" ] );

    if dry
    {
      Ok
      (
        CmdReport
        {
          command : format!( "{program} {}", args.join( " " ) ),
          path : path.as_ref().to_path_buf(),
          out : String::new(),
          err : String::new(),
        }
      )
    }
    else
    {
      process::start2_sync( program, args, path )
    }
  }
}

//

crate::mod_interface!
{
  protected use package;
  protected use publish;
}
