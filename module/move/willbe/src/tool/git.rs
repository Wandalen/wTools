mod private
{
  use crate::*;
  use std::ffi::OsString;
  use std::path::Path;
  use process_tools::process::*;
  use wtools::error::Result;
  use wtools::error::err;

  /// Adds changes to the Git staging area.
  ///
  /// # Args :
  /// - `path` - the root path
  /// - `objects` - a list of paths from the root that will be added
  /// - `dry` - a flag that indicates whether to apply the changes or not
  ///         - `true` - does not modify git state
  ///         - `false` - adds a change in the working directory to the staging area
  ///
  /// # Returns :
  /// Returns a result containing a report indicating the result of the operation.
  #[ cfg_attr( feature = "tracing", tracing::instrument( skip( path, objects ), fields( path = %path.as_ref().display() ) ) ) ]
  pub fn add< P, Os, O >( path : P, objects : Os, dry : bool ) -> Result< Report >
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
        Report
        {
          command : format!( "{program} {}", args.join( " " ) ),
          out : String::new(),
          err : String::new(),
          current_path: path.as_ref().to_path_buf(),
          error: Ok( () ),
        }
      )
    }
    else
    {
      Run::former()
      .bin_path( program )
      .args( args.into_iter().map( OsString::from ).collect::< Vec< _ > >() )
      .current_path( path.as_ref().to_path_buf() )
      .run().map_err( | report | err!( report.to_string() ) )
    }
  }

  /// Commits changes to the Git repository.
  ///
  /// # Args :
  ///
  /// - `path` - the root path
  /// - `message` - a commit message describing the changes
  /// - `dry` - a flag that indicates whether to apply the changes or not
  ///   - `true` - does not modify the Git state
  ///   - `false` - commits changes to the repository
  ///
  /// # Returns :
  /// Returns a result containing a report indicating the result of the operation.
  #[ cfg_attr( feature = "tracing", tracing::instrument( skip( path, message ), fields( path = %path.as_ref().display(), message = %message.as_ref() ) ) ) ]
  pub fn commit< P, M >( path : P, message : M, dry : bool ) -> Result< Report >
  where
    P : AsRef< Path >,
    M : AsRef< str >,
  {
    let ( program, args ) = ( "git", [ "commit", "-m", message.as_ref() ] );

    if dry
    {
      Ok
      (
        Report
        {
          command : format!( "{program} {}", args.join( " " ) ),
          out : String::new(),
          err : String::new(),
          current_path: path.as_ref().to_path_buf(),
          error: Ok( () ),
        }
      )
    }
    else
    {
      Run::former()
      .bin_path( program )
      .args( args.into_iter().map( OsString::from ).collect::< Vec< _ > >() )
      .current_path( path.as_ref().to_path_buf() )
      .run().map_err( | report | err!( report.to_string() ) )
    }
  }

  /// Pushes changes to the remote Git repository.
  ///
  /// # Args :
  ///
  /// - `path` - the root path
  /// - `dry` - a flag that indicates whether to apply the changes or not
  ///   - `true` - does not modify the Git state
  ///   - `false` - pushes changes to the remote repository
  ///
  /// # Returns :
  /// Returns a result containing a report indicating the result of the operation.
  #[ cfg_attr( feature = "tracing", tracing::instrument( skip( path ), fields( path = %path.as_ref().display() ) ) ) ]
  pub fn push< P >( path : P, dry : bool ) -> Result< Report >
  where
    P : AsRef< Path >,
  {
    let ( program, args ) = ( "git", [ "push" ] );

    if dry
    {
      Ok
      (
        Report
        {
          command : format!( "{program} {}", args.join( " " ) ),
          out : String::new(),
          err : String::new(),
          current_path: path.as_ref().to_path_buf(),
          error: Ok( () ),
        }
      )
    }
    else
    {
      Run::former()
      .bin_path( program )
      .args( args.into_iter().map( OsString::from ).collect::< Vec< _ > >() )
      .current_path( path.as_ref().to_path_buf() )
      .run().map_err( | report | err!( report.to_string() ) )
    }
  }

  /// Retrieves the remote URL of a Git repository.
  ///
  /// # Arguments
  ///
  /// * `path` - A `Path` reference to the local Git repository.
  ///
  /// # Returns
  ///
  /// A `Result` containing a `Report`, which represents the result of the command execution.
  pub fn ls_remote_url< P >( path : P ) -> Result< Report >
  where
    P : AsRef< Path >,
  {
    let ( program, args ) = ( "git", [ "ls-remote", "--get-url" ] );

    Run::former()
    .bin_path( program )
    .args( args.into_iter().map( OsString::from ).collect::< Vec< _ > >() )
    .current_path( path.as_ref().to_path_buf() )
    .run().map_err( | report | err!( report.to_string() ) )
  }
}

//

crate::mod_interface!
{
  protected use add;
  protected use commit;
  protected use push;
  protected use ls_remote_url;
}
