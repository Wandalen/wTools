//! Tests for `TempDir` directory creation and RAII cleanup.
//!
//! ## Design Decisions
//!
//! **Why add private `created_path` field?**
//! Tracks directories created by `create()`/`create_all()` for automatic cleanup.
//! Users who set paths manually (via public fields) don't get auto-cleanup.
//!
//! **Why RAII cleanup only for `create()` paths?**
//! Prevents accidental deletion of user-specified directories.
//! Only directories we explicitly created should be cleaned up.
//!
//! **Why `full_path()` method?**
//! Composes `base_path`/`prefix_path`/`postfix_path` into single `PathBuf`.
//! Essential for actual filesystem operations.

#[ cfg( not( feature = "no_std" ) ) ]
use std::path::PathBuf;

// ══════════════════════════════════════════════════════════════════════════════
// `full_path()` tests
// ══════════════════════════════════════════════════════════════════════════════

/// Tests that `full_path()` joins all path components correctly.
#[ test ]
#[ cfg( not( feature = "no_std" ) ) ]
fn full_path_joins_components()
{
  let mut temp_dir = the_module::TempDir::new();
  temp_dir.base_path = PathBuf::from( "/tmp" );
  temp_dir.prefix_path = PathBuf::from( "test" );
  temp_dir.postfix_path = PathBuf::from( "run_1" );

  let full = temp_dir.full_path();
  assert_eq!( full, PathBuf::from( "/tmp/test/run_1" ) );
}

/// Tests that `full_path()` handles empty components.
#[ test ]
#[ cfg( not( feature = "no_std" ) ) ]
fn full_path_handles_empty_components()
{
  let mut temp_dir = the_module::TempDir::new();
  temp_dir.base_path = PathBuf::from( "/tmp" );
  // prefix and postfix are empty

  let full = temp_dir.full_path();
  assert_eq!( full, PathBuf::from( "/tmp" ) );
}

/// Tests that `full_path()` with only postfix works.
#[ test ]
#[ cfg( not( feature = "no_std" ) ) ]
fn full_path_postfix_only()
{
  let mut temp_dir = the_module::TempDir::new();
  temp_dir.postfix_path = PathBuf::from( "suffix" );

  let full = temp_dir.full_path();
  assert_eq!( full, PathBuf::from( "suffix" ) );
}

// ══════════════════════════════════════════════════════════════════════════════
// create() tests
// ══════════════════════════════════════════════════════════════════════════════

/// Tests that `create()` creates directory and returns path.
/// Note: `create()` requires parent directory to exist, unlike `create_all()`.
#[ test ]
#[ cfg( not( feature = "no_std" ) ) ]
fn create_creates_directory()
{
  let mut temp_dir = the_module::TempDir::new();
  // Only use base_path + postfix_path so parent (temp_dir) exists
  temp_dir.base_path = std::env::temp_dir();
  temp_dir.postfix_path = PathBuf::from( format!( "fs_tools_create_test_{}", std::process::id() ) );

  let path = temp_dir.create().expect( "create should succeed" );

  assert!( path.exists(), "Directory should exist after create()" );
  assert!( path.is_dir(), "Path should be a directory" );

  // Cleanup handled by Drop, but verify path matches
  assert_eq!( path, temp_dir.full_path() );
}

/// Tests that `create()` fails if parent doesn't exist.
#[ test ]
#[ cfg( not( feature = "no_std" ) ) ]
fn create_fails_without_parent()
{
  let mut temp_dir = the_module::TempDir::new();
  temp_dir.base_path = PathBuf::from( "/nonexistent_parent_12345" );
  temp_dir.prefix_path = PathBuf::from( "child" );

  let result = temp_dir.create();
  assert!( result.is_err(), "create() should fail without parent" );
}

// ══════════════════════════════════════════════════════════════════════════════
// create_all() tests
// ══════════════════════════════════════════════════════════════════════════════

/// Tests that `create_all()` creates nested directories.
#[ test ]
#[ cfg( not( feature = "no_std" ) ) ]
fn create_all_creates_nested_directories()
{
  let mut temp_dir = the_module::TempDir::new();
  temp_dir.base_path = std::env::temp_dir();
  temp_dir.prefix_path = PathBuf::from( format!( "fs_tools_nested_{}", std::process::id() ) );
  temp_dir.postfix_path = PathBuf::from( "deep/nested/path" );

  let path = temp_dir.create_all().expect( "create_all should succeed" );

  assert!( path.exists(), "Directory should exist after create_all()" );
  assert!( path.is_dir(), "Path should be a directory" );

  // Cleanup handled by Drop
}

/// Tests that `create_all()` is idempotent (doesn't fail if exists).
#[ test ]
#[ cfg( not( feature = "no_std" ) ) ]
fn create_all_idempotent()
{
  let mut temp_dir = the_module::TempDir::new();
  temp_dir.base_path = std::env::temp_dir();
  temp_dir.prefix_path = PathBuf::from( format!( "fs_tools_idem_{}", std::process::id() ) );

  // Create twice - should not fail
  let path1 = temp_dir.create_all().expect( "first create_all should succeed" );
  let path2 = temp_dir.create_all().expect( "second create_all should succeed" );

  assert_eq!( path1, path2, "Both calls should return same path" );
}

// ══════════════════════════════════════════════════════════════════════════════
// RAII cleanup tests
// ══════════════════════════════════════════════════════════════════════════════

/// Tests that Drop cleans up created directory.
#[ test ]
#[ cfg( not( feature = "no_std" ) ) ]
fn drop_cleans_up_created_directory()
{
  let path_to_check;

  {
    let mut temp_dir = the_module::TempDir::new();
    temp_dir.base_path = std::env::temp_dir();
    temp_dir.prefix_path = PathBuf::from( format!( "fs_tools_drop_{}", std::process::id() ) );

    let path = temp_dir.create_all().expect( "create should succeed" );
    path_to_check = path.clone();

    assert!( path_to_check.exists(), "Directory should exist before drop" );
    // temp_dir goes out of scope here, Drop is called
  }

  assert!( !path_to_check.exists(), "Directory should be removed after drop" );
}

/// Tests that Drop doesn't clean up manually set paths (not created by us).
#[ test ]
#[ cfg( not( feature = "no_std" ) ) ]
fn drop_does_not_cleanup_manual_paths()
{
  // Create a real directory manually
  let manual_path = std::env::temp_dir().join( format!( "fs_tools_manual_{}", std::process::id() ) );
  fs::create_dir_all( &manual_path ).expect( "manual create should succeed" );

  {
    let mut temp_dir = the_module::TempDir::new();
    temp_dir.base_path = manual_path.clone();
    // Don't call create() - just set path manually
    // temp_dir goes out of scope here
  }

  // Directory should still exist because we didn't call create()
  assert!( manual_path.exists(), "Manual directory should NOT be removed by drop" );

  // Cleanup manually
  fs::remove_dir_all( &manual_path ).ok();
}

/// Tests that Drop handles already-deleted directory gracefully.
#[ test ]
#[ cfg( not( feature = "no_std" ) ) ]
fn drop_handles_already_deleted()
{
  let mut temp_dir = the_module::TempDir::new();
  temp_dir.base_path = std::env::temp_dir();
  temp_dir.prefix_path = PathBuf::from( format!( "fs_tools_predelete_{}", std::process::id() ) );

  let path = temp_dir.create_all().expect( "create should succeed" );

  // Manually delete before drop
  fs::remove_dir_all( &path ).expect( "manual delete should succeed" );

  // Drop should not panic even though directory is already gone
  drop( temp_dir );
  // Test passes if no panic occurred
}
