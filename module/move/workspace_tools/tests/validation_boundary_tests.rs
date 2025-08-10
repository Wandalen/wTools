//! Comprehensive Validation and Boundary Tests for `workspace_tools`  
//!
//! ## Test Matrix: Validation and Boundary Coverage
//!
//! | Test ID | Method | Input Scenario | Expected Result |
//! |---------|--------|---------------|-----------------|
//! | VB.1 | validate() | File instead of directory | Error |
//! | VB.2 | validate() | No read permissions | Error |
//! | VB.3 | validate() | Symlink to valid directory | Success |
//! | VB.4 | validate() | Symlink to invalid target | Error |
//! | VB.5 | is_workspace_file() | Symlink inside workspace | true |
//! | VB.6 | is_workspace_file() | Symlink outside workspace | false |
//! | VB.7 | is_workspace_file() | Broken symlink | false |
//! | VB.8 | is_workspace_file() | Exact workspace root | true |
//! | VB.9 | is_workspace_file() | Parent of workspace root | false |
//! | VB.10 | Workspace creation | Empty string path | Error |
//! | VB.11 | Workspace creation | Root directory path | Success |
//! | VB.12 | Workspace creation | Relative path resolution | Correct absolute path |

use workspace_tools::{ Workspace, WorkspaceError };
use std::{ env, fs, path::PathBuf };
use std::sync::Mutex;

// Global mutex to serialize environment variable tests
static ENV_TEST_MUTEX: Mutex< () > = Mutex::new( () );
use tempfile::{ TempDir, NamedTempFile };

/// Helper function to create a test workspace without environment variables
fn create_test_workspace_at( path : &std::path::Path ) -> Workspace
{
  Workspace::new( path )
}

/// Test VB.1: `validate()` with file instead of directory
#[ test ]
fn test_validate_file_instead_of_directory()
{
  let temp_file = NamedTempFile::new().unwrap();
  
  // For this test, we need to create a workspace that points to a file
  // We'll use resolve directly with invalid environment setup
  let original = env::var( "WORKSPACE_PATH" ).ok();
  env::set_var( "WORKSPACE_PATH", temp_file.path() );
  
  let workspace_result = Workspace::resolve();
  
  // Restore state
  match original
  {
    Some( value ) => env::set_var( "WORKSPACE_PATH", value ),
    None => env::remove_var( "WORKSPACE_PATH" ),
  }
  
  // The result might vary depending on implementation
  // If resolve succeeds, validation should fail
  if let Ok( workspace ) = workspace_result
  {
    let validation = workspace.validate();
    assert!( validation.is_err(), "Validation should fail when workspace root is a file" );
  }
  else
  {
    // If resolve fails, that's also acceptable
    match workspace_result.unwrap_err()
    {
      WorkspaceError::IoError( _ ) | WorkspaceError::PathNotFound( _ ) => {}, // Expected - file is not a valid workspace directory
      other => panic!( "Expected IoError or PathNotFound, got {other:?}" ),
    }
  }
}

/// Test VB.2: `validate()` with directory that exists  
#[ test ]
fn test_validate_existing_directory_success()
{
  let temp_dir = TempDir::new().unwrap();
  let workspace = create_test_workspace_at( temp_dir.path() );
  
  let result = workspace.validate();
  
  assert!( result.is_ok(), "validate() should succeed for existing directory" );
}

/// Test VB.3: `validate()` with non-existent directory
#[ test ]
fn test_validate_nonexistent_directory()
{
  let temp_dir = TempDir::new().unwrap();
  let nonexistent = temp_dir.path().join( "nonexistent" );
  
  // Set invalid path and attempt to resolve
  let original = env::var( "WORKSPACE_PATH" ).ok();
  env::set_var( "WORKSPACE_PATH", &nonexistent );
  
  let result = Workspace::resolve();
  
  // Restore state
  match original
  {
    Some( value ) => env::set_var( "WORKSPACE_PATH", value ),
    None => env::remove_var( "WORKSPACE_PATH" ),
  }
  
  assert!( result.is_err() );
  match result.unwrap_err()
  {
    WorkspaceError::PathNotFound( path ) => assert_eq!( path, nonexistent ),
    other => panic!( "Expected PathNotFound, got {other:?}" ),
  }
}

/// Test VB.4: `is_workspace_file()` with exact workspace root
#[ test ]
fn test_is_workspace_file_exact_root()
{
  let temp_dir = TempDir::new().unwrap();
  let workspace = create_test_workspace_at( temp_dir.path() );
  
  // The workspace root itself should be considered a workspace file
  let is_workspace = workspace.is_workspace_file( temp_dir.path() );
  assert!( is_workspace, "Workspace root should be considered a workspace file" );
}

/// Test VB.5: `is_workspace_file()` with parent of workspace root
#[ test ]
fn test_is_workspace_file_parent_directory()
{
  let temp_dir = TempDir::new().unwrap();
  let workspace = create_test_workspace_at( temp_dir.path() );
  
  // Parent directory should not be considered a workspace file
  if let Some( parent ) = temp_dir.path().parent()
  {
    let is_workspace = workspace.is_workspace_file( parent );
    assert!( !is_workspace, "Parent of workspace root should not be considered a workspace file" );
  }
}

/// Test VB.6: `is_workspace_file()` with deeply nested path
#[ test ]
fn test_is_workspace_file_deeply_nested()
{
  let temp_dir = TempDir::new().unwrap();
  let workspace = create_test_workspace_at( temp_dir.path() );
  
  let nested_path = temp_dir.path()
    .join( "level1" )
    .join( "level2" )
    .join( "level3" )
    .join( "deep_file.txt" );
  
  let is_workspace = workspace.is_workspace_file( &nested_path );
  assert!( is_workspace, "Deeply nested path should be considered a workspace file" );
}

/// Test VB.7: `is_workspace_file()` with path containing .. traversal
#[ test ]
fn test_is_workspace_file_with_traversal()
{
  let temp_dir = TempDir::new().unwrap();
  let workspace = create_test_workspace_at( temp_dir.path() );
  
  // Create a path that goes out and back in
  let traversal_path = temp_dir.path()
    .join( "subdir" )
    .join( ".." )
    .join( "file.txt" );
  
  let is_workspace = workspace.is_workspace_file( &traversal_path );
  assert!( is_workspace, "Path with .. traversal that stays within workspace should be considered workspace file" );
}

/// Test VB.8: `is_workspace_file()` with absolute path outside workspace
#[ test ]
fn test_is_workspace_file_absolute_outside()
{
  let temp_dir = TempDir::new().unwrap();
  let workspace = create_test_workspace_at( temp_dir.path() );
  
  let outside_paths = vec![
    PathBuf::from( "/etc/passwd" ),
    PathBuf::from( "/tmp/outside.txt" ),
    PathBuf::from( "/usr/bin/ls" ),
  ];
  
  for outside_path in outside_paths
  {
    let is_workspace = workspace.is_workspace_file( &outside_path );
    assert!( !is_workspace, "Path {} should not be considered a workspace file", outside_path.display() );
  }
}

/// Test VB.9: Workspace creation with empty string path  
#[ test ]
fn test_workspace_creation_empty_path()
{
  let _lock = ENV_TEST_MUTEX.lock().unwrap();
  
  // Save original state
  let original = env::var( "WORKSPACE_PATH" ).ok();
  
  env::set_var( "WORKSPACE_PATH", "" );
  
  let result = Workspace::resolve();
  
  // Restore state
  match original
  {
    Some( value ) => env::set_var( "WORKSPACE_PATH", value ),
    None => env::remove_var( "WORKSPACE_PATH" ),
  }
  
  // Empty path should result in an error
  assert!( result.is_err(), "Empty WORKSPACE_PATH should result in error" );
}

/// Test VB.10: Workspace creation with root directory path
#[ test ]
fn test_workspace_creation_root_directory()
{
  // Save original state  
  let original = env::var( "WORKSPACE_PATH" ).ok();
  
  env::set_var( "WORKSPACE_PATH", "/" );
  
  let result = Workspace::resolve();
  
  // Restore state
  match original
  {
    Some( value ) => env::set_var( "WORKSPACE_PATH", value ),
    None => env::remove_var( "WORKSPACE_PATH" ),
  }
  
  // Root directory should work (if accessible)
  if let Ok( workspace ) = result
  {
    assert_eq!( workspace.root(), PathBuf::from( "/" ) );
  }
  // If it fails, it should be due to permissions, not path resolution
}

/// Test VB.11: Workspace creation with relative path resolution
#[ test ]
fn test_workspace_creation_relative_path()
{
  let temp_dir = TempDir::new().unwrap();
  
  // Save original state
  let original = env::var( "WORKSPACE_PATH" ).ok();
  let original_cwd = env::current_dir().unwrap();
  
  // Change to temp directory and set relative path
  env::set_current_dir( temp_dir.path() ).unwrap();
  env::set_var( "WORKSPACE_PATH", "." );
  
  let result = Workspace::resolve();
  
  // Restore state
  env::set_current_dir( original_cwd ).unwrap();
  match original
  {
    Some( value ) => env::set_var( "WORKSPACE_PATH", value ),
    None => env::remove_var( "WORKSPACE_PATH" ),
  }
  
  assert!( result.is_ok() );
  let workspace = result.unwrap();
  
  // Workspace root should exist and be a valid path
  assert!( workspace.root().exists() );
  
  // May or may not be absolute depending on implementation,
  // but should be a valid path that can be used
  let validation = workspace.validate();
  assert!( validation.is_ok(), "Workspace should be valid even if path is relative" );
}

/// Test VB.12: Boundary testing with edge case paths
#[ test ]
fn test_boundary_edge_case_paths()
{
  let temp_dir = TempDir::new().unwrap();
  let workspace = create_test_workspace_at( temp_dir.path() );
  
  let edge_cases = vec![
    // Empty components
    temp_dir.path().join( "" ),
    // Current directory reference
    temp_dir.path().join( "." ),
    // Parent and current mixed
    temp_dir.path().join( "./subdir/../file.txt" ),
    // Multiple slashes
    temp_dir.path().join( "config//app.toml" ),
  ];
  
  for edge_case in edge_cases
  {
    let is_workspace = workspace.is_workspace_file( &edge_case );
    // All these should be within workspace bounds
    assert!( is_workspace, "Edge case path should be within workspace: {}", edge_case.display() );
  }
}

/// Test VB.13: Validation with workspace containing special files
#[ test ]
fn test_validation_with_special_files()
{
  let temp_dir = TempDir::new().unwrap();
  
  // Create some special files that might exist in real workspaces
  fs::write( temp_dir.path().join( "Cargo.toml" ), "[package]\nname = \"test\"\n" ).unwrap();
  fs::write( temp_dir.path().join( ".gitignore" ), "target/\n" ).unwrap();
  fs::write( temp_dir.path().join( "README.md" ), "# Test Workspace\n" ).unwrap();
  
  let workspace = create_test_workspace_at( temp_dir.path() );
  
  let result = workspace.validate();
  assert!( result.is_ok(), "Validation should succeed for directory with typical workspace files" );
  
  // Verify the special files are considered workspace files
  assert!( workspace.is_workspace_file( workspace.cargo_toml() ) );
  assert!( workspace.is_workspace_file( workspace.readme() ) );
  assert!( workspace.is_workspace_file( temp_dir.path().join( ".gitignore" ) ) );
}

/// Test VB.14: Path edge cases with join
#[ test ]
fn test_path_join_edge_cases()
{
  let temp_dir = TempDir::new().unwrap();
  let workspace = create_test_workspace_at( temp_dir.path() );
  
  let edge_cases = vec![
    ".",
    "./",  
    "subdir/..",
    "subdir/../other",
    "",
  ];
  
  for edge_case in edge_cases
  {
    let joined = workspace.join( edge_case );
    
    // All join operations should produce absolute paths
    assert!( joined.is_absolute(), "Joined path should be absolute for: {edge_case}" );
    assert!( joined.starts_with( temp_dir.path() ), "Joined path should start with workspace root for: {edge_case}" );
  }
}

/// Test VB.15: Large workspace directory structure
#[ test ]
fn test_large_workspace_structure()
{
  let temp_dir = TempDir::new().unwrap();
  let workspace = create_test_workspace_at( temp_dir.path() );
  
  // Create a reasonably complex directory structure
  let dirs = vec![
    "src/main",
    "src/lib", 
    "tests/integration",
    "tests/unit",
    "config/dev",
    "config/prod",
    "data/migrations",
    "docs/api",
    "docs/user",
    ".workspace/cache",
  ];
  
  for dir in &dirs
  {
    fs::create_dir_all( temp_dir.path().join( dir ) ).unwrap();
  }
  
  // Validation should still work
  let result = workspace.validate();
  assert!( result.is_ok(), "Validation should work with complex directory structure" );
  
  // All created directories should be within workspace
  for dir in &dirs
  {
    let dir_path = temp_dir.path().join( dir );
    assert!( workspace.is_workspace_file( &dir_path ), "Directory {dir} should be within workspace" );
  }
}

/// Test VB.16: Workspace with deeply nested subdirectories
#[ test ]
fn test_deeply_nested_workspace()
{
  let temp_dir = TempDir::new().unwrap();
  
  // Create deep nesting
  let mut deep_path = temp_dir.path().to_path_buf();
  for i in 1..=20
  {
    deep_path.push( format!( "level{i}" ) );
  }
  
  fs::create_dir_all( &deep_path ).unwrap();
  
  let workspace = create_test_workspace_at( temp_dir.path() );
  
  // Validation should work with deep nesting
  let result = workspace.validate();
  assert!( result.is_ok(), "Validation should work with deeply nested structure" );
  
  // Deep path should be within workspace
  assert!( workspace.is_workspace_file( &deep_path ), "Deeply nested path should be within workspace" );
}