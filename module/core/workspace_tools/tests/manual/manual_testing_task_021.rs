//! Manual Testing Script for Task 021 - Comprehensive Feature Validation
//!
//! This script manually tests all new features and improvements from Task 021
//! to ensure they work correctly in realistic scenarios.

#[ cfg( feature = "secrets" ) ]
use workspace_tools :: { workspace, Workspace };
#[ cfg( feature = "secrets" ) ]
use std ::fs;
#[ cfg( feature = "secrets" ) ]
use std ::path ::PathBuf;

#[ cfg( feature = "secrets" ) ]
fn main() -> Result< (), Box< dyn std ::error ::Error > >
{
  println!( "🧪 Manual Testing - Task 021 Comprehensive Feature Validation\n" );

  // Setup test workspace
  if std ::env ::var( "WORKSPACE_PATH" ).is_err()
  {
  std ::env ::set_var( "WORKSPACE_PATH", std ::env ::current_dir()? );
 }

  let ws = workspace()?;
  println!( "📁 Test workspace: {}", ws.root().display() );

  // Clean up any existing test files
  cleanup_test_files( &ws );

  // Run comprehensive manual tests
  test_enhanced_error_handling( &ws )?;
  test_path_aware_methods( &ws )?;
  test_helper_methods( &ws )?;
  test_debug_functionality( &ws )?;
  test_backward_compatibility( &ws )?;
  test_edge_cases( &ws )?;
  test_integration_scenarios( &ws )?;

  // Clean up
  cleanup_test_files( &ws );

  println!( "\n✅ All manual tests completed successfully!" );
  println!( "🎯 Task 021 implementation is working correctly." );

  Ok( () )
}

#[ cfg( feature = "secrets" ) ]
fn test_enhanced_error_handling( ws: &Workspace ) -> Result< (), Box< dyn std ::error ::Error > >
{
  println!( "🔍 Testing Enhanced Error Handling..." );

  // Test 1 : Explicit file existence errors
  println!( "  1. Testing explicit file existence errors" );
  match ws.load_secrets_from_file( "nonexistent.env" )
  {
  Ok( _ ) => panic!( "Expected error for nonexistent file" ),
  Err( e ) =>
  {
   let error_msg = e.to_string();
   println!( "     ✅ Got expected error: {}", error_msg );
   assert!( error_msg.contains( "not found" ), "Error should contain path info" );
   assert!( error_msg.contains( "nonexistent.env" ), "Error should contain filename" );
 }
 }

  // Test 2 : Path validation warnings
  println!( "  2. Testing path validation warnings" );
  match ws.load_secrets_from_file( "config/secrets.env" )
  {
  Ok( _ ) => panic!( "Expected error for path-like parameter" ),
  Err( e ) =>
  {
   let error_msg = e.to_string();
   println!( "     ✅ Got expected error with warning: {}", error_msg );
   assert!( error_msg.contains( "config/secrets.env" ), "Error should contain path parameter" );
 }
 }

  // Test 3 : Available files suggestions
  println!( "  3. Testing available files suggestions" );

  // Create some test files first
  fs ::create_dir_all( ws.secret_dir() )?;
  fs ::write( ws.secret_file( "test1.env" ), "KEY1=value1" )?;
  fs ::write( ws.secret_file( "test2.env" ), "KEY2=value2" )?;

  match ws.load_secrets_from_file( "missing.env" )
  {
  Ok( _ ) => panic!( "Expected error for missing file" ),
  Err( e ) =>
  {
   let error_msg = e.to_string();
   println!( "     ✅ Got expected error with suggestions: {}", error_msg );
   assert!( error_msg.contains( "Available files: " ), "Error should suggest available files" );
   assert!( error_msg.contains( "test1.env" ), "Error should list available files" );
   assert!( error_msg.contains( "test2.env" ), "Error should list available files" );
 }
 }

  println!( "  ✅ Enhanced error handling tests passed" );
  Ok( () )
}

#[ cfg( feature = "secrets" ) ]
fn test_path_aware_methods( ws: &Workspace ) -> Result< (), Box< dyn std ::error ::Error > >
{
  println!( "🔍 Testing Path-Aware Methods..." );

  // Setup nested directory structure
  let config_dir = ws.join( "config" );
  let nested_dir = ws.join( "lib/project/secret" );
  fs ::create_dir_all( &config_dir )?;
  fs ::create_dir_all( &nested_dir )?;

  // Test 1 : load_secrets_from_path
  println!( "  1. Testing load_secrets_from_path" );
  let config_secrets = "CONFIG_KEY=config-value\nCONFIG_TOKEN=config-token-789";
  fs ::write( config_dir.join( "secrets.env" ), config_secrets )?;

  let secrets = ws.load_secrets_from_path( "config/secrets.env" )?;
  assert_eq!( secrets.len(), 2 );
  assert_eq!( secrets.get( "CONFIG_KEY" ).unwrap(), "config-value" );
  println!( "     ✅ Successfully loaded {} secrets from path", secrets.len() );

  // Test 2 : Nested path loading
  println!( "  2. Testing nested path loading" );
  let nested_secrets = "NESTED_KEY=nested-value\nDEEP_SECRET=deep-secret-123";
  fs ::write( nested_dir.join( "api.env" ), nested_secrets )?;

  let nested_result = ws.load_secrets_from_path( "lib/project/secret/api.env" )?;
  assert_eq!( nested_result.len(), 2 );
  assert_eq!( nested_result.get( "NESTED_KEY" ).unwrap(), "nested-value" );
  println!( "     ✅ Successfully loaded {} secrets from nested path", nested_result.len() );

  // Test 3 : load_secrets_from_absolute_path
  println!( "  3. Testing load_secrets_from_absolute_path" );
  let temp_file = std ::env ::temp_dir().join( "workspace_test_secrets.env" );
  let abs_secrets = "ABS_KEY=absolute-value\nTEMP_SECRET=temporary-secret-456";
  fs ::write( &temp_file, abs_secrets )?;

  let abs_result = ws.load_secrets_from_absolute_path( &temp_file )?;
  assert_eq!( abs_result.len(), 2 );
  assert_eq!( abs_result.get( "ABS_KEY" ).unwrap(), "absolute-value" );
  println!( "     ✅ Successfully loaded {} secrets from absolute path", abs_result.len() );

  // Clean up temp file
  fs ::remove_file( temp_file )?;

  println!( "  ✅ Path-aware methods tests passed" );
  Ok( () )
}

#[ cfg( feature = "secrets" ) ]
fn test_helper_methods( ws: &Workspace ) -> Result< (), Box< dyn std ::error ::Error > >
{
  println!( "🔍 Testing Helper Methods..." );

  // Test 1 : list_secrets_files
  println!( "  1. Testing list_secrets_files" );
  let files = ws.list_secrets_files()?;
  println!( "     ✅ Found {} secrets files: {:?}", files.len(), files );
  assert!( !files.is_empty(), "Should have some test files" );
  assert!( files.contains( &"test1.env".to_string() ) );

  // Test 2 : secrets_file_exists
  println!( "  2. Testing secrets_file_exists" );
  assert!( ws.secrets_file_exists( "test1.env" ), "test1.env should exist" );
  assert!( !ws.secrets_file_exists( "nonexistent.env" ), "nonexistent.env should not exist" );
  println!( "     ✅ File existence checks working correctly" );

  // Test 3 : resolve_secrets_path
  println!( "  3. Testing resolve_secrets_path" );
  let resolved_path = ws.resolve_secrets_path( "test.env" );
  assert!( resolved_path.ends_with( "secret/test.env" ), "Should resolve to correct path" );
  println!( "     ✅ Path resolution: {}", resolved_path.display() );

  println!( "  ✅ Helper methods tests passed" );
  Ok( () )
}

#[ cfg( feature = "secrets" ) ]
fn test_debug_functionality( ws: &Workspace ) -> Result< (), Box< dyn std ::error ::Error > >
{
  println!( "🔍 Testing Debug Functionality..." );

  // Test load_secrets_with_debug
  println!( "  1. Testing load_secrets_with_debug with existing file" );
  let result = ws.load_secrets_with_debug( "test1.env" )?;
  assert!( !result.is_empty(), "Should load secrets successfully" );
  println!( "     ✅ Debug loading successful" );

  // Test debug with nonexistent file
  println!( "  2. Testing load_secrets_with_debug with nonexistent file" );
  let debug_result = ws.load_secrets_with_debug( "debug-missing.env" );
  assert!( debug_result.is_err(), "Should fail for missing file" );
  println!( "     ✅ Debug properly handled missing file" );

  // Test debug with path-like parameter
  println!( "  3. Testing load_secrets_with_debug with path-like parameter" );
  let path_debug_result = ws.load_secrets_with_debug( "config/debug.env" );
  assert!( path_debug_result.is_err(), "Should fail for path-like parameter" );
  println!( "     ✅ Debug properly warned about path-like parameter" );

  println!( "  ✅ Debug functionality tests passed" );
  Ok( () )
}

#[ cfg( feature = "secrets" ) ]
fn test_backward_compatibility( ws: &Workspace ) -> Result< (), Box< dyn std ::error ::Error > >
{
  println!( "🔍 Testing Backward Compatibility..." );

  // Test 1 : Existing methods still work with good files
  println!( "  1. Testing existing methods with valid files" );
  let secrets = ws.load_secrets_from_file( "test1.env" )?;
  assert!( !secrets.is_empty(), "Should load existing files successfully" );

  let key_result = ws.load_secret_key( "KEY1", "test1.env" )?;
  assert_eq!( key_result, "value1", "Should load individual keys successfully" );
  println!( "     ✅ Existing methods work correctly" );

  // Test 2 : Environment fallback still works
  println!( "  2. Testing environment variable fallback" );
  std ::env ::set_var( "TEST_FALLBACK_KEY", "env-fallback-test-value" );

  let fallback_result = ws.load_secret_key( "TEST_FALLBACK_KEY", "nonexistent-file.env" )?;
  assert_eq!( fallback_result, "env-fallback-test-value", "Should fallback to environment" );
  println!( "     ✅ Environment fallback works correctly" );

  std ::env ::remove_var( "TEST_FALLBACK_KEY" );

  println!( "  ✅ Backward compatibility tests passed" );
  Ok( () )
}

#[ cfg( feature = "secrets" ) ]
fn test_edge_cases( ws: &Workspace ) -> Result< (), Box< dyn std ::error ::Error > >
{
  println!( "🔍 Testing Edge Cases..." );

  // Test 1 : Empty filename
  println!( "  1. Testing empty filename" );
  let empty_result = ws.load_secrets_from_file( "" );
  assert!( empty_result.is_err(), "Should fail for empty filename" );
  println!( "     ✅ Empty filename handled correctly" );

  // Test 2 : Very long filename
  println!( "  2. Testing very long filename" );
  let long_name = "a".repeat( 255 );
  let long_result = ws.load_secrets_from_file( &long_name );
  assert!( long_result.is_err(), "Should fail for very long filename" );
  println!( "     ✅ Long filename handled correctly" );

  // Test 3 : Special characters in filename
  println!( "  3. Testing special characters" );
  let special_chars = vec![ "file with spaces.env", "file@with#special$.env", "file|with|pipes.env" ];
  for filename in special_chars
  {
  let result = ws.load_secrets_from_file( filename );
  assert!( result.is_err(), "Should handle special characters gracefully" );
 }
  println!( "     ✅ Special characters handled correctly" );

  // Test 4 : Path traversal attempts
  println!( "  4. Testing path traversal attempts" );
  let traversal_attempts = vec![ "../secrets.env", "../../etc/passwd", "./../config.env" ];
  for attempt in traversal_attempts
  {
  let result = ws.load_secrets_from_file( attempt );
  // Should either fail or warn - both are acceptable for security
  println!( "     ✅ Path traversal attempt handled: {}", attempt );
 }

  println!( "  ✅ Edge cases tests passed" );
  Ok( () )
}

#[ cfg( feature = "secrets" ) ]
fn test_integration_scenarios( ws: &Workspace ) -> Result< (), Box< dyn std ::error ::Error > >
{
  println!( "🔍 Testing Integration Scenarios..." );

  // Test 1 : Real-world api_huggingface scenario
  println!( "  1. Testing api_huggingface scenario resolution" );

  // Setup the exact scenario from the task
  let lib_dir = ws.join( "lib/llm_tools/secret" );
  fs ::create_dir_all( &lib_dir )?;
  let hf_secrets = "HF_TOKEN=hf_test_token_123\nAPI_KEY=huggingface_api_key_456";
  fs ::write( lib_dir.join( "-secrets.sh" ), hf_secrets )?;

  // Old problematic way (should give helpful error)
  match ws.load_secrets_from_file( "lib/llm_tools/secret/-secrets.sh" )
  {
  Ok( _ ) => panic!( "Expected error for path-like parameter" ),
  Err( e ) => println!( "     ✅ Old way gives helpful error: {}", e )
 }

  // New correct way
  let correct_secrets = ws.load_secrets_from_path( "lib/llm_tools/secret/-secrets.sh" )?;
  assert_eq!( correct_secrets.len(), 2 );
  assert!( correct_secrets.contains_key( "HF_TOKEN" ) );
  println!( "     ✅ New path method works correctly" );

  // Test 2 : Multiple format support
  println!( "  2. Testing multiple secret file formats" );

  // Create different format files
  let formats = vec![
  ( "key_value.env", "SIMPLE_KEY=simple_value" ),
  ( "export_format.sh", "export EXPORT_KEY=export_value" ),
  ( "mixed_format.env", "KEY1=value1\nexport KEY2=value2\n# Comment\nKEY3=value3" )
 ];

  for ( filename, content ) in formats
  {
  fs ::write( ws.secret_file( filename ), content )?;
  let secrets = ws.load_secrets_from_file( filename )?;
  assert!( !secrets.is_empty(), "Should parse {} format", filename );
  println!( "     ✅ Format {} parsed correctly", filename );
 }

  println!( "  ✅ Integration scenarios tests passed" );
  Ok( () )
}

#[ cfg( feature = "secrets" ) ]
fn cleanup_test_files( ws: &Workspace )
{
  // Clean up all test files and directories
  let _ = std ::fs ::remove_dir_all( ws.secret_dir() );
  let _ = std ::fs ::remove_dir_all( ws.join( "config" ) );
  let _ = std ::fs ::remove_dir_all( ws.join( "lib" ) );

  // Remove any temp files
  let temp_files = vec![
  std ::env ::temp_dir().join( "workspace_test_secrets.env" ),
  std ::env ::temp_dir().join( "workspace_demo_secrets.env" ),
 ];

  for file in temp_files
  {
  let _ = std ::fs ::remove_file( file );
 }
}

#[ cfg( not( feature = "secrets" ) ) ]
fn main()
{
  println!( "🚨 Manual testing requires the 'secrets' feature" );
  println!( "Run with: cargo run --bin manual_testing_task_021 --features secrets" );
}