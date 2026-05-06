//! REPL Exit Code Bug Reproduction Tests
//!
//! This test file documents and reproduces the critical bug where the REPL
//! exits with code 0 even when commands fail, violating loud failure principles.

/// Bug Reproducer: REPL exits with code 0 despite command failures (issue-001)
///
/// # Root Cause
///
/// In `src/repl.rs:110`, the `run_repl()` function ALWAYS returns `Ok(())` regardless
/// of command execution results. When commands fail during REPL execution:
///
/// 1. Line 83: `let result = pipeline.process_command( input, ctx );`
/// 2. Lines 96-99: If `!result.success`, error is printed to stderr via `eprintln!`
/// 3. Line 52: Loop continues to next iteration (no break or status tracking)
/// 4. Line 110: Function returns `Ok(())` unconditionally
/// 5. This `Ok(())` propagates to `main.rs:39` which returns it to the OS
/// 6. Process exits with code 0 (OS success mapping)
///
/// The REPL was designed to continue execution after errors (proper for interactive
/// use), but failed to track that errors occurred for final exit code reporting.
/// CLI mode (single command) in `main.rs:47-53` DOES check `result.success` and
/// calls `std::process::exit(1)` on failure, demonstrating the inconsistency.
///
/// # Why Not Caught
///
/// Existing integration tests in `tests/*_commands_test.rs` only test CLI mode
/// (single command execution via `cargo run -- .command.name args`), NOT REPL mode.
/// The test pattern used throughout the suite is:
///
/// ```rust
/// let output = std::process::Command::new( "cargo" )
///   .args( [ "run", "--quiet", "--", ".archive.new", "name::test" ] )
///   .output()?;
/// assert!( output.status.success() );
/// ```
///
/// This tests CLI mode where `main.rs:53` calls `std::process::exit(1)` on failure.
/// REPL mode is only tested in manual examples (`examples/basic_workflow.sh`), not
/// automated tests. No tests verified REPL exit codes, so the bug went undetected
/// for all 53 existing integration tests.
///
/// # Fix Applied
///
/// Modified `src/repl.rs:41-111` to track command execution status:
///
/// 1. Added `had_errors: bool = false` flag before REPL loop (after line 50)
/// 2. Set `had_errors = true` when `!result.success` (line 96)
/// 3. Changed line 110 from `Ok(())` to:
///    ```rust
///    if had_errors {
///      Err( "One or more commands failed during REPL session".into() )
///    } else {
///      Ok(())
///    }
///    ```
/// 4. This error propagates to `main.rs:39` which returns `Err`, causing non-zero exit
///
/// Implementation preserves REPL's continue-on-error behavior (interactive UX)
/// while ensuring exit code correctly reports failure (scriptability requirement).
///
/// # Prevention
///
/// 1. **Test REPL Exit Codes:** Add integration tests spawning REPL mode and
///    verifying `output.status.code()` for both success and failure scenarios
/// 2. **Test Coverage Audit:** Ensure both CLI mode AND REPL mode are tested for
///    all command categories (archive, file, parameter, content, materialization)
/// 3. **Exit Code Contract:** Document explicit contract that ALL failure modes
///    must result in non-zero exit codes, regardless of execution mode
/// 4. **Manual Example Verification:** Add exit code checks to `examples/*.sh`
///    scripts using `set -e` and explicit `if ! cargo run; then` blocks
///
/// # Pitfall
///
/// Similar pattern exists in any loop-based execution model where errors are
/// handled locally (printed/logged) but not aggregated for final reporting.
/// When designing interactive vs scripting modes, ALWAYS maintain separate
/// concerns: interactive error recovery (continue execution) vs exit code
/// reporting (aggregate status).
///
/// Watch for this pattern in future commands that execute multiple operations:
/// - Batch processing modes
/// - Transaction-based operations
/// - Any "process list of items" functionality
///
/// The `execute_many()` pattern should track individual failures even when
/// continuing to process remaining items, then return non-zero if ANY failed.
#[test]
fn repl_exits_with_nonzero_on_command_failure()
{

  // Reproduce bug: Execute failing command in REPL mode
  let output = std::process::Command::new( "cargo" )
    .args( [ "run", "--quiet", "--release" ] )
    .stdin( std::process::Stdio::piped() )
    .current_dir( env!( "CARGO_MANIFEST_DIR" ) )
    .stdout( std::process::Stdio::piped() )
    .stderr( std::process::Stdio::piped() )
    .spawn()
    .expect( "Failed to spawn genfile process" );

  // Write REPL commands to stdin (command that should fail)
  use std::io::Write;
  output.stdin
    .as_ref()
    .unwrap()
    .write_all( b".file.add path::\"test.txt\" content::\"Should fail - no archive\"\nexit\n" )
    .expect( "Failed to write to stdin" );

  let result = output.wait_with_output().expect( "Failed to wait for process" );

  // BUG: Before fix, exit code is 0 despite command failure
  // EXPECTED: Exit code should be non-zero when any command fails
  assert!(
    !result.status.success(),
    "REPL should exit with non-zero code when commands fail. \
     Exit code: {:?}, stderr: {}",
    result.status.code(),
    String::from_utf8_lossy( &result.stderr )
  );

  // Verify error was reported to stderr
  let stderr = String::from_utf8_lossy( &result.stderr );
  assert!(
    stderr.contains( "No archive loaded" ) || stderr.contains( "ERROR" ),
    "Error message should be present in stderr: {stderr}"
  );
}

/// Verify REPL still exits with code 0 when ALL commands succeed (regression guard)
#[test]
fn repl_exits_with_zero_on_all_commands_successful()
{

  use std::io::Write;

  let output = std::process::Command::new( "cargo" )
    .args( [ "run", "--quiet", "--release" ] )
    .stdin( std::process::Stdio::piped() )
    .current_dir( env!( "CARGO_MANIFEST_DIR" ) )
    .stdout( std::process::Stdio::piped() )
    .stderr( std::process::Stdio::piped() )
    .spawn()
    .expect( "Failed to spawn genfile process" );

  // Write successful REPL commands
  output.stdin
    .as_ref()
    .unwrap()
    .write_all(
      b".archive.new name::\"test\" description::\"Test\"\n\
        .file.add path::\"readme.txt\" content::\"Hello\"\n\
        exit\n"
    )
    .expect( "Failed to write to stdin" );

  let result = output.wait_with_output().expect( "Failed to wait for process" );

  // All commands successful - should exit with code 0
  assert!(
    result.status.success(),
    "REPL should exit with code 0 when all commands succeed. \
     Exit code: {:?}, stderr: {}",
    result.status.code(),
    String::from_utf8_lossy( &result.stderr )
  );
}

/// Verify REPL exits with non-zero if ANY command fails, even if others succeed
#[test]
fn repl_exits_with_nonzero_when_any_command_fails()
{

  use std::io::Write;

  let output = std::process::Command::new( "cargo" )
    .args( [ "run", "--quiet", "--release" ] )
    .stdin( std::process::Stdio::piped() )
    .current_dir( env!( "CARGO_MANIFEST_DIR" ) )
    .stdout( std::process::Stdio::piped() )
    .stderr( std::process::Stdio::piped() )
    .spawn()
    .expect( "Failed to spawn genfile process" );

  // Mix of successful and failing commands
  output.stdin
    .as_ref()
    .unwrap()
    .write_all(
      b".archive.new name::\"test\" description::\"Test\"\n\
        .file.add path::\"readme.txt\" content::\"Hello\"\n\
        .file.add path::\"\" content::\"Should fail - empty path\"\n\
        exit\n"
    )
    .expect( "Failed to write to stdin" );

  let result = output.wait_with_output().expect( "Failed to wait for process" );

  // One command failed - should exit with non-zero
  assert!(
    !result.status.success(),
    "REPL should exit with non-zero when any command fails. \
     Exit code: {:?}, stderr: {}",
    result.status.code(),
    String::from_utf8_lossy( &result.stderr )
  );
}
