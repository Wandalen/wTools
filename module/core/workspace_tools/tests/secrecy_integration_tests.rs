//! Secrecy Integration Tests
//!
//! Tests for memory-safe secret handling using the secrecy crate.
//! These tests define the expected behavior of secure secret loading
//! and integration with existing `workspace_tools` functionality.

#![ cfg( feature = "secure" ) ]

use std::fs;
use workspace_tools::testing;
use secrecy::ExposeSecret;

/// Test basic secure secret loading from file
#[ test ]
#[ cfg( feature = "secure" ) ]
fn test_load_secrets_secure_basic()
{
  let ( _temp_dir, workspace ) = testing::create_test_workspace_with_structure();
  
  // Create a secret file with test data
  let secret_content = "API_KEY=secret-key-123\nDATABASE_URL=postgresql://localhost/testdb";
  let secret_file = workspace.secret_file( "-test-secure.env" );
  fs::write( &secret_file, secret_content ).unwrap();
  
  // Load secrets securely - should return HashMap<String, SecretString>
  let secrets = workspace.load_secrets_secure( "-test-secure.env" ).unwrap();
  
  // Verify we can access keys but values are wrapped in SecretString
  assert!( secrets.contains_key( "API_KEY" ) );
  assert!( secrets.contains_key( "DATABASE_URL" ) );
  assert_eq!( secrets.len(), 2 );
  
  // Values should require explicit access via expose_secret()
  let api_key = secrets.get( "API_KEY" ).unwrap();
  assert_eq!( api_key.expose_secret(), "secret-key-123" );
  
  let db_url = secrets.get( "DATABASE_URL" ).unwrap();
  assert_eq!( db_url.expose_secret(), "postgresql://localhost/testdb" );
}

/// Test secure loading of individual secret key
#[ test ]
#[ cfg( feature = "secure" ) ]
fn test_load_secret_key_secure()
{
  let ( _temp_dir, workspace ) = testing::create_test_workspace_with_structure();
  
  let secret_content = "TOKEN=secure-token-456\nPASSWORD=super-secret";
  let secret_file = workspace.secret_file( "-key-test.env" );
  fs::write( &secret_file, secret_content ).unwrap();
  
  // Load individual key securely
  let token = workspace.load_secret_key_secure( "TOKEN", "-key-test.env" ).unwrap();
  
  // Should return SecretString, requiring explicit access
  assert_eq!( token.expose_secret(), "secure-token-456" );
  
  // Test key not found
  let result = workspace.load_secret_key_secure( "NONEXISTENT", "-key-test.env" );
  assert!( result.is_err() );
}

/// Test secure environment variable loading
#[ test ]
#[ cfg( feature = "secure" ) ]
fn test_env_secret()
{
  let ( _temp_dir, workspace ) = testing::create_test_workspace_with_structure();
  
  // Set test environment variable
  std::env::set_var( "TEST_SECRET_ENV", "env-secret-value" );
  
  // Load environment variable securely
  let env_secret = workspace.env_secret( "TEST_SECRET_ENV" ).unwrap();
  assert_eq!( env_secret.expose_secret(), "env-secret-value" );
  
  // Test missing environment variable
  let missing = workspace.env_secret( "MISSING_ENV_VAR" );
  assert!( missing.is_none() );
  
  // Cleanup
  std::env::remove_var( "TEST_SECRET_ENV" );
}

/// Test secure secret loading with fallback to environment
#[ test ]
#[ cfg( feature = "secure" ) ]
fn test_load_secret_key_secure_with_env_fallback()
{
  let ( _temp_dir, workspace ) = testing::create_test_workspace_with_structure();
  
  // Set environment variable as fallback
  std::env::set_var( "FALLBACK_SECRET", "fallback-value" );
  
  // Try to load from non-existent file, should fallback to env
  let secret = workspace.load_secret_key_secure( "FALLBACK_SECRET", "-missing-file.env" ).unwrap();
  assert_eq!( secret.expose_secret(), "fallback-value" );
  
  // Cleanup
  std::env::remove_var( "FALLBACK_SECRET" );
}

/// Test integration with existing secret loading (backward compatibility)
#[ test ]
#[ cfg( feature = "secure" ) ]
fn test_secure_and_regular_api_compatibility()
{
  let ( _temp_dir, workspace ) = testing::create_test_workspace_with_structure();
  
  let secret_content = "COMPAT_KEY=compatibility-test";
  let secret_file = workspace.secret_file( "-compat-test.env" );
  fs::write( &secret_file, secret_content ).unwrap();
  
  // Load with both APIs
  let regular_secrets = workspace.load_secrets_from_file( "-compat-test.env" ).unwrap();
  let secure_secrets = workspace.load_secrets_secure( "-compat-test.env" ).unwrap();
  
  // Both should find the same keys
  assert!( regular_secrets.contains_key( "COMPAT_KEY" ) );
  assert!( secure_secrets.contains_key( "COMPAT_KEY" ) );
  
  // Values should be equivalent when exposed
  let regular_value = regular_secrets.get( "COMPAT_KEY" ).unwrap();
  let secure_value = secure_secrets.get( "COMPAT_KEY" ).unwrap();
  assert_eq!( regular_value, secure_value.expose_secret() );
}

/// Test export statement format with secure loading
#[ test ]
#[ cfg( feature = "secure" ) ]
fn test_secure_loading_with_export_format()
{
  let ( _temp_dir, workspace ) = testing::create_test_workspace_with_structure();
  
  let secret_content = r#"
export SECURE_API_KEY="exported-secret-123"
REGULAR_KEY=regular-value
export DATABASE_PASSWORD='quoted-password'
"#;
  let secret_file = workspace.secret_file( "-export-test.env" );
  fs::write( &secret_file, secret_content ).unwrap();
  
  let secrets = workspace.load_secrets_secure( "-export-test.env" ).unwrap();
  
  assert_eq!( secrets.len(), 3 );
  assert_eq!( secrets.get( "SECURE_API_KEY" ).unwrap().expose_secret(), "exported-secret-123" );
  assert_eq!( secrets.get( "REGULAR_KEY" ).unwrap().expose_secret(), "regular-value" );
  assert_eq!( secrets.get( "DATABASE_PASSWORD" ).unwrap().expose_secret(), "quoted-password" );
}

/// Test memory safety - `SecretString` should not appear in debug output
#[ test ]
#[ cfg( feature = "secure" ) ]
fn test_secret_string_debug_safety()
{
  let ( _temp_dir, workspace ) = testing::create_test_workspace_with_structure();
  
  let secret_content = "DEBUG_TEST=sensitive-data";
  let secret_file = workspace.secret_file( "-debug-test.env" );
  fs::write( &secret_file, secret_content ).unwrap();
  
  let secrets = workspace.load_secrets_secure( "-debug-test.env" ).unwrap();
  let secret = secrets.get( "DEBUG_TEST" ).unwrap();
  
  // Debug output should not contain the actual secret value
  let debug_output = format!( "{secret:?}" );
  assert!( !debug_output.contains( "sensitive-data" ) );
  
  // But explicit access should work
  assert_eq!( secret.expose_secret(), "sensitive-data" );
}

/// Test error handling in secure API
#[ test ]
#[ cfg( feature = "secure" ) ]
fn test_secure_error_handling()
{
  let ( _temp_dir, workspace ) = testing::create_test_workspace_with_structure();
  
  // Test loading from non-existent file - new behavior returns explicit error
  let result = workspace.load_secrets_secure( "-nonexistent.env" );
  assert!( result.is_err() ); // Now returns explicit error instead of empty HashMap
  let error_msg = result.unwrap_err().to_string();
  assert!( error_msg.contains( "not found at" ) );
  
  // Test loading specific key from non-existent file (no env fallback)
  let result = workspace.load_secret_key_secure( "MISSING_KEY", "-nonexistent.env" );
  assert!( result.is_err() );
}