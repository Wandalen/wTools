//! comprehensive tests for `workspace_tools` functionality
//!
//! ## test matrix for workspace functionality
//!
//! | id   | aspect tested           | environment     | expected behavior       |
//! |------|-------------------------|-----------------|-------------------------|
//! | t1.1 | workspace resolution    | env var set     | resolves successfully   |
//! | t1.2 | workspace resolution    | env var missing | returns error          |
//! | t1.3 | workspace validation    | valid path      | validation succeeds     |
//! | t1.4 | workspace validation    | invalid path    | validation fails        |
//! | t2.1 | standard directories    | any workspace   | returns correct paths   |
//! | t2.2 | path joining           | relative paths  | joins correctly         |
//! | t2.3 | workspace boundaries    | internal path   | returns true           |
//! | t2.4 | workspace boundaries    | external path   | returns false          |
//! | t3.1 | fallback resolution     | no env, cwd     | uses current dir        |
//! | t3.2 | git root resolution     | git repo        | finds git root         |
//! | t4.1 | cross-platform paths    | any platform    | normalizes correctly    |

use workspace_tools::{ Workspace, WorkspaceError, workspace };
use tempfile::TempDir;
use std::{ env, path::PathBuf };

/// test workspace resolution with environment variable set
/// test combination: t1.1
#[ test ]
#[ ignore = "Environment variable manipulation has concurrency issues with other tests" ]
fn test_workspace_resolution_with_env_var()
{
  let temp_dir = TempDir::new().unwrap();
  env::set_var( "WORKSPACE_PATH", temp_dir.path() );
  
  let workspace = Workspace::resolve().unwrap();
  assert_eq!( workspace.root(), temp_dir.path() );
  
  // cleanup
  env::remove_var( "WORKSPACE_PATH" );
}

/// test workspace resolution with missing environment variable
/// test combination: t1.2
#[ test ]
fn test_workspace_resolution_missing_env_var()
{
  env::remove_var( "WORKSPACE_PATH" );
  
  let result = Workspace::resolve();
  assert!( result.is_err() );
  
  match result.unwrap_err()
  {
    WorkspaceError::EnvironmentVariableMissing( var ) =>
    {
      assert_eq!( var, "WORKSPACE_PATH" );
    }
    other => panic!( "expected EnvironmentVariableMissing, got {other:?}" ),
  }
}

/// test workspace validation with valid path
/// test combination: t1.3
#[ test ]
fn test_workspace_validation_valid_path()
{
  let temp_dir = TempDir::new().unwrap();
  env::set_var( "WORKSPACE_PATH", temp_dir.path() );
  
  let workspace = Workspace::resolve().unwrap();
  let result = workspace.validate();
  
  assert!( result.is_ok() );
  
  // cleanup
  env::remove_var( "WORKSPACE_PATH" );
}

/// test workspace validation with invalid path
/// test combination: t1.4
#[ test ]
fn test_workspace_validation_invalid_path()
{
  // Save original env var to restore later
  let original_workspace_path = env::var( "WORKSPACE_PATH" ).ok();
  
  let invalid_path = PathBuf::from( "/nonexistent/workspace/path/12345" );
  env::set_var( "WORKSPACE_PATH", &invalid_path );
  
  let result = Workspace::resolve();
  
  // Restore original environment immediately after resolve
  match original_workspace_path
  {
    Some( path ) => env::set_var( "WORKSPACE_PATH", path ),
    None => env::remove_var( "WORKSPACE_PATH" ),
  }
  
  // Now check the result
  assert!( result.is_err() );
  
  match result.unwrap_err()
  {
    WorkspaceError::PathNotFound( path ) =>
    {
      assert_eq!( path, invalid_path );
    }
    other => panic!( "expected PathNotFound, got {other:?}" ),
  }
}

/// test standard directory paths
/// test combination: t2.1
#[ test ]
fn test_standard_directories()
{
  let temp_dir = TempDir::new().unwrap();
  
  let workspace = Workspace::new( temp_dir.path() );
  
  assert_eq!( workspace.config_dir(), temp_dir.path().join( "config" ) );
  assert_eq!( workspace.data_dir(), temp_dir.path().join( "data" ) );
  assert_eq!( workspace.logs_dir(), temp_dir.path().join( "logs" ) );
  assert_eq!( workspace.docs_dir(), temp_dir.path().join( "docs" ) );
  assert_eq!( workspace.tests_dir(), temp_dir.path().join( "tests" ) );
  assert_eq!( workspace.workspace_dir(), temp_dir.path().join( ".workspace" ) );
}

/// test path joining functionality
/// test combination: t2.2
#[ test ]
fn test_path_joining()
{
  let temp_dir = TempDir::new().unwrap();
  env::set_var( "WORKSPACE_PATH", temp_dir.path() );
  
  let workspace = Workspace::resolve().unwrap();
  
  let joined = workspace.join( "config/app.toml" );
  let expected = temp_dir.path().join( "config/app.toml" );
  
  assert_eq!( joined, expected );
  
  // cleanup
  env::remove_var( "WORKSPACE_PATH" );
}

/// test workspace boundary checking for internal paths
/// test combination: t2.3
#[ test ]
fn test_workspace_boundaries_internal()
{
  let temp_dir = TempDir::new().unwrap();
  env::set_var( "WORKSPACE_PATH", temp_dir.path() );
  
  let workspace = Workspace::resolve().unwrap();
  let internal_path = workspace.join( "config/app.toml" );
  
  assert!( workspace.is_workspace_file( &internal_path ) );
  
  // cleanup
  env::remove_var( "WORKSPACE_PATH" );
}

/// test workspace boundary checking for external paths
/// test combination: t2.4
#[ test ]
fn test_workspace_boundaries_external()
{
  let temp_dir = TempDir::new().unwrap();
  env::set_var( "WORKSPACE_PATH", temp_dir.path() );
  
  let workspace = Workspace::resolve().unwrap();
  let external_path = PathBuf::from( "/etc/passwd" );
  
  assert!( !workspace.is_workspace_file( &external_path ) );
  
  // cleanup
  env::remove_var( "WORKSPACE_PATH" );
}

/// test fallback resolution behavior
/// test combination: t3.1
#[ test ]
fn test_fallback_resolution_current_dir()
{
  env::remove_var( "WORKSPACE_PATH" );
  
  let workspace = Workspace::resolve_or_fallback();
  
  // with cargo integration enabled, should detect cargo workspace first
  #[ cfg( feature = "cargo_integration" ) ]
  {
    // should detect actual cargo workspace (not just fallback to current dir)
    assert!( workspace.is_cargo_workspace() );
    // workspace root should exist and be a directory
    assert!( workspace.root().exists() );
    assert!( workspace.root().is_dir() );
    // should contain a Cargo.toml with workspace configuration
    assert!( workspace.cargo_toml().exists() );
  }
  
  // without cargo integration, should fallback to current directory
  #[ cfg( not( feature = "cargo_integration" ) ) ]
  {
    let current_dir = env::current_dir().unwrap();
    assert_eq!( workspace.root(), current_dir );
  }
}

/// test workspace creation from current directory
#[ test ]
fn test_from_current_dir()
{
  let workspace = Workspace::from_current_dir().unwrap();
  let current_dir = env::current_dir().unwrap();
  
  assert_eq!( workspace.root(), current_dir );
}

/// test convenience function
#[ test ]
fn test_convenience_function()
{
  // Save original env var to restore later
  let original_workspace_path = env::var( "WORKSPACE_PATH" ).ok();
  
  let temp_dir = TempDir::new().unwrap();
  env::set_var( "WORKSPACE_PATH", temp_dir.path() );
  
  let ws = workspace().unwrap();
  assert_eq!( ws.root(), temp_dir.path() );
  
  // Restore original environment
  match original_workspace_path {
    Some( path ) => env::set_var( "WORKSPACE_PATH", path ),
    None => env::remove_var( "WORKSPACE_PATH" ),
  }
}

/// test error display formatting
#[ test ]
fn test_error_display()
{
  let error = WorkspaceError::EnvironmentVariableMissing( "TEST_VAR".to_string() );
  let display = format!( "{error}" );
  
  assert!( display.contains( "TEST_VAR" ) );
  assert!( display.contains( "WORKSPACE_PATH" ) );
}

/// test workspace creation with testing utilities
#[ test ]
fn test_testing_utilities()
{
  use workspace_tools::testing::{ create_test_workspace, create_test_workspace_with_structure };
  
  // test basic workspace creation
  let ( _temp_dir, workspace ) = create_test_workspace();
  assert!( workspace.root().exists() );
  
  // test workspace with structure
  let ( _temp_dir, workspace ) = create_test_workspace_with_structure();
  assert!( workspace.config_dir().exists() );
  assert!( workspace.data_dir().exists() );
  assert!( workspace.logs_dir().exists() );
}

#[ cfg( feature = "secret_management" ) ]
mod secret_management_tests
{
  use super::*;
  use std::fs;
  
  /// test secret directory path
  #[ test ]
  fn test_secret_directory()
  {
    let temp_dir = TempDir::new().unwrap();
    env::set_var( "WORKSPACE_PATH", temp_dir.path() );
    
    let workspace = Workspace::resolve().unwrap();
    assert_eq!( workspace.secret_dir(), temp_dir.path().join( ".secret" ) );
    
    // cleanup
    env::remove_var( "WORKSPACE_PATH" );
  }
  
  /// test secret file loading
  #[ test ]
  fn test_secret_file_loading()
  {
    let temp_dir = TempDir::new().unwrap();
    env::set_var( "WORKSPACE_PATH", temp_dir.path() );
    
    let workspace = Workspace::resolve().unwrap();
    
    // create secret directory and file
    let secret_dir = workspace.secret_dir();
    fs::create_dir_all( &secret_dir ).unwrap();
    
    let secret_file = secret_dir.join( "test.env" );
    fs::write( &secret_file, "API_KEY=secret123\nDB_URL=postgres://localhost\n# comment\n" ).unwrap();
    
    // load secrets
    let secrets = workspace.load_secrets_from_file( "test.env" ).unwrap();
    
    assert_eq!( secrets.get( "API_KEY" ), Some( &"secret123".to_string() ) );
    assert_eq!( secrets.get( "DB_URL" ), Some( &"postgres://localhost".to_string() ) );
    assert!( !secrets.contains_key( "comment" ) );
    
    // cleanup
    env::remove_var( "WORKSPACE_PATH" );
  }
  
  /// test secret key loading with fallback
  #[ test ]
  fn test_secret_key_loading_with_fallback()
  {
    let temp_dir = TempDir::new().unwrap();
    env::set_var( "TEST_ENV_KEY", "env_value" );
    
    let workspace = Workspace::new( temp_dir.path() );
    
    // test fallback to environment variable
    let value = workspace.load_secret_key( "TEST_ENV_KEY", "nonexistent.env" ).unwrap();
    assert_eq!( value, "env_value" );
    
    // cleanup
    env::remove_var( "TEST_ENV_KEY" );
  }
}

#[ cfg( feature = "glob" ) ]
mod glob_tests
{
  use super::*;
  use std::fs;
  
  /// test resource discovery with glob patterns
  #[ test ]
  fn test_find_resources()
  {
    let temp_dir = TempDir::new().unwrap();
    env::set_var( "WORKSPACE_PATH", temp_dir.path() );
    
    let workspace = Workspace::resolve().unwrap();
    
    // create test files
    let src_dir = workspace.join( "src" );
    fs::create_dir_all( &src_dir ).unwrap();
    
    let test_files = vec![ "lib.rs", "main.rs", "mod.rs" ];
    for file in &test_files
    {
      fs::write( src_dir.join( file ), "// test content" ).unwrap();
    }
    
    // find rust files
    let found = workspace.find_resources( "src/*.rs" ).unwrap();
    assert_eq!( found.len(), 3 );
    
    // all found files should be rust files
    for path in found
    {
      assert!( path.extension().unwrap() == "rs" );
      assert!( workspace.is_workspace_file( &path ) );
    }
    
    // cleanup
    env::remove_var( "WORKSPACE_PATH" );
  }
  
  /// test configuration file discovery
  #[ test ]
  fn test_find_config()
  {
    let temp_dir = TempDir::new().unwrap();
    let original = env::var( "WORKSPACE_PATH" ).ok();
    
    env::set_var( "WORKSPACE_PATH", temp_dir.path() );
    
    let workspace = Workspace::resolve().unwrap();
    
    // create config directory and file
    let config_dir = workspace.config_dir();
    fs::create_dir_all( &config_dir ).unwrap();
    
    let config_file = config_dir.join( "app.toml" );
    fs::write( &config_file, "[app]\nname = \"test\"\n" ).unwrap();
    
    // find config
    let found = workspace.find_config( "app" ).unwrap();
    assert_eq!( found, config_file );
    
    // restore environment
    match original
    {
      Some( value ) => env::set_var( "WORKSPACE_PATH", value ),
      None => env::remove_var( "WORKSPACE_PATH" ),
    }
  }
  
  /// test config file discovery with multiple extensions
  #[ test ]
  fn test_find_config_multiple_extensions()
  {
    let temp_dir = TempDir::new().unwrap();
    
    let workspace = Workspace::new( temp_dir.path() );
    
    // create config directory
    let config_dir = workspace.config_dir();
    fs::create_dir_all( &config_dir ).unwrap();
    
    // create yaml config (should be found before json)
    let yaml_config = config_dir.join( "database.yaml" );
    fs::write( &yaml_config, "host: localhost\n" ).unwrap();
    
    let json_config = config_dir.join( "database.json" );
    fs::write( &json_config, "{\"host\": \"localhost\"}\n" ).unwrap();
    
    // should find yaml first (based on search order)
    let found = workspace.find_config( "database" ).unwrap();
    assert_eq!( found, yaml_config );
  }
}