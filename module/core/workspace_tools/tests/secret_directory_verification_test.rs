//! Secret Directory Verification Tests
//!
//! These tests verify that the secret management functionality correctly uses
//! the `secret` directory (not `.secrets`) and properly handles secret files.

#![ allow( unused_imports ) ]

use workspace_tools ::
{
  Workspace,
  WorkspaceError,
  testing ::create_test_workspace_with_structure,
};
use std ::
{
  fs,
  collections ::HashMap,
};

/// Test that `secret_dir` returns correct `secret` directory path
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_secret_directory_path_correctness()
{
  let ( _temp_dir, workspace ) = create_test_workspace_with_structure();
  
  let secret_dir = workspace.secret_dir();
  let expected_path = workspace.root().join( "secret" );
  
  assert_eq!( secret_dir, expected_path );
  assert!( secret_dir.file_name().unwrap() == "secret" );
  assert!( !secret_dir.to_string_lossy().contains( ".secrets" ) );
}

/// Test that `secret_file` creates paths within `secret` directory
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_secret_file_path_correctness()
{
  let ( _temp_dir, workspace ) = create_test_workspace_with_structure();
  
  let secret_file = workspace.secret_file( "-secrets.sh" );
  let expected_path = workspace.root().join( "secret" ).join( "-secrets.sh" );
  
  assert_eq!( secret_file, expected_path );
  assert!( secret_file.parent().unwrap().file_name().unwrap() == "secret" );
}

/// Test loading secrets from `-secrets.sh` file within `secret` directory
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_load_secrets_from_correct_directory()
{
  let ( _temp_dir, workspace ) = create_test_workspace_with_structure();
  
  // Create secret directory and -secrets.sh file
  let secret_dir = workspace.secret_dir();
  fs ::create_dir_all( &secret_dir ).expect( "Failed to create secret directory" );
  
  let secrets_file = secret_dir.join( "-secrets.sh" );
  let secret_content = r#"
# Test secrets file
API_KEY="test-api-key-123"
DATABASE_URL="postgresql: //localhost: 5432/testdb"
DEBUG_MODE="true"
"#;
  
  fs ::write( &secrets_file, secret_content ).expect( "Failed to write secrets file" );
  
  // Test loading secrets
  let secrets = workspace.load_secrets_from_file( "-secrets.sh" )
  .expect( "Failed to load secrets from file" );
  
  assert_eq!( secrets.len(), 3 );
  assert_eq!( secrets.get( "API_KEY" ).unwrap(), "test-api-key-123" );
  assert_eq!( secrets.get( "DATABASE_URL" ).unwrap(), "postgresql: //localhost: 5432/testdb" );
  assert_eq!( secrets.get( "DEBUG_MODE" ).unwrap(), "true" );
}

/// Test loading individual secret key from `secret` directory
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_load_secret_key_from_correct_directory()
{
  let ( _temp_dir, workspace ) = create_test_workspace_with_structure();
  
  // Create secret directory and production secrets file  
  let secret_dir = workspace.secret_dir();
  fs ::create_dir_all( &secret_dir ).expect( "Failed to create secret directory" );
  
  let prod_secrets_file = secret_dir.join( "production.env" );
  let prod_content = r#"
PROD_API_KEY="production-key-456"
PROD_DATABASE_URL="postgresql: //prod.example.com: 5432/proddb"
"#;
  
  fs ::write( &prod_secrets_file, prod_content ).expect( "Failed to write production secrets" );
  
  // Test loading individual secret key
  let api_key = workspace.load_secret_key( "PROD_API_KEY", "production.env" )
  .expect( "Failed to load production API key" );
  
  assert_eq!( api_key, "production-key-456" );
}

/// Test that `secret` directory is created by `create_test_workspace_with_structure`
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_secret_directory_exists_in_test_workspace()
{
  let ( _temp_dir, workspace ) = create_test_workspace_with_structure();
  
  let secret_dir = workspace.secret_dir();
  assert!( secret_dir.exists(), "Secret directory should exist: {}", secret_dir.display() );
  assert!( secret_dir.is_dir(), "Secret path should be a directory" );
  
  // Verify it's the correct name
  assert_eq!( secret_dir.file_name().unwrap(), "secret" );
}

/// Test that multiple secret files can coexist in `secret` directory
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_multiple_secret_files_in_directory()
{
  let ( _temp_dir, workspace ) = create_test_workspace_with_structure();
  
  let secret_dir = workspace.secret_dir();
  fs ::create_dir_all( &secret_dir ).expect( "Failed to create secret directory" );
  
  // Create multiple secret files
  let files_and_contents = vec!
  [
  ( "-secrets.sh", "SHARED_KEY=\"shared-value\"" ),
  ( "development.env", "DEV_KEY=\"dev-value\"" ),
  ( "production.env", "PROD_KEY=\"prod-value\"" ),
  ( "staging.env", "STAGING_KEY=\"staging-value\"" ),
 ];
  
  for ( filename, content ) in &files_and_contents
  {
  let file_path = secret_dir.join( filename );
  fs ::write( &file_path, content ).expect( "Failed to write secret file" );
 }
  
  // Verify all files exist and can be loaded
  for ( filename, _content ) in &files_and_contents
  {
  let file_path = workspace.secret_file( filename );
  assert!( file_path.exists(), "Secret file should exist: {}", file_path.display() );
  
  let secrets = workspace.load_secrets_from_file( filename )
   .expect( "Failed to load secrets from file" );
  assert!( !secrets.is_empty(), "Secrets should be loaded from {filename}" );
 }
}

/// Test path validation for secret directory structure
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_secret_path_validation()
{
  let ( _temp_dir, workspace ) = create_test_workspace_with_structure();
  
  let secret_dir = workspace.secret_dir();
  let secret_file = workspace.secret_file( "test.env" );
  
  // Verify paths are within workspace
  assert!( workspace.is_workspace_file( &secret_dir ) );
  assert!( workspace.is_workspace_file( &secret_file ) );
  
  // Verify directory structure
  assert!( secret_file.starts_with( &secret_dir ) );
  assert!( secret_dir.starts_with( workspace.root() ) );
  
  // Verify correct names (not typos)
  assert!( secret_dir.to_string_lossy().contains( "secret" ) );
  assert!( !secret_dir.to_string_lossy().contains( ".secrets" ) );
}