//! Binary functionality smoke tests
//!
//! Verifies that the willbe2 binary behaves as an alias to the core willbe tool.

use std::process::Command;

#[ test ]
fn binary_responds_to_list_command()
{
  // Test that willbe2 binary responds to .list command
  // Expected: Should execute without panic and produce output (like core willbe)

  let output = Command::new( "cargo" )
    .args( [ "run", "--quiet", "--", ".list" ] )
    .current_dir( env!( "CARGO_MANIFEST_DIR" ) )
    .output()
    .expect( "Failed to execute binary" );

  // Binary should exit successfully
  assert!
  (
    output.status.success(),
    "Binary should exit with success status. Stderr: {}",
    String::from_utf8_lossy( &output.stderr )
  );

  // Binary should produce output (not silent)
  let stdout = String::from_utf8_lossy( &output.stdout );
  assert!
  (
    !stdout.is_empty() || !output.stderr.is_empty(),
    "Binary should produce some output (stdout or stderr), but was completely silent"
  );
}

#[ test ]
fn binary_handles_invalid_input()
{
  // Test that willbe2 binary handles invalid input gracefully
  // Expected: Should show error message, not exit silently

  let output = Command::new( "cargo" )
    .args( [ "run", "--quiet", "--", "invalid_command_xyz" ] )
    .current_dir( env!( "CARGO_MANIFEST_DIR" ) )
    .output()
    .expect( "Failed to execute binary" );

  // Binary should produce error output (not silent)
  let stdout = String::from_utf8_lossy( &output.stdout );
  let stderr = String::from_utf8_lossy( &output.stderr );

  assert!
  (
    !stdout.is_empty() || !stderr.is_empty(),
    "Binary should show error message for invalid input, not exit silently"
  );
}
