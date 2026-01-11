//! Regression test for exit code documentation bug
//!
//! # Root Cause
//!
//! Help text in `.new.help` and `.check.help` documented exit codes as:
//! - 0: Success
//! - 1: General error
//! - 2: Invalid parameters
//! - 3: Validation failure (path doesn't exist, project exists)
//!
//! But implementation only uses:
//! - 0: Success
//! - 1: All errors (validation, I/O, issues found, already exists, path missing)
//! - 2: Unknown command (missing dot-prefix only)
//!
//! This creates mismatch between documentation and behavior, breaking CI/CD scripts
//! that expect exit codes 2 or 3 for specific error conditions.
//!
//! # Why Not Caught
//!
//! No automated test verified help text exit code documentation matches actual exit
//! codes from the implementation. Integration tests checked exit codes but didn't
//! verify they matched the documented behavior in help text.
//!
//! # Fix Applied
//!
//! Updated help text in `src/commands/help.rs` functions:
//! - `new_help()` - EXIT CODES section
//! - `check_help()` - EXIT CODES section
//! - `general_help()` - EXIT CODES section
//!
//! Changed documentation from incorrect 0/1/2/3 schema to actual 0/1/2 behavior.
//!
//! # Prevention
//!
//! This regression test ensures help text and implementation stay synchronized.
//! If exit code behavior changes, this test will fail and force documentation update.
//!
//! # Pitfall
//!
//! Never assume help text is source of truth without validating against implementation.
//! Always write tests that verify documented behavior matches actual behavior, especially
//! for contract-critical aspects like exit codes that CI/CD systems depend on.

use assert_cmd::Command;
use assert_fs::prelude::*;
use assert_fs::TempDir;

/// Verify all validation/error conditions return exit code 1 (not 2 or 3)
#[test]
fn exit_code_1_for_all_errors()
{
  let temp = TempDir::new().unwrap();

  // Empty project name → exit 1 (not 2)
  Command::cargo_bin( "cargo_unilang" )
    .unwrap()
    .arg( ".new" )
    .arg( "project::" )
    .current_dir( &temp )
    .assert()
    .failure()
    .code( 1 );

  // Path traversal → exit 1 (not 2)
  Command::cargo_bin( "cargo_unilang" )
    .unwrap()
    .arg( ".new" )
    .arg( "project::../invalid" )
    .current_dir( &temp )
    .assert()
    .failure()
    .code( 1 );

  // Invalid character → exit 1 (not 2)
  Command::cargo_bin( "cargo_unilang" )
    .unwrap()
    .arg( ".new" )
    .arg( "project::my@cli" )
    .current_dir( &temp )
    .assert()
    .failure()
    .code( 1 );

  // Invalid verbosity → exit 1 (not 2)
  Command::cargo_bin( "cargo_unilang" )
    .unwrap()
    .arg( ".new" )
    .arg( "project::test" )
    .arg( "verbosity::10" )
    .current_dir( &temp )
    .assert()
    .failure()
    .code( 1 );

  // Invalid template → exit 1 (not 2)
  Command::cargo_bin( "cargo_unilang" )
    .unwrap()
    .arg( ".new" )
    .arg( "project::test" )
    .arg( "template::invalid" )
    .current_dir( &temp )
    .assert()
    .failure()
    .code( 1 );

  // Unknown parameter → exit 1 (not 2)
  Command::cargo_bin( "cargo_unilang" )
    .unwrap()
    .arg( ".new" )
    .arg( "project::test" )
    .arg( "unknown::value" )
    .current_dir( &temp )
    .assert()
    .failure()
    .code( 1 );
}

/// Verify project already exists returns exit code 1 (not 3)
#[test]
fn exit_code_1_for_already_exists()
{
  let temp = TempDir::new().unwrap();

  // Create project first time
  Command::cargo_bin( "cargo_unilang" )
    .unwrap()
    .arg( ".new" )
    .arg( "project::test-project" )
    .arg( "verbosity::0" )
    .current_dir( &temp )
    .assert()
    .success()
    .code( 0 );

  // Try to create again → exit 1 (not 3)
  Command::cargo_bin( "cargo_unilang" )
    .unwrap()
    .arg( ".new" )
    .arg( "project::test-project" )
    .current_dir( &temp )
    .assert()
    .failure()
    .code( 1 );
}

/// Verify nonexistent path returns exit code 1 (not 3)
#[test]
fn exit_code_1_for_nonexistent_path()
{
  Command::cargo_bin( "cargo_unilang" )
    .unwrap()
    .arg( ".check" )
    .arg( "path::/definitely/does/not/exist" )
    .assert()
    .failure()
    .code( 1 );
}

/// Verify unknown command returns exit code 2 (the only use of exit 2)
#[test]
fn exit_code_2_for_unknown_command()
{
  // Missing dot-prefix: "new" instead of ".new"
  Command::cargo_bin( "cargo_unilang" )
    .unwrap()
    .arg( "new" )
    .arg( "project::test" )
    .assert()
    .failure()
    .code( 2 );

  // Missing dot-prefix: "check" instead of ".check"
  Command::cargo_bin( "cargo_unilang" )
    .unwrap()
    .arg( "check" )
    .assert()
    .failure()
    .code( 2 );
}

/// Verify success conditions return exit code 0
#[test]
fn exit_code_0_for_success()
{
  let temp = TempDir::new().unwrap();

  // Successful project creation
  Command::cargo_bin( "cargo_unilang" )
    .unwrap()
    .arg( ".new" )
    .arg( "project::success-test" )
    .arg( "verbosity::0" )
    .current_dir( &temp )
    .assert()
    .success()
    .code( 0 );

  // Successful check (no issues)
  temp.child( "clean-project" ).create_dir_all().unwrap();
  temp.child( "clean-project/Cargo.toml" )
    .write_str( "[dependencies]\nunilang = \"0.33\"\n" )
    .unwrap();

  Command::cargo_bin( "cargo_unilang" )
    .unwrap()
    .arg( ".check" )
    .arg( "path::clean-project" )
    .arg( "verbosity::0" )
    .current_dir( &temp )
    .assert()
    .success()
    .code( 0 );
}

/// Verify .check with issues found returns exit code 1
#[test]
fn exit_code_1_for_check_issues_found()
{
  let temp = TempDir::new().unwrap();
  temp.child( "problematic-project" ).create_dir_all().unwrap();
  temp.child( "problematic-project/Cargo.toml" )
    .write_str( "[dependencies]\nunilang = \"0.33\"\n" )
    .unwrap();
  temp.child( "problematic-project/build.rs" )
    .write_str( "fn main() { serde_yaml::from_str(); }" )
    .unwrap();

  Command::cargo_bin( "cargo_unilang" )
    .unwrap()
    .arg( ".check" )
    .arg( "path::problematic-project" )
    .arg( "verbosity::0" )
    .current_dir( &temp )
    .assert()
    .failure()
    .code( 1 );
}
