//! Test that examples produce expected output and demonstrate crate functionality.
//!
//! # Root Cause
//!
//! The `inspect_type_trivial.rs` example had all functional code commented out due to
//! historical nightly Rust requirements. While the crate now works on stable Rust
//! (as noted in `src/lib.rs:10`), the example was never re-enabled, leaving it as
//! an empty demonstration that produces no output.
//!
//! # Why Not Caught
//!
//! No automated test verified that examples actually produce output or demonstrate
//! functionality. Examples could be completely empty and pass compilation checks.
//!
//! # Fix Applied
//!
//! 1. Created this test to verify example produces expected output
//! 2. Will uncomment and update example code to work on stable Rust
//! 3. Test verifies both compilation and output correctness
//!
//! # Prevention
//!
//! This test ensures examples:
//! - Compile successfully
//! - Produce actual output (not empty)
//! - Demonstrate core crate functionality
//! - Can be run by users to learn the API
//!
//! # Pitfall
//!
//! Examples can become outdated or disabled during API transitions. Always verify
//! examples actually demonstrate functionality, not just compile. Examples are the
//! first touchpoint for users learning a crate - empty examples make crates appear
//! broken or incomplete.

// test_kind: bug_reproducer(issue-001)

use std::process::Command;
use std::path::PathBuf;

#[ test ]
fn example_inspect_type_trivial_produces_output()
{
  // Get crate root directory
  let manifest_dir = env!( "CARGO_MANIFEST_DIR" );
  let crate_root = PathBuf::from( manifest_dir );

  // Run the example
  let output = Command::new( "cargo" )
    .arg( "run" )
    .arg( "--example" )
    .arg( "inspect_type_trivial" )
    .arg( "--quiet" )
    .current_dir( &crate_root )
    .output()
    .expect( "Failed to execute example" );

  // Convert output to string
  let stdout = String::from_utf8_lossy( &output.stdout );
  let stderr = String::from_utf8_lossy( &output.stderr );

  // Check that example compiled successfully
  assert!
  (
    output.status.success(),
    "Example failed to compile or run.\nstderr: {stderr}\nstdout: {stdout}"
  );

  // CRITICAL: Example must produce output demonstrating functionality
  // An empty example is useless for users trying to learn the crate
  assert!
  (
    !stdout.trim().is_empty(),
    "Example produced no output! Examples must demonstrate crate functionality.\n\
     Expected output showing type inspection (e.g., 'sizeof( ... : ... ) = ...').\n\
     Got empty output instead.\n\
     This makes the example useless for users learning the crate."
  );

  // Verify output contains expected format: "sizeof( ... : ... ) = ..."
  assert!
  (
    stdout.contains( "sizeof(" ) && stdout.contains( " : " ) && stdout.contains( " ) = " ),
    "Example output doesn't match expected format.\n\
     Expected format: 'sizeof( expression : type ) = size'\n\
     Got: {stdout}"
  );

  // Verify output demonstrates slice vs array difference (key use case)
  // This is the primary value proposition of the crate
  let has_slice = stdout.contains( "&[i32]" ) || stdout.contains( "&[" );
  let has_array = stdout.contains( "[i32; " ) || stdout.contains( "; " );

  assert!
  (
    has_slice || has_array,
    "Example should demonstrate slice and/or array type inspection.\n\
     This is a core use case for type introspection.\n\
     Got output: {stdout}"
  );
}

#[ test ]
fn example_demonstrates_core_functionality()
{
  // Get crate root directory
  let manifest_dir = env!( "CARGO_MANIFEST_DIR" );
  let crate_root = PathBuf::from( manifest_dir );

  // Run the example with all features
  let output = Command::new( "cargo" )
    .arg( "run" )
    .arg( "--example" )
    .arg( "inspect_type_trivial" )
    .arg( "--all-features" )
    .arg( "--quiet" )
    .current_dir( &crate_root )
    .output()
    .expect( "Failed to execute example" );

  let stdout = String::from_utf8_lossy( &output.stdout );
  let stderr = String::from_utf8_lossy( &output.stderr );

  // Verify successful execution
  assert!
  (
    output.status.success(),
    "Example with all features failed.\nstderr: {stderr}\nstdout: {stdout}"
  );

  // Must demonstrate inspect_type_of! macro (primary API)
  // Without this, users dont know how to use the crate
  assert!
  (
    !stdout.trim().is_empty(),
    "Example must show how to use inspect_type_of! macro"
  );
}
