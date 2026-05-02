//! Integration tests for the runner and output APIs.
//!
//! ## Test Categories
//!
//! - **OUTPUT Predicates**: `CapturedOutput` field accessors and predicate methods (no cargo)
//! - **RUN SOURCE**: `run_source` executes inline Rust code via cargo
//! - **RUN FILE**: `run_file` reads a file from disk and executes it
//! - **RUN PROJECT**: `run_project` error handling for missing manifest
//! - **TIMEOUT**: `timeout_ms` enforcement — fires on infinite loops, skips on generous budgets

#[ allow( unused_imports ) ]
use super::*;

use the_module::CapturedOutput;

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// OUTPUT Predicates
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

/// Test EXIT-001: `exit_ok` returns true when exit status is zero.
#[ test ]
fn captured_output_exit_ok_true_when_zero()
{
  let out = CapturedOutput { exit_status : 0, stdout : vec![], stderr : vec![] };
  assert!( out.exit_ok() );
}

/// Test EXIT-002: `exit_ok` returns false for any non-zero exit status.
#[ test ]
fn captured_output_exit_ok_false_when_nonzero()
{
  let out = CapturedOutput { exit_status : 1, stdout : vec![], stderr : vec![] };
  assert!( !out.exit_ok() );

  let out_neg = CapturedOutput { exit_status : -1, stdout : vec![], stderr : vec![] };
  assert!( !out_neg.exit_ok() );
}

/// Test STR-001: `stdout_str` decodes raw bytes as lossy UTF-8.
#[ test ]
fn captured_output_stdout_str_decodes_bytes()
{
  let out = CapturedOutput
  {
    exit_status : 0,
    stdout : b"hello\n".to_vec(),
    stderr : vec![],
  };
  assert_eq!( out.stdout_str(), "hello\n" );
}

/// Test STR-002: `stderr_str` decodes raw bytes as lossy UTF-8.
#[ test ]
fn captured_output_stderr_str_decodes_bytes()
{
  let out = CapturedOutput
  {
    exit_status : 1,
    stdout : vec![],
    stderr : b"error message\n".to_vec(),
  };
  assert_eq!( out.stderr_str(), "error message\n" );
}

/// Test PRED-001: `stdout_eq` matches exact string; rejects inexact.
#[ test ]
fn captured_output_stdout_eq_exact_match()
{
  let out = CapturedOutput
  {
    exit_status : 0,
    stdout : b"hello\n".to_vec(),
    stderr : vec![],
  };
  assert!( out.stdout_eq( "hello\n" ) );
  assert!( !out.stdout_eq( "hello" ) );   // missing newline
  assert!( !out.stdout_eq( "world\n" ) ); // different content
}

/// Test PRED-002: `stdout_contains` finds a substring; rejects absent string.
#[ test ]
fn captured_output_stdout_contains_substring()
{
  let out = CapturedOutput
  {
    exit_status : 0,
    stdout : b"hello world\n".to_vec(),
    stderr : vec![],
  };
  assert!( out.stdout_contains( "hello" ) );
  assert!( out.stdout_contains( "world" ) );
  assert!( !out.stdout_contains( "missing" ) );
}

/// Test PRED-003: `stderr_contains` finds a substring; rejects absent string.
#[ test ]
fn captured_output_stderr_contains_substring()
{
  let out = CapturedOutput
  {
    exit_status : 1,
    stdout : vec![],
    stderr : b"error: something failed\n".to_vec(),
  };
  assert!( out.stderr_contains( "error" ) );
  assert!( !out.stderr_contains( "success" ) );
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// RUN SOURCE — integration (invokes cargo run)
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

/// Test RUN-001: `run_source` executes hello-world and captures stdout exactly.
#[ test ]
fn run_source_hello_world()
{
  use the_module::run_source;

  let output = run_source( r#"fn main() { println!( "hello" ); }"# )
    .expect( "run_source failed" );

  output.assert_exit_ok();
  output.assert_stdout_eq( "hello\n" );
}

/// Test RUN-002: `run_source` with invalid Rust returns Ok with non-zero exit status.
///
/// Compilation errors are not infrastructure failures; they produce a non-zero
/// exit code in the returned `CapturedOutput` rather than an Err variant.
#[ test ]
fn run_source_compilation_error_gives_nonzero_exit()
{
  use the_module::run_source;

  let output = run_source( "this is not valid rust" )
    .expect( "run_source must return Ok even for compilation errors" );

  assert!
  (
    !output.exit_ok(),
    "expected non-zero exit status for invalid Rust; got exit_status={}",
    output.exit_status,
  );
  assert!
  (
    !output.stderr_str().is_empty(),
    "expected compiler diagnostics in stderr for invalid Rust"
  );
}

/// Test RUN-003: `run` with `capture=false` exits ok and returns empty byte buffers.
///
/// Forwarding mode inherits the process stdio, so stdout/stderr flow to the test
/// runner's terminal rather than being buffered. The returned `CapturedOutput`
/// fields must be empty.
#[ test ]
fn run_capture_false_exits_ok_with_empty_buffers()
{
  use the_module::{ run, Plan, RunOptions };

  let opts = RunOptions { capture : false, ..Default::default() };
  let plan = Plan::former()
    .program()
      .source()
        .file_path( "src/main.rs".to_string() )
        .data( r#"fn main() { println!( "forwarded" ); }"#.to_string() )
        .end()
      .end()
    .run_options( opts )
    .form();

  let output = run( plan ).expect( "run failed in forwarding mode" );

  output.assert_exit_ok();
  assert!
  (
    output.stdout.is_empty(),
    "stdout buffer must be empty in forwarding mode; got {} bytes",
    output.stdout.len(),
  );
  assert!
  (
    output.stderr.is_empty(),
    "stderr buffer must be empty in forwarding mode; got {} bytes",
    output.stderr.len(),
  );
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// RUN FILE — integration (invokes cargo run)
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

/// Test FILE-001: `run_file` reads a file from disk and executes it.
#[ test ]
fn run_file_executes_source_from_disk()
{
  use the_module::run_file;
  use std::path::PathBuf;

  let pid = std::process::id();
  let tmp : PathBuf = std::env::temp_dir().join( format!( "program_tools_run_file_{pid}" ) );
  std::fs::create_dir_all( &tmp ).expect( "failed to create tmp dir" );

  let path = tmp.join( "hello.rs" );
  std::fs::write( &path, r#"fn main() { println!( "from file" ); }"# )
    .expect( "failed to write source file" );

  let output = run_file( &path ).expect( "run_file failed" );
  std::fs::remove_dir_all( &tmp ).ok();

  output.assert_exit_ok();
  output.assert_stdout_contains( "from file" );
}

/// Test FILE-002: `run_file` returns Err when the source file does not exist.
#[ test ]
fn run_file_err_when_source_missing()
{
  use the_module::run_file;

  let result = run_file( "/nonexistent/path/does_not_exist_program_tools.rs" );
  assert!( result.is_err(), "expected Err for a missing source file" );
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// RUN PROJECT — error handling (no cargo invocation needed for error path)
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

/// Test PROJ-001: `run_project` returns Err when no `Cargo.toml` is found.
#[ test ]
fn run_project_err_when_manifest_absent()
{
  use the_module::{ run_project, RunOptions };
  use std::path::PathBuf;

  let pid = std::process::id();
  let tmp : PathBuf = std::env::temp_dir()
    .join( format!( "program_tools_run_project_{pid}" ) );
  std::fs::create_dir_all( &tmp ).expect( "failed to create tmp dir" );

  // Guarantee no Cargo.toml is present.
  std::fs::remove_file( tmp.join( "Cargo.toml" ) ).ok();

  let result = run_project( &tmp, &RunOptions::default() );
  std::fs::remove_dir_all( &tmp ).ok();

  assert!( result.is_err(), "expected Err when no Cargo.toml is present" );
  let err_msg = result.unwrap_err().to_string();
  assert!
  (
    err_msg.contains( "no Cargo.toml" ),
    "expected 'no Cargo.toml' in error message, got: {err_msg}",
  );
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// TIMEOUT — enforcement (invokes cargo run with a time budget)
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

/// Test TIMEOUT-001: `run` with `timeout_ms=200` on an infinite loop returns Err
/// containing "timed out".
///
/// Verifies that the timeout budget is enforced: a process that runs forever must
/// produce an infrastructure-level `Err`, not block indefinitely.
#[ test ]
fn run_timeout_fires()
{
  use the_module::{ run, Plan, RunOptions };

  let opts = RunOptions { timeout_ms : Some( 200 ), ..Default::default() };
  let plan = Plan::former()
    .program()
      .source()
        .file_path( "src/main.rs".to_string() )
        .data( "fn main() { loop {} }".to_string() )
        .end()
      .end()
    .run_options( opts )
    .form();

  let result = run( plan );
  assert!( result.is_err(), "expected Err when timeout fires, got: {result:?}" );
  let err_msg = result.unwrap_err().to_string();
  assert!
  (
    err_msg.contains( "timed out" ),
    "expected 'timed out' in error message, got: {err_msg}",
  );
}

/// Test TIMEOUT-002: `run` with `timeout_ms=60_000` on hello-world completes within
/// budget and returns Ok with `exit_status=0`.
///
/// Verifies the happy path: a well-behaved process is not incorrectly killed when
/// the timeout budget is generous.
#[ test ]
fn run_timeout_does_not_fire()
{
  use the_module::{ run, Plan, RunOptions };

  let opts = RunOptions { timeout_ms : Some( 60_000 ), ..Default::default() };
  let plan = Plan::former()
    .program()
      .source()
        .file_path( "src/main.rs".to_string() )
        .data( r#"fn main() { println!( "hello" ); }"#.to_string() )
        .end()
      .end()
    .run_options( opts )
    .form();

  let output = run( plan ).expect( "run failed unexpectedly with generous timeout" );
  output.assert_exit_ok();
  output.assert_stdout_contains( "hello" );
}
