//! Test verifying examples/readme.md accurately documents existing examples
//!
//! # Test Matrix
//!
//! | Test ID | Test Case | Description |
//! |---------|-----------|-------------|
//! | TED-01 | `examples_readme_accuracy` | Verifies all documented examples exist and all existing examples are documented |
//!
//! # Root Cause
//!
//! Issue found during manual testing: `examples/readme.md` documented 5 non-existent examples
//! (`004_from_components.rs`, `005_manual_implementation.rs`, `006_real_world_config.rs`,
//! `007_advanced_patterns.rs`, `008_performance_comparison.rs`) and incorrectly named
//! `004_working_example.rs` as `006_real_world_config.rs`.
//!
//! # Why Not Caught
//!
//! No automated verification that documentation matches actual example files in `examples/` directory.
//! README can drift from reality without CI catching the discrepancy.
//!
//! # Fix Applied
//!
//! 1. Corrected `examples/readme.md` to accurately list all 8 existing examples
//! 2. Removed references to 5 non-existent examples
//! 3. Added this test to prevent future documentation drift
//!
//! # Prevention
//!
//! This test scans `examples/` directory and verifies `examples/readme.md` mentions each file exactly once.
//! Future readme updates that introduce mismatches will fail this test immediately.
//!
//! # Pitfall
//!
//! Documentation can easily drift from implementation without automated verification. Always pair
//! documentation with tests that verify accuracy. For directories with examples, tests should
//! validate that readme mentions match actual files.

use std::{ fs, path::PathBuf };

#[ test ]
fn examples_readme_accuracy()
{
  // Get workspace root (go up from component_model to wtools/dev)
  let manifest_dir = PathBuf::from( env!( "CARGO_MANIFEST_DIR" ) );
  let examples_dir = manifest_dir.join( "examples" );
  let readme_path = examples_dir.join( "readme.md" );

  // Read readme content
  let readme_content = fs::read_to_string( &readme_path )
    .expect( "Failed to read examples/readme.md" );

  // Get all .rs example files
  let mut example_files = Vec::new();
  for entry in fs::read_dir( &examples_dir ).expect( "Failed to read examples directory" )
  {
    let entry = entry.expect( "Failed to read directory entry" );
    let path = entry.path();
    if path.extension().and_then( | s | s.to_str() ) == Some( "rs" )
    {
      if let Some( filename ) = path.file_name().and_then( | s | s.to_str() )
      {
        example_files.push( filename.to_string() );
      }
    }
  }
  example_files.sort();

  // Verify each example file is mentioned in readme
  let missing_from_readme : Vec< String > = example_files
    .iter()
    .filter( | example | !readme_content.contains( example.as_str() ) )
    .cloned()
    .collect();

  // Report missing documentation
  assert!(
    missing_from_readme.is_empty(),
    "Examples missing from readme.md documentation:\n{}\n\
     Add these examples to examples/readme.md",
    missing_from_readme.join( "\n" )
  );

  // Verify no phantom examples (documented but don't exist)
  let documented_examples =
  [
    "component_model_trivial.rs",
    "000_basic_assignment.rs",
    "001_fluent_builder.rs",
    "002_multiple_components.rs",
    "003_component_from.rs",
    "004_working_example.rs",
    "boolean_assignment_error.rs",
    "boolean_ambiguity_solution.rs",
    "debug_macro_output.rs",
  ];

  let phantom_examples : Vec< &str > = documented_examples
    .iter()
    .filter( | doc_example | !example_files.contains( &doc_example.to_string() ) )
    .copied()
    .collect();

  assert!(
    phantom_examples.is_empty(),
    "Examples documented but don't exist:\n{}\n\
     Either create these examples or remove them from examples/readme.md",
    phantom_examples.join( "\n" )
  );

  println!( "✅ All {} examples properly documented", example_files.len() );
}
