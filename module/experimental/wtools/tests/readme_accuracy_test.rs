//! Test suite for readme.md accuracy and correctness.
//!
//! This test file validates that readme.md contains accurate information and working examples.
//!
//! ## Bug Reproduction Tests
//!
//! ### Bug: Incorrect Example Path in Readme (issue-wtools-003)
//!
//! **Root Cause**: Readme.md contained instructions to `cd examples/wtools_trivial` which is
//! incorrect because `wtools_trivial` is a file (`wtools_trivial.rs`), not a directory. This
//! occurred because the documentation was written assuming a directory-based example structure
//! when the actual example is a single file.
//!
//! **Why Not Caught**: No automated validation existed to check that readme paths and commands
//! are accurate. Manual testing of readme instructions was not part of the review process.
//!
//! **Fix Applied**: Replaced incorrect path-based command with correct cargo command:
//! Changed from `cd examples/wtools_trivial; cargo run` to `cargo run --example wtools_trivial`.
//! This matches Rust's standard example execution pattern.
//!
//! **Prevention**: This test validates that readme doesn't contain incorrect path references
//! to example files. Future changes to readme will be caught by this test.
//!
//! **Pitfall**: Always verify that example execution instructions in readme match actual project
//! structure. Rust examples in `examples/` directory are run with `cargo run --example <name>`,
//! not by cd-ing into a directory. Don't assume directory structure without verification.

use std::fs;
use std::path::PathBuf;

/// Get the readme.md file path
fn readme_path() -> PathBuf
{
  let manifest_dir = env!( "CARGO_MANIFEST_DIR" );
  PathBuf::from( manifest_dir ).join( "readme.md" )
}

/// Get the examples directory path
fn examples_dir() -> PathBuf
{
  let manifest_dir = env!( "CARGO_MANIFEST_DIR" );
  PathBuf::from( manifest_dir ).join( "examples" )
}

/// Test: Verify readme does not contain incorrect cd commands for examples
///
/// This test reproduces bug issue-wtools-003 where readme instructed users
/// to `cd examples/wtools_trivial` when `wtools_trivial` is a file, not a directory.
// test_kind: bug_reproducer(issue-wtools-003)
#[ test ]
fn readme_no_incorrect_example_paths()
{
  let readme = readme_path();
  let content = fs::read_to_string( &readme )
    .expect( "Failed to read readme.md" );

  // Check that readme doesn't tell users to cd into example files
  // Pattern to avoid: "cd examples/wtools_trivial" when wtools_trivial.rs is a file
  let examples = examples_dir();

  if examples.exists()
  {
    // Get all example files
    for entry in fs::read_dir( &examples ).expect( "Failed to read examples dir" )
    {
      let entry = entry.expect( "Failed to read entry" );
      let path = entry.path();

      if path.is_file() && path.extension().and_then( |s| s.to_str() ) == Some( "rs" )
      {
        if let Some( stem ) = path.file_stem().and_then( |s| s.to_str() )
        {
          // Check if readme contains incorrect "cd examples/{stem}" pattern
          let incorrect_pattern = format!( "cd examples/{stem}" );

          assert!(
            !content.contains( &incorrect_pattern ),
            "Readme contains incorrect path '{incorrect_pattern}' - '{stem}' is a file, not a directory. \
             Use 'cargo run --example {stem}' instead."
          );
        }
      }
    }
  }
}

/// Test: Verify readme uses correct cargo example execution pattern
// test_kind: bug_reproducer(issue-wtools-003)
#[ test ]
fn readme_uses_correct_example_execution()
{
  let readme = readme_path();
  let content = fs::read_to_string( &readme )
    .expect( "Failed to read readme.md" );

  // If readme mentions examples, verify it uses correct cargo syntax
  if content.contains( "wtools_trivial" )
  {
    // Should use correct cargo command for examples
    let correct_patterns = [
      "cargo run --example wtools_trivial",
      "`cargo run --example wtools_trivial`",
    ];

    let has_correct_pattern = correct_patterns.iter()
      .any( |pattern| content.contains( pattern ) );

    assert!(
      has_correct_pattern,
      "Readme should use 'cargo run --example wtools_trivial' to run examples"
    );
  }
}

/// Test: Verify readme file exists
#[ test ]
fn readme_exists()
{
  let readme = readme_path();
  assert!(
    readme.exists(),
    "readme.md does not exist at {}",
    readme.display()
  );
}

/// Test: Verify readme has content
#[ test ]
fn readme_has_content()
{
  let readme = readme_path();
  let content = fs::read_to_string( &readme )
    .expect( "Failed to read readme.md" );

  assert!(
    content.len() > 100,
    "Readme appears to be empty or too short"
  );

  assert!(
    content.contains( "wtools" ),
    "Readme should mention the crate name 'wtools'"
  );
}

/// Test: Verify readme mentions repository correctly
#[ test ]
fn readme_has_repository_info()
{
  let readme = readme_path();
  let content = fs::read_to_string( &readme )
    .expect( "Failed to read readme.md" );

  // Should mention GitHub repository
  assert!(
    content.contains( "github.com" ) || content.contains( "GitHub" ),
    "Readme should mention GitHub repository"
  );
}
