#![ warn( missing_docs ) ]
#![ warn( missing_debug_implementations ) ]

//!
//! Run external processes.
//!

use std::process::
{
  Command,
  Output,
  Stdio,
};

///
/// Run external processes.
///

pub fn start_sync<'a>( exec_path : &'a str, current_path : &'a str ) -> anyhow::Result<Output>
{
  let child = if cfg!( target_os = "windows" )
  {
    Command::new( "cmd" )
    .args( ["/C", exec_path ] )
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
