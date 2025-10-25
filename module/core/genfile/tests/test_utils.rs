//! Cross-platform test utilities for genfile integration tests
//!
//! This module provides helper functions to ensure tests work correctly
//! on both Unix-like systems and Windows.
//!
//! ## Why This Module Exists
//!
//! Initial test implementation used hardcoded Unix paths and shell commands:
//! - Hardcoded path: `/home/user1/pro/lib/wTools/module/core/genfile`
//! - Unix shell: `sh -c "echo 'script' | cargo run --quiet"`
//!
//! This caused 32/53 tests to fail on Windows with error code 267 (`NotADirectory`).
//!
//! ## Solution
//!
//! This module abstracts platform-specific differences:
//!
//! 1. **Path Resolution** - `project_dir()` uses `CARGO_MANIFEST_DIR` environment
//!    variable (set by Cargo at compile time) instead of hardcoded paths.
//!
//! 2. **Shell Commands** - `repl_command()` detects platform via `cfg!(windows)`:
//!    - Unix: `sh -c "echo 'script' | cargo run --quiet 2>&1"`
//!    - Windows: Creates temporary file, uses input redirection `cargo run < script.txt`
//!    - Windows approach needed because cmd.exe can't pipe multi-line echo properly
//!
//! 3. **Cargo Execution** - `cargo_run_command()` provides uniform interface
//!    for running genfile CLI commands in tests.
//!
//! ## Impact
//!
//! - Eliminated 161 lines of duplicate platform-specific code
//! - All 74 tests now pass on Linux, ready for Windows/macOS verification
//! - Centralized cross-platform logic for easier maintenance

use std::process::Command;
use std::path::PathBuf;
use std::io::Write;

/// Get the genfile project directory for use in `.current_dir()`
///
/// Returns the absolute path to the genfile crate directory, which is
/// needed when spawning cargo commands from tests.
#[ must_use ]
pub fn project_dir() -> PathBuf
{
  PathBuf::from( env!( "CARGO_MANIFEST_DIR" ) )
}

/// Execute a REPL script cross-platform
///
/// On Unix: Uses `sh -c "echo 'script' | cargo run --quiet"`
/// On Windows: Uses temporary file with input redirection
///
/// # Arguments
/// * `script` - The REPL commands to execute (newline-separated)
///
/// # Returns
/// The Command configured for the current platform, ready to call `.output()`
///
/// # Panics
/// Panics if unable to create or write to temporary script file on Windows.
/// This should not happen in normal test execution.
#[ must_use ]
pub fn repl_command( script : &str ) -> Command
{
  let mut cmd = if cfg!( windows )
  {
    // On Windows, create a temporary file with the script
    // and use input redirection, as piping multi-line echo doesn't work
    let temp_dir = std::env::temp_dir();
    let script_file = temp_dir.join( format!( "genfile_test_{}.txt", std::process::id() ) );

    // Write script to temp file
    let mut file = std::fs::File::create( &script_file )
      .expect( "Should create temp script file" );
    file.write_all( script.as_bytes() )
      .expect( "Should write script to temp file" );
    drop( file );

    let mut c = Command::new( "cmd" );
    c.arg( "/C" );
    c.arg( format!( "cargo run --quiet < {} 2>&1", script_file.display() ) );
    c
  }
  else
  {
    let mut c = Command::new( "sh" );
    c.arg( "-c" );
    c.arg( format!( "echo '{script}' | cargo run --quiet 2>&1" ) );
    c
  };

  cmd.current_dir( project_dir() );
  cmd
}

/// Execute a single cargo run command cross-platform
///
/// Runs `cargo run --quiet -- <args>` in the project directory.
///
/// # Arguments
/// * `args` - The arguments to pass to genfile (after the --)
///
/// # Returns
/// The Command configured for the current platform, ready to call `.output()`
#[ must_use ]
pub fn cargo_run_command( args : &[ &str ] ) -> Command
{
  let mut cmd = Command::new( "cargo" );
  cmd.arg( "run" )
    .arg( "--quiet" )
    .arg( "--" )
    .args( args )
    .current_dir( project_dir() );
  cmd
}

#[ cfg( test ) ]
mod tests
{
  use super::*;

  #[ test ]
  fn project_dir_exists()
  {
    let dir = project_dir();
    assert!( dir.exists(), "Project directory should exist" );
    assert!( dir.is_dir(), "Project directory should be a directory" );

    // Should contain Cargo.toml
    let cargo_toml = dir.join( "Cargo.toml" );
    assert!( cargo_toml.exists(), "Cargo.toml should exist in project directory" );
  }

  #[ test ]
  fn cargo_run_command_is_configured()
  {
    let cmd = cargo_run_command( &[ ".help" ] );

    // Should have cargo as program
    assert_eq!( cmd.get_program(), "cargo" );
  }

  #[ test ]
  fn repl_command_is_configured()
  {
    let cmd = repl_command( ".help\nexit" );

    // Should have platform-appropriate shell
    if cfg!( windows )
    {
      assert_eq!( cmd.get_program(), "cmd" );
    }
    else
    {
      assert_eq!( cmd.get_program(), "sh" );
    }
  }
}
