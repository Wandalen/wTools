mod private
{
  use std::path::Path;
  use crate::process;
  use crate::process::CmdReport;
  use crate::wtools::error::Result;

  /// Adds changes to the Git staging area.
  ///
  /// Args:
  /// - path - the root path
  /// - objects - a list of paths from the root that will be added
  /// - dry - a flag that indicates whether to apply the changes or not
  ///         - true - does not modify git state
  ///         - false - adds a change in the working directory to the staging area
  ///
  /// Returns:
  /// Returns a result containing a report indicating the result of the operation.
  pub fn add< P, Os, O >( path : P, objects : Os, dry : bool ) -> Result< CmdReport >
  where
    P : AsRef< Path >,
    Os : AsRef< [ O ] >,
    O : AsRef< str >,
  {
    let objects = objects.as_ref().iter().map( | x | x.as_ref() );

    let ( program, args ) = ( "git", Some( "add" ).into_iter().chain( objects ).collect::< Vec< _ > >() );

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
      // qqq : for Bohdan : process::start_sync is overkill. sh is not needed. introduce process::start2_sync
      process::start2_sync( program, args, path )
    }
  }

  /// Commits changes to the Git repository.
  ///
  /// Args:
  ///
  /// - path - the root path
  /// - message - a commit message describing the changes
  /// - dry - a flag that indicates whether to apply the changes or not
  ///   - true - does not modify the Git state
  ///   - false - commits changes to the repository
  ///
  /// Returns:
  /// Returns a result containing a report indicating the result of the operation.
  pub fn commit< P, M >( path : P, message : M, dry : bool ) -> Result< CmdReport >
  where
    P : AsRef< Path >,
    M : AsRef< str >,
  {
    let ( program, args ) = ( "git", [ "commit", "-m", message.as_ref() ] );

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
      // qqq : for Bohdan : process::start_sync is overkill. sh is not needed. introduce process::start2_sync
      process::start2_sync( program, args, path )
    }
  }

  /// Pushes changes to the remote Git repository.
  ///
  /// Args:
  ///
  /// - path - the root path
  /// - dry - a flag that indicates whether to apply the changes or not
  ///   - true - does not modify the Git state
  ///   - false - pushes changes to the remote repository
  ///
  /// Returns:
  /// Returns a result containing a report indicating the result of the operation.
  pub fn push< P >( path : P, dry : bool ) -> Result< CmdReport >
  where
    P : AsRef< Path >,
  {
    let ( program, args ) = ( "git", [ "push" ] );

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
  protected use add;
  protected use commit;
  protected use push;
}