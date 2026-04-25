//! Test suite validating example code quality and output correctness.
//!
//! This test module ensures example files demonstrate the `for_each!` macro
//! correctly without misleading duplicate code, confusing patterns, or
//! unresolved TODO markers.

use std::process::Command;

/// Reproduces bug where `for_each_trivial.rs` produces duplicate output.
///
/// ## Root Cause
/// Example contains both macro invocation AND manually written dbg! calls
/// (lines 10-12), causing 6 outputs instead of expected 3.
///
/// ## Why Not Caught Initially
/// No automated example quality tests existed to verify output correctness.
///
/// ## Fix Applied
/// Convert manually written dbg! calls to comments to serve as documentation
/// without executing.
///
/// ## Prevention
/// This test now validates example output contains exactly expected number
/// of debug statements.
///
/// ## Pitfall to Avoid
/// Never mix executable code with "// generates" comments - either use
/// proper multi-line comments or remove the duplicate code entirely.
// test_kind: bug_reproducer(issue-for_each-duplicate-output)
#[test]
fn test_for_each_trivial_output_not_duplicated()
{
  let output = Command::new( "cargo" )
    .args( [ "run", "--example", "for_each_trivial", "--features", "enabled" ] )
    .current_dir( env!( "CARGO_MANIFEST_DIR" ) )
    .output()
    .expect( "Failed to execute example" );

  let stdout = String::from_utf8_lossy( &output.stdout );
  let stderr = String::from_utf8_lossy( &output.stderr );

  // Count debug output lines (should be exactly 3, not 6)
  let debug_count = stdout.lines()
    .chain( stderr.lines() )
    .filter( | line | line.contains( "for_each_trivial.rs:" ) && line.contains( '=' ) )
    .count();

  assert_eq!(
    debug_count,
    3,
    "Example should produce exactly 3 debug outputs, found {debug_count}\nStdout:\n{stdout}\nStderr:\n{stderr}"
  );
}

/// Reproduces bug where `for_each_map_style_sample.rs` produces duplicate output.
///
/// ## Root Cause
/// Example contains both macro invocation AND manually written dbg! calls
/// (lines 16-18), causing 6 outputs instead of expected 3.
///
/// ## Why Not Caught Initially
/// No automated example quality tests existed to verify output correctness.
///
/// ## Fix Applied
/// Convert manually written dbg! calls to comments to serve as documentation
/// without executing.
///
/// ## Prevention
/// This test now validates example output contains exactly expected number
/// of debug statements.
///
/// ## Pitfall to Avoid
/// Documentation comments should not contain executable code. Use proper
/// comment syntax or separate documentation from code.
// test_kind: bug_reproducer(issue-for_each-duplicate-output)
#[test]
fn test_for_each_map_style_output_not_duplicated()
{
  let output = Command::new( "cargo" )
    .args( [ "run", "--example", "for_each_map_style_sample", "--features", "enabled" ] )
    .current_dir( env!( "CARGO_MANIFEST_DIR" ) )
    .output()
    .expect( "Failed to execute example" );

  let stdout = String::from_utf8_lossy( &output.stdout );
  let stderr = String::from_utf8_lossy( &output.stderr );

  // Count debug output lines (should be exactly 3, not 6)
  let debug_count = stdout.lines()
    .chain( stderr.lines() )
    .filter( | line | line.contains( "for_each_map_style_sample.rs:" ) && line.contains( '=' ) )
    .count();

  assert_eq!(
    debug_count,
    3,
    "Example should produce exactly 3 debug outputs, found {debug_count}\nStdout:\n{stdout}\nStderr:\n{stderr}"
  );
}

/// Test that examples have proper module-level documentation.
///
/// Verifies examples don't have unresolved TODO markers (`qqq:`).
#[test]
fn test_examples_have_proper_documentation()
{
  let trivial_src = std::fs::read_to_string(
    std::path::Path::new( env!( "CARGO_MANIFEST_DIR" ) )
      .join( "examples" )
      .join( "for_each_trivial.rs" )
  ).expect( "Failed to read for_each_trivial.rs" );

  assert!(
    !trivial_src.contains( "qqq:" ),
    "for_each_trivial.rs should not contain unresolved TODO markers (qqq:)"
  );

  let map_style_src = std::fs::read_to_string(
    std::path::Path::new( env!( "CARGO_MANIFEST_DIR" ) )
      .join( "examples" )
      .join( "for_each_map_style_sample.rs" )
  ).expect( "Failed to read for_each_map_style_sample.rs" );

  assert!(
    !map_style_src.contains( "qqq:" ),
    "for_each_map_style_sample.rs should not contain unresolved TODO markers (qqq:)"
  );
}
