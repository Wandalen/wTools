//! Integration tests for session detection.

use claude_session::check_session_exists;
use tempfile::TempDir;

#[ test ]
fn check_session_exists_returns_false_for_nonexistent_directory()
{
  let temp_dir = TempDir::new().unwrap();
  let session_dir = temp_dir.path().join( "nonexistent" );

  assert!( !check_session_exists( &session_dir ) );
}

#[ test ]
fn check_session_exists_returns_false_without_conversation_files()
{
  use std::path::PathBuf;

  let temp_dir = TempDir::new().unwrap();
  let session_dir = temp_dir.path().join( "test-session" );
  std::fs::create_dir_all( &session_dir ).unwrap();

  // Create Claude storage but with no conversation files
  let home_dir = std::env::var( "HOME" ).unwrap();
  let session_path_str = session_dir.display().to_string();
  let escaped_path = session_path_str
    .chars()
    .map( | c | if matches!( c, '/' | '_' | '.' | '@' | '#' | '%' | '&' | ' ' ) { '-' } else { c } )
    .collect::< String >();

  let claude_storage = PathBuf::from( &home_dir )
    .join( ".claude" )
    .join( "projects" )
    .join( &escaped_path );

  std::fs::create_dir_all( &claude_storage ).unwrap();

  // No conversation files - should return false
  assert!( !check_session_exists( &session_dir ) );

  // Clean up
  let _ = std::fs::remove_dir_all( &claude_storage );
}

#[ test ]
fn check_session_exists_returns_true_with_jsonl_conversation_file()
{
  use std::path::PathBuf;

  let temp_dir = TempDir::new().unwrap();
  let session_dir = temp_dir.path().join( "test-session" );
  std::fs::create_dir_all( &session_dir ).unwrap();

  // Create Claude storage with conversation file
  let home_dir = std::env::var( "HOME" ).unwrap();
  let session_path_str = session_dir.display().to_string();
  let escaped_path = session_path_str
    .chars()
    .map( | c | if matches!( c, '/' | '_' | '.' | '@' | '#' | '%' | '&' | ' ' ) { '-' } else { c } )
    .collect::< String >();

  let claude_storage = PathBuf::from( &home_dir )
    .join( ".claude" )
    .join( "projects" )
    .join( &escaped_path );

  std::fs::create_dir_all( &claude_storage ).unwrap();
  std::fs::write(
    claude_storage.join( "ce2efe82-3c31-40d9-a6b1-33c22c13aea5.jsonl" ),
    r#"{"message":"test"}"#
  ).unwrap();

  // Should return true with conversation file
  assert!( check_session_exists( &session_dir ) );

  // Clean up
  let _ = std::fs::remove_dir_all( &claude_storage );
}

#[ test ]
fn check_session_exists_skips_agent_definition_files()
{
  use std::path::PathBuf;

  let temp_dir = TempDir::new().unwrap();
  let session_dir = temp_dir.path().join( "test-session" );
  std::fs::create_dir_all( &session_dir ).unwrap();

  // Create Claude storage with ONLY agent definition files
  let home_dir = std::env::var( "HOME" ).unwrap();
  let session_path_str = session_dir.display().to_string();
  let escaped_path = session_path_str
    .chars()
    .map( | c | if matches!( c, '/' | '_' | '.' | '@' | '#' | '%' | '&' | ' ' ) { '-' } else { c } )
    .collect::< String >();

  let claude_storage = PathBuf::from( &home_dir )
    .join( ".claude" )
    .join( "projects" )
    .join( &escaped_path );

  std::fs::create_dir_all( &claude_storage ).unwrap();
  std::fs::write(
    claude_storage.join( "agent-custom.jsonl" ),
    r#"{"agent":"definition"}"#
  ).unwrap();

  // Should return false - agent files don't count as conversations
  assert!( !check_session_exists( &session_dir ) );

  // Clean up
  let _ = std::fs::remove_dir_all( &claude_storage );
}

#[ test ]
fn check_session_exists_skips_empty_files()
{
  use std::path::PathBuf;

  let temp_dir = TempDir::new().unwrap();
  let session_dir = temp_dir.path().join( "test-session" );
  std::fs::create_dir_all( &session_dir ).unwrap();

  // Create Claude storage with ONLY empty conversation files
  let home_dir = std::env::var( "HOME" ).unwrap();
  let session_path_str = session_dir.display().to_string();
  let escaped_path = session_path_str
    .chars()
    .map( | c | if matches!( c, '/' | '_' | '.' | '@' | '#' | '%' | '&' | ' ' ) { '-' } else { c } )
    .collect::< String >();

  let claude_storage = PathBuf::from( &home_dir )
    .join( ".claude" )
    .join( "projects" )
    .join( &escaped_path );

  std::fs::create_dir_all( &claude_storage ).unwrap();
  std::fs::write(
    claude_storage.join( "empty.jsonl" ),
    ""  // Empty file (0 bytes)
  ).unwrap();

  // Should return false - empty files don't count
  assert!( !check_session_exists( &session_dir ) );

  // Clean up
  let _ = std::fs::remove_dir_all( &claude_storage );
}
