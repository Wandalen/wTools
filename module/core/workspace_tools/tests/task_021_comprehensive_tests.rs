#![ allow( clippy ::uninlined_format_args, clippy ::doc_markdown, clippy ::useless_vec ) ]

//! Comprehensive tests for Task 021 - Improve Secrets API UX and Error Handling
//!
//! Tests all acceptance criteria from the task specification :
//! - Enhanced error handling and validation
//! - API method improvements
//! - Documentation consistency
//! - Backward compatibility

#[ cfg( feature = "secrets" ) ]
use workspace_tools ::testing;
#[ cfg( feature = "secrets" ) ]
use std ::fs;

/// Test Phase 1 : Enhanced error handling and validation
#[ cfg( feature = "secrets" ) ]
mod phase_1_enhanced_error_handling
{
  use super :: *;

  /// Test explicit file existence errors (replaces silent empty HashMap returns)
  #[ test ]
  #[ cfg( feature = "secrets" ) ]
  fn test_explicit_file_existence_errors()
  {
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();

  // Test nonexistent file returns error instead of empty HashMap
  let result = workspace.load_secrets_from_file( "nonexistent.env" );
  assert!( result.is_err() );

  let error_msg = result.unwrap_err().to_string();
  assert!( error_msg.contains( "not found" ) );
  assert!( error_msg.contains( "nonexistent.env" ) );
 }

  /// Test path validation warnings
  #[ test ]
  #[ cfg( feature = "secrets" ) ]
  fn test_path_validation_warnings()
  {
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();

  let path_like_params = vec![
   "config/secrets.env",
   "lib/project/secret/api.env",
   "../secrets/prod.env",
   "dir\\windows\\style.env",
 ];

  for param in path_like_params
  {
   // Should emit warning and return error (not empty HashMap)
   let result = workspace.load_secrets_from_file( param );
   assert!( result.is_err() );

   let error_msg = result.unwrap_err().to_string();
   assert!( error_msg.contains( "not found" ) );
 }
 }

  /// Test enhanced error context with resolved paths
  #[ test ]
  #[ cfg( feature = "secrets" ) ]
  fn test_enhanced_error_context()
  {
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();

  let result = workspace.load_secrets_from_file( "missing-file.env" );
  assert!( result.is_err() );

  let error_msg = result.unwrap_err().to_string();
  assert!( error_msg.contains( "missing-file.env" ) );
  assert!( error_msg.contains( "not found" ) );
  // Check for path components instead of exact path (cross-platform)
  assert!( error_msg.contains( "secret" ) );
  assert!( error_msg.contains( "missing-file.env" ) );
 }

  /// Test available files suggestions
  #[ test ]
  #[ cfg( feature = "secrets" ) ]
  fn test_available_files_suggestions()
  {
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();

  // Create some test files
  let secret_dir = workspace.secret_dir();
  fs ::create_dir_all( &secret_dir ).unwrap();
  fs ::write( workspace.secret_file( "test1.env" ), "KEY1=value1" ).unwrap();
  fs ::write( workspace.secret_file( "test2.env" ), "KEY2=value2" ).unwrap();

  let result = workspace.load_secrets_from_file( "missing.env" );
  assert!( result.is_err() );

  let error_msg = result.unwrap_err().to_string();
  assert!( error_msg.contains( "Available files: " ) );
  assert!( error_msg.contains( "test1.env" ) );
  assert!( error_msg.contains( "test2.env" ) );
 }

  /// Test enhanced error context in load_secret_key
  #[ test ]
  #[ cfg( feature = "secrets" ) ]
  fn test_load_secret_key_enhanced_errors()
  {
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();

  let result = workspace.load_secret_key( "API_KEY", "missing.env" );
  assert!( result.is_err() );

  let error_msg = result.unwrap_err().to_string();
  assert!( error_msg.contains( "API_KEY not found in secrets file 'missing.env'" ) );
  assert!( error_msg.contains( "resolved to: " ) );
  // Check for path components instead of exact path (cross-platform)
  assert!( error_msg.contains( "secret" ) );
  assert!( error_msg.contains( "missing.env" ) );
 }
}

/// Test Phase 2 : API method improvements
#[ cfg( feature = "secrets" ) ]
mod phase_2_api_improvements
{
  use super :: *;

  /// Test new load_secrets_from_path method
  #[ test ]
  #[ cfg( feature = "secrets" ) ]
  fn test_load_secrets_from_path()
  {
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();

  // Create nested structure
  let config_dir = workspace.join( "config" );
  fs ::create_dir_all( &config_dir ).unwrap();

  let secret_content = "PATH_KEY=path-test-value\nCONFIG_TOKEN=config-token";
  fs ::write( config_dir.join( "secrets.env" ), secret_content ).unwrap();

  // Test path-based loading
  let secrets = workspace.load_secrets_from_path( "config/secrets.env" ).unwrap();
  assert_eq!( secrets.len(), 2 );
  assert_eq!( secrets.get( "PATH_KEY" ).unwrap(), "path-test-value" );
  assert_eq!( secrets.get( "CONFIG_TOKEN" ).unwrap(), "config-token" );
 }

  /// Test new load_secrets_from_absolute_path method
  #[ test ]
  #[ cfg( feature = "secrets" ) ]
  fn test_load_secrets_from_absolute_path()
  {
  use tempfile ::NamedTempFile;

  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();

  // Create temporary file outside workspace
  let temp_file = NamedTempFile ::new().unwrap();
  let secret_content = "ABS_KEY=absolute-value\nEXTERNAL_TOKEN=external-token";
  fs ::write( &temp_file, secret_content ).unwrap();

  // Test absolute path loading
  let secrets = workspace.load_secrets_from_absolute_path( temp_file.path() ).unwrap();
  assert_eq!( secrets.len(), 2 );
  assert_eq!( secrets.get( "ABS_KEY" ).unwrap(), "absolute-value" );
  assert_eq!( secrets.get( "EXTERNAL_TOKEN" ).unwrap(), "external-token" );
 }

  /// Test helper methods
  #[ test ]
  #[ cfg( feature = "secrets" ) ]
  fn test_helper_methods()
  {
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();

  // Initially no files
  assert!( workspace.list_secrets_files().unwrap().is_empty() );
  assert!( !workspace.secrets_file_exists( "test.env" ) );

  // Create a file
  let secret_content = "HELPER_KEY=helper-value";
  fs ::write( workspace.secret_file( "helper.env" ), secret_content ).unwrap();

  // Now should be detected
  let files = workspace.list_secrets_files().unwrap();
  assert_eq!( files.len(), 1 );
  assert!( files.contains( &"helper.env".to_string() ) );
  assert!( workspace.secrets_file_exists( "helper.env" ) );

  // Test path resolution
  let path = workspace.resolve_secrets_path( "test.env" );
  assert!( path.ends_with( "secret/test.env" ) );
 }

  /// Test debug helper method
  #[ test ]
  #[ cfg( feature = "secrets" ) ]
  fn test_debug_helper_method()
  {
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();

  // Create a test file
  let debug_content = "DEBUG_KEY=debug-value\nTEST_TOKEN=test-token";
  fs ::write( workspace.secret_file( "debug.env" ), debug_content ).unwrap();

  // Test debug loading
  let secrets = workspace.load_secrets_with_debug( "debug.env" ).unwrap();
  assert_eq!( secrets.len(), 2 );
  assert_eq!( secrets.get( "DEBUG_KEY" ).unwrap(), "debug-value" );

  // Test debug with nonexistent file
  let result = workspace.load_secrets_with_debug( "nonexistent.env" );
  assert!( result.is_err() );
 }

  /// Test secure versions of new methods
  #[ test ]
  #[ cfg( feature = "secure" ) ]
  fn test_secure_path_methods()
  {
  use secrecy ::ExposeSecret;

  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();

  // Create nested structure for path test
  let secure_dir = workspace.join( "secure" );
  fs ::create_dir_all( &secure_dir ).unwrap();

  let secret_content = "SECURE_PATH_KEY=secure-path-value";
  fs ::write( secure_dir.join( "secrets.env" ), secret_content ).unwrap();

  // Test secure path loading
  let secrets = workspace.load_secrets_from_path_secure( "secure/secrets.env" ).unwrap();
  assert_eq!( secrets.len(), 1 );
  let secure_value = secrets.get( "SECURE_PATH_KEY" ).unwrap();
  assert_eq!( secure_value.expose_secret(), "secure-path-value" );
 }

  /// Test secure debug method
  #[ test ]
  #[ cfg( feature = "secure" ) ]
  fn test_secure_debug_method()
  {
  use secrecy ::ExposeSecret;

  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();

  // Create test file
  let debug_content = "SECURE_DEBUG_KEY=secure-debug-value";
  fs ::write( workspace.secret_file( "secure-debug.env" ), debug_content ).unwrap();

  // Test secure debug loading
  let secrets = workspace.load_secrets_with_debug_secure( "secure-debug.env" ).unwrap();
  assert_eq!( secrets.len(), 1 );
  let secure_value = secrets.get( "SECURE_DEBUG_KEY" ).unwrap();
  assert_eq!( secure_value.expose_secret(), "secure-debug-value" );
 }
}

/// Test Phase 3 : Error message improvements
#[ cfg( feature = "secrets" ) ]
mod phase_3_error_improvements
{
  use super :: *;

  /// Test error messages include both parameter and resolved path
  #[ test ]
  #[ cfg( feature = "secrets" ) ]
  fn test_error_messages_include_paths()
  {
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();

  // Test file not found error
  let result = workspace.load_secrets_from_file( "test.env" );
  assert!( result.is_err() );

  let error_msg = result.unwrap_err().to_string();
  assert!( error_msg.contains( "test.env" ) ); // Original parameter
  // Check for path components instead of exact path (cross-platform)
  assert!( error_msg.contains( "secret" ) );
  assert!( error_msg.contains( "test.env" ) );

  // Test path method error
  let path_result = workspace.load_secrets_from_path( "config/missing.env" );
  assert!( path_result.is_err() );

  let path_error_msg = path_result.unwrap_err().to_string();
  assert!( path_error_msg.contains( "config/missing.env" ) ); // Original parameter
  assert!( path_error_msg.contains( "Failed to read" ) || path_error_msg.contains( "Absolute path" ) ); // Error explanation
 }

  /// Test path-like parameter warnings
  #[ test ]
  #[ cfg( feature = "secrets" ) ]
  fn test_path_warnings_emitted()
  {
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();

  // These should emit warnings (we can't easily capture stderr in tests)
  // but we can verify the methods still work and return proper errors
  let path_like_params = vec![
   "config/secrets.env",
   "lib\\project\\secrets.env",
   "../secrets.env",
 ];

  for param in path_like_params
  {
   let result = workspace.load_secrets_from_file( param );
   assert!( result.is_err() );

   let error_msg = result.unwrap_err().to_string();
   assert!( error_msg.contains( "not found" ) );
 }
 }
}

/// Test Phase 4 : Backward compatibility
#[ cfg( feature = "secrets" ) ]
mod phase_4_backward_compatibility
{
  use super :: *;

  /// Test existing API methods still work
  #[ test ]
  #[ cfg( feature = "secrets" ) ]
  fn test_existing_methods_still_work()
  {
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();

  // Create test file
  let secret_content = "COMPAT_KEY=compatibility-value\nOLD_TOKEN=old-token-123";
  fs ::write( workspace.secret_file( "compat.env" ), secret_content ).unwrap();

  // Test existing methods still work (just with better error handling)
  let secrets = workspace.load_secrets_from_file( "compat.env" ).unwrap();
  assert_eq!( secrets.len(), 2 );
  assert_eq!( secrets.get( "COMPAT_KEY" ).unwrap(), "compatibility-value" );

  let key = workspace.load_secret_key( "COMPAT_KEY", "compat.env" ).unwrap();
  assert_eq!( key, "compatibility-value" );

  // Test secure versions still work
  #[ cfg( feature = "secure" ) ]
  {
   use secrecy ::ExposeSecret;

   let secure_secrets = workspace.load_secrets_secure( "compat.env" ).unwrap();
   assert_eq!( secure_secrets.len(), 2 );
   let secure_key = secure_secrets.get( "COMPAT_KEY" ).unwrap();
   assert_eq!( secure_key.expose_secret(), "compatibility-value" );

   let secure_single = workspace.load_secret_key_secure( "COMPAT_KEY", "compat.env" ).unwrap();
   assert_eq!( secure_single.expose_secret(), "compatibility-value" );
 }
 }

  /// Test environment fallback still works
  #[ test ]
  #[ cfg( feature = "secrets" ) ]
  fn test_environment_fallback_compatibility()
  {
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();

  // Set environment variable
  std ::env ::set_var( "TEST_FALLBACK_KEY", "env-fallback-value" );

  // Should fallback to environment when file doesn't exist
  let key = workspace.load_secret_key( "TEST_FALLBACK_KEY", "nonexistent.env" ).unwrap();
  assert_eq!( key, "env-fallback-value" );

  // Clean up
  std ::env ::remove_var( "TEST_FALLBACK_KEY" );
 }
}

/// Integration tests combining multiple features
#[ cfg( feature = "secrets" ) ]
mod integration_tests
{
  use super :: *;

  /// Test the exact scenario from task description
  #[ test ]
  #[ cfg( feature = "secrets" ) ]
  fn test_api_huggingface_scenario_resolution()
  {
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();

  // Recreate the exact scenario from task 021
  let lib_dir = workspace.join( "lib/llm_tools/secret" );
  fs ::create_dir_all( &lib_dir ).unwrap();

  let secret_content = "API_KEY=huggingface-api-key\nTOKEN=hf-token-123";
  let secrets_file = lib_dir.join( "-secrets.sh" );
  fs ::write( &secrets_file, secret_content ).unwrap();

  // Before: This would silently return empty HashMap
  // After: This returns helpful error with suggestions
  let old_attempt = workspace.load_secrets_from_file( "lib/llm_tools/secret/-secrets.sh" );
  assert!( old_attempt.is_err() );
  let error_msg = old_attempt.unwrap_err().to_string();
  assert!( error_msg.contains( "not found" ) );

  // Now developer can use correct method
  let correct_result = workspace.load_secrets_from_path( "lib/llm_tools/secret/-secrets.sh" ).unwrap();
  assert_eq!( correct_result.len(), 2 );
  assert_eq!( correct_result.get( "API_KEY" ).unwrap(), "huggingface-api-key" );
  assert_eq!( correct_result.get( "TOKEN" ).unwrap(), "hf-token-123" );
 }

  /// Test all error conditions produce helpful messages
  #[ test ]
  #[ cfg( feature = "secrets" ) ]
  fn test_comprehensive_error_scenarios()
  {
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();

  // Create some available files for context
  fs ::create_dir_all( workspace.secret_dir() ).unwrap();
  fs ::write( workspace.secret_file( "available1.env" ), "KEY1=value1" ).unwrap();
  fs ::write( workspace.secret_file( "available2.env" ), "KEY2=value2" ).unwrap();

  // Test various error scenarios
  let file_error_scenarios = vec![
   // ( method_description, result, expected_error_contains )
   ( "nonexistent file", workspace.load_secrets_from_file( "missing.env" ), vec![ "not found", "Available files: ", "available1.env", "available2.env" ] ),
   ( "path-like parameter", workspace.load_secrets_from_file( "config/secrets.env" ), vec![ "not found", "config/secrets.env" ] ),
   ( "path method missing path", workspace.load_secrets_from_path( "missing/path.env" ), vec![ "Failed to read", "missing/path.env" ] ),
 ];

  for ( description, result, expected_parts ) in file_error_scenarios
  {
   println!( "Testing error scenario: {}", description );
   assert!( result.is_err(), "Expected error for: {}", description );

   let error_msg = result.unwrap_err().to_string();
   for expected in expected_parts
   {
  assert!( error_msg.contains( expected ),
   "Error message for '{}' should contain '{}'. Got: {}",
   description, expected, error_msg );
 }
 }

  // Test load_secret_key separately since it returns String, not HashMap
  let key_result = workspace.load_secret_key( "API_KEY", "missing.env" );
  assert!( key_result.is_err() );
  let key_error_msg = key_result.unwrap_err().to_string();
  // Check for components instead of exact path (cross-platform)
  for expected in vec![ "API_KEY not found", "missing.env" ]
  {
   assert!( key_error_msg.contains( expected ),
  "load_secret_key error message should contain '{}'. Got: {}",
  expected, key_error_msg );
 }
 }
}

#[ cfg( not( feature = "secrets" ) ) ]
fn main() 
{
  println!( "These tests require the 'secrets' feature" );
}