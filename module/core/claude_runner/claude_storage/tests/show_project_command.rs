//! Test coverage for .show.project command
//!
//! This command addresses the UX gap where users want to explore a project
//! (see all sessions, statistics) without needing to know specific session UUIDs.

mod common;

use std::path::PathBuf;
use std::fs;

#[ test ]
fn test_show_project_with_path()
{
  // Test: .show.project /path/to/project
  // Should display project stats + list all sessions

  // Setup test storage
  let test_path = std::env::temp_dir().join( "test-project-path-show" );
  fs::create_dir_all( &test_path ).unwrap();

  // Use real Claude storage
  let home = std::env::var( "HOME" ).unwrap();
  let storage_root = PathBuf::from( &home ).join( ".claude" );

  // Create project storage
  use claude_storage_core::encode_path;
  let encoded = encode_path( &test_path ).unwrap();
  let project_dir = storage_root.join( "projects" ).join( &encoded );
  fs::create_dir_all( &project_dir ).unwrap();

  // Create multiple test sessions
  let sessions = vec![
    "test-session-1-uuid",
    "test-session-2-uuid",
    "test-agent-abc123",
  ];

  for session_id in &sessions
  {
    let file = project_dir.join( format!( "{}.jsonl", session_id ) );
    fs::write( &file, r#"{"type":"user","text":"test entry 1"}
{"type":"assistant","text":"test response"}"# ).unwrap();
  }

  // Execute .show.project
  let output = common::claude_storage_cmd()
    .args( [ ".show.project",
      &format!( "project::{}", test_path.display() )
    ] )
    .output()
    .unwrap();

  // Cleanup
  for session_id in &sessions
  {
    let file = project_dir.join( format!( "{}.jsonl", session_id ) );
    fs::remove_file( file ).ok();
  }
  fs::remove_dir( &project_dir ).ok();
  fs::remove_dir_all( &test_path ).ok();

  let stdout = String::from_utf8_lossy( &output.stdout );
  let stderr = String::from_utf8_lossy( &output.stderr );

  // Assertions
  assert!(
    output.status.success(),
    "Command should succeed. stderr: {}",
    stderr
  );

  // Should show project path
  assert!(
    stdout.contains( &test_path.to_string_lossy().to_string() ) || stdout.contains( "Project:" ),
    "Should display project path. stdout: {}",
    stdout
  );

  // Should show session count
  assert!(
    stdout.contains( "Sessions:" ) || stdout.contains( "3" ),
    "Should show session count. stdout: {}",
    stdout
  );

  // Should list all sessions
  for session_id in &sessions
  {
    assert!(
      stdout.contains( session_id ),
      "Should list session {}. stdout: {}",
      session_id,
      stdout
    );
  }
}

#[ test ]
fn test_show_project_from_list_output()
{
  // Test: Copy Path(...) from .list, paste into .show.project
  // This is the primary UX improvement

  let test_path = std::env::temp_dir().join( "test-copy-paste-show" );
  fs::create_dir_all( &test_path ).unwrap();

  let home = std::env::var( "HOME" ).unwrap();
  let storage_root = PathBuf::from( &home ).join( ".claude" );

  use claude_storage_core::encode_path;
  let encoded = encode_path( &test_path ).unwrap();
  let project_dir = storage_root.join( "projects" ).join( &encoded );
  fs::create_dir_all( &project_dir ).unwrap();

  let session_file = project_dir.join( "test-session.jsonl" );
  fs::write( &session_file, r#"{"type":"user"}"# ).unwrap();

  // Simulate .list output format
  let list_output = format!( r#"Path("{}")"#, test_path.display() );

  // Use that output as parameter
  let output = common::claude_storage_cmd()
    .args( [ ".show.project",
      &format!( "project::{}", list_output )
    ] )
    .output()
    .unwrap();

  fs::remove_file( &session_file ).ok();
  fs::remove_dir( &project_dir ).ok();
  fs::remove_dir_all( &test_path ).ok();

  let stdout = String::from_utf8_lossy( &output.stdout );
  let stderr = String::from_utf8_lossy( &output.stderr );

  assert!(
    output.status.success(),
    "Should handle Debug format from .list. stderr: {}",
    stderr
  );

  assert!(
    stdout.contains( &test_path.to_string_lossy().to_string() ) || stdout.contains( "Project:" ),
    "Should parse Path(...) format. stdout: {}",
    stdout
  );
}

#[ test ]
fn test_show_project_nonexistent()
{
  // Test: Graceful error for non-existent project

  let output = common::claude_storage_cmd()
    .args( [ ".show.project",
      "project::/nonexistent/path/to/project-test-show"
    ] )
    .output()
    .unwrap();

  let stderr = String::from_utf8_lossy( &output.stderr );

  // Should fail gracefully
  assert!(
    !output.status.success(),
    "Should fail for non-existent project"
  );

  assert!(
    stderr.contains( "not found" ) || stderr.contains( "Project" ),
    "Should have clear error message. stderr: {}",
    stderr
  );
}
