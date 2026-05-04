//! Test suite for example file quality and documentation standards.
//!
//! This test file validates that example files meet quality standards including:
//! - Proper documentation (no placeholder markers)
//! - Correct code formatting
//! - Adherence to code style guidelines
//!
//! ## Bug Reproduction Tests
//!
//! ### Bug: Missing Example Documentation (issue-wtools-001)
//!
//! **Root Cause**: Example file contained placeholder documentation marker `//! qqq: write proper description`
//! instead of actual documentation. This occurred because the example was created from a template
//! and the placeholder was never replaced with meaningful content.
//!
//! **Why Not Caught**: No automated check existed to prevent placeholder markers in example files.
//! Code review process didn't specifically check for "qqq:" markers in documentation.
//!
//! **Fix Applied**: Replaced placeholder with proper documentation explaining what the example
//! demonstrates and how the `implements!` macro works. Added descriptive comment block.
//!
//! **Prevention**: This test now checks for placeholder markers. Any future placeholder markers
//! will cause test failure, forcing developers to replace them with real documentation.
//!
//! **Pitfall**: Always check for and remove placeholder markers (`qqq:`, `xxx:`, `TODO:`)
//! before committing examples. Placeholder markers are acceptable in internal code but NOT
//! in examples which serve as user-facing documentation.
//!
//! ### Bug: Inconsistent Spacing in Code (issue-wtools-002)
//!
//! **Root Cause**: Code contained `Box ::new` with extra space before `::new`, violating
//! consistent spacing rules. This was likely a typo or inconsistent manual formatting.
//!
//! **Why Not Caught**: No automated formatter enforced (cargo fmt is forbidden, custom codestyle
//! used). Manual review didn't catch this minor spacing issue.
//!
//! **Fix Applied**: Removed extra space, changed `Box ::new` to `Box::new` for consistency
//! with standard Rust formatting and codestyle rules.
//!
//! **Prevention**: This test checks for common spacing violations. Manual review should focus
//! on spacing consistency around `::` operator.
//!
//! **Pitfall**: Spacing around `::` must be consistent: no space before `::` in path expressions.
//! Pattern `Type ::method` is incorrect, should be `Type::method`. Be careful when manually
//! formatting code without automated tooling.

use std::fs;
use std::path::PathBuf;

/// Get the examples directory path
fn examples_dir() -> PathBuf
{
  let manifest_dir = env!( "CARGO_MANIFEST_DIR" );
  PathBuf::from( manifest_dir ).join( "examples" )
}

/// Test: Verify example documentation contains no placeholder markers
///
/// This test reproduces bug issue-wtools-001 where example file contained
/// "qqq: write proper description" instead of actual documentation.
// test_kind: bug_reproducer(issue-wtools-001)
#[ test ]
fn example_documentation_no_placeholders()
{
  let example_path = examples_dir().join( "wtools_trivial.rs" );
  let content = fs::read_to_string( &example_path )
    .expect( "Failed to read example file" );

  // Check for common placeholder markers
  assert!(
    !content.contains( "qqq:" ),
    "Example contains 'qqq:' placeholder marker at {}: {}",
    example_path.display(),
    content.lines().find( |line| line.contains( "qqq:" ) ).unwrap_or( "" )
  );

  assert!(
    !content.contains( "xxx:" ),
    "Example contains 'xxx:' placeholder marker"
  );

  assert!(
    !content.contains( "TODO: write" ) && !content.contains( "TODO:write" ),
    "Example contains 'TODO: write' placeholder"
  );

  // Ensure first line is actual documentation, not placeholder
  let first_line = content.lines().next().expect( "File is empty" );
  assert!(
    first_line.starts_with( "//!" ),
    "Example should start with documentation comment"
  );
  assert!(
    first_line.len() > 10,
    "Documentation should be meaningful, not just '//!'"
  );
}

/// Test: Verify example code has consistent spacing around :: operator
///
/// This test reproduces bug issue-wtools-002 where code had inconsistent
/// spacing like `Box ::new` instead of `Box::new`.
// test_kind: bug_reproducer(issue-wtools-002)
#[ test ]
fn example_consistent_spacing_around_scope_operator()
{
  let example_path = examples_dir().join( "wtools_trivial.rs" );
  let content = fs::read_to_string( &example_path )
    .expect( "Failed to read example file" );

  // Check for space before :: in code (not in comments)
  for ( line_num, line ) in content.lines().enumerate()
  {
    // Skip comment lines
    if line.trim().starts_with( "//" ) || line.trim().starts_with( "//!" )
    {
      continue;
    }

    // Check for pattern "Word ::something" (space before ::)
    // This is incorrect spacing
    let problematic_patterns = [
      " ::",  // General case: space before ::
    ];

    for pattern in &problematic_patterns
    {
      if line.contains( pattern )
      {
        // Additional check: ensure it's not in a comment or string literal
        if let Some( pos ) = line.find( pattern )
        {
          let before_pattern = &line[ ..pos ];

          // Skip if this is inside a string literal (simple heuristic)
          let quote_count = before_pattern.matches( '"' ).count();
          if quote_count % 2 == 1
          {
            continue; // Inside string literal
          }

          // Skip if this is in a comment
          if before_pattern.contains( "//" )
          {
            continue;
          }

          panic!(
            "Inconsistent spacing at line {}: found '{}' - should not have space before ::\nLine: {}",
            line_num + 1,
            pattern,
            line
          );
        }
      }
    }
  }
}

/// Test: Verify example file exists and is accessible
#[ test ]
fn example_file_exists()
{
  let example_path = examples_dir().join( "wtools_trivial.rs" );
  assert!(
    example_path.exists(),
    "Example file does not exist at {}",
    example_path.display()
  );
}

/// Test: Verify example uses correct import pattern
#[ test ]
fn example_uses_conditional_imports()
{
  let example_path = examples_dir().join( "wtools_trivial.rs" );
  let content = fs::read_to_string( &example_path )
    .expect( "Failed to read example file" );

  // Verify example has proper feature-gated imports
  assert!(
    content.contains( "#[ cfg( any( feature = \"typing_implements\", feature = \"typing\") ) ]" )
      || content.contains( "#[cfg(any(feature = \"typing_implements\", feature = \"typing\"))]" ),
    "Example should use conditional compilation for typing features"
  );

  // Verify it imports from wtools
  assert!(
    content.contains( "use wtools::" ) || content.contains( "use wtools ::" ),
    "Example should import from wtools crate"
  );
}

