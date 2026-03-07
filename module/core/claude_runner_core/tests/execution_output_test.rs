//! Tests for `ExecutionOutput` struct
//!
//! Verifies the `ExecutionOutput` type provides correct field access,
//! Display formatting, and derived trait behavior.

use claude_runner_core::ExecutionOutput;

#[test]
fn execution_output_has_all_fields()
{
  let output = ExecutionOutput
  {
    stdout : "hello".to_string(),
    stderr : "warning".to_string(),
    exit_code : 0,
  };

  assert_eq!( output.stdout, "hello" );
  assert_eq!( output.stderr, "warning" );
  assert_eq!( output.exit_code, 0 );
}

#[test]
fn execution_output_display_shows_stdout()
{
  let output = ExecutionOutput
  {
    stdout : "result text".to_string(),
    stderr : "some warning".to_string(),
    exit_code : 0,
  };

  // Display should show stdout only
  let displayed = format!( "{output}" );
  assert_eq!( displayed, "result text" );
}

#[test]
fn execution_output_display_empty_stdout()
{
  let output = ExecutionOutput
  {
    stdout : String::new(),
    stderr : "error output".to_string(),
    exit_code : 1,
  };

  let displayed = format!( "{output}" );
  assert_eq!( displayed, "" );
}

#[test]
fn execution_output_clone()
{
  let output = ExecutionOutput
  {
    stdout : "hello".to_string(),
    stderr : String::new(),
    exit_code : 0,
  };

  let cloned = output.clone();
  assert_eq!( output, cloned );
}

#[test]
fn execution_output_debug()
{
  let output = ExecutionOutput
  {
    stdout : "out".to_string(),
    stderr : "err".to_string(),
    exit_code : 42,
  };

  let debug = format!( "{output:?}" );
  assert!( debug.contains( "out" ) );
  assert!( debug.contains( "err" ) );
  assert!( debug.contains( "42" ) );
}

#[test]
fn execution_output_equality()
{
  let a = ExecutionOutput
  {
    stdout : "same".to_string(),
    stderr : String::new(),
    exit_code : 0,
  };
  let b = ExecutionOutput
  {
    stdout : "same".to_string(),
    stderr : String::new(),
    exit_code : 0,
  };
  let c = ExecutionOutput
  {
    stdout : "different".to_string(),
    stderr : String::new(),
    exit_code : 0,
  };

  assert_eq!( a, b );
  assert_ne!( a, c );
}
