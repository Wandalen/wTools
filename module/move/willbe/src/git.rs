mod private
{
  use std::path::Path;
  use wca::wtools::Itertools;
  use crate::process;
  use crate::process::CmdReport;
  use crate::wtools::error::Result;

  /// Git add.
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

  /// Git commit.
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

  /// Git push.
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