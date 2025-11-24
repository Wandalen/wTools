//! Tests for glob crate re-export.
//!
//! Verifies that the `glob` crate is properly re-exported through `fs_tools`
//! and all functionality is accessible.
//!
//! ## Design Decisions
//!
//! **Why module-only re-export (`pub use ::glob;`)?**
//! Cannot have both `fs_tools::glob` as module AND `fs_tools::glob` as function.
//! Re-exporting the module avoids name collision. Users access via `fs_tools::glob::glob()`.
//!
//! **Why `glob` feature requires `enabled`?**
//! Follows wtools pattern where utility features depend on the master switch.
//! Ensures consistent behavior and simplifies feature flag logic.
//!
//! **Why re-export vs custom wrappers?**
//! The glob crate (300M+ downloads) provides comprehensive API. Custom wrappers
//! would add maintenance burden without significant value. Re-export gives users
//! full access to all glob functionality.

use super::*;

// ══════════════════════════════════════════════════════════════════════════════
// Basic re-export accessibility tests
// ══════════════════════════════════════════════════════════════════════════════

#[ test ]
fn glob_module_accessible()
{
  // Verify glob module is accessible via fs_tools
  use the_module::glob;

  // Verify key types exist by assigning to typed binding
  let glob_fn: fn( &str ) -> Result< glob::Paths, glob::PatternError > = glob::glob;
  assert!( glob_fn( "*.nonexistent_extension_xyz" ).is_ok() );
}

#[ test ]
fn glob_types_accessible()
{
  use the_module::glob::{ Pattern, MatchOptions, GlobError, PatternError };

  // Verify types are usable
  let pattern_result: Result< Pattern, PatternError > = Pattern::new( "*.rs" );
  assert!( pattern_result.is_ok() );

  let options = MatchOptions
  {
    case_sensitive : true,
    require_literal_separator : false,
    require_literal_leading_dot : false,
  };
  assert!( options.case_sensitive );

  // GlobError is used in iteration, verify it's accessible as a type
  let _: Option< GlobError > = None;
}

// ══════════════════════════════════════════════════════════════════════════════
// Functional tests
// ══════════════════════════════════════════════════════════════════════════════

#[ test ]
fn glob_traversal_works()
{
  use the_module::glob::glob;

  // Find this crate's Cargo.toml using absolute path
  let pattern = concat!( env!( "CARGO_MANIFEST_DIR" ), "/Cargo.toml" );
  let results: Vec< _ > = glob( pattern )
    .expect( "valid pattern" )
    .filter_map( Result::ok )
    .collect();

  assert_eq!( results.len(), 1 );
  assert!( results[ 0 ].ends_with( "Cargo.toml" ) );
}

#[ test ]
fn glob_with_options_works()
{
  use the_module::glob::{ glob_with, MatchOptions };

  let options = MatchOptions
  {
    case_sensitive : true,
    require_literal_separator : false,
    require_literal_leading_dot : false,
  };

  // Test glob_with function works correctly
  let pattern = concat!( env!( "CARGO_MANIFEST_DIR" ), "/Cargo.toml" );
  let results: Vec< _ > = glob_with( pattern, options )
    .expect( "valid pattern" )
    .filter_map( Result::ok )
    .collect();

  assert_eq!( results.len(), 1, "glob_with should find Cargo.toml" );
}

#[ test ]
fn pattern_matching_works()
{
  use the_module::glob::Pattern;

  let pattern = Pattern::new( "*.rs" ).expect( "valid pattern" );

  assert!( pattern.matches( "lib.rs" ) );
  assert!( pattern.matches( "test.rs" ) );
  assert!( !pattern.matches( "Cargo.toml" ) );
  // Pattern::matches does match paths with separators (pure string matching)
  // Use MatchOptions::require_literal_separator for strict behavior
  assert!( pattern.matches( "src/lib.rs" ) );
}

#[ test ]
fn pattern_with_options_works()
{
  use the_module::glob::{ Pattern, MatchOptions };

  let pattern = Pattern::new( "*.RS" ).expect( "valid pattern" );

  let case_sensitive = MatchOptions
  {
    case_sensitive : true,
    require_literal_separator : false,
    require_literal_leading_dot : false,
  };

  let case_insensitive = MatchOptions
  {
    case_sensitive : false,
    require_literal_separator : false,
    require_literal_leading_dot : false,
  };

  // Case sensitive: *.RS should not match lib.rs
  assert!( !pattern.matches_with( "lib.rs", case_sensitive ) );

  // Case insensitive: *.RS should match lib.rs
  assert!( pattern.matches_with( "lib.rs", case_insensitive ) );
}

#[ test ]
fn pattern_escape_works()
{
  use the_module::glob::Pattern;

  // Escape special characters
  let escaped = Pattern::escape( "test[1].rs" );
  let pattern = Pattern::new( &escaped ).expect( "valid pattern" );

  // Should match literal brackets
  assert!( pattern.matches( "test[1].rs" ) );
  // Should not match as character class
  assert!( !pattern.matches( "test1.rs" ) );
}

#[ test ]
fn recursive_glob_pattern_works()
{
  use the_module::glob::glob;

  // Find all .rs files in src directory
  let pattern = concat!( env!( "CARGO_MANIFEST_DIR" ), "/src/**/*.rs" );
  let results: Vec< _ > = glob( pattern )
    .expect( "valid pattern" )
    .filter_map( Result::ok )
    .collect();

  // Should find at least lib.rs and fs.rs
  assert!( results.len() >= 2, "Should find at least 2 .rs files in src/" );

  // All results should be .rs files
  for path in &results
  {
    assert!(
      path.extension().is_some_and( | ext | ext == "rs" ),
      "All matched files should have .rs extension: {path:?}"
    );
  }
}

#[ test ]
fn dependency_namespace_accessible()
{
  // Verify glob is accessible via dependency namespace
  use the_module::dependency::glob;

  let glob_fn: fn( &str ) -> Result< glob::Paths, glob::PatternError > = glob::glob;
  assert!( glob_fn( "*.nonexistent_extension_xyz" ).is_ok() );
}
