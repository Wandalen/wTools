//! Test to verify required examples exist
//!
//! This test reproduces Issue #1 and #2 from manual testing session 2026-01-04:
//! - Issue #1: Example `proper_tools_trivial` referenced in readme:44 doesn't exist
//! - Issue #2: No examples/ directory in crate root
//!
//! This is a regression guard ensuring examples remain present after implementation.

use std::path::Path;

#[ test ]
fn test_examples_directory_exists()
{
  let manifest_dir = env!( "CARGO_MANIFEST_DIR" );
  let examples_dir = Path::new( manifest_dir ).join( "examples" );

  assert!
  (
    examples_dir.exists(),
    "examples/ directory must exist at crate root ({})",
    examples_dir.display()
  );

  assert!
  (
    examples_dir.is_dir(),
    "examples/ must be a directory, not a file"
  );
}

#[ test ]
fn test_proper_tools_trivial_example_exists()
{
  let manifest_dir = env!( "CARGO_MANIFEST_DIR" );
  let example_file = Path::new( manifest_dir ).join( "examples" ).join( "proper_tools_trivial.rs" );

  assert!
  (
    example_file.exists(),
    "examples/proper_tools_trivial.rs must exist (referenced in readme.md:44)"
  );

  assert!
  (
    example_file.is_file(),
    "examples/proper_tools_trivial.rs must be a file"
  );
}

#[ test ]
fn test_example_compiles_and_runs()
{
  // This test verifies the example can be compiled and run successfully
  // It runs as part of the test suite to catch compilation errors early

  let output = std::process::Command::new( "cargo" )
    .args( [ "run", "--example", "proper_tools_trivial" ] )
    .current_dir( env!( "CARGO_MANIFEST_DIR" ) )
    .output()
    .expect( "Failed to execute cargo run --example" );

  assert!
  (
    output.status.success(),
    "Example proper_tools_trivial must compile and run successfully.\nstdout: {}\nstderr: {}",
    String::from_utf8_lossy( &output.stdout ),
    String::from_utf8_lossy( &output.stderr )
  );
}
