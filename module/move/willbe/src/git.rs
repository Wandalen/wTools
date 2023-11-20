mod private
{
  use std::path::Path;
  use wca::wtools::Itertools;
  use crate::process;
  use crate::process::CmdReport;
  use crate::wtools::error::Result;

  /// Adds changes to the Git staging area.
  ///
  /// Args:
  /// - path - the root path
  /// - objects - a list of objects from the root that will be added
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
    let objects = objects.as_ref().iter().map( | x | x.as_ref() ).join( " " );
    let command = format!( "git add {objects}" );

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
      process::start_sync( &command, path.as_ref() )
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
    let command = format!( "git commit -m {}", message.as_ref() );

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
      process::start_sync( &command, path.as_ref() )
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
    let command = "git push";
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
      process::start_sync( command, path.as_ref() )
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