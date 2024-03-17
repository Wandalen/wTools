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
  use std::ffi::OsString;
  use duct::cmd;
  use error_tools::err;
  use error_tools::for_app::Error;
  use former::Former;
  use wtools::
  {
    iter::Itertools,
    error::{ anyhow::Context, Result },
  };

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
  /// A `Result` containing a `Report` on success, which includes the command's output,
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
  -> Result< Report, ( Report, Error ) >
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
    let options = Run::former()
    .application( program )
    .args( args.into_iter().map( OsString::from ).collect::< Vec< _ > >() )
    .path( current_path )
    .form();
    // xxx : qqq : for Petro : implement run for former та для Run
    run( options )
  }

  /// Process command output.
  #[ derive( Debug, Clone, Default ) ]
  pub struct Report
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

  impl std::fmt::Display for Report
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

  /// Option for `run` function
  #[ derive( Debug, Former ) ]
  pub struct Run
  {
    application : PathBuf,
    args : Vec< OsString >,
    path : PathBuf,
    #[ default( false ) ]
    joining_streams : bool,
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
  /// A `Result` containing `Report` on success, detailing execution output,
  /// or an error message on failure.
  ///
  /// # Errors:
  /// Returns an error if the process fails to spawn, complete, or if output
  /// cannot be decoded as UTF-8.
  pub fn run( options : Run ) -> Result< Report, ( Report, Error ) >
  {
    let application : &Path = options.application.as_ref();
    let path : &Path = options.path.as_ref();

    if options.joining_streams
    {
      let output = cmd( application.as_os_str(), &options.args )
      .dir( path )
      .stderr_to_stdout()
      .stdout_capture()
      .unchecked()
      .run()
      .map_err( | e | ( Default::default(), e.into() ) )?;

      let report = Report
      {
        command : format!( "{} {}", application.display(), options.args.iter().map( | a | a.to_string_lossy() ).join( " " ) ),
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
    else
    {
      let child = Command::new( application )
      .args( &options.args )
      .stdout( Stdio::piped() )
      .stderr( Stdio::piped() )
      .current_dir( path )
      .spawn()
      .context( "failed to spawn process" )
      .map_err( | e | ( Default::default(), e.into() ) )?;

      let output = child
      .wait_with_output()
      .context( "failed to wait on child" )
      .map_err( | e | ( Default::default(), e.into() ) )?;

      let report = Report
      {
        command : format!( "{} {}", application.display(), options.args.iter().map( | a | a.to_string_lossy() ).join( " " ) ),
        path : path.to_path_buf(),
        out : String::from_utf8( output.stdout ).context( "Found invalid UTF-8" ).map_err( | e | ( Default::default(), e.into() ) )?,
        err : String::from_utf8( output.stderr ).context( "Found invalid UTF-8" ).map_err( | e | ( Default::default(), e.into() ) )?,
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
}

crate::mod_interface!
{
  protected use Report;
  protected use run_with_shell;
  protected use run;
  protected use Run;
}
