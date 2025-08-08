//! Comprehensive Path Operations Tests for workspace_tools
//!
//! ## Test Matrix: Path Operations Coverage
//!
//! | Test ID | Method | Input Scenario | Expected Result |
//! |---------|--------|---------------|-----------------|
//! | PO.1 | join() | Relative path | Correct joined path |
//! | PO.2 | join() | Absolute path | Returns absolute path as-is |
//! | PO.3 | join() | Empty path | Returns workspace root |
//! | PO.4 | join() | Path with .. traversal | Normalized path |
//! | PO.5 | join() | Path with . current dir | Normalized path |
//! | PO.6 | cargo_toml() | Any workspace | workspace_root/Cargo.toml |
//! | PO.7 | readme() | Any workspace | workspace_root/README.md |
//! | PO.8 | normalize_path() | Valid relative path | Normalized absolute path |
//! | PO.9 | normalize_path() | Path with .. traversal | Normalized path |
//! | PO.10 | normalize_path() | Non-existent path | Normalized path works |
//! | PO.11 | normalize_path() | Already absolute path | Same absolute path |
//! | PO.12 | Path operations | Unicode characters | Correct handling |
//! | PO.13 | Path operations | Special characters | Correct handling |
//! | PO.14 | Path operations | Very long paths | Correct handling |

use workspace_tools::Workspace;
use std::{ env, path::PathBuf };
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

/// Test PO.1: join() with relative path
#[ test ]
fn test_join_relative_path()
{
  let temp_dir = TempDir::new().unwrap();
  let workspace = create_test_workspace_at( temp_dir.path() );
  
  let joined = workspace.join( "config/app.toml" );
  let expected = temp_dir.path().join( "config/app.toml" );
  
  assert_eq!( joined, expected );
}

/// Test PO.2: join() with absolute path
#[ test ]
fn test_join_absolute_path()
{
  let temp_dir = TempDir::new().unwrap();
  let workspace = create_test_workspace_at( temp_dir.path() );
  
  let absolute_path = PathBuf::from( "/etc/hosts" );
  let joined = workspace.join( &absolute_path );
  
  // join() should return the absolute path as-is
  assert_eq!( joined, absolute_path );
}

/// Test PO.3: join() with empty path
#[ test ] 
fn test_join_empty_path()
{
  let temp_dir = TempDir::new().unwrap();
  let workspace = create_test_workspace_at( temp_dir.path() );
  
  let joined = workspace.join( "" );
  
  // Empty path should return workspace root
  assert_eq!( joined, workspace.root() );
}

/// Test PO.4: join() with parent directory traversal
#[ test ]
fn test_join_parent_traversal()
{
  let temp_dir = TempDir::new().unwrap();
  let workspace = create_test_workspace_at( temp_dir.path() );
  
  let joined = workspace.join( "config/../data/file.txt" );
  let expected = temp_dir.path().join( "config/../data/file.txt" );
  
  assert_eq!( joined, expected );
}

/// Test PO.5: join() with current directory references
#[ test ]
fn test_join_current_directory()
{
  let temp_dir = TempDir::new().unwrap();
  let workspace = create_test_workspace_at( temp_dir.path() );
  
  let joined = workspace.join( "./config/./app.toml" );
  let expected = temp_dir.path().join( "./config/./app.toml" );
  
  assert_eq!( joined, expected );
}

/// Test PO.6: cargo_toml() returns correct path
#[ test ]
fn test_cargo_toml_path()
{
  let temp_dir = TempDir::new().unwrap();
  let workspace = create_test_workspace_at( temp_dir.path() );
  
  let cargo_path = workspace.cargo_toml();
  let expected = temp_dir.path().join( "Cargo.toml" );
  
  assert_eq!( cargo_path, expected );
}

/// Test PO.7: readme() returns correct path  
#[ test ]
fn test_readme_path()
{
  let temp_dir = TempDir::new().unwrap();
  let workspace = create_test_workspace_at( temp_dir.path() );
  
  let readme_path = workspace.readme();
  let expected = temp_dir.path().join( "readme.md" );
  
  assert_eq!( readme_path, expected );
}

/// Test PO.8: Path operations work correctly
#[ test ]
fn test_path_operations_work()
{
  let temp_dir = TempDir::new().unwrap();
  let workspace = create_test_workspace_at( temp_dir.path() );
  
  // Test that basic path operations work
  let config_path = workspace.join( "config/app.toml" );
  assert!( config_path.is_absolute() );
  assert!( config_path.starts_with( temp_dir.path() ) );
  assert!( config_path.ends_with( "config/app.toml" ) );
}

/// Test PO.12: Path operations with Unicode characters
#[ test ]
fn test_unicode_path_handling()
{
  let temp_dir = TempDir::new().unwrap();
  let workspace = create_test_workspace_at( temp_dir.path() );
  
  // Test with various Unicode characters
  let unicode_paths = vec![
    "配置/应用.toml",           // Chinese
    "конфигурация/файл.txt",    // Cyrillic
    "العربية/ملف.json",        // Arabic
    "日本語/設定.yaml",          // Japanese
    "🚀/config/🎯.toml",        // Emojis
  ];
  
  for unicode_path in unicode_paths
  {
    let joined = workspace.join( unicode_path );
    let expected = temp_dir.path().join( unicode_path );
    assert_eq!( joined, expected );
    
    // Basic path operations should work with Unicode
    assert!( joined.is_absolute() );
    assert!( joined.starts_with( temp_dir.path() ) );
  }
}

/// Test PO.13: Path operations with special characters
#[ test ]
fn test_special_characters_path_handling()
{
  let temp_dir = TempDir::new().unwrap();
  let workspace = create_test_workspace_at( temp_dir.path() );
  
  // Test with special characters (platform appropriate)
  let special_paths = vec![
    "config with spaces/app.toml",
    "config-with-dashes/app.toml",
    "config_with_underscores/app.toml",
    "config.with.dots/app.toml",
    "config@with@symbols/app.toml",
  ];
  
  for special_path in special_paths
  {
    let joined = workspace.join( special_path );
    let expected = temp_dir.path().join( special_path );
    assert_eq!( joined, expected );
    
    // Basic path operations should work with special characters
    assert!( joined.is_absolute() );
    assert!( joined.starts_with( temp_dir.path() ) );
  }
}

/// Test PO.14: Path operations with very long paths
#[ test ]
fn test_very_long_path_handling()
{
  let temp_dir = TempDir::new().unwrap();
  let workspace = create_test_workspace_at( temp_dir.path() );
  
  // Create a very long path (but reasonable for testing)
  let long_dir_name = "a".repeat( 50 );
  let mut long_path = PathBuf::new();
  
  // Create nested structure
  for i in 0..10
  {
    long_path.push( format!( "{}_{}", long_dir_name, i ) );
  }
  long_path.push( "final_file.txt" );
  
  let joined = workspace.join( &long_path );
  let expected = temp_dir.path().join( &long_path );
  assert_eq!( joined, expected );
  
  // Basic operations should work with long paths
  assert!( joined.is_absolute() );
  assert!( joined.starts_with( temp_dir.path() ) );
}

/// Test PO.15: Multiple join operations chaining
#[ test ]
fn test_multiple_join_operations()
{
  let temp_dir = TempDir::new().unwrap();
  let workspace = create_test_workspace_at( temp_dir.path() );
  
  let path1 = workspace.join( "config" );
  let path2 = workspace.join( "data" );
  let path3 = workspace.join( "logs/debug.log" );
  
  assert_eq!( path1, temp_dir.path().join( "config" ) );
  assert_eq!( path2, temp_dir.path().join( "data" ) );
  assert_eq!( path3, temp_dir.path().join( "logs/debug.log" ) );
  
  // Ensure they're all different
  assert_ne!( path1, path2 );
  assert_ne!( path2, path3 );
  assert_ne!( path1, path3 );
}

/// Test PO.16: Standard directory paths are correct
#[ test ]
fn test_all_standard_directory_paths()
{
  let temp_dir = TempDir::new().unwrap();
  let workspace = create_test_workspace_at( temp_dir.path() );
  
  let expected_mappings = vec![
    ( workspace.config_dir(), "config" ),
    ( workspace.data_dir(), "data" ),
    ( workspace.logs_dir(), "logs" ),
    ( workspace.docs_dir(), "docs" ),
    ( workspace.tests_dir(), "tests" ),
    ( workspace.workspace_dir(), ".workspace" ),
    ( workspace.cargo_toml(), "Cargo.toml" ),
    ( workspace.readme(), "readme.md" ),
  ];
  
  for ( actual_path, expected_suffix ) in expected_mappings
  {
    let expected = temp_dir.path().join( expected_suffix );
    assert_eq!( actual_path, expected, "Mismatch for {}", expected_suffix );
  }
}

/// Test PO.17: Secret directory path (when feature enabled)
#[ cfg( feature = "secret_management" ) ]
#[ test ]  
fn test_secret_directory_path()
{
  let temp_dir = TempDir::new().unwrap();
  let workspace = create_test_workspace_at( temp_dir.path() );
  
  let secret_dir = workspace.secret_dir();
  let expected = temp_dir.path().join( ".secret" );
  
  assert_eq!( secret_dir, expected );
}

/// Test PO.18: Secret file path (when feature enabled)
#[ cfg( feature = "secret_management" ) ]
#[ test ]
fn test_secret_file_path()
{
  let temp_dir = TempDir::new().unwrap();
  let workspace = create_test_workspace_at( temp_dir.path() );
  
  let secret_file = workspace.secret_file( "api.env" );
  let expected = temp_dir.path().join( ".secret/api.env" );
  
  assert_eq!( secret_file, expected );
}

/// Test PO.19: Root path immutability
#[ test ]
fn test_root_path_immutability()
{
  let temp_dir = TempDir::new().unwrap();
  let workspace = create_test_workspace_at( temp_dir.path() );
  
  let root1 = workspace.root();
  let root2 = workspace.root();
  
  // Should always return the same path
  assert_eq!( root1, root2 );
  assert_eq!( root1, temp_dir.path() );
}

/// Test PO.20: Path operations are consistent across calls
#[ test ]
fn test_path_operations_consistency()
{
  let temp_dir = TempDir::new().unwrap();
  let workspace = create_test_workspace_at( temp_dir.path() );
  
  // Multiple calls should return identical results
  for _ in 0..5
  {
    assert_eq!( workspace.config_dir(), temp_dir.path().join( "config" ) );
    assert_eq!( workspace.join( "test.txt" ), temp_dir.path().join( "test.txt" ) );
    
    let join_result1 = workspace.join( "test/file.txt" );
    let join_result2 = workspace.join( "test/file.txt" );
    
    // Multiple calls should return identical results
    assert_eq!( join_result1, join_result2 );
  }
}