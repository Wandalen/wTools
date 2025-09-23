#![ allow( clippy ::uninlined_format_args, clippy ::doc_markdown ) ]

//! Manual Validation Tests for Task 021
//!
//! These tests manually validate all the new functionality works correctly
//! in realistic scenarios.

#[ cfg( feature = "secrets" ) ]
use workspace_tools ::testing;
#[ cfg( feature = "secrets" ) ]
use std ::fs;

/// Manual test to verify enhanced error handling works in practice
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_manual_enhanced_error_handling()
{
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();

  // Test explicit file existence errors
  match workspace.load_secrets_from_file( "nonexistent.env" )
  {
  Ok( _ ) => panic!( "Expected error for nonexistent file" ),
  Err( e ) =>
  {
   let error_msg = e.to_string();
   assert!( error_msg.contains( "not found at" ), "Error should contain path info" );
   assert!( error_msg.contains( "nonexistent.env" ), "Error should contain filename" );
 }
 }

  // Test path validation warnings
  match workspace.load_secrets_from_file( "config/secrets.env" )
  {
  Ok( _ ) => panic!( "Expected error for path-like parameter" ),
  Err( e ) =>
  {
   let error_msg = e.to_string();
   assert!( error_msg.contains( "config/secrets.env" ), "Error should contain path parameter" );
 }
 }

  // Test available files suggestions
  fs ::create_dir_all( workspace.secret_dir() ).unwrap();
  fs ::write( workspace.secret_file( "available1.env" ), "KEY1=value1" ).unwrap();
  fs ::write( workspace.secret_file( "available2.env" ), "KEY2=value2" ).unwrap();

  match workspace.load_secrets_from_file( "missing.env" )
  {
  Ok( _ ) => panic!( "Expected error for missing file" ),
  Err( e ) =>
  {
   let error_msg = e.to_string();
   assert!( error_msg.contains( "Available files: " ), "Error should suggest available files" );
   assert!( error_msg.contains( "available1.env" ), "Error should list available files" );
   assert!( error_msg.contains( "available2.env" ), "Error should list available files" );
 }
 }
}

/// Manual test to verify path-aware methods work correctly
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_manual_path_aware_methods()
{
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();

  // Setup nested directory structure
  let config_dir = workspace.join( "config" );
  let nested_dir = workspace.join( "lib/project/.secret" );
  fs ::create_dir_all( &config_dir ).unwrap();
  fs ::create_dir_all( &nested_dir ).unwrap();

  // Test load_secrets_from_path
  let config_secrets = "CONFIG_KEY=config-value\nCONFIG_TOKEN=config-token-789";
  fs ::write( config_dir.join( "secrets.env" ), config_secrets ).unwrap();

  let secrets = workspace.load_secrets_from_path( "config/secrets.env" ).unwrap();
  assert_eq!( secrets.len(), 2 );
  assert_eq!( secrets.get( "CONFIG_KEY" ).unwrap(), "config-value" );

  // Test nested path loading (the api_huggingface scenario)
  let nested_secrets = "HF_TOKEN=hf_test_token_123\nAPI_KEY=huggingface_api_key_456";
  fs ::write( nested_dir.join( "-secrets.sh" ), nested_secrets ).unwrap();

  let nested_result = workspace.load_secrets_from_path( "lib/project/.secret/-secrets.sh" ).unwrap();
  assert_eq!( nested_result.len(), 2 );
  assert_eq!( nested_result.get( "HF_TOKEN" ).unwrap(), "hf_test_token_123" );

  // Test load_secrets_from_absolute_path
  let temp_file = std ::env ::temp_dir().join( "workspace_manual_test_secrets.env" );
  let abs_secrets = "ABS_KEY=absolute-value\nTEMP_SECRET=temporary-secret-456";
  fs ::write( &temp_file, abs_secrets ).unwrap();

  let abs_result = workspace.load_secrets_from_absolute_path( &temp_file ).unwrap();
  assert_eq!( abs_result.len(), 2 );
  assert_eq!( abs_result.get( "ABS_KEY" ).unwrap(), "absolute-value" );

  // Clean up
  fs ::remove_file( temp_file ).unwrap();
}

/// Manual test to verify helper methods work correctly
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_manual_helper_methods()
{
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();

  // Setup test files
  fs ::create_dir_all( workspace.secret_dir() ).unwrap();
  fs ::write( workspace.secret_file( "helper1.env" ), "KEY1=value1" ).unwrap();
  fs ::write( workspace.secret_file( "helper2.env" ), "KEY2=value2" ).unwrap();

  // Test list_secrets_files
  let files = workspace.list_secrets_files().unwrap();
  assert!( files.len() >= 2, "Should have at least 2 test files" );
  assert!( files.contains( &"helper1.env".to_string() ) );
  assert!( files.contains( &"helper2.env".to_string() ) );

  // Test secrets_file_exists
  assert!( workspace.secrets_file_exists( "helper1.env" ), "helper1.env should exist" );
  assert!( !workspace.secrets_file_exists( "nonexistent.env" ), "nonexistent.env should not exist" );

  // Test resolve_secrets_path
  let resolved_path = workspace.resolve_secrets_path( "test.env" );
  assert!( resolved_path.ends_with( ".secret/test.env" ), "Should resolve to correct path" );
}

/// Manual test to verify debug functionality provides useful information
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_manual_debug_functionality()
{
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();

  // Setup test file
  fs ::create_dir_all( workspace.secret_dir() ).unwrap();
  fs ::write( workspace.secret_file( "debug_test.env" ), "DEBUG_KEY=debug_value" ).unwrap();

  // Test load_secrets_with_debug with existing file
  let result = workspace.load_secrets_with_debug( "debug_test.env" ).unwrap();
  assert!( !result.is_empty(), "Should load secrets successfully" );
  assert_eq!( result.get( "DEBUG_KEY" ).unwrap(), "debug_value" );

  // Test debug with nonexistent file
  let debug_result = workspace.load_secrets_with_debug( "debug-missing.env" );
  assert!( debug_result.is_err(), "Should fail for missing file" );

  // Test debug with path-like parameter
  let path_debug_result = workspace.load_secrets_with_debug( "config/debug.env" );
  assert!( path_debug_result.is_err(), "Should fail for path-like parameter" );
}

/// Manual test to verify backward compatibility is maintained
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_manual_backward_compatibility()
{
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();

  // Setup test file
  fs ::create_dir_all( workspace.secret_dir() ).unwrap();
  fs ::write( workspace.secret_file( "compat_test.env" ), "COMPAT_KEY=compat_value" ).unwrap();

  // Test existing methods still work with good files
  let secrets = workspace.load_secrets_from_file( "compat_test.env" ).unwrap();
  assert!( !secrets.is_empty(), "Should load existing files successfully" );

  let key_result = workspace.load_secret_key( "COMPAT_KEY", "compat_test.env" ).unwrap();
  assert_eq!( key_result, "compat_value", "Should load individual keys successfully" );

  // Test environment fallback still works
  std ::env ::set_var( "TEST_MANUAL_FALLBACK_KEY", "env-fallback-test-value" );

  let fallback_result = workspace.load_secret_key( "TEST_MANUAL_FALLBACK_KEY", "nonexistent-file.env" ).unwrap();
  assert_eq!( fallback_result, "env-fallback-test-value", "Should fallback to environment" );

  std ::env ::remove_var( "TEST_MANUAL_FALLBACK_KEY" );
}

/// Manual test to verify the exact api_huggingface scenario is resolved
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_manual_api_huggingface_scenario()
{
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();

  // Setup the exact scenario from the task description
  let lib_dir = workspace.join( "lib/llm_tools/.secret" );
  fs ::create_dir_all( &lib_dir ).unwrap();
  let hf_secrets = "HF_TOKEN=hf_test_token_123\nAPI_KEY=huggingface_api_key_456";
  fs ::write( lib_dir.join( "-secrets.sh" ), hf_secrets ).unwrap();

  // Old problematic way (should give helpful error now)
  match workspace.load_secrets_from_file( "lib/llm_tools/.secret/-secrets.sh" )
  {
  Ok( _ ) => panic!( "Expected error for path-like parameter" ),
  Err( e ) =>
  {
   let error_msg = e.to_string();
   // Should get warning about path separators and helpful error about file not found
   assert!( error_msg.contains( "not found at" ), "Should provide helpful error" );
 }
 }

  // New correct way should work perfectly
  let correct_secrets = workspace.load_secrets_from_path( "lib/llm_tools/.secret/-secrets.sh" ).unwrap();
  assert_eq!( correct_secrets.len(), 2 );
  assert_eq!( correct_secrets.get( "HF_TOKEN" ).unwrap(), "hf_test_token_123" );
  assert_eq!( correct_secrets.get( "API_KEY" ).unwrap(), "huggingface_api_key_456" );
}

/// Test secure versions of the new methods work correctly
#[ test ]
#[ cfg( feature = "secure" ) ]
fn test_manual_secure_methods()
{
  use secrecy ::ExposeSecret;

  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();

  // Setup nested structure for secure testing
  let secure_dir = workspace.join( "secure" );
  fs ::create_dir_all( &secure_dir ).unwrap();

  let secret_content = "SECURE_KEY=secure-test-value";
  fs ::write( secure_dir.join( "secrets.env" ), secret_content ).unwrap();

  // Test secure path loading
  let secrets = workspace.load_secrets_from_path_secure( "secure/secrets.env" ).unwrap();
  assert_eq!( secrets.len(), 1 );
  let secure_value = secrets.get( "SECURE_KEY" ).unwrap();
  assert_eq!( secure_value.expose_secret(), "secure-test-value" );

  // Test secure debug method
  fs ::create_dir_all( workspace.secret_dir() ).unwrap();
  fs ::write( workspace.secret_file( "secure_debug.env" ), "DEBUG_SECURE_KEY=debug_secure_value" ).unwrap();

  let debug_secrets = workspace.load_secrets_with_debug_secure( "secure_debug.env" ).unwrap();
  assert_eq!( debug_secrets.len(), 1 );
  let debug_secure_value = debug_secrets.get( "DEBUG_SECURE_KEY" ).unwrap();
  assert_eq!( debug_secure_value.expose_secret(), "debug_secure_value" );
}

#[ cfg( not( feature = "secrets" ) ) ]
fn main() 
{
  println!( "Manual validation tests require the 'secrets' feature" );
}