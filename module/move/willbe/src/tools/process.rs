/// Internal namespace.
pub( crate ) mod private
{
  use crate::*;
  use std::fmt::Formatter;
  use std::path::{ Path, PathBuf };
  use std::process::
  {
    Command,
    Stdio,
  };
  use wca::wtools::Itertools;
  use wtools::error;


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
      // qqq : for Bohdan : why trim?
      f.write_fmt( format_args!( "> {}\n", self.command ) )?;
      if !self.out.trim().is_empty()
      {
        f.write_fmt( format_args!( "\t{}\n", self.out.replace( '\n', "\n\t" ) ) )?;
      }
      if !self.err.trim().is_empty()
      {
        f.write_fmt( format_args!( "\tpath: {}\n\t{}\n", self.path.display(), self.err.replace( '\n', "\n\t" ) ) )?;
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
  -> error::for_app::Result< CmdReport >
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
  -> error::for_app::Result< CmdReport >
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
    .expect( "failed to spawn process" );

    let output = child
    .wait_with_output()
    .expect( "failed to wait on child" );

    let report = CmdReport
    {
      command : format!( "{} {}", application.display(), args.iter().map( | a | a.to_string_lossy() ).join( " " ) ),
      path : path.to_path_buf(),
      out : String::from_utf8( output.stdout ).expect( "Found invalid UTF-8" ),
      err : String::from_utf8( output.stderr ).expect( "Found invalid UTF-8" ),
    };

    if output.status.success()
    {
      Ok( report )
    }
    else
    {
      Err( error::for_app::anyhow!( report ) )
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

