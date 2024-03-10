/// Internal namespace.
pub( crate ) mod private
{
  use crate::*;

  use std::
  {
    fmt::Formatter,
    path::{ Path, PathBuf },
    process::{ Command, Stdio },
  };
  use duct::cmd;
  use error_tools::err;
  use error_tools::for_app::Error;
  use wtools::
  {
    iter::Itertools,
    error::{ anyhow::{ Context, format_err }, Result },
  };


  /// Process command output.
  #[ derive( Debug, Clone, Default ) ]
  pub struct CmdReport
  {
    /// Command that was executed.
    pub command : String,
    /// Path where command was executed.
    pub path : PathBuf,
    /// Stdout.
    pub out : String,
    /// Stderr.
    pub err : String,
  }

  impl std::fmt::Display for CmdReport
  {
    fn fmt( &self, f : &mut Formatter< '_ > ) -> std::fmt::Result
    {
      // Trim prevents writing unnecessary whitespace or empty lines
      f.write_fmt( format_args!( "> {}\n", self.command ) )?;
      if !self.out.trim().is_empty()
      {
        f.write_fmt( format_args!( "  {}\n", self.out.replace( '\n', "\n  " ) ) )?;
      }
      if !self.err.trim().is_empty()
      {
        f.write_fmt( format_args!( "  path : {}\n  {}\n", self.path.display(), self.err.replace( '\n', "\n  " ) ) )?;
      }

      Ok( () )
    }
  }

  ///
  /// Executes an external process using the system shell.
  ///
  /// This function abstracts over the differences between shells on Windows and Unix-based
  /// systems, allowing for a unified interface to execute shell commands.
  ///
  /// # Parameters:
  /// - `exec_path`: The command line string to execute in the shell.
  /// - `current_path`: The working directory path where the command is executed.
  ///
  /// # Returns:
  /// A `Result` containing a `CmdReport` on success, which includes the command's output,
  /// or an error if the command fails to execute or complete.
  ///
  /// # Examples:
  /// ```rust
  /// use willbe::process;
  ///
  /// let report = process::run_with_shell( "echo Hello World", "." ).unwrap();
  /// println!( "{}", report.out );
  /// ```
  ///

  pub fn run_with_shell
  (
    exec_path : &str,
    current_path : impl Into< PathBuf >,
  )
  -> Result< CmdReport >
  {
    let current_path = current_path.into();
    let ( program, args ) =
    if cfg!( target_os = "windows" )
    {
      ( "cmd", [ "/C", exec_path ] )
    }
    else
    {
      ( "sh", [ "-c", exec_path ] )
    };

    run(program, args, current_path )
  }

  ///
  /// Executes an external process in a specified directory without using a shell.
  ///
  /// # Arguments:
  /// - `application`: Path to the executable application.
  /// - `args`: Command-line arguments for the application.
  /// - `path`: Directory path to run the application in.
  ///
  /// # Returns:
  /// A `Result` containing `CmdReport` on success, detailing execution output,
  /// or an error message on failure.
  ///
  /// # Errors:
  /// Returns an error if the process fails to spawn, complete, or if output
  /// cannot be decoded as UTF-8.
  ///
  /// # Example
  /// ```rust
  /// use std::path::Path;
  /// use willbe::process;
  ///
  /// let command = if cfg!( target_os = "windows" ) { "dir" } else { "ls" };
  /// let args : [ String ; 0 ] = [];
  /// let path = ".";
  ///
  /// let report = process::run( command, args, Path::new( path ) ).unwrap();
  /// println!( "Command output: {}", report.out );
  /// ```
  pub fn run< AP, Args, Arg, P >
  (
    application : AP,
    args : Args,
    path : P,
  )
  -> Result< CmdReport >
  where
    AP : AsRef< Path >,
    Args : IntoIterator< Item = Arg >,
    Arg : AsRef< std::ffi::OsStr >,
    P : AsRef< Path >,
  {
    let ( application, path ) = ( application.as_ref(), path.as_ref() );
    let args = args.into_iter().map( | a | a.as_ref().into() ).collect::< Vec< std::ffi::OsString > >();

    let child = Command::new( application )
    .args( &args )
    .stdout( Stdio::piped() )
    .stderr( Stdio::piped() )
    .current_dir( path )
    .spawn()
    .context( "failed to spawn process" )?;

    let output = child
    .wait_with_output()
    .context( "failed to wait on child" )?;

    let report = CmdReport
    {
      command : format!( "{} {}", application.display(), args.iter().map( | a | a.to_string_lossy() ).join( " " ) ),
      path : path.to_path_buf(),
      out : String::from_utf8( output.stdout ).context( "Found invalid UTF-8" )?,
      err : String::from_utf8( output.stderr ).context( "Found invalid UTF-8" )?,
    };

    if output.status.success()
    {
      Ok( report )
    }
    else
    {
      Err( format_err!( report ) )
    }
  }

  ///
  /// Run external processes. Natural ordered out will be in std::out (std::err - None)
  ///
  /// # Args :
  /// - `application` - path to executable application
  /// - `args` - command-line arguments to the application
  /// - `path` - path to directory where to run the application
  ///
  pub fn process_run_with_param_and_joined_steams< AP, Args, Arg, P >
  (
    application : AP,
    args : Args,
    path : P,
  )
  -> Result< CmdReport, ( CmdReport, Error ) >
  where
    AP : AsRef< Path >,
    Args : IntoIterator< Item = Arg >,
    Arg : AsRef< std::ffi::OsStr >,
    P : AsRef< Path >,
  {
    let ( application, path ) = ( application.as_ref(), path.as_ref() );
    let args = args.into_iter().map( | a | a.as_ref().into() ).collect::< Vec< std::ffi::OsString > >();
    let output = cmd( application.as_os_str(), &args )
    .dir( path )
    .stderr_to_stdout()
    .stdout_capture()
    .unchecked()
    .run()
    .map_err( | e | ( Default::default(), e.into() ) )?;
    let report = CmdReport
    {
      command : format!( "{} {}", application.display(), args.iter().map( | a | a.to_string_lossy() ).join( " " ) ),
      path : path.to_path_buf(),
      out : String::from_utf8( output.stdout ).context( "Found invalid UTF-8" ).map_err( | e | ( Default::default(), e.into() ) )?,
      err : Default::default(),
    };

    if output.status.success()
    {
      Ok( report )
    }
    else
    {
      Err( ( report, err!( "Process was finished with error code : {}", output.status ) ) )
    }
  }

}

//

crate::mod_interface!
{
  protected use CmdReport;
  protected use run_with_shell;
  protected use run;
  protected use process_run_with_param_and_joined_steams;
  // qqq : for Petro : rid off process_run_with_param_and_joined_steams
  // add functionality of process_run_with_param_and_joined_steams under option/argument into process::run
}
