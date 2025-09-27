#![ allow( clippy ::uninlined_format_args, clippy ::doc_markdown ) ]

//! Test to reproduce the secrets API UX issues described in task 021
//!
//! This test reproduces the exact problem reported where developers
//! try to use paths like "lib/llm_tools/secret/-secrets.sh" expecting
//! it to work as a path, but the API treats it as a filename.

#[ cfg( feature = "secrets" ) ]
use workspace_tools ::testing;
#[ cfg( feature = "secrets" ) ]
use std ::fs;

/// Reproduce the exact issue from api_huggingface project
/// Developer expects `load_secrets_from_file("lib/llm_tools/secret/-secrets.sh")` to work as a path
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_reproduce_path_vs_filename_confusion()
{
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();

  // Create a nested directory structure like real projects
  let lib_dir = workspace.join( "lib/llm_tools/secret" );
  fs ::create_dir_all( &lib_dir ).unwrap();

  // Create a secret file in the nested location
  let secret_content = "API_KEY=secret-from-nested-location\nTOKEN=nested-token-123";
  let nested_secret_file = lib_dir.join( "-secrets.sh" );
  fs ::write( &nested_secret_file, secret_content ).unwrap();

  println!( "Created secret file at: {}", nested_secret_file.display() );

  // This is what developers try to do (treating it as a path)
  let developer_attempt_path = "lib/llm_tools/secret/-secrets.sh";

  // Current API behavior - this should fail silently (return empty HashMap)
  let result = workspace.load_secrets_from_file( developer_attempt_path );

  println!( "Developer attempt result: {:?}", result );

  // The current implementation treats this as a filename, so it looks for :
  // workspace_root/secret/lib/llm_tools/secret/-secrets.sh (doesn't exist)
  let expected_wrong_path = workspace.secret_file( developer_attempt_path );
  println!( "Current API looks for file at: {}", expected_wrong_path.display() );
  println!( "File exists: {}", expected_wrong_path.exists() );

  // New improved behavior: returns helpful error instead of empty HashMap
  assert!( result.is_err() );
  let error_msg = result.unwrap_err().to_string();
  assert!( error_msg.contains( "not found at" ) );
  assert!( error_msg.contains( "No files found in secrets directory" ) );

  // What the developer actually wanted was this path to work :
  println!( "What developer wanted to access: {}", nested_secret_file.display() );
  println!( "That file exists: {}", nested_secret_file.exists() );
}

/// Test the current confusing error messages
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_current_poor_error_messages()
{
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();

  // Try to load a secret key from a nonexistent file
  let result = workspace.load_secret_key( "API_KEY", "nonexistent-file.env" );

  println!( "Current error message: {:?}", result );

  // Current error message is :
  // "API_KEY not found. please add it to workspace_root/secret/nonexistent-file.env or set environment variable"
  // This doesn't explain :
  // 1. That the file doesn't exist
  // 2. What files ARE available
  // 3. The path resolution logic
  assert!( result.is_err() );

  let error_msg = result.unwrap_err().to_string();
  println!( "Error message: {}", error_msg );

  // The error message doesn't distinguish between "file not found" vs "key not in file"
  assert!( error_msg.contains( "not found" ) );
  assert!( !error_msg.contains( "file does not exist" ) ); // Missing helpful context
  assert!( !error_msg.contains( "available files" ) ); // Missing suggestions
}

/// Test parameter validation for path-like filenames
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_should_warn_about_path_like_parameters()
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
  println!( "Testing parameter: {}", param );

  // New implementation warns about path-like parameters and returns proper errors
  let result = workspace.load_secrets_from_file( param );

  // It now returns helpful errors instead of empty HashMap
  assert!( result.is_err() );
  let error_msg = result.unwrap_err().to_string();
  assert!( error_msg.contains( "not found at" ) );

  // Should have emitted a warning to stderr (we can't easily test this in unit tests)
 }
}

#[ cfg( not( feature = "secrets" ) ) ]
fn main() 
{
  println!( "This test requires the 'secrets' feature" );
}