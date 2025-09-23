#![ allow( clippy ::uninlined_format_args, clippy ::format_push_string, clippy ::redundant_closure_for_method_calls, clippy ::unnecessary_unwrap ) ]

//! Task 021 Edge Cases Test Suite
//!
//! Validates edge cases and boundary conditions for the enhanced secrets API

#[ cfg( feature = "secrets" ) ]
use workspace_tools ::testing;
#[ cfg( feature = "secrets" ) ]
use std ::fs;

/// Test empty filename edge case
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_empty_filename_edge_case()
{
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();

  // Test empty filename
  let result = workspace.load_secrets_from_file( "" );
  assert!( result.is_err(), "Empty filename should fail" );
  let error_msg = result.unwrap_err().to_string();
  println!( "Empty filename error: {}", error_msg );
  assert!( !error_msg.is_empty(), "Should provide some error message" );
}

/// Test special characters in filenames
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_special_characters_edge_cases()
{
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();

  let special_names = vec![
  "file with spaces.env",
  "file@with#special$.env",
  "file|with|pipes.env",
  "file\"with\"quotes.env",
  "file'with'apostrophe.env",
 ];

  for filename in special_names
  {
  let result = workspace.load_secrets_from_file( filename );
  assert!( result.is_err(), "Special character filename '{}' should fail gracefully", filename );
 }
}

/// Test very long filename edge case
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_very_long_filename_edge_case()
{
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();

  // Create filename that's too long (255+ chars)
  let long_name = "a".repeat( 300 ) + ".env";
  let result = workspace.load_secrets_from_file( &long_name );
  assert!( result.is_err(), "Very long filename should fail" );
}

/// Test path traversal security edge cases
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_path_traversal_security_edge_cases()
{
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();

  let traversal_attempts = vec![
  "../secrets.env",
  "../../etc/passwd",
  "./../config.env",
  "..\\..\\windows\\system32\\config",
  "/etc/passwd",
  "/tmp/malicious.env",
 ];

  for attempt in traversal_attempts
  {
  let result = workspace.load_secrets_from_file( attempt );
  // Should either fail or warn - both are acceptable for security
  if result.is_ok()
  {
   // If it succeeds, it should be because it treated it as a relative filename,
   // not because it actually traversed paths
   let secrets = result.unwrap();
   assert!( secrets.is_empty(), "Path traversal should not succeed in finding files" );
 }
 }
}

/// Test unicode and non-ASCII filename edge cases
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_unicode_filename_edge_cases()
{
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();

  let unicode_names = vec![
  "файл.env",          // Cyrillic
  "文件.env",           // Chinese
  "ファイル.env",        // Japanese
  "🔒secrets🔑.env",   // Emoji
  "café.env",          // Accented characters
 ];

  for filename in unicode_names
  {
  let result = workspace.load_secrets_from_file( filename );
  // Unicode filenames should be handled gracefully (either work or fail cleanly)
  if result.is_err()
  {
   let error_msg = result.unwrap_err().to_string();
   assert!( !error_msg.is_empty(), "Should provide some error message" );
 }
 }
}

/// Test null byte injection edge case
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_null_byte_injection_edge_case()
{
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();

  // Test null byte injection attempts
  let null_byte_attempts = vec![
  "file\x00.env",
  "file.env\x00",
  "\x00malicious",
 ];

  for attempt in null_byte_attempts
  {
  let result = workspace.load_secrets_from_file( attempt );
  // Should fail safely without panic
  assert!( result.is_err(), "Null byte injection should be rejected" );
 }
}

/// Test concurrent access edge cases (if applicable)
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_concurrent_access_edge_case()
{
  use std ::sync ::Arc;
  use std ::thread;

  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();

  // Setup test file
  fs ::create_dir_all( workspace.secret_dir() ).unwrap();
  fs ::write( workspace.secret_file( "concurrent_test.env" ), "KEY=value" ).unwrap();

  let workspace = Arc ::new( workspace );
  let mut handles = vec![];

  // Spawn multiple threads trying to access the same file
  for i in 0..5
  {
  let ws = Arc ::clone( &workspace );
  let handle = thread ::spawn( move ||
  {
   let result = ws.load_secrets_from_file( "concurrent_test.env" );
   assert!( result.is_ok(), "Concurrent access {} should succeed", i );
   let secrets = result.unwrap();
   assert_eq!( secrets.get( "KEY" ), Some( &"value".to_string() ) );
 });
  handles.push( handle );
 }

  // Wait for all threads
  for handle in handles
  {
  handle.join().unwrap();
 }
}

/// Test malformed content edge cases
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_malformed_content_edge_cases()
{
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();

  fs ::create_dir_all( workspace.secret_dir() ).unwrap();

  let malformed_contents = vec![
  ( "binary.env", b"\x00\x01\x02\x03\xff\xfe\xfd" as &[ u8] ),
  ( "empty.env", b"" ),
  ( "only_newlines.env", b"\n\n\n\n" ),
  ( "only_comments.env", b"# comment 1\n# comment 2\n# comment 3" ),
  ( "malformed_lines.env", b"KEY1\nKEY2=\n=VALUE3\nKEY4==double_equals" ),
 ];

  for ( filename, content ) in malformed_contents
  {
  fs ::write( workspace.secret_file( filename ), content ).unwrap();

  let result = workspace.load_secrets_from_file( filename );
  // Should handle malformed content gracefully
  if result.is_err()
  {
   let error_msg = result.unwrap_err().to_string();
   assert!( !error_msg.is_empty(), "Should provide error message for malformed content" );
 }
  else
  {
   // If it succeeds, it should return a HashMap (possibly empty)
   let _secrets = result.unwrap();
 }
 }
}

/// Test large file edge case
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_large_file_edge_case()
{
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();

  fs ::create_dir_all( workspace.secret_dir() ).unwrap();

  // Create a large file (1MB of key-value pairs)
  let mut large_content = String ::new();
  for i in 0..10000
  {
  large_content.push_str( &format!( "KEY{}=value{}\n", i, i ) );
 }

  fs ::write( workspace.secret_file( "large.env" ), large_content ).unwrap();

  let result = workspace.load_secrets_from_file( "large.env" );
  // Should handle large files without panic or excessive memory usage
  if result.is_ok()
  {
  let secrets = result.unwrap();
  assert!( secrets.len() > 9000, "Should load most of the keys" );
 }
  else
  {
  // If it fails, should be a reasonable error
  let error_msg = result.unwrap_err().to_string();
  assert!( !error_msg.is_empty(), "Should provide error message for large file" );
 }
}

#[ cfg( not( feature = "secrets" ) ) ]
fn main() 
{
  println!( "Edge case tests require the 'secrets' feature" );
}