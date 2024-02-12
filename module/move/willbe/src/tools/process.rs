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
  use wtools::
  {
    iter::Itertools,
    error::{ anyhow::{ Context, format_err }, Result },
  };


  /// Process command output.
  #[ derive( Debug, Clone ) ]
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
        f.write_fmt( format_args!( "  path: {}\n  {}\n", self.path.display(), self.err.replace( '\n', "\n  " ) ) )?;
      }

      Ok( () )
    }
  }

  ///
  /// Run external processes.
  ///

  pub fn start_sync
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

    start2_sync( program, args, current_path )
  }

  ///
  /// Run external processes.
  ///
  /// # Args:
  /// - `application` - path to executable application
  /// - `args` - command-line arguments to the application
  /// - `path` - path to directory where to run the application
  ///
  pub fn start2_sync< AP, Args, Arg, P >
  (
    application : AP,
    args: Args,
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
}

//

crate::mod_interface!
{
  protected use CmdReport;
  protected use start_sync;
  protected use start2_sync;
}

