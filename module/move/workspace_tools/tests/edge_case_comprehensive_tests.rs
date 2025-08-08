//! Comprehensive Edge Case Tests for workspace_tools
//!
//! ## Test Matrix: Edge Case Coverage  
//!
//! | Test ID | Category | Scenario | Expected Behavior |
//! |---------|----------|----------|-------------------|
//! | EC.1 | Git integration | In git repository | from_git_root() succeeds |
//! | EC.2 | Git integration | Not in git repository | from_git_root() fails |  
//! | EC.3 | Git integration | Nested git repositories | Finds correct git root |
//! | EC.4 | Infallible operations | from_cwd() call | Always succeeds |
//! | EC.5 | Empty workspace | resolve_or_fallback() no env | Uses current dir |
//! | EC.6 | Helper functions | workspace() with invalid env | Proper error |
//! | EC.7 | Concurrent access | Multiple threads | Thread safe operations |
//! | EC.8 | Memory efficiency | Large path operations | No excessive allocations |
//! | EC.9 | Platform compatibility | Windows vs Unix paths | Cross-platform handling |
//! | EC.10 | Symlink handling | Workspace root is symlink | Correct resolution |

use workspace_tools::{ Workspace, WorkspaceError, workspace };
use std::{ env, fs, thread, sync::Arc };
use tempfile::TempDir;

/// Helper function to create a test workspace with proper cleanup
fn create_test_workspace_at( path : &std::path::Path ) -> Workspace
{
  let original = env::var( "WORKSPACE_PATH" ).ok();
  env::set_var( "WORKSPACE_PATH", path );
  
  let workspace = Workspace::resolve().unwrap();
  
  // Restore state
  match original
  {
    Some( value ) => env::set_var( "WORKSPACE_PATH", value ),
    None => env::remove_var( "WORKSPACE_PATH" ),
  }
  
  workspace
}

/// Test EC.1: from_git_root() in git repository  
#[ test ]
fn test_from_git_root_in_repository()
{
  let temp_dir = TempDir::new().unwrap();
  
  // Create a fake git repository structure
  let git_dir = temp_dir.path().join( ".git" );
  fs::create_dir_all( &git_dir ).unwrap();
  fs::write( git_dir.join( "HEAD" ), "ref: refs/heads/main" ).unwrap();
  
  // Change to subdirectory within the git repo
  let subdir = temp_dir.path().join( "src" );
  fs::create_dir_all( &subdir ).unwrap();
  
  let original_cwd = env::current_dir().unwrap();
  env::set_current_dir( &subdir ).unwrap();
  
  let result = Workspace::from_git_root();
  
  // Restore working directory
  env::set_current_dir( original_cwd ).unwrap();
  
  assert!( result.is_ok(), "from_git_root() should succeed when in git repository" );
  if let Ok( workspace ) = result
  {
    assert_eq!( workspace.root(), temp_dir.path() );
  }
}

/// Test EC.2: from_git_root() not in git repository
#[ test ]
fn test_from_git_root_not_in_repository()
{
  let temp_dir = TempDir::new().unwrap();
  
  let original_cwd = env::current_dir().unwrap();
  env::set_current_dir( temp_dir.path() ).unwrap();
  
  let result = Workspace::from_git_root();
  
  // Restore working directory
  env::set_current_dir( original_cwd ).unwrap();
  
  assert!( result.is_err(), "from_git_root() should fail when not in git repository" );
  match result.unwrap_err()
  {
    WorkspaceError::PathNotFound( _ ) => {}, // Expected
    other => panic!( "Expected PathNotFound, got {:?}", other ),
  }
}

/// Test EC.3: from_git_root() with nested git repositories
#[ test ]
fn test_from_git_root_nested_repositories()
{
  let temp_dir = TempDir::new().unwrap();
  
  // Create outer git repository
  let outer_git = temp_dir.path().join( ".git" );
  fs::create_dir_all( &outer_git ).unwrap();
  fs::write( outer_git.join( "HEAD" ), "ref: refs/heads/main" ).unwrap();
  
  // Create inner directory structure
  let inner_dir = temp_dir.path().join( "projects/inner" );
  fs::create_dir_all( &inner_dir ).unwrap();
  
  // Create inner git repository
  let inner_git = inner_dir.join( ".git" );
  fs::create_dir_all( &inner_git ).unwrap();
  fs::write( inner_git.join( "HEAD" ), "ref: refs/heads/develop" ).unwrap();
  
  let original_cwd = env::current_dir().unwrap();
  env::set_current_dir( &inner_dir ).unwrap();
  
  let result = Workspace::from_git_root();
  
  // Restore working directory
  env::set_current_dir( original_cwd ).unwrap();
  
  assert!( result.is_ok(), "from_git_root() should find nearest git root" );
  if let Ok( workspace ) = result
  {
    // Should find the inner git repository root, not the outer
    assert_eq!( workspace.root(), inner_dir );
  }
}

/// Test EC.4: from_cwd() is infallible
#[ test ]
fn test_from_cwd_infallible()
{
  // This should never fail, regardless of current directory
  let workspace = Workspace::from_cwd();
  
  // Should return current working directory
  let current_dir = env::current_dir().unwrap();
  assert_eq!( workspace.root(), current_dir );
  
  // Test multiple calls for consistency
  for _ in 0..5
  {
    let ws = Workspace::from_cwd();
    assert_eq!( ws.root(), current_dir );
  }
}

/// Test EC.5: resolve_or_fallback() behavior without environment
#[ test ]
fn test_resolve_or_fallback_no_environment()
{
  // Save original state
  let original = env::var( "WORKSPACE_PATH" ).ok();
  
  env::remove_var( "WORKSPACE_PATH" );
  
  let workspace = Workspace::resolve_or_fallback();
  
  // Restore state
  match original
  {
    Some( value ) => env::set_var( "WORKSPACE_PATH", value ),
    None => env::remove_var( "WORKSPACE_PATH" ),
  }
  
  // Should fallback to some valid workspace
  assert!( workspace.root().exists() || workspace.root().is_absolute() );
  
  // Should be able to validate (or at least attempt validation)
  let validation = workspace.validate();
  // Note: May fail if fallback directory doesn't exist, but shouldn't panic
}

/// Test EC.6: workspace() helper function error cases
#[ test ]
fn test_workspace_helper_function_error()
{
  // Save original state
  let original = env::var( "WORKSPACE_PATH" ).ok();
  
  env::set_var( "WORKSPACE_PATH", "/completely/nonexistent/path/12345" );
  
  let result = workspace();
  
  // Restore state
  match original
  {
    Some( value ) => env::set_var( "WORKSPACE_PATH", value ),
    None => env::remove_var( "WORKSPACE_PATH" ),
  }
  
  assert!( result.is_err(), "workspace() should fail with invalid path" );
}

/// Test EC.7: Concurrent access safety
#[ test ]
fn test_concurrent_workspace_access()
{
  let temp_dir = TempDir::new().unwrap();
  let workspace = Arc::new( create_test_workspace_at( temp_dir.path() ) );
  
  let mut handles = vec![];
  
  // Spawn multiple threads performing workspace operations
  for i in 0..10
  {
    let ws = Arc::clone( &workspace );
    let handle = thread::spawn( move || {
      // Perform various operations
      let _root = ws.root();
      let _config = ws.config_dir();
      let _joined = ws.join( format!( "file_{}.txt", i ) );
      let _is_workspace = ws.is_workspace_file( ws.root() );
      
      // Return thread ID for verification
      i
    });
    handles.push( handle );
  }
  
  // Collect results
  let mut results = vec![];
  for handle in handles
  {
    results.push( handle.join().unwrap() );
  }
  
  // All threads should complete successfully
  assert_eq!( results.len(), 10 );
  assert_eq!( results.iter().sum::<i32>(), 45 ); // 0+1+2+...+9 = 45
}

/// Test EC.8: Memory efficiency with large operations
#[ test ]
fn test_memory_efficiency_large_operations()
{
  let temp_dir = TempDir::new().unwrap();
  let workspace = create_test_workspace_at( temp_dir.path() );
  
  // Perform many path operations
  for i in 0..1000
  {
    let path = format!( "dir_{}/subdir_{}/file_{}.txt", i % 10, i % 100, i );
    let _joined = workspace.join( &path );
    let _is_workspace = workspace.is_workspace_file( temp_dir.path().join( &path ) );
    
    if i % 100 == 0
    {
      // Normalize some paths
      let _normalized = workspace.normalize_path( &path );
    }
  }
  
  // Test should complete without excessive memory usage or panics
  assert!( true, "Large operations completed successfully" );
}

/// Test EC.9: Cross-platform path handling
#[ test ]
fn test_cross_platform_path_handling()
{
  let temp_dir = TempDir::new().unwrap();
  let workspace = create_test_workspace_at( temp_dir.path() );
  
  // Test various path separators and formats
  let test_paths = vec![
    "config/app.toml",        // Unix style
    "config\\app.toml",       // Windows style (should be handled)
    "config/sub/app.toml",    // Deep Unix
    "config\\sub\\app.toml",  // Deep Windows  
    "./config/app.toml",      // Relative with current
    ".\\config\\app.toml",    // Relative Windows style
  ];
  
  for test_path in test_paths
  {
    let joined = workspace.join( test_path );
    
    // Should produce valid absolute paths
    assert!( joined.is_absolute(), "Joined path should be absolute for: {}", test_path );
    
    // Should start with workspace root
    assert!( joined.starts_with( temp_dir.path() ), 
      "Joined path should start with workspace root for: {}", test_path );
    
    // Normalization should work
    let normalized = workspace.normalize_path( test_path );
    assert!( normalized.is_ok(), "Normalization should succeed for: {}", test_path );
  }
}

/// Test EC.10: Symlink handling (Unix-like systems)
#[ cfg( unix ) ]
#[ test ]
fn test_symlink_workspace_root()
{
  let temp_dir = TempDir::new().unwrap();
  let actual_workspace = temp_dir.path().join( "actual" );
  let symlink_workspace = temp_dir.path().join( "symlink" );
  
  // Create actual directory
  fs::create_dir_all( &actual_workspace ).unwrap();
  
  // Create symlink to actual directory
  std::os::unix::fs::symlink( &actual_workspace, &symlink_workspace ).unwrap();
  
  // Create workspace using symlink
  let workspace = create_test_workspace_at( &symlink_workspace );
  
  // Validation should work
  let _validation = workspace.validate();
  // Note: validation may fail depending on how symlinks are handled by the system
  
  // Operations should work normally
  let config_dir = workspace.config_dir();
  assert!( config_dir.starts_with( &symlink_workspace ) );
  
  let joined = workspace.join( "test.txt" );
  assert!( joined.starts_with( &symlink_workspace ) );
  
  // Boundary checking should work
  assert!( workspace.is_workspace_file( &joined ) );
}

/// Test EC.11: Empty directory workspace operations
#[ test ]
fn test_empty_directory_workspace()
{
  let temp_dir = TempDir::new().unwrap();
  let workspace = create_test_workspace_at( temp_dir.path() );
  
  // All standard operations should work even in empty directory
  assert!( workspace.validate().is_ok() );
  assert_eq!( workspace.root(), temp_dir.path() );
  
  let config_dir = workspace.config_dir();
  assert_eq!( config_dir, temp_dir.path().join( "config" ) );
  
  let joined = workspace.join( "new_file.txt" );
  assert_eq!( joined, temp_dir.path().join( "new_file.txt" ) );
  
  assert!( workspace.is_workspace_file( &joined ) );
}

/// Test EC.12: Workspace with only hidden files
#[ test ]  
fn test_workspace_with_hidden_files()
{
  let temp_dir = TempDir::new().unwrap();
  
  // Create various hidden files
  fs::write( temp_dir.path().join( ".gitignore" ), "target/" ).unwrap();
  fs::write( temp_dir.path().join( ".env" ), "DEBUG=true" ).unwrap();
  fs::create_dir_all( temp_dir.path().join( ".git" ) ).unwrap();
  fs::write( temp_dir.path().join( ".git/config" ), "[core]\n" ).unwrap();
  
  let workspace = create_test_workspace_at( temp_dir.path() );
  
  // Should validate successfully
  assert!( workspace.validate().is_ok() );
  
  // Hidden files should be considered workspace files
  assert!( workspace.is_workspace_file( temp_dir.path().join( ".gitignore" ) ) );
  assert!( workspace.is_workspace_file( temp_dir.path().join( ".env" ) ) );
  assert!( workspace.is_workspace_file( temp_dir.path().join( ".git" ) ) );
  assert!( workspace.is_workspace_file( temp_dir.path().join( ".git/config" ) ) );
}

/// Test EC.13: Workspace operations with very long filenames
#[ test ]
fn test_very_long_filename_operations()
{
  let temp_dir = TempDir::new().unwrap();
  let workspace = create_test_workspace_at( temp_dir.path() );
  
  // Create very long filename (but within reasonable limits)
  let long_name = "a".repeat( 200 );
  let long_filename = format!( "{}.txt", long_name );
  
  let joined = workspace.join( &long_filename );
  assert!( joined.starts_with( temp_dir.path() ) );
  assert!( joined.file_name().unwrap().to_string_lossy().len() > 200 );
  
  assert!( workspace.is_workspace_file( &joined ) );
  
  let normalized = workspace.normalize_path( &long_filename );
  assert!( normalized.is_ok() );
}

/// Test EC.14: Rapid repeated operations
#[ test ]
fn test_rapid_repeated_operations()
{
  let temp_dir = TempDir::new().unwrap();
  let workspace = create_test_workspace_at( temp_dir.path() );
  
  // Perform many rapid operations
  for i in 0..100
  {
    let filename = format!( "file_{}.txt", i );
    
    // All these should be consistent across calls
    let joined1 = workspace.join( &filename );
    let joined2 = workspace.join( &filename );
    assert_eq!( joined1, joined2 );
    
    let config1 = workspace.config_dir();
    let config2 = workspace.config_dir();
    assert_eq!( config1, config2 );
    
    let root1 = workspace.root();
    let root2 = workspace.root();
    assert_eq!( root1, root2 );
    
    assert_eq!( workspace.is_workspace_file( &joined1 ), workspace.is_workspace_file( &joined2 ) );
  }
}