/// Define a private namespace for all its items.
#[ allow( clippy ::std_instead_of_alloc, clippy ::std_instead_of_core ) ]
mod private
{
  use std ::
  {
    fmt ::Formatter,
    path ::{ Path, PathBuf },
    process ::{ Command, Output, Stdio },
  };
  use std ::collections ::HashMap;
  use std ::ffi ::OsString;
  use duct ::cmd;
  use error_tools ::
  {
    untyped ::{ Error, Context, format_err },
  };
  use former ::Former;
  use ::itertools ::Itertools;

  ///
  /// Executes a process with the given parameters.
  ///
  /// Routes to the duct backend when `joining_streams` is true (stderr merged into stdout),
  /// or to `std::process::Command` when false (separate stdout and stderr capture).
  ///
  fn execute
  (
    bin_path : &Path,
    args : &[ OsString ],
    current_path : &Path,
    joining_streams : bool,
    env : HashMap< String, String >,
  ) -> Result< Output, Error >
  {
    if joining_streams
    {
      cmd( bin_path.as_os_str(), args )
      .dir( current_path )
      .full_env( env )
      .stderr_to_stdout()
      .stdout_capture()
      .unchecked()
      .run()
      .map_err( Into ::into )
    }
    else
    {
      Command ::new( bin_path )
      .args( args )
      .envs( env )
      .stdout( Stdio ::piped() )
      .stderr( Stdio ::piped() )
      .current_dir( current_path )
      .spawn()
      .context( "failed to spawn process" )?
      .wait_with_output()
      .context( "failed to wait on child" )
    }
  }

  ///
  /// Executes an external process in a specified directory without using a shell.
  ///
  /// # Arguments
  /// - `options` : Configured [`Run`] describing the process to execute.
  ///
  /// # Returns
  /// `Ok( Report )` on successful execution (exit code zero).
  /// `Err( Report )` when the process fails to spawn, exits non-zero, or produces non-UTF-8 output.
  ///
  /// # Errors
  /// Returns `Err( Report )` if:
  /// - The binary cannot be found or spawned.
  /// - The process exits with a non-zero exit code.
  /// - Captured output is not valid UTF-8.
  ///
  pub fn run( options : Run ) -> Result< Report, Report >
  {
    // Destructure upfront to avoid partial-move conflicts when consuming env_variable.
    let Run { bin_path, current_path, args, joining_streams, env_variable } = options;
    let bin_path_ref : &Path = bin_path.as_ref();
    let current_path_ref : &Path = current_path.as_ref();

    let mut env : HashMap< String, String > = std ::env ::vars().collect();
    env.extend( env_variable );

    let mut report = Report
    {
      command : format!( "{} {}", bin_path_ref.display(), args.iter().map( | a | a.to_string_lossy() ).join( " " ) ),
      current_path : current_path_ref.to_path_buf(),
      ..Report ::default()
    };

    let output = match execute( bin_path_ref, &args, current_path_ref, joining_streams, env )
    {
      Ok( o ) => o,
      Err( e ) =>
      {
        report.error = Err( e );
        return Err( report );
      }
    };

    let out = match String ::from_utf8( output.stdout ).context( "Found invalid UTF-8" )
    {
      Ok( s ) => s,
      Err( e ) =>
      {
        report.error = Err( e );
        return Err( report );
      }
    };
    report.out = out;

    let err = match String ::from_utf8( output.stderr ).context( "Found invalid UTF-8" )
    {
      Ok( s ) => s,
      Err( e ) =>
      {
        report.error = Err( e );
        return Err( report );
      }
    };
    report.err = err;

    if output.status.success()
    {
      Ok( report )
    }
    else
    {
      report.error = Err( format_err!( "Process was finished with error code: {}", output.status ) );
      Err( report )
    }
  }

  /// Options for the [`run`] function.
  #[ derive( Debug, Former ) ]
  pub struct Run
  {
    /// Path to the executable to run.
    bin_path : PathBuf,
    /// Working directory for the process.
    current_path : PathBuf,
    /// Command-line arguments passed to the executable.
    args : Vec< OsString >,
    /// When `true`, stderr is merged into stdout via duct. When `false`, streams are captured separately.
    #[ former( default = false ) ]
    joining_streams : bool,
    /// Additional environment variables merged on top of the current process environment.
    env_variable : HashMap< String, String >,
  }

  impl RunFormer
  {
    /// Execute the configured process, returning a [`Report`].
    ///
    /// # Returns
    /// `Ok( Report )` on zero exit code, `Err( Report )` on failure.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use process_tools ::process ::Run;
    /// let report = Run ::former()
    /// .bin_path( "echo" )
    /// .args( vec![ "hello".into() ] )
    /// .current_path( "." )
    /// .run()
    /// .expect( "echo should succeed" );
    ///
    /// assert!( report.out.contains( "hello" ) );
    /// ```
    pub fn run( self ) -> Result< Report, Report >
    {
      run( self.form() )
    }

    /// Executes an external process using the system shell.
    ///
    /// Abstracts over shell differences between Windows (`cmd /C`) and Unix (`sh -c`),
    /// enabling shell features such as pipes and redirections in the command string.
    ///
    /// # Arguments
    /// - `exec_path` : The shell command string to execute.
    ///
    /// # Returns
    /// `Ok( Report )` on zero exit code, `Err( Report )` on failure.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use process_tools ::process ::Run;
    /// let report = Run ::former()
    /// .current_path( "." )
    /// .run_with_shell( "echo hello | grep hello" )
    /// .expect( "piped command should succeed" );
    ///
    /// assert!( report.out.contains( "hello" ) );
    /// ```
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
      .args( args.into_iter().map( OsString ::from ).collect ::< Vec< _ > >() )
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
    /// Captured stdout.
    pub out : String,
    /// Captured stderr.
    pub err : String,
    /// `Ok(())` if the process succeeded, `Err` with the failure reason otherwise.
    pub error : Result< (), Error >,
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
        // Error is not Clone; stringify to preserve the message across the clone boundary.
        error : self.error.as_ref().map_err( | e | Error ::msg( e.to_string() ) ).copied(),
      }
    }
  }

  impl Default for Report
  {
    fn default() -> Self
    {
      Report
      {
        command : String ::default(),
        current_path : PathBuf ::new(),
        out : String ::default(),
        err : String ::default(),
        error : Ok( () ),
      }
    }
  }

  impl core ::fmt ::Display for Report
  {
    fn fmt( &self, f : &mut Formatter< '_ > ) -> core ::fmt ::Result
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

crate ::mod_interface!
{
  own use run;
  own use Run;
  own use Report;
}
