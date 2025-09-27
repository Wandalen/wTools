#![ allow( clippy ::doc_markdown, clippy ::redundant_closure_for_method_calls, clippy ::uninlined_format_args ) ]

//! Backward Compatibility Validation for Task 021
//!
//! Ensures that all existing functionality continues to work exactly as before.
//! No existing code should break after the task 021 implementation.

#[ cfg( feature = "secrets" ) ]
use workspace_tools ::testing;
#[ cfg( feature = "secrets" ) ]
use std ::fs;

/// Test that existing code patterns still work - from actual api_huggingface usage
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_real_world_usage_patterns()
{
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();

  // Create test secrets file
  fs ::create_dir_all( workspace.secret_dir() ).unwrap();
  fs ::write( workspace.secret_file( "api.env" ), "API_KEY=test_key_123\nTOKEN=test_token_456" ).unwrap();

  // This is the pattern that should continue to work
  let secrets = workspace.load_secrets_from_file( "api.env" ).unwrap();
  assert_eq!( secrets.get( "API_KEY" ).unwrap(), "test_key_123" );
  assert_eq!( secrets.get( "TOKEN" ).unwrap(), "test_token_456" );

  // Individual key loading should still work
  let api_key = workspace.load_secret_key( "API_KEY", "api.env" ).unwrap();
  assert_eq!( api_key, "test_key_123" );

  // Environment fallback should still work
  std ::env ::set_var( "FALLBACK_TEST_KEY", "fallback_value" );
  let fallback = workspace.load_secret_key( "FALLBACK_TEST_KEY", "nonexistent.env" ).unwrap();
  assert_eq!( fallback, "fallback_value" );
  std ::env ::remove_var( "FALLBACK_TEST_KEY" );
}

/// Test that method signatures haven't changed
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_method_signatures_unchanged()
{
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();

  // Create test file
  fs ::create_dir_all( workspace.secret_dir() ).unwrap();
  fs ::write( workspace.secret_file( "test.env" ), "KEY=value" ).unwrap();

  // Test all existing method signatures compile and work
  let _: Result< std ::collections ::HashMap< String, String >, _ > = workspace.load_secrets_from_file( "test.env" );
  let _: Result< String, _ > = workspace.load_secret_key( "KEY", "test.env" );
  let _: std ::path ::PathBuf = workspace.secret_dir();
  let _: std ::path ::PathBuf = workspace.secret_file( "test.env" );
}

/// Test that error types haven't changed for existing methods
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_error_types_unchanged()
{
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();

  // Test that error types are still the same
  let result = workspace.load_secrets_from_file( "nonexistent.env" );
  assert!( result.is_err() );

  let error = result.unwrap_err();
  // Should still be WorkspaceError - if this fails to compile, we broke backward compatibility
  let _: workspace_tools ::WorkspaceError = error;
}

/// Test that existing code expecting empty HashMap now gets errors (this is intentional breaking change documented in task)
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_silent_failure_now_explicit()
{
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();

  // Previously this would return empty HashMap, now it should return explicit error
  let result = workspace.load_secrets_from_file( "nonexistent.env" );
  assert!( result.is_err(), "Should now return explicit error instead of empty HashMap" );

  let error_msg = result.unwrap_err().to_string();
  assert!( error_msg.contains( "not found" ), "Error should be informative" );
}

/// Test that secure versions work with existing patterns
#[ test ]
#[ cfg( all( feature = "secrets", feature = "secure" ) ) ]
fn test_secure_backward_compatibility()
{
  use secrecy ::ExposeSecret;

  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();

  // Create test file
  fs ::create_dir_all( workspace.secret_dir() ).unwrap();
  fs ::write( workspace.secret_file( "secure.env" ), "SECRET_KEY=secret_value" ).unwrap();

  // Test existing secure method still works
  let secrets = workspace.load_secrets_secure( "secure.env" ).unwrap();
  assert_eq!( secrets.get( "SECRET_KEY" ).unwrap().expose_secret(), "secret_value" );

  let secret_key = workspace.load_secret_key_secure( "SECRET_KEY", "secure.env" ).unwrap();
  assert_eq!( secret_key.expose_secret(), "secret_value" );
}

/// Test directory resolution hasn't changed
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_directory_resolution_unchanged()
{
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();

  // Secret directory should still resolve to the same location
  let secret_dir = workspace.secret_dir();
  assert!( secret_dir.ends_with( "secret" ), "Secret directory should still end with secret" );

  // Secret file resolution should work the same
  let secret_file = workspace.secret_file( "test.env" );
  assert!( secret_file.parent().unwrap().ends_with( "secret" ), "Should resolve to secret directory" );
  assert!( secret_file.ends_with( "test.env" ), "Should end with filename" );
}

/// Test environment variable behavior unchanged
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_environment_fallback_unchanged()
{
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();

  // Set up environment variable
  std ::env ::set_var( "TEST_ENV_FALLBACK", "env_value_123" );

  // Should still fallback to environment when file doesn't exist
  let result = workspace.load_secret_key( "TEST_ENV_FALLBACK", "nonexistent_file.env" );
  assert!( result.is_ok(), "Should still fallback to environment variables" );
  assert_eq!( result.unwrap(), "env_value_123" );

  std ::env ::remove_var( "TEST_ENV_FALLBACK" );
}

/// Test existing file parsing behavior unchanged
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_file_parsing_unchanged()
{
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();

  fs ::create_dir_all( workspace.secret_dir() ).unwrap();

  // Test various file formats that should still work
  let test_cases = vec![
  ( "simple.env", "KEY1=value1\nKEY2=value2", 2 ),
  ( "with_export.env", "export KEY1=value1\nKEY2=value2", 2 ),
  ( "with_comments.env", "# Comment\nKEY1=value1\n# Another comment\nKEY2=value2", 2 ),
  ( "with_spaces.env", "KEY1 = value1\nKEY2= value2 ", 2 ),
  ( "empty_lines.env", "KEY1=value1\n\n\nKEY2=value2\n", 2 ),
 ];

  for ( filename, content, expected_count ) in test_cases
  {
  fs ::write( workspace.secret_file( filename ), content ).unwrap();

  let secrets = workspace.load_secrets_from_file( filename ).unwrap();
  assert_eq!( secrets.len(), expected_count, "File {} should parse {} keys", filename, expected_count );
 }
}

/// Test that helper methods work the same way (these are new but should be stable)
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_helper_methods_consistency()
{
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();

  fs ::create_dir_all( workspace.secret_dir() ).unwrap();
  fs ::write( workspace.secret_file( "helper_test.env" ), "KEY=value" ).unwrap();

  // These are new methods but should be consistent with existing patterns
  let files = workspace.list_secrets_files().unwrap();
  assert!( files.contains( &"helper_test.env".to_string() ) );

  assert!( workspace.secrets_file_exists( "helper_test.env" ) );
  assert!( !workspace.secrets_file_exists( "nonexistent.env" ) );

  let resolved = workspace.resolve_secrets_path( "test.env" );
  assert!( resolved.ends_with( "secret/test.env" ) );
}

#[ cfg( not( feature = "secrets" ) ) ]
fn main() 
{
  println!( "Backward compatibility tests require the 'secrets' feature" );
}