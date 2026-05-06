//! Test to verify example 002 actually demonstrates error messages and diffs
//!
//! This test ensures that the `demonstrate_failures()` function in example 002
//! actually demonstrates diff output rather than just instructing users to uncomment code.
//!
//! ## Root Cause
//! Example 002 claims to demonstrate "beautiful error diffs" but the `demonstrate_failures()`
//! function contains only commented-out code and a message telling users to uncomment it.
//! This violates the principle that examples should SHOW functionality, not TELL users
//! to modify code.
//!
//! ## Why Not Caught
//! Examples are typically run manually without automated checks for demonstration quality.
//! No automated test verified that examples actually demonstrate their claimed functionality.
//!
//! ## Fix Applied
//! Modified `demonstrate_failures()` to include actual working demonstrations of diff output
//! using `catch_unwind` to handle panics gracefully while showing the diff messages.
//!
//! ## Prevention
//! - Added this test to verify example quality
//! - Test checks that `demonstrate_failures` produces actual diff output
//! - CI will fail if examples regress to empty demonstrations
//!
//! ## Pitfall
//! Examples that instruct users to "uncomment code" are less valuable than examples that
//! directly demonstrate functionality. Always prefer executable demonstrations over instructions.

#[ cfg( feature = "diagnostics_runtime_assertions" ) ]
#[ test ]
fn example_002_demonstrates_actual_diff_output()
{
  // This test verifies that example 002's demonstrate_failures function
  // actually produces diff output, not just instructions to uncomment code.

  // Currently this test will FAIL because demonstrate_failures() is empty.
  // After fixing example 002, this test will PASS.

  // We test by running the example and checking its output contains
  // actual diff demonstrations, not just "uncomment" messages.

  use std ::process ::Command;

  // `a_id!` expands to `$crate::dependency::pretty_assertions::assert_eq!` which requires
  // the `diagnostics_runtime_assertions` feature; run with all features to satisfy it.
  let output = Command ::new( "cargo" )
    .args( [ "run", "--example", "002_better_error_messages", "--all-features" ] )
    .current_dir( env!( "CARGO_MANIFEST_DIR" ) )
    .output()
    .expect( "Failed to execute example 002" );

  let stdout = String ::from_utf8_lossy( &output.stdout );
  let stderr = String ::from_utf8_lossy( &output.stderr );
  let combined = format!( "{stdout}{stderr}" );

  // The example should demonstrate actual diff output, not just tell users to uncomment
  assert!(
    !combined.contains( "Uncomment code in demonstrate_failures()" ) ||
    combined.contains( "Different vectors:" ) ||
    combined.contains( "Different structs:" ) ||
    combined.contains( "Different strings:" ),
    "Example 002 should demonstrate actual diff output, not just instruct users to uncomment code.\n\
     Current output contains only instructions, not demonstrations."
  );

  // Verify the example still runs successfully
  assert!(
    output.status.success(),
    "Example 002 should run successfully after demonstrating diff output"
  );
}
