//! Cross-Platform Compatibility Tests
//!
//! These tests ensure `workspace_tools` works correctly on all platforms
//! by handling platform-specific path differences and behaviors.

use workspace_tools :: { Workspace, WorkspaceError };
use std :: { env, fs, path ::PathBuf };
use tempfile :: { TempDir, NamedTempFile };

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
  fs ::create_dir_all( workspace.workspace_dir() ).ok();

  ( temp_dir, workspace )
}

/// Tests platform-appropriate absolute path handling
#[ test ]
fn test_cross_platform_absolute_paths()
{
  let ( _temp_dir, workspace ) = create_test_workspace_with_structure();
  
  // Test platform-appropriate absolute paths
  #[ cfg( windows ) ]
  let absolute_path = "C: \\Windows\\System32\\cmd.exe";
  #[ cfg( not( windows ) ) ]
  let absolute_path = "/usr/bin/ls";
  
  let joined = workspace.join( absolute_path );
  
  // PathBuf ::join behavior: absolute path components replace the entire path
  assert_eq!( joined, PathBuf ::from( absolute_path ) );
}

/// Tests boundary checking with platform-appropriate external paths
#[ test ]
fn test_cross_platform_boundary_checking()
{
  let ( _temp_dir, workspace ) = create_test_workspace_with_structure();
  
  // Create list of external paths appropriate for each platform
  let mut external_paths = vec![ env ::temp_dir() ];
  
  #[ cfg( windows ) ]
  {
  external_paths.push( PathBuf ::from( "C: \\" ) );
  external_paths.push( PathBuf ::from( "D: \\" ) );
 }
  
  #[ cfg( not( windows ) ) ]
  {
  external_paths.push( PathBuf ::from( "/" ) );
  external_paths.push( PathBuf ::from( "/usr" ) );
  external_paths.push( PathBuf ::from( "/tmp" ) );
 }
  
  // All these paths should be outside workspace
  for path in external_paths
  {
  assert!( 
   !workspace.is_workspace_file( &path ),
   "path should be outside workspace: {}", 
   path.display() 
 );
 }
}

/// Tests file vs directory validation behavior
#[ test ]
fn test_cross_platform_file_directory_validation()
{
  let temp_file = NamedTempFile ::new().expect( "Failed to create temp file" );
  let original_workspace_path = env ::var( "WORKSPACE_PATH" ).ok();
  
  // Set workspace path to a file instead of directory
  env ::set_var( "WORKSPACE_PATH", temp_file.path() );
  
  // Resolve should succeed (file exists) 
  let workspace = Workspace ::resolve().expect( "Resolve should succeed for existing file" );
  
  // But validate should fail (file is not a directory)
  let validation_result = workspace.validate();
  
  // Restore original environment
  match original_workspace_path
  {
  Some( path ) => env ::set_var( "WORKSPACE_PATH", path ),
  None => env ::remove_var( "WORKSPACE_PATH" ),
 }
  
  // Assert validation fails with proper error
  assert!( validation_result.is_err(), "Validation should fail for file path" );
  
  match validation_result.unwrap_err()
  {
  WorkspaceError ::ConfigurationError( msg ) => 
  {
   assert!( 
  msg.contains( "not a directory" ),
  "Error message should mention directory issue: {msg}" 
 );
 },
  other => panic!( "Expected ConfigurationError, got: {other:?}" ),
 }
}

/// Tests guaranteed nonexistent path behavior across platforms  
#[ test ]
fn test_cross_platform_nonexistent_paths()
{
  let original_workspace_path = env ::var( "WORKSPACE_PATH" ).ok();
  
  // Create a guaranteed nonexistent path using system temp + unique components
  let thread_id = std ::thread ::current().id();
  let timestamp = std ::time ::SystemTime ::now()
  .duration_since( std ::time ::UNIX_EPOCH )
  .unwrap_or_default()
  .as_nanos();
  
  let nonexistent_path = env ::temp_dir()
  .join( format!( "workspace_test_{thread_id:?}_{timestamp}" ) )
  .join( "definitely_nonexistent_subdir" )
  .join( "another_level" );
  
  // Ensure this path absolutely doesn't exist
  if nonexistent_path.exists()
  {
  fs ::remove_dir_all( &nonexistent_path ).ok();
 }
  
  env ::set_var( "WORKSPACE_PATH", &nonexistent_path );
  
  let resolve_result = Workspace ::resolve();
  
  // Restore original environment
  match original_workspace_path
  {
  Some( path ) => env ::set_var( "WORKSPACE_PATH", path ),
  None => env ::remove_var( "WORKSPACE_PATH" ),
 }
  
  // Should fail with PathNotFound
  assert!( resolve_result.is_err(), "Resolve should fail for nonexistent path" );
  
  match resolve_result.unwrap_err()
  {
  WorkspaceError ::PathNotFound( path ) => 
  {
   assert_eq!( path, nonexistent_path, "Error should contain the correct nonexistent path" );
 },
  WorkspaceError ::EnvironmentVariableMissing( _ ) => 
  {
   // Acceptable in case of race condition with parallel tests
   eprintln!( "Warning: Environment variable was cleared by parallel test" );
 },
  other => panic!( "Expected PathNotFound or EnvironmentVariableMissing, got: {other:?}" ),
 }
}

/// Tests config file creation and finding across platforms
#[ test ]
fn test_cross_platform_config_files()
{
  let ( _temp_dir, workspace ) = create_test_workspace_with_structure();
  
  // Test config file creation and finding  
  let config_file = workspace.config_dir().join( "test_app.toml" );
  
  // Ensure parent directory exists (should already exist from create_test_workspace_with_structure)
  if let Some( parent ) = config_file.parent()
  {
  fs ::create_dir_all( parent ).expect( "Failed to create config directory" );
 }
  
  // Write config file
  fs ::write( &config_file, "[app]\nname = \"cross_platform_test\"\n" )
  .expect( "Failed to write config file" );
  
  // Find the config file
  let found_config = workspace.find_config( "test_app" )
  .expect( "Should find the config file" );
  
  assert_eq!( found_config, config_file, "Found config should match created config" );
  assert!( found_config.exists(), "Found config file should exist" );
}

/// Tests path normalization across platforms
#[ test ]  
fn test_cross_platform_path_normalization()
{
  let ( _temp_dir, workspace ) = create_test_workspace_with_structure();
  
  // Create a test file for normalization
  let test_file = workspace.join( "normalize_test.txt" );
  fs ::write( &test_file, "test content" ).expect( "Failed to write test file" );
  
  // Test normalization of existing file
  let normalized = workspace.normalize_path( "normalize_test.txt" )
  .expect( "Normalization should succeed for existing file" );
  
  assert!( normalized.is_absolute(), "Normalized path should be absolute" );
  assert!( normalized.exists(), "Normalized path should exist" );
  
  // Test normalization of nonexistent file (should fail)
  let nonexistent_result = workspace.normalize_path( "nonexistent_file.txt" );
  assert!( nonexistent_result.is_err(), "Normalization should fail for nonexistent file" );
}