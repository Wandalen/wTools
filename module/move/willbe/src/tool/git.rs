/// Internal namespace.
mod private
{
  #[ allow( unused_imports ) ]
  use crate::tool::*;

  use std::ffi::OsString;
  use std::path::Path;
  use process_tools::process::*;
  use error::err;
  // qqq : group dependencies

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
  // qqq : should be typed error, apply err_with
  #[ cfg_attr( feature = "tracing", tracing::instrument( skip( path, objects ), fields( path = %path.as_ref().display() ) ) ) ]
  pub fn add< P, Os, O >( path : P, objects : Os, dry : bool )
  -> error::untyped::Result< Report >
  // qqq : use typed error
  where
    P : AsRef< Path >,
    Os : AsRef< [ O ] >,
    O : AsRef< str >,
  {
    let objects = objects.as_ref().iter().map( | x | x.as_ref() );

    // qqq : for Bohdan : don't enlarge length of lines artificially
    let ( program, args ) : ( _, Vec< _ > ) = ( "git", Some( "add" ).into_iter().chain( objects ).collect() );

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
  // qqq : should be typed error, apply err_with
  #[ cfg_attr( feature = "tracing", tracing::instrument( skip( path, message ), fields( path = %path.as_ref().display(), message = %message.as_ref() ) ) ) ]
  pub fn commit< P, M >( path : P, message : M, dry : bool ) -> error::untyped::Result< Report >
  // qqq : don't use 1-prameter Result
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

  // qqq : should be typed error, apply err_with

  #[ cfg_attr( feature = "tracing", tracing::instrument( skip( path ), fields( path = %path.as_ref().display() ) ) ) ]
  pub fn push< P >( path : P, dry : bool ) -> error::untyped::Result< Report >
  // qqq : don't use 1-prameter Result
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

  /// This function is a wrapper around the `git reset` command.
  ///
  /// # Args :
  ///
  /// - `path`: The path to the directory on which the `git reset` command will be executed.
  /// - `hard`: A boolean indicating whether to perform a hard reset or not.
  /// - `commits_count`: The number of commits to reset(at least 1).
  /// - `dry`: A boolean indicating whether to execute the command in dry-run mode or not.
  ///
  /// # Returns :
  /// This function returns a `Result` containing a `Report` if the command is executed successfully. The `Report` contains the command executed, the output
  /// git reset command wrapper

  // qqq : should be typed error, apply err_with

  pub fn reset< P >( path : P, hard : bool, commits_count : usize, dry : bool )
  -> error::untyped::Result< Report >
  // qqq : don't use 1-prameter Result
  where
    P : AsRef< Path >,
  {
    if commits_count < 1 { return Err( err!( "Cannot reset, the count of commits must be greater than 0" ) ) }
    let ( program, args ) : ( _, Vec< _ > ) =
    (
      "git",
      Some( "reset" )
      .into_iter()
      .chain( if hard { Some( "--hard" ) } else { None } )
      .map( String::from )
      .chain( Some( format!( "HEAD~{}", commits_count ) ) )
      .collect()
    );

    if dry
    {
      Ok
      (
        Report
        {
          command : format!( "{program} {}", args.join( " " ) ),
          out : String::new(),
          err : String::new(),
          current_path : path.as_ref().to_path_buf(),
          error : Ok( () ),
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

  // qqq : should be typed error, apply err_with
  // qqq : don't use 1-prameter Result

  pub fn ls_remote_url< P >( path : P ) -> error::untyped::Result< Report >
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
  own use add;
  own use commit;
  own use push;
  own use reset;
  own use ls_remote_url;
}
