/// Internal namespace.
pub( crate ) mod private
{
  use std::path::PathBuf;
  use std::process::
  {
    Command,
    Stdio,
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

  ///
  /// Run external processes.
  ///

  pub fn start_sync
  (
    exec_path : &str,
    current_path : impl Into< std::path::PathBuf >,
  ) -> anyhow::Result< CmdReport >
  {
    let current_path = current_path.into();

    let child = if cfg!( target_os = "windows" )
    {
      Command::new( "cmd" )
      .args( [ "/C", exec_path ] )
      .stdout( Stdio::piped() )
      .stderr( Stdio::piped() )
      .current_dir( &current_path )
      .spawn()
      .expect( "failed to spawn process" )
    }
    else
    {
      Command::new( "sh" )
      .args( [ "-c", exec_path ] )
      .stdout( Stdio::piped() )
      .stderr( Stdio::piped() )
      .current_dir( &current_path )
      .spawn()
      .expect( "failed to spawn process" )
    };
    let output = child
    .wait_with_output()
    .expect( "failed to wait on child" );

    let report = CmdReport
    {
      command : exec_path.to_string(),
      path : current_path,
      out : String::from_utf8( output.stdout ).expect( "Found invalid UTF-8" ),
      err : String::from_utf8( output.stderr ).expect( "Found invalid UTF-8" ),
    };

    Ok( report )
  }
}

//

crate::mod_interface!
{
  protected use CmdReport;
  protected use start_sync;
}

