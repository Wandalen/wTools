//! Edge case and corner case tests for `process_tools`.
//!
//! Tests covering extreme scenarios, boundary conditions, and unusual inputs.

use process_tools ::process;
use std ::collections ::HashMap;

#[ test ]
fn test_report_display_formatting()
{
  let report = process ::Run ::former()
  .bin_path( "echo" )
  .args( vec![ "test_output".into() ] )
  .current_path( "." )
  .run()
  .expect( "echo should succeed" );

  let formatted = format!( "{report}" );

  assert!( formatted.contains( "echo" ) );
  assert!( formatted.contains( "test_output" ) );
}

#[ test ]
fn test_report_clone()
{
  let report1 = process ::Run ::former()
  .bin_path( "echo" )
  .args( vec![ "test".into() ] )
  .current_path( "." )
  .run()
  .expect( "echo should succeed" );

  let report2 = report1.clone();

  assert_eq!( report1.command, report2.command );
  assert_eq!( report1.out, report2.out );
  assert_eq!( report1.err, report2.err );
  assert_eq!( report1.current_path, report2.current_path );
}

#[ test ]
fn test_empty_stdout_empty_stderr()
{
  let cmd = if cfg!( target_os = "windows" )
  {
    "cmd /c exit 0"
  }
  else
  {
    "true"
  };

  let report = process ::Run ::former()
  .current_path( "." )
  .run_with_shell( cmd )
  .expect( "silent command should succeed" );

  assert!( report.out.is_empty() );
  assert!( report.err.is_empty() );
}

#[ test ]
fn test_large_output()
{
  // POSIX-compatible while loop; avoids bash brace expansion
  let cmd = if cfg!( target_os = "windows" )
  {
    "for /L %i in (1,1,100) do @echo Line %i"
  }
  else
  {
    "i=1; while [ $i -le 100 ]; do echo Line $i; i=$((i + 1)); done"
  };

  let report = process ::Run ::former()
  .current_path( "." )
  .run_with_shell( cmd )
  .expect( "large output command should succeed" );

  let line_count = report.out.lines().count();
  assert!( line_count >= 100, "Expected at least 100 lines, got {line_count}" );
}

#[ test ]
fn test_unicode_in_output()
{
  let cmd = if cfg!( target_os = "windows" )
  {
    "echo Hello世界🌍"
  }
  else
  {
    "echo 'Hello世界🌍'"
  };

  let report = process ::Run ::former()
  .current_path( "." )
  .run_with_shell( cmd )
  .expect( "unicode command should succeed" );

  assert!( report.out.contains( "Hello" ) );
}

#[ test ]
fn test_args_with_spaces()
{
  let report = process ::Run ::former()
  .bin_path( "echo" )
  .args( vec![ "arg with spaces".into() ] )
  .current_path( "." )
  .run()
  .expect( "echo with spaced arg should succeed" );

  assert!( report.out.contains( "arg with spaces" ) );
}

#[ test ]
fn test_multiple_environment_variables()
{
  let mut env = HashMap ::new();
  env.insert( "VAR1".to_string(), "value1".to_string() );
  env.insert( "VAR2".to_string(), "value2".to_string() );
  env.insert( "VAR3".to_string(), "value3".to_string() );

  let cmd = if cfg!( target_os = "windows" )
  {
    "echo %VAR1% %VAR2% %VAR3%"
  }
  else
  {
    "echo $VAR1 $VAR2 $VAR3"
  };

  let report = process ::Run ::former()
  .current_path( "." )
  .env_variable( env )
  .run_with_shell( cmd )
  .expect( "multiple env vars should succeed" );

  assert!( report.out.contains( "value1" ) );
  assert!( report.out.contains( "value2" ) );
  assert!( report.out.contains( "value3" ) );
}

#[ test ]
fn test_env_variable_empty_value()
{
  let mut env = HashMap ::new();
  env.insert( "EMPTY_VAR".to_string(), String ::new() );

  let cmd = if cfg!( target_os = "windows" )
  {
    "echo '%EMPTY_VAR%'"
  }
  else
  {
    "echo '$EMPTY_VAR'"
  };

  let report = process ::Run ::former()
  .current_path( "." )
  .env_variable( env )
  .run_with_shell( cmd )
  .expect( "empty env var should succeed" );

  assert!( !report.out.is_empty() );
}

#[ test ]
fn test_relative_current_path()
{
  let report = process ::Run ::former()
  .bin_path( "echo" )
  .args( vec![ "test".into() ] )
  .current_path( "." )
  .run()
  .expect( "relative path should work" );

  assert!( report.out.contains( "test" ) );
}

#[ test ]
fn test_builder_minimal_configuration()
{
  let report = process ::Run ::former()
  .bin_path( "echo" )
  .current_path( "." )
  .run()
  .expect( "minimal config should succeed" );

  assert!( report.error.is_ok() );
}

#[ test ]
fn test_shell_complex_expression()
{
  let cmd = "echo a && echo b";

  let report = process ::Run ::former()
  .current_path( "." )
  .run_with_shell( cmd )
  .expect( "complex shell expression should succeed" );

  assert!( report.out.contains( 'a' ) );
  assert!( report.out.contains( 'b' ) );
}

#[ test ]
fn test_stderr_only_output()
{
  let cmd = if cfg!( target_os = "windows" )
  {
    "echo error 1>&2"
  }
  else
  {
    "echo error >&2"
  };

  let report = process ::Run ::former()
  .current_path( "." )
  .joining_streams( false )
  .run_with_shell( cmd )
  .expect( "stderr output should succeed" );

  assert!( report.out.trim().is_empty() );
  assert!( report.err.contains( "error" ) );
}

#[ test ]
fn test_stdout_only_output()
{
  let report = process ::Run ::former()
  .bin_path( "echo" )
  .args( vec![ "stdout_only".into() ] )
  .current_path( "." )
  .joining_streams( false )
  .run()
  .expect( "stdout output should succeed" );

  assert!( report.out.contains( "stdout_only" ) );
  assert!( report.err.is_empty() );
}

#[ test ]
fn test_report_debug_formatting()
{
  let report = process ::Run ::former()
  .bin_path( "echo" )
  .args( vec![ "test".into() ] )
  .current_path( "." )
  .run()
  .expect( "echo should succeed" );

  let debug_output = format!( "{report:?}" );

  assert!( debug_output.contains( "Report" ) );
}

#[ test ]
fn test_empty_shell_command_behavior()
{
  // Platform-dependent: may succeed (no-op) or fail (empty command) — must not panic
  let _ignore = process ::Run ::former()
  .current_path( "." )
  .run_with_shell( "" );
}

#[ test ]
fn test_whitespace_only_args()
{
  let report = process ::Run ::former()
  .bin_path( "echo" )
  .args( vec![ "   ".into() ] )
  .current_path( "." )
  .run()
  .expect( "echo with whitespace arg should succeed" );

  assert!( !report.out.is_empty() );
}
