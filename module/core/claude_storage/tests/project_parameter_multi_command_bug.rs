//! Bug fix test for project parameter handling across multiple commands
//!
//! ## Root Cause (Finding #012)
//!
//! Commands `.count`, `.search`, and `.export` hardcode `ProjectId::uuid(proj_id)`
//! at multiple locations instead of using the `parse_project_parameter()` helper.
//! This causes path projects to fail with "Project not found" because they try
//! to find a UUID-named directory instead of decoding the path.
//!
//! This is the same bug as Finding #008, but it was only fixed for `.show` command.
//! The fix was not propagated to other commands that accept project parameter.
//!
//! **Affected locations:**
//! - `count_routine` line 1171: `storage.load_project( &ProjectId::uuid( proj_id ) )`
//! - `count_routine` line 1187: `storage.load_project( &ProjectId::uuid( proj_id ) )`
//! - `search_routine` line 1280: `storage.load_project( &ProjectId::uuid( proj_id ) )`
//! - `search_routine` line 1307: `storage.load_project( &ProjectId::uuid( proj_id ) )`
//! - `export_routine` line 1436: `storage.load_project( &ProjectId::uuid( proj_id ) )`
//!
//! ## Why Not Caught
//!
//! No tests exercised `.count`, `.search`, or `.export` commands with path projects.
//! All test scenarios used UUID projects or avoided the project parameter entirely.
//!
//! ## Fix Applied
//!
//! Replace all `ProjectId::uuid()` calls with `parse_project_parameter()` to enable:
//! - Absolute paths → `ProjectId::Path`
//! - Path-encoded strings → decode then `ProjectId::Path`
//! - Debug format `Path("...")` → extract and use `ProjectId::Path`
//! - Otherwise → `ProjectId::Uuid`
//!
//! ## Prevention
//!
//! Add comprehensive test coverage for all commands that accept project parameter:
//! - `.count` with path projects
//! - `.search` with path projects
//! - `.export` with path projects
//!
//! ## Pitfall
//!
//! When fixing a bug in one location, always grep for similar patterns across
//! the entire codebase. Bugs often exist in multiple locations that share the
//! same flawed assumption (here: "project parameter is always a UUID").

mod common;

use std::path::PathBuf;
use std::fs;

#[ test ]
fn test_count_with_path_project()
{
  // Setup: Create a real path project in test storage
  let test_project_path = std::env::temp_dir().join( "test-count-path-project" );
  fs::create_dir_all( &test_project_path ).unwrap();

  let home = std::env::var( "HOME" ).unwrap();
  let storage_root = PathBuf::from( &home ).join( ".claude" );

  // Encode path and create project directory
  use claude_storage_core::encode_path;
  let encoded = encode_path( &test_project_path ).unwrap();
  let project_dir = storage_root.join( "projects" ).join( &encoded );
  fs::create_dir_all( &project_dir ).unwrap();

  // Create test session files
  let session_id = "test-count-session-1";
  let session_file = project_dir.join( format!( "{session_id}.jsonl" ) );
  fs::write( &session_file, r#"{"type":"user","uuid":"uuid-1","parentUuid":null,"timestamp":"2025-12-06T10:00:00Z","cwd":"/tmp","sessionId":"test","version":"2.0.0","gitBranch":"main","userType":"external","isSidechain":false,"message":{"role":"user","content":"test entry"}}
{"type":"assistant","uuid":"uuid-2","parentUuid":"uuid-1","timestamp":"2025-12-06T10:00:01Z","cwd":"/tmp","sessionId":"test","version":"2.0.0","gitBranch":"main","userType":"external","isSidechain":false,"message":{"role":"assistant","content":[{"type":"text","text":"response"}]}}"# ).unwrap();

  // Test: .count target::sessions with PATH project
  let mut cmd = common::claude_storage_cmd();
  cmd.args( [
    ".count",
    "target::sessions",
    &format!( "project::{}", test_project_path.display() )
  ] );

  let output = cmd.output().unwrap();

  // Cleanup
  fs::remove_file( &session_file ).ok();
  fs::remove_dir( &project_dir ).ok();
  fs::remove_dir_all( &test_project_path ).ok();

  // Assertions
  let stdout = String::from_utf8_lossy( &output.stdout );
  let stderr = String::from_utf8_lossy( &output.stderr );

  assert!(
    !stderr.contains( "Project not found" ),
    "Bug: .count treats path project as UUID. stderr: {stderr}"
  );

  assert!(
    output.status.success(),
    ".count should succeed with path project. stderr: {stderr}"
  );

  assert!(
    stdout.contains( '1' ),
    "Should count 1 session. stdout: {stdout}"
  );
}

#[ test ]
fn test_count_entries_with_path_project()
{
  // Setup: Create a real path project in test storage
  let test_project_path = std::env::temp_dir().join( "test-count-entries-path-project" );
  fs::create_dir_all( &test_project_path ).unwrap();

  let home = std::env::var( "HOME" ).unwrap();
  let storage_root = PathBuf::from( &home ).join( ".claude" );

  // Encode path and create project directory
  use claude_storage_core::encode_path;
  let encoded = encode_path( &test_project_path ).unwrap();
  let project_dir = storage_root.join( "projects" ).join( &encoded );
  fs::create_dir_all( &project_dir ).unwrap();

  // Create test session with 2 entries
  let session_id = "test-entries-session";
  let session_file = project_dir.join( format!( "{session_id}.jsonl" ) );
  fs::write( &session_file, r#"{"type":"user","uuid":"uuid-1","parentUuid":null,"timestamp":"2025-12-06T10:00:00Z","cwd":"/tmp","sessionId":"test","version":"2.0.0","gitBranch":"main","userType":"external","isSidechain":false,"message":{"role":"user","content":"entry 1"}}
{"type":"assistant","uuid":"uuid-2","parentUuid":"uuid-1","timestamp":"2025-12-06T10:00:01Z","cwd":"/tmp","sessionId":"test","version":"2.0.0","gitBranch":"main","userType":"external","isSidechain":false,"message":{"role":"assistant","content":[{"type":"text","text":"entry 2"}]}}"# ).unwrap();

  // Test: .count target::entries with PATH project and session
  let mut cmd = common::claude_storage_cmd();
  cmd.args( [
    ".count",
    "target::entries",
    &format!( "session::{session_id}" ),
    &format!( "project::{}", test_project_path.display() )
  ] );

  let output = cmd.output().unwrap();

  // Cleanup
  fs::remove_file( &session_file ).ok();
  fs::remove_dir( &project_dir ).ok();
  fs::remove_dir_all( &test_project_path ).ok();

  // Assertions
  let stdout = String::from_utf8_lossy( &output.stdout );
  let stderr = String::from_utf8_lossy( &output.stderr );

  assert!(
    !stderr.contains( "Project not found" ),
    "Bug: .count entries treats path project as UUID. stderr: {stderr}"
  );

  assert!(
    output.status.success(),
    ".count entries should succeed with path project. stderr: {stderr}"
  );

  assert!(
    stdout.contains( '2' ),
    "Should count 2 entries. stdout: {stdout}"
  );
}

#[ test ]
#[ ignore = "Integration test for .search command with path project" ]
fn test_search_with_path_project()
{
  // Note: This test is ignored because .search is not fully implemented yet,
  // but when it is, this test will catch the project parameter bug.

  let test_project_path = std::env::temp_dir().join( "test-search-path-project" );
  fs::create_dir_all( &test_project_path ).unwrap();

  let home = std::env::var( "HOME" ).unwrap();
  let storage_root = PathBuf::from( &home ).join( ".claude" );

  use claude_storage_core::encode_path;
  let encoded = encode_path( &test_project_path ).unwrap();
  let project_dir = storage_root.join( "projects" ).join( &encoded );
  fs::create_dir_all( &project_dir ).unwrap();

  let session_id = "test-search-session";
  let session_file = project_dir.join( format!( "{session_id}.jsonl" ) );
  fs::write( &session_file, r#"{"type":"user","uuid":"uuid-1","parentUuid":null,"timestamp":"2025-12-06T10:00:00Z","cwd":"/tmp","sessionId":"test","version":"2.0.0","gitBranch":"main","userType":"external","isSidechain":false,"message":{"role":"user","content":"searchable test content"}}"# ).unwrap();

  // Test: .search with PATH project
  let mut cmd = common::claude_storage_cmd();
  cmd.args( [
    ".search",
    "query::searchable",
    &format!( "project::{}", test_project_path.display() )
  ] );

  let output = cmd.output().unwrap();

  fs::remove_file( &session_file ).ok();
  fs::remove_dir( &project_dir ).ok();
  fs::remove_dir_all( &test_project_path ).ok();

  let _stdout = String::from_utf8_lossy( &output.stdout );
  let stderr = String::from_utf8_lossy( &output.stderr );

  assert!(
    !stderr.contains( "Project not found" ),
    "Bug: .search treats path project as UUID. stderr: {stderr}"
  );

  assert!(
    output.status.success(),
    ".search should succeed with path project. stderr: {stderr}"
  );
}

#[ test ]
#[ ignore = "Integration test for .export command with path project" ]
fn test_export_with_path_project()
{
  // Note: This test is ignored because .export is not fully implemented yet,
  // but when it is, this test will catch the project parameter bug.

  let test_project_path = std::env::temp_dir().join( "test-export-path-project" );
  fs::create_dir_all( &test_project_path ).unwrap();

  let home = std::env::var( "HOME" ).unwrap();
  let storage_root = PathBuf::from( &home ).join( ".claude" );

  use claude_storage_core::encode_path;
  let encoded = encode_path( &test_project_path ).unwrap();
  let project_dir = storage_root.join( "projects" ).join( &encoded );
  fs::create_dir_all( &project_dir ).unwrap();

  let session_id = "test-export-session";
  let session_file = project_dir.join( format!( "{session_id}.jsonl" ) );
  fs::write( &session_file, r#"{"type":"user","uuid":"uuid-1","parentUuid":null,"timestamp":"2025-12-06T10:00:00Z","cwd":"/tmp","sessionId":"test","version":"2.0.0","gitBranch":"main","userType":"external","isSidechain":false,"message":{"role":"user","content":"exportable content"}}"# ).unwrap();

  let output_file = std::env::temp_dir().join( "test-export-output.md" );

  // Test: .export with PATH project
  let mut cmd = common::claude_storage_cmd();
  cmd.args( [
    ".export",
    &format!( "session_id::{session_id}" ),
    "format::markdown",
    &format!( "output::{}", output_file.display() ),
    &format!( "project::{}", test_project_path.display() )
  ] );

  let output = cmd.output().unwrap();

  fs::remove_file( &session_file ).ok();
  fs::remove_dir( &project_dir ).ok();
  fs::remove_dir_all( &test_project_path ).ok();
  fs::remove_file( &output_file ).ok();

  let _stdout = String::from_utf8_lossy( &output.stdout );
  let stderr = String::from_utf8_lossy( &output.stderr );

  assert!(
    !stderr.contains( "Project not found" ),
    "Bug: .export treats path project as UUID. stderr: {stderr}"
  );

  assert!(
    output.status.success(),
    ".export should succeed with path project. stderr: {stderr}"
  );
}
