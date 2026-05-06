//! Test: Example Quality Validation
//!
//! ## Purpose
//! Validates that examples demonstrate appropriate functionality for an error-handling crate.
//! This test reproduces the issue where `error_tools_trivial.rs` demonstrates no error handling.
//!
//! ## Root Cause
//! `error_tools_trivial.rs` was created as a placeholder "Hello, world!" example without
//! demonstrating any error handling functionality, despite being the trivial example for
//! an error-handling library.
//!
//! ## Why Not Caught
//! No automated validation exists for example quality. Examples are typically run manually
//! without verification that they demonstrate appropriate functionality for the crate.
//!
//! ## Fix Applied
//! Replace `error_tools_trivial.rs` with a minimal but realistic error handling example that
//! demonstrates Result<T, E>, error creation, and error propagation.
//!
//! ## Prevention
//! This test validates that the trivial example demonstrates core error handling patterns.
//! Future changes to examples must maintain this standard.
//!
//! ## Pitfall
//! When creating "trivial" examples, ensure they still demonstrate the crate's core purpose.
//! A trivial example for an error-handling crate MUST show error handling, even if minimal.

use std ::fs;
use std ::path ::Path;

#[ test ]
fn test_trivial_example_demonstrates_error_handling()
{
  let example_path = Path ::new( "examples/error_tools_trivial.rs" );

  // Verify example file exists
  assert!( example_path.exists(), "error_tools_trivial.rs must exist" );

  // Read example source code
  let source = fs ::read_to_string( example_path )
    .expect( "Failed to read error_tools_trivial.rs" );

  // Verify example demonstrates error handling patterns
  // A trivial error handling example MUST demonstrate:
  // 1. Result<T, E> return type
  // 2. Error creation or propagation
  // 3. Basic error handling pattern

  // Check 1: Uses Result type (core error handling pattern)
  assert!(
    source.contains( "Result<" ) || source.contains( "Result <" ),
    "Trivial example must demonstrate Result<T, E> return type. \
     Current implementation shows no error handling."
  );

  // Check 2: Demonstrates error creation (format_err!, anyhow!, or Error creation)
  let demonstrates_error_creation = source.contains( "format_err!" )
    || source.contains( "anyhow!" )
    || source.contains( "Error" )
    || source.contains( "Err(" );

  assert!(
    demonstrates_error_creation,
    "Trivial example must demonstrate error creation or propagation. \
     'Hello, world!' does not demonstrate error handling."
  );

  // Check 3: Contains error handling keywords (?, match, unwrap, etc.)
  let demonstrates_error_handling = source.contains( '?' )
    || source.contains( "match" )
    || source.contains( "unwrap" )
    || source.contains( "expect" );

  assert!(
    demonstrates_error_handling,
    "Trivial example must show basic error handling pattern. \
     Current implementation is just a greeting function."
  );
}

#[ test ]
fn test_trivial_example_compiles_and_runs()
{
  // Verify the example compiles
  let output = std ::process ::Command ::new( "cargo" )
    .args( [ "build", "--example", "error_tools_trivial", "--all-features" ] )
    .current_dir( env!( "CARGO_MANIFEST_DIR" ) )
    .output()
    .expect( "Failed to compile example" );

  assert!(
    output.status.success(),
    "error_tools_trivial.rs must compile successfully. Error: {}",
    String ::from_utf8_lossy( &output.stderr )
  );

  // Verify the example runs
  let output = std ::process ::Command ::new( "cargo" )
    .args( [ "run", "--example", "error_tools_trivial", "--all-features" ] )
    .current_dir( env!( "CARGO_MANIFEST_DIR" ) )
    .output()
    .expect( "Failed to run example" );

  assert!(
    output.status.success(),
    "error_tools_trivial.rs must run successfully. Error: {}",
    String ::from_utf8_lossy( &output.stderr )
  );
}
