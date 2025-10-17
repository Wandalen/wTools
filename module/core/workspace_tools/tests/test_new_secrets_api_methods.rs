#![ allow( clippy ::doc_markdown ) ]

//! Test new secrets API methods added in task 021
//!
//! Tests for the new path-aware methods and debug helpers

#[ cfg( feature = "secrets" ) ]
use workspace_tools ::testing;
#[ cfg( feature = "secrets" ) ]
use std ::fs;

/// Test new load_secrets_from_path method
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_load_secrets_from_path()
{
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();

  // Create a nested directory structure with secrets
  let config_dir = workspace.join( "config" );
  fs ::create_dir_all( &config_dir ).unwrap();

  let secret_content = "API_KEY=path-test-key\nDATABASE_URL=path-test-db";
  let config_secrets_file = config_dir.join( "secrets.env" );
  fs ::write( &config_secrets_file, secret_content ).unwrap();

  // Test the new path-based loading
  let secrets = workspace.load_secrets_from_path( "config/secrets.env" ).unwrap();

  assert_eq!( secrets.len(), 2 );
  assert_eq!( secrets.get( "API_KEY" ).unwrap(), "path-test-key" );
  assert_eq!( secrets.get( "DATABASE_URL" ).unwrap(), "path-test-db" );
}

/// Test that path method correctly solves the original issue
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_path_method_solves_developer_issue()
{
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();

  // Create the exact scenario from the task description
  let lib_dir = workspace.join( "lib/llm_tools/secret" );
  fs ::create_dir_all( &lib_dir ).unwrap();

  let secret_content = "API_KEY=correct-nested-secret\nTOKEN=nested-token-123";
  let nested_secret_file = lib_dir.join( "-secrets.sh" );
  fs ::write( &nested_secret_file, secret_content ).unwrap();

  // Now the developer can use the correct method for their intent
  let secrets = workspace.load_secrets_from_path( "lib/llm_tools/secret/-secrets.sh" ).unwrap();

  assert_eq!( secrets.len(), 2 );
  assert_eq!( secrets.get( "API_KEY" ).unwrap(), "correct-nested-secret" );
  assert_eq!( secrets.get( "TOKEN" ).unwrap(), "nested-token-123" );
}

/// Test helper methods work correctly
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_helper_methods()
{
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();

  // Initially no secrets files should exist
  let files = workspace.list_secrets_files().unwrap();
  assert!( files.is_empty() );

  // Check file existence
  assert!( !workspace.secrets_file_exists( "test.env" ) );

  // Get resolved path
  let path = workspace.resolve_secrets_path( "test.env" );
  assert!( path.ends_with( "secret/test.env" ) );

  // Create a secrets file and test again
  let secret_content = "TEST_KEY=test-value";
  let secret_file = workspace.secret_file( "test.env" );
  fs ::write( &secret_file, secret_content ).unwrap();

  // Now should be detected
  let files = workspace.list_secrets_files().unwrap();
  assert_eq!( files.len(), 1 );
  assert!( files.contains( &"test.env".to_string() ) );

  assert!( workspace.secrets_file_exists( "test.env" ) );
}

/// Test absolute path loading
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_load_secrets_from_absolute_path()
{
  use tempfile ::NamedTempFile;

  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();

  // Create a temporary file outside the workspace
  let temp_file = NamedTempFile ::new().unwrap();
  let secret_content = "EXTERNAL_KEY=external-value\nEXTERNAL_TOKEN=external-token";
  fs ::write( &temp_file, secret_content ).unwrap();

  // Test loading from absolute path
  let secrets = workspace.load_secrets_from_absolute_path( temp_file.path() ).unwrap();

  assert_eq!( secrets.len(), 2 );
  assert_eq!( secrets.get( "EXTERNAL_KEY" ).unwrap(), "external-value" );
  assert_eq!( secrets.get( "EXTERNAL_TOKEN" ).unwrap(), "external-token" );
}

/// Test secure versions of new methods
#[ test ]
#[ cfg( feature = "secure" ) ]
fn test_secure_path_methods()
{
  use secrecy ::ExposeSecret;

  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();

  // Create a config directory with secrets
  let config_dir = workspace.join( "config" );
  fs ::create_dir_all( &config_dir ).unwrap();

  let secret_content = "SECURE_KEY=secure-path-value";
  let config_secrets_file = config_dir.join( "secure.env" );
  fs ::write( &config_secrets_file, secret_content ).unwrap();

  // Test secure path loading
  let secrets = workspace.load_secrets_from_path_secure( "config/secure.env" ).unwrap();

  assert_eq!( secrets.len(), 1 );
  let secure_value = secrets.get( "SECURE_KEY" ).unwrap();
  assert_eq!( secure_value.expose_secret(), "secure-path-value" );
}

/// Test error messages for nonexistent paths
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_path_error_messages()
{
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();

  // Test error for nonexistent path
  let result = workspace.load_secrets_from_path( "nonexistent/secrets.env" );
  assert!( result.is_err() );

  let error_msg = result.unwrap_err().to_string();
  assert!( error_msg.contains( "not found at path" ) );
  assert!( error_msg.contains( "nonexistent/secrets.env" ) );
  assert!( error_msg.contains( "resolved to: " ) );
}

#[ cfg( not( feature = "secrets" ) ) ]
fn main() 
{
  println!( "This test requires the 'secrets' feature" );
}