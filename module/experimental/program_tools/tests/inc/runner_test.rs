//! Integration tests for the runner and output APIs.
//!
//! ## Test Categories
//!
//! - **OUTPUT Predicates**: `CapturedOutput` field accessors and predicate methods (no cargo)
//! - **RUN SOURCE**: `run_source` — hello world, compile errors, exit codes, stderr, multiline
//! - **RUN FILE**: `run_file` — disk execution, missing source, invalid Rust
//! - **RUN PROJECT**: `run_project` error handling for missing manifest
//! - **TIMEOUT**: capture-mode and forwarding-mode enforcement; zero-budget fires immediately
//! - **ENV VARS**: `split_once('=')` parsing — no-equals ignored, value preserved past first `=`
//! - **CLEANUP**: `cleanup=false` leaves workspace on disk; PID-scoped to avoid concurrent races

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

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// RUN SOURCE — exit codes and stdio channels
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

/// Test RUN-004: `run_source` preserves the script's non-zero exit code verbatim.
///
/// `process::exit(42)` must produce `exit_status=42`, not normalised to 1.
/// This validates that exit code forwarding is exact and no layer swallows it.
#[ test ]
fn run_source_preserves_exit_code_42()
{
  use the_module::run_source;

  let output = run_source( "fn main() { std::process::exit( 42 ); }" )
    .expect( "run_source must return Ok even for non-zero exit" );

  assert_eq!
  (
    output.exit_status,
    42,
    "expected exit_status=42; got {}",
    output.exit_status,
  );
  assert!( !output.exit_ok() );
}

/// Test RUN-005: `run_source` captures stderr produced by `eprintln!`.
///
/// Verifies that the stderr channel is independent from stdout and that content
/// written to it is captured correctly in `CapturedOutput::stderr`.
#[ test ]
fn run_source_stderr_captured()
{
  use the_module::run_source;

  let output = run_source
  (
    r#"fn main() { eprintln!( "from stderr" ); println!( "from stdout" ); }"#,
  )
  .expect( "run_source failed" );

  output.assert_exit_ok();
  output.assert_stdout_contains( "from stdout" );
  output.assert_stderr_contains( "from stderr" );
}

/// Test RUN-006: `run_source` with multiline stdout captures all lines in order.
#[ test ]
fn run_source_multiline_stdout()
{
  use the_module::run_source;

  let output = run_source
  (
    r#"fn main() { println!( "line1" ); println!( "line2" ); println!( "line3" ); }"#,
  )
  .expect( "run_source failed" );

  output.assert_exit_ok();
  let stdout = output.stdout_str();
  assert!( stdout.contains( "line1" ) );
  assert!( stdout.contains( "line2" ) );
  assert!( stdout.contains( "line3" ) );
  // Order: line1 must appear before line3 in the captured output
  let pos1 = stdout.find( "line1" ).unwrap();
  let pos3 = stdout.find( "line3" ).unwrap();
  assert!( pos1 < pos3, "lines must appear in order" );
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// RUN FILE — additional coverage
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

/// Test FILE-003: `run_file` with invalid Rust returns `Ok` with non-zero exit.
///
/// Compilation errors are not infrastructure failures for `run_file` either —
/// the same contract as `run_source` applies: non-zero exit code in `CapturedOutput`.
#[ test ]
fn run_file_invalid_rust_gives_nonzero_exit()
{
  use the_module::run_file;

  let pid = std::process::id();
  let tmp = std::env::temp_dir().join( format!( "program_tools_rf_invalid_{pid}" ) );
  std::fs::create_dir_all( &tmp ).expect( "failed to create tmp dir" );
  let path = tmp.join( "broken.rs" );
  std::fs::write( &path, "fn main() { this_is_not_valid_rust }" ).expect( "write failed" );

  let output = run_file( &path ).expect( "run_file must return Ok even on compile error" );
  std::fs::remove_dir_all( &tmp ).ok();

  assert!
  (
    !output.exit_ok(),
    "expected non-zero exit for invalid Rust; got {}",
    output.exit_status,
  );
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// TIMEOUT — forwarding mode and zero timeout
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

/// Test TIMEOUT-003: `timeout_ms=Some(0)` fires immediately in capture mode.
///
/// A zero-millisecond budget is always expired by the time `recv_timeout`
/// evaluates, so any process (regardless of runtime) must return Err("timed out").
#[ test ]
fn run_timeout_zero_fires_immediately()
{
  use the_module::{ run, Plan, RunOptions };

  let opts = RunOptions { timeout_ms : Some( 0 ), ..Default::default() };
  let plan = Plan::former()
    .program()
      .source()
        .file_path( "src/main.rs".to_string() )
        .data( r#"fn main() { println!( "never reached" ); }"#.to_string() )
        .end()
      .end()
    .run_options( opts )
    .form();

  let result = run( plan );
  assert!( result.is_err(), "timeout_ms=0 must fire immediately; got: {result:?}" );
  let msg = result.unwrap_err().to_string();
  assert!( msg.contains( "timed out" ), "expected 'timed out' in error; got: {msg}" );
}

/// Test TIMEOUT-004: Forwarding mode (`capture=false`) with `timeout_ms` fires and returns Err.
///
/// Verifies the polling-based timeout path: `try_wait()` + deadline check + `child.kill()`.
#[ test ]
fn run_timeout_forwarding_mode_fires()
{
  use the_module::{ run, Plan, RunOptions };

  let opts = RunOptions { timeout_ms : Some( 1 ), capture : false, ..Default::default() };
  let plan = Plan::former()
    .program()
      .source()
        .file_path( "src/main.rs".to_string() )
        .data( r#"fn main() { println!( "forwarded" ); }"#.to_string() )
        .end()
      .end()
    .run_options( opts )
    .form();

  let result = run( plan );
  assert!( result.is_err(), "forwarding mode timeout must fire; got: {result:?}" );
  let msg = result.unwrap_err().to_string();
  assert!( msg.contains( "timed out" ), "expected 'timed out' in error; got: {msg}" );
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// ENV VARS — splitting and handling
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

/// Test ENV-001: `env_vars` entry without `=` is silently ignored.
///
/// The runner splits on `'='` via `split_once`; entries with no `=` produce
/// `None` and are skipped. The process must still run and exit 0.
#[ test ]
fn run_env_var_no_equals_ignored()
{
  use the_module::{ run, Plan, RunOptions };

  // "NO_EQUALS_VAR" has no '=' — must be silently ignored
  let opts = RunOptions { env_vars : vec![ "NO_EQUALS_VAR".to_string() ], ..Default::default() };
  let plan = Plan::former()
    .program()
      .source()
        .file_path( "src/main.rs".to_string() )
        .data( r#"fn main() { println!( "ok" ); }"#.to_string() )
        .end()
      .end()
    .run_options( opts )
    .form();

  let output = run( plan ).expect( "run must succeed despite malformed env_var entry" );
  output.assert_exit_ok();
  output.assert_stdout_contains( "ok" );
}

/// Test ENV-002: `env_vars` entry `KEY=value` is passed to the script.
///
/// Validates the happy path: the script reads the var set by the caller.
#[ test ]
fn run_env_var_passed_to_script()
{
  use the_module::{ run, Plan, RunOptions };

  let opts = RunOptions
  {
    env_vars : vec![ "PT_TEST_VAR=hello_env".to_string() ],
    ..Default::default()
  };
  let plan = Plan::former()
    .program()
      .source()
        .file_path( "src/main.rs".to_string() )
        .data
        (
          r#"fn main() { println!( "{}", std::env::var( "PT_TEST_VAR" ).unwrap_or_default() ); }"#
          .to_string()
        )
        .end()
      .end()
    .run_options( opts )
    .form();

  let output = run( plan ).expect( "run failed" );
  output.assert_exit_ok();
  output.assert_stdout_contains( "hello_env" );
}

/// Test ENV-003: `env_vars` entry `KEY=a=b` splits only at the first `=`.
///
/// `split_once('=')` splits at the first occurrence only, so `"MY=a=b"` produces
/// key `"MY"` and value `"a=b"`. The `=` within the value is preserved.
#[ test ]
fn run_env_var_equals_in_value_preserved()
{
  use the_module::{ run, Plan, RunOptions };

  let opts = RunOptions
  {
    env_vars : vec![ "PT_EQ_VAR=hello=world".to_string() ],
    ..Default::default()
  };
  let plan = Plan::former()
    .program()
      .source()
        .file_path( "src/main.rs".to_string() )
        .data
        (
          r#"fn main() { println!( "{}", std::env::var( "PT_EQ_VAR" ).unwrap_or_default() ); }"#
          .to_string()
        )
        .end()
      .end()
    .run_options( opts )
    .form();

  let output = run( plan ).expect( "run failed" );
  output.assert_exit_ok();
  output.assert_stdout_contains( "hello=world" );
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// CLEANUP — workspace retention
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

/// Test CLEANUP-001: `cleanup=false` leaves the temporary workspace on disk.
///
/// `run()` creates `program_tools_<pid>_<nanos>` in `temp_dir()`. With
/// `cleanup=false`, that directory must survive after the run. We filter by
/// the current process ID so we never touch concurrent tests' workspaces.
#[ test ]
fn run_cleanup_false_leaves_workspace()
{
  use the_module::{ run, Plan, RunOptions };
  use std::collections::HashSet;

  let temp_dir = std::env::temp_dir();
  // Filter by PID + thread ID so we only see workspaces created by THIS test
  // thread.  Under `cargo test` all threads share the same PID, so a PID-only
  // prefix would match sibling threads' workspaces and cause spurious deletions.
  let pid = std::process::id();
  let tid = the_module::thread_id_str();
  let thread_prefix = format!( "program_tools_{pid}_{tid}_" );

  let snapshot_dirs = | | -> HashSet< String >
  {
    std::fs::read_dir( &temp_dir )
      .expect( "failed to read temp dir" )
      .filter_map( core::result::Result::ok )
      .filter_map( | e | e.file_name().into_string().ok() )
      .filter( | name | name.starts_with( &thread_prefix ) )
      .collect()
  };

  let before = snapshot_dirs();

  let opts = RunOptions { cleanup : false, timeout_ms : Some( 0 ), ..Default::default() };
  let plan = Plan::former()
    .program()
      .source()
        .file_path( "src/main.rs".to_string() )
        .data( "fn main() {}".to_string() )
        .end()
      .end()
    .run_options( opts )
    .form();

  // Execution fails (timeout=0) but workspace must NOT be cleaned up.
  let _ = run( plan );

  let after = snapshot_dirs();
  let new_dirs : Vec< String > = after.difference( &before ).cloned().collect();

  // Cleanup only THIS thread's leftover directories — never touch sibling threads'.
  for name in &new_dirs
  {
    std::fs::remove_dir_all( temp_dir.join( name ) ).ok();
  }

  assert!
  (
    !new_dirs.is_empty(),
    "expected at least one workspace directory to remain when cleanup=false; none found",
  );
}
