//! Tests for path traversal utilities.

#![ cfg( all( feature = "enabled", not( feature = "no_std" ) ) ) ]
//!
//! # Purpose
//!
//! Validates fs_tools::path module functions for upward directory traversal
//! and hierarchical file collection. These utilities are critical for finding
//! project roots, configuration files, and workspace boundaries.
//!
//! # Conditional Compilation
//!
//! This test file requires `#![ cfg( all( feature = "enabled", not( feature = "no_std" ) ) ) ]`
//! because the path module functions are only exported when both conditions are met:
//! - `feature = "enabled"` - Activates path utilities
//! - `not( feature = "no_std" )` - Requires std library for filesystem operations
//!
//! **Why this matters:** Running tests with `--all-features` activates BOTH `enabled`
//! and `no_std` features simultaneously. Since path functions use `std::fs` and `std::io`,
//! they cannot exist in no_std mode. The conditional compilation on this test file
//! ensures tests are skipped (not failed) when `no_std` is active.
//!
//! **Root Cause:** Feature flags are not mutually exclusive in Cargo. `--all-features`
//! activates all features listed in Cargo.toml regardless of logical conflicts.
//!
//! **Fix Applied:** Added conditional compilation attribute matching the path module's
//! export conditions. Without this, tests would fail to compile with unresolved imports
//! when `--all-features` is used.
//!
//! # Test Organization
//!
//! Tests are organized into four categories:
//!
//! 1. **traverse_upward tests** (5+ tests)
//!    - Generic upward traversal with custom predicates
//!    - Tests depth limits, target finding, and edge cases
//!
//! 2. **collect_files_in_ancestors tests** (8+ tests)
//!    - Hierarchical file collection from root to target
//!    - Tests ordering, deduplication, symlink handling, depth limits
//!
//! 3. **Helper function tests** (12+ tests)
//!    - file_upward_find, dir_upward_find, matching_upward_find
//!    - Tests convenience wrappers built on traverse_upward
//!
//! 4. **Edge case tests** (8+ tests)
//!    - Unicode filenames, deep hierarchies, symlink loops
//!    - Broken symlinks, empty filenames, special characters
//!
//! # Critical Behaviors Validated
//!
//! - Symlink deduplication prevents duplicate file collection
//! - Circular symlinks handled gracefully (OS ELOOP protection)
//! - Root-to-target ordering preserved in hierarchical collection
//! - Max depth limits prevent infinite loops
//! - Empty/nonexistent paths handled with appropriate errors

use fs_tools::path::
{
  traverse_upward,
  collect_files_in_ancestors,
  file_upward_find,
  dir_upward_find,
  matching_upward_find,
};
use std::path::PathBuf;
use std::fs;

// qqq: for Dmytro: add test setup helper macro or function

// =============================================================================
// traverse_upward tests (5+ tests)
// =============================================================================

#[ test ]
fn traverse_upward_finds_target_at_root()
{
  let temp = tempfile::tempdir().unwrap();
  let root = temp.path();

  // Create target file at root
  fs::write( root.join( "target.txt" ), "root" ).unwrap();

  let result = traverse_upward(
    root,
    | dir |
    {
      let candidate = dir.join( "target.txt" );
      if candidate.exists()
      {
        Some( candidate )
      }
      else
      {
        None
      }
    },
    10
  );

  assert!( result.is_some() );
  assert_eq!( result.unwrap(), root.join( "target.txt" ) );
}

#[ test ]
fn traverse_upward_finds_target_at_intermediate_level()
{
  let temp = tempfile::tempdir().unwrap();
  let root = temp.path();

  // Create hierarchy: root/level1/level2/level3
  fs::create_dir_all( root.join( "level1/level2/level3" ) ).unwrap();

  // Place target at level1
  fs::write( root.join( "level1/target.txt" ), "level1" ).unwrap();

  // Search from level3
  let start = root.join( "level1/level2/level3" );
  let result = traverse_upward(
    &start,
    | dir |
    {
      let candidate = dir.join( "target.txt" );
      if candidate.exists()
      {
        Some( candidate )
      }
      else
      {
        None
      }
    },
    10
  );

  assert!( result.is_some() );
  assert_eq!( result.unwrap(), root.join( "level1/target.txt" ) );
}

#[ test ]
fn traverse_upward_target_not_found_returns_none()
{
  let temp = tempfile::tempdir().unwrap();
  let root = temp.path();

  // Create hierarchy but no target file
  fs::create_dir_all( root.join( "level1/level2" ) ).unwrap();

  let start = root.join( "level1/level2" );
  let result = traverse_upward(
    &start,
    | dir |
    {
      let candidate = dir.join( "nonexistent.txt" );
      if candidate.exists()
      {
        Some( candidate )
      }
      else
      {
        None
      }
    },
    10
  );

  assert!( result.is_none() );
}

#[ test ]
fn traverse_upward_respects_max_depth()
{
  let temp = tempfile::tempdir().unwrap();
  let root = temp.path();

  // Create hierarchy: root/l1/l2/l3/l4/l5
  fs::create_dir_all( root.join( "l1/l2/l3/l4/l5" ) ).unwrap();

  // Place target at root (6 levels up from l5)
  fs::write( root.join( "target.txt" ), "root" ).unwrap();

  // Search from l5 with max_depth=3 (won't reach root)
  let start = root.join( "l1/l2/l3/l4/l5" );
  let result = traverse_upward(
    &start,
    | dir |
    {
      let candidate = dir.join( "target.txt" );
      if candidate.exists()
      {
        Some( candidate )
      }
      else
      {
        None
      }
    },
    3
  );

  assert!( result.is_none() );

  // Now search with max_depth=10 (will reach root)
  let result = traverse_upward(
    &start,
    | dir |
    {
      let candidate = dir.join( "target.txt" );
      if candidate.exists()
      {
        Some( candidate )
      }
      else
      {
        None
      }
    },
    10
  );

  assert!( result.is_some() );
}

#[ test ]
fn traverse_upward_with_complex_return_type()
{
  let temp = tempfile::tempdir().unwrap();
  let root = temp.path();

  // Create hierarchy
  fs::create_dir_all( root.join( "project/src" ) ).unwrap();
  fs::write( root.join( "project/Cargo.toml" ), "[package]\nname = \"test\"" ).unwrap();

  let start = root.join( "project/src" );
  let result: Option< ( PathBuf, String ) > = traverse_upward(
    &start,
    | dir |
    {
      let cargo = dir.join( "Cargo.toml" );
      if cargo.exists()
      {
        let content = fs::read_to_string( &cargo ).ok()?;
        Some( ( cargo, content ) )
      }
      else
      {
        None
      }
    },
    10
  );

  assert!( result.is_some() );
  let ( path, content ) = result.unwrap();
  assert_eq!( path, root.join( "project/Cargo.toml" ) );
  assert!( content.contains( "name = \"test\"" ) );
}

// =============================================================================
// collect_files_in_ancestors tests (8+ tests)
// =============================================================================

#[ test ]
fn collect_files_basic_hierarchy()
{
  let temp = tempfile::tempdir().unwrap();
  let root = temp.path();

  // Create hierarchy: root/l1/l2
  fs::create_dir_all( root.join( "l1/l2" ) ).unwrap();

  // Place config files at each level
  fs::write( root.join( "config.toml" ), "root" ).unwrap();
  fs::write( root.join( "l1/config.toml" ), "l1" ).unwrap();
  fs::write( root.join( "l1/l2/config.toml" ), "l2" ).unwrap();

  let target = root.join( "l1/l2" );
  let results = collect_files_in_ancestors(
    target.as_path(),
    | path | path.file_name().and_then( | n | n.to_str() ) == Some( "config.toml" ),
    None,
    false
  ).unwrap();

  assert_eq!( results.len(), 3 );
  // Results should be in root->target order
  assert!( results[ 0 ].ends_with( "config.toml" ) );
  assert!( results[ 0 ].parent().unwrap().ends_with( root.file_name().unwrap() ) );
}

#[ test ]
fn collect_files_empty_hierarchy()
{
  let temp = tempfile::tempdir().unwrap();
  let root = temp.path();

  // Create hierarchy with no matching files
  fs::create_dir_all( root.join( "l1/l2" ) ).unwrap();
  fs::write( root.join( "other.txt" ), "data" ).unwrap();

  let target = root.join( "l1/l2" );
  let results = collect_files_in_ancestors(
    target.as_path(),
    | path | path.file_name().and_then( | n | n.to_str() ) == Some( "config.toml" ),
    None,
    false
  ).unwrap();

  assert_eq!( results.len(), 0 );
}

#[ test ]
fn collect_files_with_pattern_matching()
{
  let temp = tempfile::tempdir().unwrap();
  let root = temp.path();

  // Create hierarchy
  fs::create_dir_all( root.join( "project/src" ) ).unwrap();

  // Mix of files
  fs::write( root.join( "readme.md" ), "root readme" ).unwrap();
  fs::write( root.join( "Cargo.toml" ), "cargo" ).unwrap();
  fs::write( root.join( "project/readme.md" ), "project readme" ).unwrap();
  fs::write( root.join( "project/license" ), "mit" ).unwrap();

  let target = root.join( "project/src" );
  let results = collect_files_in_ancestors(
    target.as_path(),
    | path | path.extension().and_then( | e | e.to_str() ) == Some( "md" ),
    Some( 3 ),  // Limit depth to avoid system directories
    false
  ).unwrap();

  assert_eq!( results.len(), 2 );
  assert!( results.iter().all( | p | p.extension().and_then( | e | e.to_str() ) == Some( "md" ) ) );
}

#[ test ]
fn collect_files_respects_max_depth()
{
  let temp = tempfile::tempdir().unwrap();
  let root = temp.path();

  // Deep hierarchy
  fs::create_dir_all( root.join( "l1/l2/l3/l4/l5" ) ).unwrap();

  // Config at each level
  fs::write( root.join( "config" ), "0" ).unwrap();
  fs::write( root.join( "l1/config" ), "1" ).unwrap();
  fs::write( root.join( "l1/l2/config" ), "2" ).unwrap();
  fs::write( root.join( "l1/l2/l3/config" ), "3" ).unwrap();
  fs::write( root.join( "l1/l2/l3/l4/config" ), "4" ).unwrap();

  let target = root.join( "l1/l2/l3/l4/l5" );

  // Collect with depth limit of 3
  let results = collect_files_in_ancestors(
    target.as_path(),
    | path | path.file_name().and_then( | n | n.to_str() ) == Some( "config" ),
    Some( 3 ),
    false
  ).unwrap();

  // Should only get l4/l5 (current), l3, l2 (3 levels)
  assert!( results.len() <= 3 );
}

#[ test ]
fn collect_files_deduplicates_symlinks()
{
  let temp = tempfile::tempdir().unwrap();
  let root = temp.path();

  // Create structure
  fs::create_dir_all( root.join( "dir" ) ).unwrap();
  fs::write( root.join( "dir/config.toml" ), "data" ).unwrap();

  // Create symlink to same file (Unix only)
  #[ cfg( unix ) ]
  {
    use std::os::unix::fs::symlink;
    symlink( "config.toml", root.join( "dir/config_link.toml" ) ).unwrap();

    let results = collect_files_in_ancestors(
      root.join( "dir" ).as_path(),
      | path |
      {
        path.file_name()
          .and_then( | n | n.to_str() )
          .is_some_and( | n | n.contains( "config" ) )
      },
      Some( 2 ),  // Limit depth to avoid system directories
      true  // deduplicate
    ).unwrap();

    // Should deduplicate: only one entry even though two filenames match
    assert_eq!( results.len(), 1 );
  }
}

#[ test ]
fn collect_files_without_deduplication()
{
  let temp = tempfile::tempdir().unwrap();
  let root = temp.path();

  fs::create_dir_all( root.join( "dir" ) ).unwrap();
  fs::write( root.join( "dir/config.toml" ), "data" ).unwrap();

  #[ cfg( unix ) ]
  {
    use std::os::unix::fs::symlink;
    symlink( "config.toml", root.join( "dir/config_link.toml" ) ).unwrap();

    let results = collect_files_in_ancestors(
      root.join( "dir" ).as_path(),
      | path |
      {
        path.file_name()
          .and_then( | n | n.to_str() )
          .is_some_and( | n | n.contains( "config" ) )
      },
      Some( 2 ),  // Limit depth to avoid system directories
      false  // no deduplicate
    ).unwrap();

    // Without deduplication, should get both entries
    assert_eq!( results.len(), 2 );
  }
}

#[ test ]
fn collect_files_handles_broken_symlinks()
{
  let temp = tempfile::tempdir().unwrap();
  let root = temp.path();

  fs::create_dir_all( root.join( "dir" ) ).unwrap();
  fs::write( root.join( "dir/real.toml" ), "data" ).unwrap();

  #[ cfg( unix ) ]
  {
    use std::os::unix::fs::symlink;
    // Create broken symlink
    symlink( "nonexistent.toml", root.join( "dir/broken.toml" ) ).unwrap();

    let results = collect_files_in_ancestors(
      root.join( "dir" ).as_path(),
      | path |
      {
        path.extension()
          .is_some_and( | ext | ext.eq_ignore_ascii_case( "toml" ) )
      },
      None,
      true
    );

    // Should succeed despite broken symlink
    assert!( results.is_ok() );
    let files = results.unwrap();
    assert_eq!( files.len(), 1 );
    assert!( files[ 0 ].ends_with( "real.toml" ) );
  }
}

#[ test ]
fn collect_files_ordering_root_to_target()
{
  let temp = tempfile::tempdir().unwrap();
  let root = temp.path();

  // Create 4-level hierarchy
  fs::create_dir_all( root.join( "a/b/c" ) ).unwrap();

  // Create files with level indicators
  fs::write( root.join( "level0.txt" ), "0" ).unwrap();
  fs::write( root.join( "a/level1.txt" ), "1" ).unwrap();
  fs::write( root.join( "a/b/level2.txt" ), "2" ).unwrap();
  fs::write( root.join( "a/b/c/level3.txt" ), "3" ).unwrap();

  let results = collect_files_in_ancestors(
    root.join( "a/b/c" ).as_path(),
    | path | path.extension().and_then( | e | e.to_str() ) == Some( "txt" ),
    Some( 4 ),  // Limit depth to avoid system directories
    false
  ).unwrap();

  assert_eq!( results.len(), 4 );

  // Verify ordering: root first, target last
  assert!( results[ 0 ].ends_with( "level0.txt" ) );
  assert!( results[ 1 ].ends_with( "level1.txt" ) );
  assert!( results[ 2 ].ends_with( "level2.txt" ) );
  assert!( results[ 3 ].ends_with( "level3.txt" ) );
}

// =============================================================================
// Helper functions tests (12+ tests)
// =============================================================================

#[ test ]
fn file_upward_find_finds_file_at_root()
{
  let temp = tempfile::tempdir().unwrap();
  let root = temp.path();

  fs::write( root.join( "Cargo.toml" ), "test" ).unwrap();

  let result = file_upward_find( root, "Cargo.toml", 10 );

  assert!( result.is_some() );
  assert_eq!( result.unwrap(), root.join( "Cargo.toml" ) );
}

#[ test ]
fn file_upward_find_finds_file_at_intermediate_level()
{
  let temp = tempfile::tempdir().unwrap();
  let root = temp.path();

  fs::create_dir_all( root.join( "project/src/module" ) ).unwrap();
  fs::write( root.join( "project/Cargo.toml" ), "test" ).unwrap();

  let start = root.join( "project/src/module" );
  let result = file_upward_find( &start, "Cargo.toml", 10 );

  assert!( result.is_some() );
  assert_eq!( result.unwrap(), root.join( "project/Cargo.toml" ) );
}

#[ test ]
fn file_upward_find_file_not_found_returns_none()
{
  let temp = tempfile::tempdir().unwrap();
  let root = temp.path();

  fs::create_dir_all( root.join( "project" ) ).unwrap();

  let result = file_upward_find( root.join( "project" ).as_path(), "nonexistent.txt", 10 );

  assert!( result.is_none() );
}

#[ test ]
fn file_upward_find_ignores_directory_with_same_name()
{
  let temp = tempfile::tempdir().unwrap();
  let root = temp.path();

  // Create directory named "config" instead of file
  fs::create_dir_all( root.join( "config/subdir" ) ).unwrap();

  let result = file_upward_find( root.join( "config/subdir" ).as_path(), "config", 10 );

  // Should not find directory, only files
  assert!( result.is_none() );
}

#[ test ]
fn dir_upward_find_finds_dir_at_root()
{
  let temp = tempfile::tempdir().unwrap();
  let root = temp.path();

  fs::create_dir( root.join( ".git" ) ).unwrap();

  let result = dir_upward_find( root, ".git", 10 );

  assert!( result.is_some() );
  assert_eq!( result.unwrap(), root.join( ".git" ) );
}

#[ test ]
fn dir_upward_find_finds_dir_at_intermediate_level()
{
  let temp = tempfile::tempdir().unwrap();
  let root = temp.path();

  fs::create_dir_all( root.join( "project/.git" ) ).unwrap();
  fs::create_dir_all( root.join( "project/src/module" ) ).unwrap();

  let start = root.join( "project/src/module" );
  let result = dir_upward_find( &start, ".git", 10 );

  assert!( result.is_some() );
  assert_eq!( result.unwrap(), root.join( "project/.git" ) );
}

#[ test ]
fn dir_upward_find_dir_not_found_returns_none()
{
  let temp = tempfile::tempdir().unwrap();
  let root = temp.path();

  fs::create_dir_all( root.join( "project" ) ).unwrap();

  let result = dir_upward_find( root.join( "project" ).as_path(), ".git", 10 );

  assert!( result.is_none() );
}

#[ test ]
fn dir_upward_find_ignores_file_with_same_name()
{
  let temp = tempfile::tempdir().unwrap();
  let root = temp.path();

  // Create file named ".git" instead of directory
  fs::create_dir_all( root.join( "project" ) ).unwrap();
  fs::write( root.join( ".git" ), "not a dir" ).unwrap();

  let result = dir_upward_find( root.join( "project" ).as_path(), ".git", 10 );

  // Should not find file, only directories
  assert!( result.is_none() );
}

#[ test ]
fn matching_upward_find_finds_match_at_root()
{
  let temp = tempfile::tempdir().unwrap();
  let root = temp.path();

  fs::create_dir( root.join( ".git" ) ).unwrap();
  fs::write( root.join( "Cargo.toml" ), "test" ).unwrap();

  let result = matching_upward_find(
    root,
    | dir | dir.join( ".git" ).exists() && dir.join( "Cargo.toml" ).exists(),
    10
  );

  assert!( result.is_some() );
  assert_eq!( result.unwrap(), root );
}

#[ test ]
fn matching_upward_find_no_match_returns_none()
{
  let temp = tempfile::tempdir().unwrap();
  let root = temp.path();

  fs::create_dir_all( root.join( "project" ) ).unwrap();

  let result = matching_upward_find(
    root.join( "project" ).as_path(),
    | dir | dir.join( ".git" ).exists() && dir.join( "Cargo.toml" ).exists(),
    10
  );

  assert!( result.is_none() );
}

#[ test ]
fn matching_upward_find_respects_max_depth()
{
  let temp = tempfile::tempdir().unwrap();
  let root = temp.path();

  // Create deep hierarchy
  fs::create_dir_all( root.join( "l1/l2/l3/l4" ) ).unwrap();
  fs::write( root.join( "marker.txt" ), "root" ).unwrap();

  let start = root.join( "l1/l2/l3/l4" );

  // With depth=2, won't reach root
  let result = matching_upward_find(
    &start,
    | dir | dir.join( "marker.txt" ).exists(),
    2
  );

  assert!( result.is_none() );

  // With depth=10, will reach root
  let result = matching_upward_find(
    &start,
    | dir | dir.join( "marker.txt" ).exists(),
    10
  );

  assert!( result.is_some() );
}

#[ test ]
fn matching_upward_find_complex_predicate_finds_match()
{
  let temp = tempfile::tempdir().unwrap();
  let root = temp.path();

  // Create Rust project structure
  fs::create_dir_all( root.join( "project/src" ) ).unwrap();
  fs::create_dir( root.join( "project/.git" ) ).unwrap();
  fs::write( root.join( "project/Cargo.toml" ), "test" ).unwrap();
  fs::write( root.join( "project/Cargo.lock" ), "lock" ).unwrap();

  let start = root.join( "project/src" );
  let result = matching_upward_find(
    &start,
    | dir |
    {
      dir.join( ".git" ).exists()
        && dir.join( "Cargo.toml" ).exists()
        && dir.join( "Cargo.lock" ).exists()
    },
    10
  );

  assert!( result.is_some() );
  assert_eq!( result.unwrap(), root.join( "project" ) );
}

// =============================================================================
// Edge cases tests (8+ tests)
// =============================================================================

#[ test ]
fn edge_case_unicode_filenames()
{
  let temp = tempfile::tempdir().unwrap();
  let root = temp.path();

  // Unicode filename
  let filename = "配置.toml";
  fs::write( root.join( filename ), "unicode" ).unwrap();

  let result = file_upward_find( root, filename, 10 );

  assert!( result.is_some() );
  assert_eq!( result.unwrap(), root.join( filename ) );
}

#[ test ]
fn edge_case_deep_hierarchy_no_stack_overflow()
{
  let temp = tempfile::tempdir().unwrap();
  let root = temp.path();

  // Create very deep hierarchy (100 levels)
  let mut path = root.to_path_buf();
  for i in 0..100
  {
    path.push( format!( "level{i}" ) );
  }
  fs::create_dir_all( &path ).unwrap();

  // Place marker at root
  fs::write( root.join( "marker" ), "deep" ).unwrap();

  // Search from deepest level
  let result = file_upward_find( &path, "marker", 150 );

  assert!( result.is_some() );
}

#[ test ]
fn edge_case_symlink_to_parent_no_infinite_loop()
{
  let temp = tempfile::tempdir().unwrap();
  let root = temp.path();

  #[ cfg( unix ) ]
  {
    use std::os::unix::fs::symlink;

    fs::create_dir_all( root.join( "child" ) ).unwrap();

    // Create symlink from child back to parent (circular)
    symlink( root, root.join( "child/parent_link" ) ).unwrap();

    // This should not infinite loop
    let result = collect_files_in_ancestors(
      root.join( "child" ).as_path(),
      | path | path.file_name().and_then( | n | n.to_str() ) == Some( "marker" ),
      None,
      true
    );

    assert!( result.is_ok() );
  }
}

#[ test ]
fn edge_case_max_depth_zero()
{
  let temp = tempfile::tempdir().unwrap();
  let root = temp.path();

  fs::write( root.join( "file.txt" ), "test" ).unwrap();

  // Max depth 0 should still check current directory
  let _result = file_upward_find( root, "file.txt", 0 );

  // Implementation may vary: either finds in current dir or returns none
  // This tests that it doesn't panic with 0 depth
}

#[ test ]
fn edge_case_nonexistent_start_directory()
{
  let temp = tempfile::tempdir().unwrap();
  let root = temp.path();

  let nonexistent = root.join( "does_not_exist" );

  // collect_files_in_ancestors should return error for nonexistent path
  let result = collect_files_in_ancestors(
    nonexistent.as_path(),
    | _path | true,
    None,
    false
  );

  assert!( result.is_err() );
}

#[ test ]
fn edge_case_empty_filename()
{
  let temp = tempfile::tempdir().unwrap();
  let root = temp.path();

  // Empty filename should not panic
  let result = file_upward_find( root, "", 10 );

  // Should return None (can't find empty filename)
  assert!( result.is_none() );
}

#[ test ]
fn edge_case_special_characters_in_filename()
{
  let temp = tempfile::tempdir().unwrap();
  let root = temp.path();

  // Filename with spaces and special chars
  let filename = "my config (v2).toml";
  fs::write( root.join( filename ), "special" ).unwrap();

  let result = file_upward_find( root, filename, 10 );

  assert!( result.is_some() );
  assert_eq!( result.unwrap(), root.join( filename ) );
}

#[ test ]
fn edge_case_multiple_files_same_level()
{
  let temp = tempfile::tempdir().unwrap();
  let root = temp.path();

  // Multiple matching files at same level
  fs::write( root.join( "a.toml" ), "a" ).unwrap();
  fs::write( root.join( "b.toml" ), "b" ).unwrap();
  fs::write( root.join( "c.toml" ), "c" ).unwrap();

  let results = collect_files_in_ancestors(
    root,
    | path | path.extension().and_then( | e | e.to_str() ) == Some( "toml" ),
    None,
    false
  ).unwrap();

  // Should find all three
  assert_eq!( results.len(), 3 );
}
