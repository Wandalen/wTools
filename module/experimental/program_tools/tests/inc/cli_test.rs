//! CLI integration tests for the `program_tools` binary.
//!
//! Exercises the `program_tools run` subcommand end-to-end by spawning the
//! compiled binary as a subprocess and asserting on its exit code and output.
//!
//! ## Test Categories
//!
//! - **TC-1**: Happy path — single Rust file executes and forwards stdout
//! - **TC-2**: Happy path — existing Cargo project directory runs via `run_project`
//! - **TC-3**: Missing `<TARGET>` argument — clap rejects, exits non-zero
//! - **TC-4**: Non-existent path — binary prints error and exits 1
//! - **TC-5**: Unknown flag — clap rejects, exits non-zero
//! - **TC-6**: Non-zero target exit code forwarded verbatim (not normalised to 1)
//! - **TC-7**: Infrastructure error — `--cargo /nonexistent` exits 1 with diagnostic
//! - **TC-8**: Compilation error — compiler diagnostics appear in stderr, exits non-zero
//! - **TC-9**: `--help` exits 0 with usage text in stdout
//! - **TC-10**: `run --help` exits 0 with subcommand usage text
//! - **TC-11**: No subcommand — clap requires one; exits non-zero
//! - **TC-12**: `--capture` flag collects script stdout via capture→print path
//! - **TC-13**: `--env KEY=VALUE` injects env var into the script process

#[ allow( unused_imports ) ]
use super::*;

use std::process::Command;

fn program_tools_bin() -> &'static str
{
  env!( "CARGO_BIN_EXE_program_tools" )
}

fn run_cli( args : &[ &str ] ) -> std::process::Output
{
  Command::new( program_tools_bin() )
    .args( args )
    .output()
    .expect( "failed to invoke program_tools binary" )
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// TC-1: Happy path — single Rust file
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

/// Test TC-1: `program_tools run <file>` exits 0 and forwards program stdout.
///
/// Verifies that a single Rust source file is compiled and executed, its stdout
/// flows through the binary to the caller, and the exit code is 0.
#[ test ]
fn cli_run_single_file_happy_path()
{
  use std::path::PathBuf;

  let pid = std::process::id();
  let tmp : PathBuf = std::env::temp_dir()
    .join( format!( "pt_cli_tc1_{pid}" ) );
  std::fs::create_dir_all( &tmp ).expect( "failed to create tmp dir" );
  let source = tmp.join( "hello.rs" );
  std::fs::write( &source, r#"fn main() { println!( "hello from cli" ); }"# )
    .expect( "failed to write source" );

  let output = run_cli( &[ "run", source.to_str().expect( "path is valid UTF-8" ) ] );
  std::fs::remove_dir_all( &tmp ).ok();

  assert_eq!
  (
    output.status.code(),
    Some( 0 ),
    "expected exit code 0; stderr: {}",
    String::from_utf8_lossy( &output.stderr ),
  );
  let stdout = String::from_utf8_lossy( &output.stdout );
  assert!
  (
    stdout.contains( "hello from cli" ),
    "expected 'hello from cli' in stdout; got: {stdout}",
  );
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// TC-3: Missing <TARGET> argument
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

/// Test TC-3: `program_tools run` (no target) exits non-zero with usage error.
///
/// Clap rejects the invocation before any Cargo work starts. The error message
/// referencing the missing `<TARGET>` argument appears in stderr.
#[ test ]
fn cli_run_missing_target_exits_nonzero()
{
  let output = run_cli( &[ "run" ] );

  assert_ne!
  (
    output.status.code(),
    Some( 0 ),
    "expected non-zero exit code when TARGET is missing",
  );
  let stderr = String::from_utf8_lossy( &output.stderr );
  assert!
  (
    !stderr.is_empty(),
    "expected stderr to contain a usage error message",
  );
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// TC-4: Non-existent path
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

/// Test TC-4: `program_tools run /nonexistent/path.rs` exits 1 with path error.
///
/// The binary detects the read failure before invoking Cargo, prints an error
/// to stderr that names the missing path, and exits with code 1.
#[ test ]
fn cli_run_nonexistent_path_exits_one()
{
  let output = run_cli( &[ "run", "/nonexistent/pt_cli_tc4_does_not_exist.rs" ] );

  assert_eq!
  (
    output.status.code(),
    Some( 1 ),
    "expected exit code 1 for a missing source file",
  );
  let stderr = String::from_utf8_lossy( &output.stderr );
  assert!
  (
    !stderr.is_empty(),
    "expected an error message in stderr for a missing path",
  );
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// TC-5: Unknown flag rejected
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

/// Test TC-5: `program_tools run --not-a-real-flag target` exits non-zero.
///
/// Clap rejects unrecognised flags before any Cargo work starts. The error
/// referencing the unknown flag appears in stderr.
#[ test ]
fn cli_run_unknown_flag_exits_nonzero()
{
  let output = run_cli( &[ "run", "--not-a-real-flag", "dummy.rs" ] );

  assert_ne!
  (
    output.status.code(),
    Some( 0 ),
    "expected non-zero exit code for an unrecognised flag",
  );
  let stderr = String::from_utf8_lossy( &output.stderr );
  assert!
  (
    !stderr.is_empty(),
    "expected an error message in stderr for an unrecognised flag",
  );
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// TC-2: Happy path — Cargo project directory
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

/// Test TC-2: `program_tools run <dir>` runs an existing Cargo project.
///
/// Verifies that when the target is a directory containing a valid Cargo.toml,
/// the binary uses `run_project` (no manifest generation) and exits 0.
#[ test ]
fn cli_run_cargo_project_directory()
{
  use std::path::PathBuf;

  let pid = std::process::id();
  let tmp : PathBuf = std::env::temp_dir()
    .join( format!( "pt_cli_tc2_{pid}" ) );
  let src_dir = tmp.join( "src" );
  std::fs::create_dir_all( &src_dir ).expect( "failed to create src dir" );

  std::fs::write
  (
    tmp.join( "Cargo.toml" ),
    "[package]\nname = \"pt_tc2\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\n",
  ).expect( "failed to write Cargo.toml" );
  std::fs::write
  (
    src_dir.join( "main.rs" ),
    r#"fn main() { println!( "project output" ); }"#,
  ).expect( "failed to write main.rs" );

  let output = run_cli( &[ "run", tmp.to_str().expect( "path is valid UTF-8" ) ] );
  std::fs::remove_dir_all( &tmp ).ok();

  assert_eq!
  (
    output.status.code(),
    Some( 0 ),
    "expected exit code 0 for project run; stderr: {}",
    String::from_utf8_lossy( &output.stderr ),
  );
  let stdout = String::from_utf8_lossy( &output.stdout );
  assert!
  (
    stdout.contains( "project output" ),
    "expected 'project output' in stdout; got: {stdout}",
  );
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// TC-6: Target program exit code forwarded verbatim
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

/// Test TC-6: `program_tools run exit_42.rs` forwards the target's exit code.
///
/// Verifies that when the target program exits with a non-zero code, the CLI
/// propagates that exact code rather than normalising it to 1.
#[ test ]
fn cli_run_target_exit_code_forwarded()
{
  use std::path::PathBuf;

  let pid = std::process::id();
  let tmp : PathBuf = std::env::temp_dir()
    .join( format!( "pt_cli_tc6_{pid}" ) );
  std::fs::create_dir_all( &tmp ).expect( "failed to create tmp dir" );
  let source = tmp.join( "exit42.rs" );
  std::fs::write( &source, "fn main() { std::process::exit( 42 ); }" )
    .expect( "failed to write source" );

  let output = run_cli( &[ "run", source.to_str().expect( "path is valid UTF-8" ) ] );
  std::fs::remove_dir_all( &tmp ).ok();

  assert_eq!
  (
    output.status.code(),
    Some( 42 ),
    "expected exit code 42 forwarded from target program; got {:?}",
    output.status.code(),
  );
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// TC-7: Infrastructure error — Cargo binary not found
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

/// Test TC-7: `program_tools run --cargo /nonexistent/cargo` exits 1.
///
/// Verifies that when the Cargo binary cannot be found, the CLI reports an
/// infrastructure error to stderr and exits with code 1.
#[ test ]
fn cli_run_cargo_not_found_exits_one()
{
  use std::path::PathBuf;

  let pid = std::process::id();
  let tmp : PathBuf = std::env::temp_dir()
    .join( format!( "pt_cli_tc7_{pid}" ) );
  std::fs::create_dir_all( &tmp ).expect( "failed to create tmp dir" );
  let source = tmp.join( "hello.rs" );
  std::fs::write( &source, r#"fn main() { println!( "hi" ); }"# )
    .expect( "failed to write source" );

  let output = run_cli
  (
    &[
      "run",
      "--cargo",
      "/nonexistent/cargo_pt_tc7",
      source.to_str().expect( "path is valid UTF-8" ),
    ]
  );
  std::fs::remove_dir_all( &tmp ).ok();

  assert_eq!
  (
    output.status.code(),
    Some( 1 ),
    "expected exit code 1 when Cargo binary is missing",
  );
  let stderr = String::from_utf8_lossy( &output.stderr );
  assert!
  (
    !stderr.is_empty(),
    "expected an error diagnostic in stderr when Cargo is not found",
  );
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// TC-8: Compilation error — target program does not compile
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

/// Test TC-8: `program_tools run broken.rs` exits non-zero with compiler diagnostics.
///
/// Verifies that compilation errors produce a non-zero exit code and Cargo's
/// diagnostic output appears in stderr. The CLI must not suppress compiler output.
#[ test ]
fn cli_run_compilation_error_exits_nonzero()
{
  use std::path::PathBuf;

  let pid = std::process::id();
  let tmp : PathBuf = std::env::temp_dir()
    .join( format!( "pt_cli_tc8_{pid}" ) );
  std::fs::create_dir_all( &tmp ).expect( "failed to create tmp dir" );
  let source = tmp.join( "broken.rs" );
  std::fs::write( &source, "fn main() { this_does_not_compile }" )
    .expect( "failed to write source" );

  let output = run_cli( &[ "run", source.to_str().expect( "path is valid UTF-8" ) ] );
  std::fs::remove_dir_all( &tmp ).ok();

  assert_ne!
  (
    output.status.code(),
    Some( 0 ),
    "expected non-zero exit for a compilation error",
  );
  let stderr = String::from_utf8_lossy( &output.stderr );
  assert!
  (
    !stderr.is_empty(),
    "expected compiler diagnostics in stderr for broken code",
  );
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// TC-9 / TC-10: Help flags exit zero
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

/// Test TC-9: `program_tools --help` exits 0 with usage text.
///
/// Clap must emit help text and exit 0 for the top-level `--help` flag.
#[ test ]
fn cli_help_exits_zero()
{
  let output = run_cli( &[ "--help" ] );

  assert_eq!
  (
    output.status.code(),
    Some( 0 ),
    "expected exit code 0 for --help",
  );
  let stdout = String::from_utf8_lossy( &output.stdout );
  assert!
  (
    !stdout.is_empty(),
    "expected usage text in stdout for --help",
  );
}

/// Test TC-10: `program_tools run --help` exits 0 with subcommand usage text.
///
/// Clap must emit the `run` subcommand's help and exit 0.
#[ test ]
fn cli_run_help_exits_zero()
{
  let output = run_cli( &[ "run", "--help" ] );

  assert_eq!
  (
    output.status.code(),
    Some( 0 ),
    "expected exit code 0 for 'run --help'",
  );
  let stdout = String::from_utf8_lossy( &output.stdout );
  assert!
  (
    !stdout.is_empty(),
    "expected usage text in stdout for 'run --help'",
  );
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// TC-11: No subcommand
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

/// Test TC-11: `program_tools` with no subcommand exits non-zero.
///
/// Clap requires a subcommand; omitting it must produce a usage error, not a
/// successful no-op run.
#[ test ]
fn cli_no_subcommand_exits_nonzero()
{
  let output = run_cli( &[] );

  assert_ne!
  (
    output.status.code(),
    Some( 0 ),
    "expected non-zero exit code when no subcommand is given",
  );
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// TC-12: --capture flag
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

/// Test TC-12: `--capture` flag collects script stdout and makes it available.
///
/// With `--capture`, the runner buffers the script's output and the CLI then
/// `print!`s it to its own stdout. Verifies the capture→print path end-to-end.
#[ test ]
fn cli_capture_flag_collects_stdout()
{
  use std::path::PathBuf;

  let pid = std::process::id();
  let tmp : PathBuf = std::env::temp_dir()
    .join( format!( "pt_cli_tc12_{pid}" ) );
  std::fs::create_dir_all( &tmp ).expect( "failed to create tmp dir" );
  let source = tmp.join( "capture.rs" );
  std::fs::write( &source, r#"fn main() { println!( "captured output" ); }"# )
    .expect( "failed to write source" );

  let output = run_cli
  (
    &[ "run", "--capture", source.to_str().expect( "path is valid UTF-8" ) ]
  );
  std::fs::remove_dir_all( &tmp ).ok();

  assert_eq!
  (
    output.status.code(),
    Some( 0 ),
    "expected exit code 0 with --capture; stderr: {}",
    String::from_utf8_lossy( &output.stderr ),
  );
  let stdout = String::from_utf8_lossy( &output.stdout );
  assert!
  (
    stdout.contains( "captured output" ),
    "expected 'captured output' in CLI stdout when using --capture; got: {stdout}",
  );
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// TC-13: --env flag
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

/// Test TC-13: `--env KEY=VALUE` passes an environment variable to the script.
///
/// Verifies the full CLI→RunOptions→subprocess env-var injection path.
#[ test ]
fn cli_env_flag_passes_env_var_to_script()
{
  use std::path::PathBuf;

  let pid = std::process::id();
  let tmp : PathBuf = std::env::temp_dir()
    .join( format!( "pt_cli_tc13_{pid}" ) );
  std::fs::create_dir_all( &tmp ).expect( "failed to create tmp dir" );
  let source = tmp.join( "env_test.rs" );
  std::fs::write
  (
    &source,
    r#"fn main() { println!( "{}", std::env::var( "PT_CLI_VAR" ).unwrap_or( "MISSING".to_string() ) ); }"#,
  )
  .expect( "failed to write source" );

  let output = run_cli
  (
    &[
      "run",
      "--capture",
      "--env",
      "PT_CLI_VAR=from_cli_flag",
      source.to_str().expect( "path is valid UTF-8" ),
    ]
  );
  std::fs::remove_dir_all( &tmp ).ok();

  assert_eq!
  (
    output.status.code(),
    Some( 0 ),
    "expected exit code 0; stderr: {}",
    String::from_utf8_lossy( &output.stderr ),
  );
  let stdout = String::from_utf8_lossy( &output.stdout );
  assert!
  (
    stdout.contains( "from_cli_flag" ),
    "expected env var value in stdout; got: {stdout}",
  );
}
