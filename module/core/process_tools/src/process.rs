/// Internal namespace.
pub( crate ) mod private
{
  // use crate::*;

  use std::
  {
    fmt::Formatter,
    path::{ Path, PathBuf },
    process::{ Command, Stdio },
  };
  use std::collections::HashMap;
  use std::ffi::OsString;
  use duct::cmd;
  use error_tools::
  {
    untyped::{ Error, Context, anyhow },
    // Result,
  };
  use former::Former;
  use iter_tools::iter::Itertools;

  // ///
  // /// Executes an external process using the system shell.
  // ///
  // /// This function abstracts over the differences between shells on Windows and Unix-based
  // /// systems, allowing for a unified interface to execute shell commands.
  // ///
  // /// # Parameters:
  // /// - `exec_path`: The command line string to execute in the shell.
  // /// - `current_path`: The working directory current_path where the command is executed.
  // ///
  // /// # Returns:
  // /// A `Result` containing a `Report` on success, which includes the command's output,
  // /// or an error if the command fails to execute or complete.
  // ///
  // /// # Examples:
  // /// ```rust
  // /// use process_tools::process;
  // ///
  // /// let report = process::run_with_shell( "echo Hello World", "." ).unwrap();
  // /// println!( "{}", report.out );
  // /// ```
  // ///
  //
  // pub fn run_with_shell
  // (
  //   exec_path : &str,
  //   current_path : impl Into< PathBuf >,
  // )
  // -> Result< Report, Report >
  // {
  //   let current_path = current_path.into();
  //   let ( program, args ) =
  //   if cfg!( target_os = "windows" )
  //   {
  //     ( "cmd", [ "/C", exec_path ] )
  //   }
  //   else
  //   {
  //     ( "sh", [ "-c", exec_path ] )
  //   };
  //   let options = Run::former()
  //   .bin_path( program )
  //   .args( args.into_iter().map( OsString::from ).collect::< Vec< _ > >() )
  //   .current_path( current_path )
  //   .form();
  //   // xxx : qqq : for Petro : implement run for former та для Run
  //   run( options )
  // }

  ///
  /// Executes an external process in a specified directory without using a shell.
  ///
  /// # Arguments:
  /// - `bin_path`: Path to the executable bin_path.
  /// - `args`: Command-line arguments for the bin_path.
  /// - `current_path`: Directory current_path to run the bin_path in.
  ///
  /// # Returns:
  /// A `Result` containing `Report` on success, detailing execution output,
  /// or an error message on failure.
  ///
  /// # Errors:
  /// Returns an error if the process fails to spawn, complete, or if output
  /// cannot be decoded as UTF-8.
  //
  // qqq : for Petro : use typed error
  // qqq : for Petro : write example
  pub fn run( options : Run ) -> Result< Report, Report >
  {
    let bin_path : &Path = options.bin_path.as_ref();
    let current_path : &Path = options.current_path.as_ref();

    let mut report = Report
    {
      command : format!( "{} {}", bin_path.display(), options.args.iter().map( | a | a.to_string_lossy() ).join( " " ) ),
      current_path : current_path.to_path_buf(),
      .. Report::default()
    };

    let mut env: HashMap<String, String> = std::env::vars().collect();
    env.extend( options.env_variable );

    let output = if options.joining_streams
    {
      let output = cmd( bin_path.as_os_str(), &options.args )
      .dir( current_path )
      .full_env( env )
      .stderr_to_stdout()
      .stdout_capture()
      .unchecked()
      .run()
      .map_err( | e |
      {
        report.error = Err( e.into() );
        Err::< (), () >( () )
      });

      output
    }
    else
    {
      let child = Command::new( bin_path )
      .args( &options.args )
      .envs( env )
      .stdout( Stdio::piped() )
      .stderr( Stdio::piped() )
      .current_dir( current_path )
      .spawn()
      .context( "failed to spawn process" )
      .map_err( | e |
      {
        report.error = Err( e.into() );
        Err::< (), () >( () )
      });

      if report.error.is_err()
      {
        return Err( report );
      }
      let child = child.unwrap();

      let output = child
      .wait_with_output()
      .context( "failed to wait on child" )
      .map_err( | e |
      {
        report.error = Err( e.into() );
        Err::< (), () >( () )
      });

      output
    };

    if report.error.is_err()
    {
      return Err( report );
    }
    let output = output.unwrap();

    let out = String::from_utf8( output.stdout )
    .context( "Found invalid UTF-8" )
    .map_err( | e |
    {
      report.error = Err( e.into() );
      Err::< (), () >( () )
    });

    if out.is_err()
    {
      return Err( report );
    }
    let out = out.unwrap();

    report.out = out;

    let err = String::from_utf8( output.stderr )
    .context( "Found invalid UTF-8" )
    .map_err( | e |
      {
        report.error = Err( e.into() );
        Err::< (), () >( () )
      });

    if err.is_err()
    {
      return Err( report );
    }
    let err = err.unwrap();

    report.err = err;

    if output.status.success()
    {
      Ok( report )
    }
    else
    {
      report.error = Err( anyhow!( "Process was finished with error code : {}", output.status ) );
      Err( report )
    }

  }

  /// Option for `run` function
  #[ derive( Debug, Former ) ]
  // #[ debug ]
  pub struct Run
  {
    bin_path : PathBuf,
    current_path : PathBuf,
    args : Vec< OsString >,
    #[ former( default = false ) ]
    joining_streams : bool,
    env_variable : HashMap< String, String >,
  }

  impl RunFormer
  {
    pub fn run( self ) -> Result< Report, Report >
    {
      run( self.form() )
    }

    /// Executes an external process using the system shell.
    ///
    /// This function abstracts over the differences between shells on Windows and Unix-based
    /// systems, allowing for a unified interface to execute shell commands.
    ///
    /// # Parameters:
    /// - `exec_path`: The command line string to execute in the shell.
    ///
    /// # Returns:
    /// A `Result` containing a `Report` on success, which includes the command's output,
    /// or an error if the command fails to execute or complete.
    pub fn run_with_shell( self, exec_path : &str, ) -> Result< Report, Report >
    {
      let ( program, args ) =
      if cfg!( target_os = "windows" )
      {
        ( "cmd", [ "/C", exec_path ] )
      }
      else
      {
        ( "sh", [ "-c", exec_path ] )
      };
      self
      .args( args.into_iter().map( OsString::from ).collect::< Vec< _ > >() )
      .bin_path( program )
      .run()
    }
  }

  /// Process command output.
  #[ derive( Debug, ) ]
  pub struct Report
  {
    /// Command that was executed.
    pub command : String,
    /// Path where command was executed.
    pub current_path : PathBuf,
    /// Stdout.
    pub out : String,
    /// Stderr.
    pub err : String,
    /// Error if any
    pub error : Result< (), Error >
  }

  impl Clone for Report
  {
    fn clone( &self ) -> Self
    {
      Self
      {
        command : self.command.clone(),
        current_path : self.current_path.clone(),
        out : self.out.clone(),
        err : self.err.clone(),
        error : self.error.as_ref().map_err( | e | Error::msg( e.to_string() ) ).copied(),
        // error : self.error.as_ref().map_err( | e | Error::new( e ) ).copied(),
      }
    }
  }

  impl Default for Report
  {
    fn default() -> Self
    {
      Report
      {
        command : Default::default(),
        current_path : PathBuf::new(),
        out : Default::default(),
        err : Default::default(),
        error : Ok( () ),
      }
    }
  }

  impl core::fmt::Display for Report
  {
    fn fmt( &self, f : &mut Formatter< '_ > ) -> core::fmt::Result
    {
      // Trim prevents writing unnecessary whitespace or empty lines
      f.write_fmt( format_args!( "> {}\n", self.command ) )?;
      f.write_fmt( format_args!( "  @ {}\n\n", self.current_path.display() ) )?;

      if !self.out.trim().is_empty()
      {
        f.write_fmt( format_args!( "  {}\n", self.out.replace( '\n', "\n  " ) ) )?;
      }
      if !self.err.trim().is_empty()
      {
        f.write_fmt( format_args!( "  {}\n", self.err.replace( '\n', "\n  " ) ) )?;
      }

      Ok( () )
    }
  }

}

crate::mod_interface!
{
  // protected use run_with_shell;
  protected use run;
  protected use Run;
  protected use Report;
}
