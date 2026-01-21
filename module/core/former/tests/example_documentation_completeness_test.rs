//! Tests that all examples are properly documented in `examples/readme.md`.
//!
//! ## Root Cause (issue-manual-test-002)
//!
//! Seven example files were missing from `examples/readme.md` documentation:
//! - `basic_test.rs`
//! - `former_trivial_expanded.rs` (previously `former_trivial_expaned.rs`)
//! - `lifetime_test.rs`, `lifetime_test2.rs`, `minimal_lifetime_test.rs`, `debug_lifetime.rs`
//! - `former_with_serde.rs`
//!
//! This occurred because examples were added over time without updating the readme index.
//! There was no automated verification that all examples were documented, leading to
//! incomplete user-facing documentation.
//!
//! ## Why Not Caught
//!
//! No existing tests verified readme.md completeness. The examples compiled and ran
//! correctly, but users browsing the readme would not discover these examples. Manual
//! review processes did not enforce documentation updates when adding new examples.
//!
//! ## Fix Applied
//!
//! 1. Updated `examples/readme.md` to include all 7 missing examples
//! 2. Organized examples into logical groups (Basic Usage, Lifetimes, Integration, Debugging)
//! 3. Added descriptive text for each example explaining its purpose
//! 4. Created this test to prevent future documentation gaps
//!
//! ## Prevention
//!
//! 1. Run this test in CI to enforce documentation of all examples
//! 2. Add checklist item to PR template: "Updated `examples/readme.md` if adding new examples"
//! 3. Use automated tools to generate example indices from file listings
//! 4. Review `examples/` directory periodically for undocumented files
//!
//! ## Pitfall
//!
//! Documentation drift occurs when code and docs are updated separately. Always update
//! user-facing documentation (like example indices) atomically with code changes. Consider
//! using doc-generation tools to eliminate manual documentation maintenance where possible.

#[ test ]
fn test_all_examples_documented_in_readme()
{
  // This test verifies every example file is documented in examples/readme.md

  let examples_dir = std::path::Path::new( env!( "CARGO_MANIFEST_DIR" ) )
    .join( "examples" );

  let readme_path = examples_dir.join( "readme.md" );

  // Read all .rs files in examples/
  let entries = std::fs::read_dir( &examples_dir )
    .expect( "Failed to read examples directory" );

  let mut example_files = Vec::new();

  for entry in entries
  {
    let entry = entry.expect( "Failed to read directory entry" );
    let filename = entry.file_name();
    let filename_str = filename.to_string_lossy().to_string();

    // Only check .rs files (skip readme.md and other non-example files)
    if std::path::Path::new( &filename_str )
      .extension()
      .is_some_and( | ext | ext.eq_ignore_ascii_case( "rs" ) )
    {
      example_files.push( filename_str );
    }
  }

  // Read readme.md content
  let readme_content = std::fs::read_to_string( &readme_path )
    .expect( "Failed to read examples/readme.md" );

  // Check each example file is referenced in readme
  let mut undocumented = Vec::new();

  for example_file in &example_files
  {
    // Look for markdown link to the file: [example_name.rs](./example_name.rs)
    // or just the filename in the content
    if !readme_content.contains( example_file )
    {
      undocumented.push( example_file.clone() );
    }
  }

  assert!
  (
    undocumented.is_empty(),
    "Found {} example(s) not documented in examples/readme.md:\n{}\n\n\
    Please add these examples to the readme with appropriate descriptions.",
    undocumented.len(),
    undocumented.join( "\n" )
  );
}

#[ test ]
fn test_readme_links_point_to_existing_files()
{
  // This test verifies all example links in readme.md point to actual files

  let examples_dir = std::path::Path::new( env!( "CARGO_MANIFEST_DIR" ) )
    .join( "examples" );

  let readme_path = examples_dir.join( "readme.md" );
  let readme_content = std::fs::read_to_string( &readme_path )
    .expect( "Failed to read examples/readme.md" );

  // Extract all markdown links matching pattern [text](./filename.rs)
  // Simple string parsing approach without regex dependency
  let mut broken_links = Vec::new();

  for line in readme_content.lines()
  {
    // Look for markdown link pattern: ](./something.rs)
    if let Some( start_idx ) = line.find( "](./" )
    {
      let after_link = &line[ start_idx + 4.. ]; // Skip "](. /"
      if let Some( end_idx ) = after_link.find( ')' )
      {
        let filename = &after_link[ ..end_idx ];
        if std::path::Path::new( filename )
          .extension()
          .is_some_and( | ext | ext.eq_ignore_ascii_case( "rs" ) )
        {
          let file_path = examples_dir.join( filename );
          if !file_path.exists()
          {
            broken_links.push( filename.to_string() );
          }
        }
      }
    }
  }

  assert!
  (
    broken_links.is_empty(),
    "Found {} broken link(s) in examples/readme.md:\n{}\n\n\
    These files are referenced but don't exist.",
    broken_links.len(),
    broken_links.join( "\n" )
  );
}
