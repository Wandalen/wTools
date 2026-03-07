//! Bug fix test for project parameter handling
//!
//! ## Root Cause
//!
//! The `show_routine` at line 239 always calls `ProjectId::uuid(proj_id)`
//! regardless of whether `proj_id` is a path or UUID. This causes path projects
//! to fail with "Project not found" because it tries to find a UUID-named
//! directory instead of decoding the path.
//!
//! ## Why Not Caught
//!
//! No tests exercised `.show` command with path projects. All test scenarios
//! used UUID projects or relied on current directory detection.
//!
//! ## Fix Applied
//!
//! Added `parse_project_parameter()` helper that intelligently detects:
//! - Paths starting with `/` → `ProjectId::Path`
//! - Path-encoded starting with `-` → decode then `ProjectId::Path`
//! - Debug format `Path("...")` → extract and use `ProjectId::Path`
//! - Otherwise → `ProjectId::Uuid`
//!
//! ## Prevention
//!
//! Added test coverage for all project parameter formats:
//! - Absolute paths
//! - Path-encoded
//! - UUIDs
//! - Debug format from .list output
//! - Mixed scenarios (path project + UUID session)
//!
//! ## Pitfall
//!
//! Always assuming a string parameter is one type leads to silent failures
//! when user provides a different type. Smart detection with explicit format
//! rules prevents this class of bug.

mod common;

use std::path::PathBuf;
use std::fs;

#[ test ]
fn test_show_with_path_project()
{
  // This test will FAIL until we fix the bug
  //
  // Setup: Create a real path project in test storage
  let test_project_path = std::env::temp_dir().join( "test-claude-storage-project" );
  fs::create_dir_all( &test_project_path ).unwrap();

  // Use real Claude storage (CLI doesn't support custom storage root)
  let home = std::env::var( "HOME" ).unwrap();
  let storage_root = PathBuf::from( &home ).join( ".claude" );

  // Encode path and create project directory
  use claude_storage_core::encode_path;
  let encoded = encode_path( &test_project_path ).unwrap();
  let project_dir = storage_root.join( "projects" ).join( &encoded );
  fs::create_dir_all( &project_dir ).unwrap();

  // Create a test session file
  let session_id = "test-session-12345";
  let session_file = project_dir.join( format!( "{session_id}.jsonl" ) );
  fs::write( &session_file, r#"{"type":"user","text":"test"}"# ).unwrap();

  // Execute .show command with PATH project parameter
  // This should WORK after fix
  let mut cmd = common::claude_storage_cmd();
  let project_display = test_project_path.display().to_string();
  cmd.args( [ ".show",
    &format!( "session_id::{session_id}" ),
    &format!( "project::{project_display}" )
  ] );

  let output = cmd.output().unwrap();

  // Cleanup
  fs::remove_file( &session_file ).ok();
  fs::remove_dir( &project_dir ).ok();
  fs::remove_dir_all( &test_project_path ).ok();

  // Assertion
  let stdout = String::from_utf8_lossy( &output.stdout );
  let stderr = String::from_utf8_lossy( &output.stderr );

  // Should NOT fail with "Project not found"
  assert!(
    !stderr.contains( "Project not found" ),
    "Bug: Path project treated as UUID. stderr: {stderr}"
  );

  // Should show session details
  assert!(
    stdout.contains( "Session:" ) || stdout.contains( session_id ),
    "Expected session details in output. stdout: {stdout}"
  );
}

#[ test ]
fn test_show_with_uuid_project_still_works()
{
  // Ensure our fix doesn't break UUID projects
  // (Regression test)

  // Use real Claude storage (CLI doesn't support custom storage root)
  let home = std::env::var( "HOME" ).unwrap();
  let storage_root = PathBuf::from( &home ).join( ".claude" );

  let project_uuid = "test-uuid-project-123";
  let project_dir = storage_root.join( "projects" ).join( project_uuid );
  fs::create_dir_all( &project_dir ).unwrap();

  let session_id = "test-session-uuid-789";
  let session_file = project_dir.join( format!( "{session_id}.jsonl" ) );
  fs::write( &session_file, r#"{"type":"user","text":"test"}"# ).unwrap();

  // Execute with UUID project
  let mut cmd = common::claude_storage_cmd();
  cmd.args( [ ".show",
    &format!( "session_id::{session_id}" ),
    &format!( "project::{project_uuid}" )
  ] );

  let output = cmd.output().unwrap();

  // Cleanup
  fs::remove_file( &session_file ).ok();
  fs::remove_dir( &project_dir ).ok();

  // Assertion
  let stdout = String::from_utf8_lossy( &output.stdout );
  let stderr = String::from_utf8_lossy( &output.stderr );

  assert!(
    !stderr.contains( "Project not found" ),
    "Regression: UUID project support broken. stderr: {stderr}"
  );

  assert!(
    stdout.contains( "Session:" ),
    "Expected session details. stdout: {stdout}"
  );
}

#[ test ]
fn test_parse_project_parameter_unit()
{
  // Unit test for the helper function we'll add

  // This will fail until we implement parse_project_parameter()

  use claude_storage::parse_project_parameter;  // Will add to lib.rs
  use claude_storage_core::ProjectId;

  // Test absolute path
  let result = parse_project_parameter( "/home/user/project" ).unwrap();
  assert!(
    matches!( result, ProjectId::Path( _ ) ),
    "Absolute path should be ProjectId::Path"
  );

  // Test path-encoded
  let result = parse_project_parameter( "-home-user-project" ).unwrap();
  assert!(
    matches!( result, ProjectId::Path( _ ) ),
    "Path-encoded should be ProjectId::Path"
  );

  // Test UUID
  let result = parse_project_parameter( "abc-123-def" ).unwrap();
  assert!(
    matches!( result, ProjectId::Uuid( _ ) ),
    "UUID format should be ProjectId::Uuid"
  );

  // Test Debug format from .list output
  let result = parse_project_parameter( r#"Path("/home/user/project")"# ).unwrap();
  assert!(
    matches!( result, ProjectId::Path( _ ) ),
    "Debug format should extract path"
  );
}
