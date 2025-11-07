//! Test Matrix for Rulebook Compliance Verification
//! 
//! | ID   | Test Factor       | Value    | Expected Behavior |
//! |------|-------------------|----------|-------------------|
//! | T1.1 | Workspace Creation| Valid    | Instance created successfully |
//! | T1.2 | Path Resolution   | Relative | Correct absolute path returned |
//! | T1.3 | Error Handling    | Missing  | Proper error returned |
//! | T1.4 | Directory Creation| Standard | All directories created |

use workspace_tools :: { Workspace, WorkspaceError, workspace };
use std :: { path ::PathBuf, fs };
use tempfile ::TempDir;

/// helper to create test workspace with standard directory structure
fn create_test_workspace_with_structure() -> ( TempDir, Workspace )
{
  let temp_dir = TempDir ::new().expect( "Failed to create temp directory" );
  let workspace = Workspace ::new( temp_dir.path() );

  // create standard directories
  fs ::create_dir_all( workspace.config_dir() ).ok();
  fs ::create_dir_all( workspace.data_dir() ).ok();
  fs ::create_dir_all( workspace.logs_dir() ).ok();
  fs ::create_dir_all( workspace.docs_dir() ).ok();
  fs ::create_dir_all( workspace.tests_dir() ).ok();

  ( temp_dir, workspace )
}

/// Tests that workspace creation works with explicit parameters.
/// Test Combination: T1.1
#[ test ]
fn test_workspace_creation_explicit_path()
{
  let temp_dir = std ::env ::temp_dir();
  let test_path = temp_dir.join( "test_workspace_explicit" );
  
  // Create test directory structure
  std ::fs ::create_dir_all( &test_path ).expect( "Failed to create test directory" );
  
  // Test with explicit path - no default parameters used
  let workspace = Workspace ::new( test_path.clone() );
  
  assert_eq!( workspace.root(), test_path.as_path() );
  
  // Cleanup
  std ::fs ::remove_dir_all( &test_path ).ok();
}

/// Tests workspace-relative path resolution with explicit components.
/// Test Combination: T1.2  
#[ test ]
fn test_path_resolution_explicit_components()
{
  let ( _temp_dir, workspace ) = create_test_workspace_with_structure();
  
  // Test explicit path joining - no default behavior relied upon
  let config_path = workspace.join( "config/app.toml" );
  let data_path = workspace.join( "data/cache.db" );
  
  assert!( config_path.starts_with( workspace.root() ) );
  assert!( data_path.starts_with( workspace.root() ) );
  assert!( config_path.ends_with( "config/app.toml" ) );
  assert!( data_path.ends_with( "data/cache.db" ) );
}

/// Tests proper error handling for missing environment variable.
/// Test Combination: T1.3
#[ test ]
fn test_error_handling_missing_env_var()
{
  // Temporarily remove the environment variable
  let original_value = std ::env ::var( "WORKSPACE_PATH" ).ok();
  std ::env ::remove_var( "WORKSPACE_PATH" );
  
  // Test should return proper error - explicit error verification
  let result = Workspace ::resolve();
  
  match result
  {
  Err( WorkspaceError ::EnvironmentVariableMissing( var ) ) =>
  {
   assert_eq!( var, "WORKSPACE_PATH" );
 },
  _ => panic!( "Expected EnvironmentVariableMissing error" ),
 }
  
  // Restore environment variable if it existed
  if let Some( value ) = original_value
  {
  std ::env ::set_var( "WORKSPACE_PATH", value );
 }
}

/// Tests standard directory creation with explicit directory list.
/// Test Combination: T1.4
#[ test ]
fn test_standard_directory_structure_explicit()
{
  let ( _temp_dir, workspace ) = create_test_workspace_with_structure();
  
  // Explicit verification of each directory - no defaults assumed
  let expected_dirs = vec!
  [
  workspace.config_dir(),
  workspace.data_dir(), 
  workspace.logs_dir(),
  workspace.docs_dir(),
  workspace.tests_dir(),
  workspace.workspace_dir(),
 ];
  
  for dir in expected_dirs
  {
  assert!( dir.exists(), "Directory should exist: {}", dir.display() );
  assert!( dir.is_dir(), "Path should be a directory: {}", dir.display() );
  assert!( dir.starts_with( workspace.root() ), "Directory should be within workspace: {}", dir.display() );
 }
}

/// Tests workspace boundary validation with explicit paths.
/// Test Combination: T1.5
#[ test ]
fn test_workspace_boundary_validation_explicit()
{
  let ( _temp_dir, workspace ) = create_test_workspace_with_structure();
  
  // Test explicit workspace file detection
  let internal_path = workspace.join( "config/test.toml" );
  let external_path = PathBuf ::from( "/tmp/external.toml" );
  
  assert!( workspace.is_workspace_file( &internal_path ) );
  assert!( !workspace.is_workspace_file( &external_path ) );
}

/// Tests configuration directory getter with explicit comparison.
/// Test Combination: T1.6
#[ test ] 
fn test_config_dir_explicit_path_construction()
{
  let ( _temp_dir, workspace ) = create_test_workspace_with_structure();
  
  // Explicit path construction verification - no implicit behavior
  let config_dir = workspace.config_dir();
  let expected_path = workspace.root().join( "config" );
  
  assert_eq!( config_dir, expected_path );
  assert!( config_dir.is_absolute() );
}