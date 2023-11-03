/// Internal namespace.
pub( crate ) mod private
{
  use std::process::
  {
    Command,
    Output,
    Stdio,
  };

  ///
  /// Run external processes.
  ///

  pub fn start_sync
  (
    exec_path : &str,
    current_path : impl Into< std::path::PathBuf > + AsRef< std::path::Path >
  ) -> anyhow::Result< Output >
  {
    let child = if cfg!( target_os = "windows" )
    {
      Command::new( "cmd" )
      .args( [ "/C", exec_path ] )
      .stdout( Stdio::piped() )
      .stderr( Stdio::piped() )
      .current_dir( current_path )
      .spawn()
      .expect( "failed to spawn process" )
    }
    else
    {
      Command::new( "sh" )
      .args( [ "-c", exec_path ] )
      .stdout( Stdio::piped() )
      .stderr( Stdio::piped() )
      .current_dir( current_path )
      .spawn()
      .expect( "failed to spawn process" )
    };
    let output = child
    .wait_with_output()
    .expect( "failed to wait on child" );

    Ok( output )
  }

  ///
  /// Log output.
  ///

  pub fn log_output( output : &Output )
  {
    println!( "{}", std::str::from_utf8( &output.stdout ).expect( "Found invalid UTF-8" ) );
    eprintln!( "{}", std::str::from_utf8( &output.stderr ).expect( "Found invalid UTF-8" ) );
  }
}

//

crate::mod_interface!
{
  prelude use start_sync;
  prelude use log_output;
}

