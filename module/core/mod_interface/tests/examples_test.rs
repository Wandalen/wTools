//! Tests validating that examples compile and execute correctly.
//!
//! # Root Cause
//!
//! Examples used `workspace = true` in Cargo.toml dependencies without being workspace members,
//! causing "current package believes it's in a workspace when it's not" error.
//!
//! # Why Not Caught
//!
//! - No automated tests validated example compilation/execution
//! - Examples were assumed to work if crate tests passed
//! - CI/CD likely didnt test examples directory standalone execution
//!
//! # Fix Applied
//!
//! - Added `[workspace]` table to make examples standalone packages
//! - Changed dependencies from `workspace = true` to `path = "../.."`
//! - Verified both examples compile and run successfully
//!
//! # Prevention
//!
//! - This test suite validates all examples compile and execute
//! - Run `cargo run` in each example directory as integration test
//! - Catch workspace configuration errors early
//!
//! # Pitfall
//!
//! When creating example packages that reference workspace crates, either:
//! 1. Make example a workspace member (add to workspace Cargo.toml), OR
//! 2. Make example standalone with `[workspace]` table + path dependencies
//!
//!    Never use `workspace = true` in non-member packages.

use std::process::Command;
use std::path::PathBuf;

/// Get the `mod_interface` crate root directory.
fn crate_root() -> PathBuf
{
  PathBuf::from( env!( "CARGO_MANIFEST_DIR" ) )
}

/// Test that `mod_interface_trivial` example compiles and runs successfully.
///
/// This test validates:
/// - Example uses correct Cargo.toml configuration (standalone package)
/// - All dependencies resolve correctly
/// - Example compiles without errors
/// - Example executes and completes successfully (exit code 0)
#[ test ]
fn example_trivial_compiles_and_runs()
{
  let example_dir = crate_root().join( "examples" ).join( "mod_interface_trivial" );

  // Run the example
  let output = Command::new( "cargo" )
    .arg( "run" )
    .current_dir( &example_dir )
    .output()
    .expect( "Failed to execute mod_interface_trivial example" );

  // Check compilation and execution succeeded
  assert!
  (
    output.status.success(),
    "mod_interface_trivial example failed:\nstdout: {}\nstderr: {}",
    String::from_utf8_lossy( &output.stdout ),
    String::from_utf8_lossy( &output.stderr )
  );
}

/// Test that `mod_interface_debug` example compiles and runs successfully.
///
/// This test validates:
/// - Example uses correct Cargo.toml configuration (standalone package)
/// - All dependencies resolve correctly
/// - Example compiles without errors
/// - Example executes and completes successfully (exit code 0)
#[ test ]
fn example_debug_compiles_and_runs()
{
  let example_dir = crate_root().join( "examples" ).join( "mod_interface_debug" );

  // Run the example
  let output = Command::new( "cargo" )
    .arg( "run" )
    .current_dir( &example_dir )
    .output()
    .expect( "Failed to execute mod_interface_debug example" );

  // Check compilation and execution succeeded
  assert!
  (
    output.status.success(),
    "mod_interface_debug example failed:\nstdout: {}\nstderr: {}",
    String::from_utf8_lossy( &output.stdout ),
    String::from_utf8_lossy( &output.stderr )
  );
}

/// Test that `mod_interface_debug` example has debug directive documentation.
///
/// This test validates:
/// - Debug directive is documented with instructions for enabling
/// - Example can be manually modified to enable debug output
/// - Documentation explains what debug directive does
///
/// Note: The debug directive functionality is manually verified. This test only
/// validates the documentation is present, to avoid test race conditions from
/// modifying example files during parallel test execution.
#[ test ]
fn example_debug_directive_documented()
{
  let example_dir = crate_root().join( "examples" ).join( "mod_interface_debug" );

  // Read the main.rs file
  let main_rs_path = example_dir.join( "src" ).join( "main.rs" );
  let content = std::fs::read_to_string( &main_rs_path )
    .expect( "Failed to read main.rs" );

  // Verify debug directive is documented
  assert!
  (
    content.contains( "#![ debug ]" ),
    "Example should contain debug directive (commented or uncommented)"
  );

  assert!
  (
    content.contains( "Uncomment" ) || content.contains( "debug output" ) || content.contains( "Debug" ),
    "Example should document how to use debug directive"
  );
}
