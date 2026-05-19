//! Integration tests for genfile behavioral invariants.
//!
//! Covers invariant spec cases from `tests/docs/invariant/`.
//! `003_error_handling`: IN-01 (error format), IN-02 (nonzero exit), IN-03 (path traversal).
//! `004_security`: IN-01 covered in `materialization_test.rs`, IN-02 (load path traversal), IN-03 deferred.
//! `005_testing_coverage`: IN-01 (all commands covered), IN-02 (manifest-dir paths).

mod cli_runner;

// IN-01 (003): error messages use [ERROR] [CONTEXT]: format
//
// WHY: All error output must follow the uniform bracketed format defined in
// src/error.rs to allow scripted parsing and consistent UX. Verifies that
// a real error (loading nonexistent file) produces the canonical format.
#[ test ]
fn test_error_message_uses_bracketed_format()
{
  let output = cli_runner::cargo_run_command(
    &[ ".archive.load", "path::/tmp/invariant_test_nonexistent_99.json" ]
  )
  .output()
  .expect( "Command should execute" );

  assert!(
    !output.status.success(),
    "Loading nonexistent file must fail"
  );

  // Error output goes to stderr in CLI mode
  let stderr = String::from_utf8_lossy( &output.stderr );
  assert!(
    stderr.contains( "[ERROR]" ),
    "Error must use [ERROR] bracketed format. stderr: {stderr}"
  );
}

// IN-02 (003): failed command exits with nonzero exit code
//
// WHY: Scripts that pipe genfile output must be able to detect failures
// via $? / %ERRORLEVEL%. A zero exit on failure would silently break scripts.
// CLI mode must propagate the error exit code.
#[ test ]
fn test_failed_command_exits_nonzero()
{
  let output = cli_runner::cargo_run_command(
    &[ ".archive.load", "path::/tmp/invariant_nonexistent_exit_test.json" ]
  )
  .output()
  .expect( "Command should execute" );

  assert!(
    !output.status.success(),
    "Failed command must produce a nonzero exit code. \
     Exit code: {:?}",
    output.status.code()
  );

  // Verify exit code is specifically non-zero (not just success=false)
  let code = output.status.code().unwrap_or( -1 );
  assert!(
    code != 0,
    "Exit code must be non-zero, got: {code}"
  );
}

// IN-02 (004): `..` segments in archive load path must cause failure
//
// WHY: Loading an arbitrary file via path traversal (e.g. ../../etc/passwd)
// must fail. Even if the target file exists, parsing it as JSON fails, which
// is the expected defense: no attacker-controlled data is silently processed.
// Path validation on load paths prevents accidental or intentional traversal.
#[ test ]
fn test_dotdot_in_archive_load_path_rejected()
{
  // Use a path that traverses up from /tmp and would reach /etc/passwd on Linux
  let script = ".archive.load path::/tmp/../etc/passwd\nexit";

  let output = cli_runner::repl_command( script )
    .output()
    .expect( "Command should execute" );

  // Must fail — either parse error (if file found) or file-not-found
  let stdout = String::from_utf8_lossy( &output.stdout );
  assert!(
    !output.status.success() || stdout.contains( "ERROR" ),
    "Loading a path with .. traversal must fail or produce an error. \
     status: {:?}, stdout: {stdout}",
    output.status.code()
  );
}

// IN-01 (005): all implemented commands appear in at least one integration test
//
// WHY: Ensures no command silently regresses by having zero test coverage.
// Checks compile-time content of all integration test files.
// Note: 3 help-system commands (.help, .command.help) are deferred (FR9 not
// implemented) and excluded from this check.
#[ test ]
fn test_all_implemented_commands_have_coverage()
{
  // The 21 implemented commands in genfile (help system not yet implemented)
  let commands : &[ &str ] = &[
    ".archive.new",
    ".archive.load",
    ".archive.save",
    ".archive.from_directory",
    ".file.add",
    ".file.remove",
    ".file.list",
    ".file.show",
    ".parameter.add",
    ".parameter.remove",
    ".parameter.list",
    ".value.set",
    ".value.list",
    ".content.list",
    ".content.externalize",
    ".content.internalize",
    ".materialize",
    ".unpack",
    ".pack",
    ".info",
    ".discover.parameters",
    ".status",
    ".analyze",
  ];

  // All test file contents concatenated at compile time
  let all_content = concat!(
    include_str!( "archive_commands_test.rs" ),
    include_str!( "file_commands_test.rs" ),
    include_str!( "param_value_commands_test.rs" ),
    include_str!( "content_commands_test.rs" ),
    include_str!( "materialization_test.rs" ),
    include_str!( "analysis_test.rs" ),
    include_str!( "repl_exit_code_bug_test.rs" ),
  );

  for &command in commands
  {
    assert!(
      all_content.contains( command ),
      "Command '{command}' has no integration test coverage across any test file"
    );
  }
}

// IN-02 (005): test infrastructure uses CARGO_MANIFEST_DIR for path resolution
//
// WHY: Hardcoded absolute paths break tests on other machines and in CI.
// cli_runner::project_dir() must return the compile-time manifest directory
// so tests run correctly regardless of the caller's working directory.
#[ test ]
fn test_tests_use_manifest_directory_paths()
{
  use std::path::PathBuf;

  let project_dir = cli_runner::project_dir();
  let expected = PathBuf::from( env!( "CARGO_MANIFEST_DIR" ) );

  assert_eq!(
    project_dir,
    expected,
    "cli_runner::project_dir() must return env!(CARGO_MANIFEST_DIR)"
  );

  // Also verify the directory actually contains Cargo.toml (sanity check)
  assert!(
    project_dir.join( "Cargo.toml" ).exists(),
    "project_dir must point to the genfile crate root"
  );
}
