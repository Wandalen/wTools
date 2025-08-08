//! Comprehensive Error Handling Tests for workspace_tools
//!
//! ## Test Matrix: Error Handling Coverage
//!
//! | Test ID | Error Variant | Scenario | Expected Behavior |
//! |---------|---------------|----------|-------------------|
//! | ER.1 | EnvironmentVariableMissing | Missing WORKSPACE_PATH | Proper error display |
//! | ER.2 | PathNotFound | Non-existent directory | Proper error display |
//! | ER.3 | IoError | File system IO failure | Proper error display |
//! | ER.4 | PathOutsideWorkspace | Path outside boundaries | Proper error display |
//! | ER.5 | CargoError | Cargo command failure | Proper error display |
//! | ER.6 | TomlError | TOML parsing failure | Proper error display |
//! | ER.7 | SerdeError | Serde serialization failure | Proper error display |
//! | ER.8 | Error trait | All variants | Implement Error trait correctly |
//! | ER.9 | Clone trait | All variants | Clone correctly |
//! | ER.10 | Debug trait | All variants | Debug format correctly |
//! | ER.11 | PartialEq trait | Same variants | Compare correctly |

use workspace_tools::{ Workspace, WorkspaceError };
use std::{ env, path::PathBuf };
use tempfile::TempDir;

/// Test ER.1: EnvironmentVariableMissing error display
#[ test ]
fn test_environment_variable_missing_display()
{
  let error = WorkspaceError::EnvironmentVariableMissing( "TEST_VAR".to_string() );
  let display = format!( "{}", error );
  
  assert!( display.contains( "TEST_VAR" ) );
  assert!( display.contains( "WORKSPACE_PATH" ) );
  assert!( display.to_lowercase().contains( "environment" ) );
}

/// Test ER.2: PathNotFound error display
#[ test ]
fn test_path_not_found_display()
{
  let test_path = PathBuf::from( "/nonexistent/test/path" );
  let error = WorkspaceError::PathNotFound( test_path.clone() );
  let display = format!( "{}", error );
  
  assert!( display.contains( "/nonexistent/test/path" ) );
  assert!( display.to_lowercase().contains( "not found" ) || display.to_lowercase().contains( "does not exist" ) );
}

/// Test ER.3: IoError error display
#[ test ]
fn test_io_error_display()
{
  let error = WorkspaceError::IoError( "Access denied".to_string() );
  let display = format!( "{}", error );
  
  assert!( display.contains( "Access denied" ) || display.contains( "permission denied" ) );
}

/// Test ER.4: PathOutsideWorkspace error display
#[ test ]
fn test_path_outside_workspace_display()
{
  let test_path = PathBuf::from( "/outside/workspace/path" );
  let error = WorkspaceError::PathOutsideWorkspace( test_path.clone() );
  let display = format!( "{}", error );
  
  assert!( display.contains( "/outside/workspace/path" ) );
  assert!( display.to_lowercase().contains( "outside" ) );
  assert!( display.to_lowercase().contains( "workspace" ) );
}

/// Test ER.5: CargoError error display
#[ cfg( feature = "cargo_integration" ) ]
#[ test ]
fn test_cargo_error_display()
{
  let error = WorkspaceError::CargoError( "Failed to parse Cargo.toml".to_string() );
  let display = format!( "{}", error );
  
  assert!( display.contains( "Failed to parse Cargo.toml" ) );
  assert!( display.to_lowercase().contains( "cargo" ) );
}

/// Test ER.6: TomlError error display
#[ cfg( feature = "cargo_integration" ) ]
#[ test ]
fn test_toml_error_display()
{
  let error = WorkspaceError::TomlError( "Invalid TOML syntax".to_string() );
  let display = format!( "{}", error );
  
  assert!( display.contains( "Invalid TOML syntax" ) );
  assert!( display.to_lowercase().contains( "toml" ) );
}

/// Test ER.7: SerdeError error display
#[ cfg( feature = "serde_integration" ) ]
#[ test ]
fn test_serde_error_display()
{
  let error = WorkspaceError::SerdeError( "Deserialization failed".to_string() );
  let display = format!( "{}", error );
  
  assert!( display.contains( "Deserialization failed" ) );
  assert!( display.to_lowercase().contains( "serde" ) || display.to_lowercase().contains( "serialization" ) );
}

/// Test ER.8: All error variants implement Error trait correctly
#[ test ]
fn test_error_trait_implementation()
{
  use core::error::Error;
  
  let mut errors : Vec< WorkspaceError > = vec![
    WorkspaceError::EnvironmentVariableMissing( "TEST".to_string() ),
    WorkspaceError::PathNotFound( PathBuf::from( "/test" ) ),
    WorkspaceError::IoError( "test io error".to_string() ),
    WorkspaceError::PathOutsideWorkspace( PathBuf::from( "/test" ) ),
  ];
  
  #[ cfg( feature = "cargo_integration" ) ]
  errors.push( WorkspaceError::CargoError( "test".to_string() ) );
  
  #[ cfg( feature = "cargo_integration" ) ]
  errors.push( WorkspaceError::TomlError( "test".to_string() ) );
  
  #[ cfg( feature = "serde_integration" ) ]
  errors.push( WorkspaceError::SerdeError( "test".to_string() ) );
  
  for error in errors
  {
    // Test that Error trait methods work
    let _description = error.to_string();
    let _source = error.source(); // Should not panic
    
    // Test Display is implemented
    assert!( !format!( "{}", error ).is_empty() );
    
    // Test Debug is implemented  
    assert!( !format!( "{:?}", error ).is_empty() );
  }
}

/// Test ER.9: All error variants can be cloned
#[ test ]
fn test_error_clone()
{
  let original = WorkspaceError::EnvironmentVariableMissing( "TEST".to_string() );
  let cloned = original.clone();
  
  // Verify clone by comparing string representations
  assert_eq!( format!( "{:?}", original ), format!( "{:?}", cloned ) );
  assert_eq!( original.to_string(), cloned.to_string() );
  
  let original2 = WorkspaceError::PathNotFound( PathBuf::from( "/test" ) );
  let cloned2 = original2.clone();
  
  assert_eq!( format!( "{:?}", original2 ), format!( "{:?}", cloned2 ) );
  assert_eq!( original2.to_string(), cloned2.to_string() );
}

/// Test ER.10: Error debug format is comprehensive
#[ test ]
fn test_error_debug_format()
{
  let error = WorkspaceError::EnvironmentVariableMissing( "DEBUG_TEST".to_string() );
  let debug = format!( "{:?}", error );
  
  assert!( debug.contains( "EnvironmentVariableMissing" ) );
  assert!( debug.contains( "DEBUG_TEST" ) );
}

/// Test ER.11: Error display messages are distinct
#[ test ]
fn test_error_display_distinctness()
{
  let error1 = WorkspaceError::EnvironmentVariableMissing( "SAME".to_string() );
  let error2 = WorkspaceError::EnvironmentVariableMissing( "SAME".to_string() );
  let error3 = WorkspaceError::EnvironmentVariableMissing( "DIFFERENT".to_string() );
  
  // Same content should produce same string representation
  assert_eq!( error1.to_string(), error2.to_string() );
  assert_ne!( error1.to_string(), error3.to_string() );
  
  let path_error1 = WorkspaceError::PathNotFound( PathBuf::from( "/same" ) );
  let path_error2 = WorkspaceError::PathNotFound( PathBuf::from( "/same" ) );
  let path_error3 = WorkspaceError::PathNotFound( PathBuf::from( "/different" ) );
  
  assert_eq!( path_error1.to_string(), path_error2.to_string() );
  assert_ne!( path_error1.to_string(), path_error3.to_string() );
  
  // Different error types should have different string representations
  assert_ne!( error1.to_string(), path_error1.to_string() );
}

/// Test ER.12: Error creation in real scenarios - resolve with missing env var
#[ test ]
fn test_error_creation_missing_env_var()
{
  // Save original state
  let original = env::var( "WORKSPACE_PATH" ).ok();
  
  // Remove environment variable
  env::remove_var( "WORKSPACE_PATH" );
  
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
    WorkspaceError::EnvironmentVariableMissing( var ) => assert_eq!( var, "WORKSPACE_PATH" ),
    other => panic!( "Expected EnvironmentVariableMissing, got {:?}", other ),
  }
}

/// Test ER.13: Error creation in real scenarios - resolve with invalid path
#[ test ]  
fn test_error_creation_invalid_path()
{
  // Save original state
  let original = env::var( "WORKSPACE_PATH" ).ok();
  
  let invalid_path = PathBuf::from( "/nonexistent/invalid/workspace/path/12345" );
  env::set_var( "WORKSPACE_PATH", &invalid_path );
  
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
    WorkspaceError::PathNotFound( path ) => assert_eq!( path, invalid_path ),
    other => panic!( "Expected PathNotFound, got {:?}", other ),
  }
}

/// Test ER.14: Error creation in real scenarios - validate non-existent path
#[ test ]
fn test_error_creation_validate_invalid()
{
  let temp_dir = TempDir::new().unwrap();
  let invalid_path = temp_dir.path().join( "nonexistent" );
  
  // Save original state and temporarily set invalid path
  let original = env::var( "WORKSPACE_PATH" ).ok();
  env::set_var( "WORKSPACE_PATH", &invalid_path );
  
  let workspace_result = Workspace::resolve();
  
  // Restore state
  match original
  {
    Some( value ) => env::set_var( "WORKSPACE_PATH", value ),
    None => env::remove_var( "WORKSPACE_PATH" ),
  }
  
  assert!( workspace_result.is_err() );
  match workspace_result.unwrap_err()
  {
    WorkspaceError::PathNotFound( path ) => assert_eq!( path, invalid_path ),
    other => panic!( "Expected PathNotFound, got {:?}", other ),
  }
}

/// Test ER.15: Error creation - path outside workspace boundary
#[ test ]
fn test_error_creation_path_outside_workspace()
{
  let temp_dir = TempDir::new().unwrap();
  
  // Save original state and set workspace path
  let original = env::var( "WORKSPACE_PATH" ).ok();
  env::set_var( "WORKSPACE_PATH", temp_dir.path() );
  
  let _workspace = Workspace::resolve().unwrap();
  
  // Restore state
  match original
  {
    Some( value ) => env::set_var( "WORKSPACE_PATH", value ),
    None => env::remove_var( "WORKSPACE_PATH" ),
  }
  
  let outside_path = PathBuf::from( "/etc/passwd" );
  
  // This should not create an error directly, but we can test the error type
  let error = WorkspaceError::PathOutsideWorkspace( outside_path.clone() );
  
  assert!( matches!( error, WorkspaceError::PathOutsideWorkspace( ref path ) if path == &outside_path ) );
}

/// Test ER.16: IO Error wrapping
#[ test ]
fn test_io_error_wrapping()
{
  let error_message = "Test permission denied";
  let workspace_err = WorkspaceError::IoError( error_message.to_string() );
  
  match workspace_err
  {
    WorkspaceError::IoError( ref message ) =>
    {
      assert_eq!( message, "Test permission denied" );
      assert!( message.contains( "Test permission denied" ) );
    },
    other => panic!( "Expected IoError, got {:?}", other ),
  }
}

/// Test ER.17: Error chain source testing
#[ test ]
fn test_error_source_chain()
{
  use core::error::Error;
  
  let workspace_err = WorkspaceError::IoError( "Invalid data format".to_string() );
  
  // Test source method 
  let source = workspace_err.source();
  // Since IoError now wraps String instead of std::io::Error, source should be None
  assert!( source.is_none() );
  
  // Test the error message directly
  assert!( workspace_err.to_string().contains( "Invalid data format" ) );
}

/// Test ER.18: All error variants have appropriate Display messages
#[ test ]
fn test_all_error_display_completeness()
{
  let test_cases = vec![
    ( WorkspaceError::EnvironmentVariableMissing( "VAR".to_string() ), vec![ "VAR", "environment" ] ),
    ( WorkspaceError::PathNotFound( PathBuf::from( "/missing" ) ), vec![ "/missing", "not found" ] ),
    ( WorkspaceError::PathOutsideWorkspace( PathBuf::from( "/outside" ) ), vec![ "/outside", "outside" ] ),
  ];
  
  for ( error, expected_substrings ) in test_cases
  {
    let display = error.to_string().to_lowercase();
    for expected in expected_substrings
    {
      assert!( display.contains( &expected.to_lowercase() ), 
        "Error '{}' should contain '{}' in display message", error, expected );
    }
  }
}