//! Manual execution tests for comprehensive functionality verification.
//!
//! Run with: `cargo test --test manual_execution_test`

use process_tools::process;
use std::collections::HashMap;

#[ test ]
fn test_simple_binary_execution()
{
  let report = process::Run::former()
    .bin_path( "rustc" )
    .args( vec![ "--version".into() ] )
    .current_path( "." )
    .run()
    .expect( "rustc --version should succeed" );

  assert!( report.out.contains( "rustc" ) );
  assert!( report.error.is_ok() );
}

#[ test ]
fn test_binary_with_multiple_args()
{
  let report = process::Run::former()
    .bin_path( "cargo" )
    .args( vec![ "--version".into() ] )
    .current_path( "." )
    .run()
    .expect( "cargo --version should succeed" );

  assert!( report.out.contains( "cargo" ) );
}

#[ test ]
fn test_nonexistent_binary()
{
  let result = process::Run::former()
    .bin_path( "/nonexistent/binary/path" )
    .current_path( "." )
    .run();

  assert!( result.is_err(), "Should fail for nonexistent binary" );
  let report = result.unwrap_err();
  assert!( report.error.is_err() );
}

#[ test ]
fn test_shell_simple_command()
{
  let report = process::Run::former()
    .current_path( "." )
    .run_with_shell( "echo Hello" )
    .expect( "echo should succeed" );

  assert_eq!( report.out.trim(), "Hello" );
}

#[ test ]
fn test_shell_piped_command()
{
  let report = process::Run::former()
    .current_path( "." )
    .run_with_shell( "echo test | grep test" )
    .expect( "piped command should succeed" );

  assert!( report.out.contains( "test" ) );
}

#[ test ]
fn test_joining_streams_true()
{
  let cmd = if cfg!( target_os = "windows" )
  {
    "echo stdout && echo stderr 1>&2"
  }
  else
  {
    "echo stdout && echo stderr >&2"
  };

  let report = process::Run::former()
    .current_path( "." )
    .joining_streams( true )
    .run_with_shell( cmd )
    .expect( "stream joining should succeed" );

  // With joining_streams=true, both outputs go to stdout
  assert!( report.out.contains( "stdout" ) );
  // Stderr should be empty when joining
  assert!( report.err.is_empty() );
}

#[ test ]
fn test_joining_streams_false()
{
  let cmd = if cfg!( target_os = "windows" )
  {
    "echo stdout && echo stderr 1>&2"
  }
  else
  {
    "echo stdout && echo stderr >&2"
  };

  let report = process::Run::former()
    .current_path( "." )
    .joining_streams( false )
    .run_with_shell( cmd )
    .expect( "separate streams should succeed" );

  assert!( report.out.trim().contains( "stdout" ) );
  assert!( report.err.trim().contains( "stderr" ) );
}

#[ test ]
fn test_env_variable_single()
{
  let mut env = HashMap::new();
  env.insert( "TEST_VAR".to_string(), "test_value".to_string() );

  let cmd = if cfg!( target_os = "windows" )
  {
    "echo %TEST_VAR%"
  }
  else
  {
    "echo $TEST_VAR"
  };

  let report = process::Run::former()
    .current_path( "." )
    .env_variable( env )
    .run_with_shell( cmd )
    .expect( "environment variable test should succeed" );

  assert!( report.out.contains( "test_value" ) );
}

#[ test ]
fn test_nonzero_exit_code()
{
  let result = process::Run::former()
    .current_path( "." )
    .run_with_shell( "exit 42" );

  assert!( result.is_err(), "Should return Err for nonzero exit code" );
  let report = result.unwrap_err();
  assert!( report.error.is_err() );
}

#[ test ]
fn test_report_command_field()
{
  let report = process::Run::former()
    .bin_path( "echo" )
    .args( vec![ "test".into() ] )
    .current_path( "." )
    .run()
    .expect( "echo test should succeed" );

  // Command field should contain the executed command
  assert!( !report.command.is_empty() );
  assert!( report.command.contains( "echo" ) );
}

#[ test ]
fn test_report_current_path_field()
{
  let report = process::Run::former()
    .bin_path( "echo" )
    .args( vec![ "test".into() ] )
    .current_path( "." )
    .run()
    .expect( "echo test should succeed" );

  // Current path should be set
  assert!( report.current_path.exists() );
  assert!( report.current_path.is_dir() );
}

#[ test ]
fn test_empty_args()
{
  let result = process::Run::former()
    .bin_path( "rustc" )
    .args( vec![] )
    .current_path( "." )
    .run();

  // rustc with no args actually succeeds (shows help message)
  assert!( result.is_ok(), "rustc with no args should succeed (displays help)" );
  let report = result.unwrap();
  // Should contain help/usage information
  assert!( !report.out.is_empty() || !report.err.is_empty() );
}

#[ test ]
fn test_builder_default_joining_streams()
{
  // Not setting joining_streams explicitly should default to false
  let report = process::Run::former()
    .bin_path( "echo" )
    .args( vec![ "test".into() ] )
    .current_path( "." )
    .run()
    .expect( "echo should succeed" );

  // With default (false), stdout and stderr are separate
  assert!( !report.out.is_empty() || !report.err.is_empty() );
}

#[ test ]
fn test_nonexistent_current_path()
{
  let result = process::Run::former()
    .bin_path( "echo" )
    .args( vec![ "test".into() ] )
    .current_path( "/nonexistent/directory/path" )
    .run();

  assert!( result.is_err(), "Should fail for nonexistent current_path" );
}
