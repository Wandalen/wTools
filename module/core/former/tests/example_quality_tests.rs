//! Tests for example code quality and naming correctness.
//!
//! ## Root Cause (issue-manual-test-001)
//!
//! Example file `former_trivial_expaned.rs` contained typo "expaned" instead of "expanded".
//! This occurred because there was no automated verification of example filenames against
//! common spelling errors. The file was created manually without spell-checking, and the
//! typo persisted undetected since the file compiled and ran correctly.
//!
//! ## Why Not Caught
//!
//! No existing tests verified example filename spelling. While the code compiled and executed
//! correctly, the filename typo was not visible to the compiler or runtime. Manual review
//! processes did not catch this cosmetic issue.
//!
//! ## Fix Applied
//!
//! 1. Renamed `former_trivial_expaned.rs` to `former_trivial_expanded.rs` via git mv
//! 2. Created this test to verify example filenames don't contain common typos
//! 3. Test checks for known problematic patterns (expaned→expanded, etc.)
//!
//! ## Prevention
//!
//! 1. Run this test as part of CI to catch filename typos early
//! 2. Use spell-checking tools on all documentation and filenames
//! 3. Follow consistent naming conventions for generated/expanded code examples
//! 4. Add more typo patterns to this test as they are discovered
//!
//! ## Pitfall
//!
//! Filename typos are invisible to compilers but harm documentation quality and user
//! experience. Always verify filenames match their documented purpose. Consider using
//! automated filename validation in CI pipelines for public-facing examples.

#[ test ]
fn test_example_filenames_no_common_typos()
{
  // This test verifies example filenames don't contain common spelling mistakes

  let examples_dir = std::path::Path::new( env!( "CARGO_MANIFEST_DIR" ) )
    .join( "examples" );

  let entries = std::fs::read_dir( &examples_dir )
    .expect( "Failed to read examples directory" );

  let mut typo_found = false;
  let mut typo_messages = Vec::new();

  for entry in entries
  {
    let entry = entry.expect( "Failed to read directory entry" );
    let filename = entry.file_name();
    let filename_str = filename.to_string_lossy();

    // Check for common typos
    let typo_patterns =
    [
      ( "expaned", "expanded" ),
      ( "seperate", "separate" ),
      ( "occured", "occurred" ),
      ( "defintion", "definition" ),
    ];

    for ( typo, correct ) in typo_patterns
    {
      if filename_str.contains( typo )
      {
        typo_found = true;
        typo_messages.push
        (
          format!
          (
            "File '{filename_str}' contains typo '{typo}' (should be '{correct}')"
          )
        );
      }
    }
  }

  assert!
  (
    !typo_found,
    "Found typos in example filenames:\n{}",
    typo_messages.join( "\n" )
  );
}

#[ test ]
fn test_example_filenames_follow_conventions()
{
  // Verify example filenames follow naming conventions

  let examples_dir = std::path::Path::new( env!( "CARGO_MANIFEST_DIR" ) )
    .join( "examples" );

  let entries = std::fs::read_dir( &examples_dir )
    .expect( "Failed to read examples directory" );

  for entry in entries
  {
    let entry = entry.expect( "Failed to read directory entry" );
    let filename = entry.file_name();
    let filename_str = filename.to_string_lossy();

    // Skip readme and non-rust files
    let is_rust_file = std::path::Path::new( &*filename_str )
      .extension()
      .is_some_and( | ext | ext.eq_ignore_ascii_case( "rs" ) );

    if filename_str == "readme.md" || !is_rust_file
    {
      continue;
    }

    // Verify snake_case naming (no camelCase, no spaces, no uppercase except .rs)
    let name_without_ext = filename_str.trim_end_matches( ".rs" );

    for c in name_without_ext.chars()
    {
      assert!
      (
        c.is_lowercase() || c.is_numeric() || c == '_',
        "Example filename '{filename_str}' should use lowercase snake_case only"
      );
    }
  }
}
