//! Test coverage for smart .show command (location-aware behavior)
//!
//! ## Feature
//!
//! The `.show` command adapts its behavior based on which parameters are provided:
//! - No parameters → Shows current directory project (all sessions)
//! - session_id only → Shows that session in current project
//! - project only → Shows that project (all sessions)
//! - Both parameters → Shows that session in that project
//!
//! ## Root Cause (Original Issue)
//!
//! User complained: ".show without arguments must work perfectly fine and show for current path"
//!
//! Original behavior:
//! - `.show` required `session_id` parameter (parser rejected without it)
//! - Code had logic for current directory (line 311 in show_routine) but it was UNREACHABLE
//! - YAML definition had `optional: false` for session_id (line 149)
//!
//! ## Fix Applied
//!
//! 1. YAML: Changed session_id to `optional: true`
//! 2. Implementation: Refactored show_routine to match(session_id, project_id) with 4 cases
//! 3. Spec: Updated documentation to reflect smart behavior
//!
//! ## Prevention
//!
//! - Always test commands with all parameter combinations (none, one, both)
//! - Verify parser configuration matches intended UX
//! - Test location-aware commands from different directories
//!
//! ## Pitfall
//!
//! **Parser configuration must match implementation expectations**. When code expects
//! optional parameters but YAML marks them as required, the implementation code becomes
//! unreachable because parser rejects before routine executes.

mod common;

use std::path::PathBuf;
use std::fs;

#[ test ]
fn test_show_parser_accepts_no_args()
{
  // Test: .show (parser should accept no arguments)
  // This is a smoke test - the detailed "current directory" behavior
  // is tested via unit tests in src/cli/mod.rs

  // Execute .show with no arguments (parser should not reject)
  let output = common::claude_storage_cmd()
    .args( [ ".show" ] )
    .output()
    .unwrap();

  let stderr = String::from_utf8_lossy( &output.stderr );

  // Parser should NOT reject with "missing required argument"
  // (It may fail for other reasons like "project not found", which is fine)
  assert!(
    !stderr.contains( "required argument" ) && !stderr.contains( "session_id"),
    "Parser should not require session_id parameter. stderr: {}",
    stderr
  );
}

#[ test ]
fn test_show_with_session_id_and_project()
{
  // Test: .show session_id::abc project::/path
  // This tests explicit parameters (current directory logic tested separately)

  let test_path = std::env::temp_dir().join( "test-smart-show-session-explicit" );
  fs::create_dir_all( &test_path ).unwrap();

  let home = std::env::var( "HOME" ).unwrap();
  let storage_root = PathBuf::from( &home ).join( ".claude" );

  use claude_storage_core::encode_path;
  let encoded = encode_path( &test_path ).unwrap();
  let project_dir = storage_root.join( "projects" ).join( &encoded );
  fs::create_dir_all( &project_dir ).unwrap();

  let session_id = "test-session-specific";
  let file = project_dir.join( format!( "{}.jsonl", session_id ) );
  fs::write( &file, r#"{"type":"user","text":"specific test entry"}
{"type":"assistant","text":"specific response"}"# ).unwrap();

  // Execute .show session_id::{id} project::/path
  let output = common::claude_storage_cmd()
    .args( [
      ".show",
      &format!( "session_id::{}", session_id ),
      &format!( "project::{}", test_path.display() )
    ] )
    .output()
    .unwrap();

  fs::remove_file( &file ).ok();
  fs::remove_dir( &project_dir ).ok();
  fs::remove_dir_all( &test_path ).ok();

  let stdout = String::from_utf8_lossy( &output.stdout );
  let stderr = String::from_utf8_lossy( &output.stderr );

  assert!(
    output.status.success(),
    "Command should succeed. stderr: {}",
    stderr
  );

  // Should show session details
  assert!(
    stdout.contains( session_id ),
    "Should display session ID. stdout: {}",
    stdout
  );

  // Should show entry count (2 entries)
  assert!(
    stdout.contains( "Entries:" ) || stdout.contains( "2" ),
    "Should show entry count. stdout: {}",
    stdout
  );
}

#[ test ]
fn test_show_with_project_only()
{
  // Test: .show project::/path/to/project
  // Expected: Shows that project (all sessions)

  let test_path = std::env::temp_dir().join( "test-smart-show-project-only" );
  fs::create_dir_all( &test_path ).unwrap();

  let home = std::env::var( "HOME" ).unwrap();
  let storage_root = PathBuf::from( &home ).join( ".claude" );

  use claude_storage_core::encode_path;
  let encoded = encode_path( &test_path ).unwrap();
  let project_dir = storage_root.join( "projects" ).join( &encoded );
  fs::create_dir_all( &project_dir ).unwrap();

  let sessions = vec![ "session-a", "session-b", "session-c" ];
  for session_id in &sessions
  {
    let file = project_dir.join( format!( "{}.jsonl", session_id ) );
    fs::write( &file, r#"{"type":"user","text":"test"}"# ).unwrap();
  }

  // Execute .show project::/path (from different directory)
  let output = common::claude_storage_cmd()
    .args( [ ".show", &format!( "project::{}", test_path.display() ) ] )
    .output()
    .unwrap();

  for session_id in &sessions
  {
    let file = project_dir.join( format!( "{}.jsonl", session_id ) );
    fs::remove_file( file ).ok();
  }
  fs::remove_dir( &project_dir ).ok();
  fs::remove_dir_all( &test_path ).ok();

  let stdout = String::from_utf8_lossy( &output.stdout );
  let stderr = String::from_utf8_lossy( &output.stderr );

  assert!(
    output.status.success(),
    "Command should succeed. stderr: {}",
    stderr
  );

  // Should show project info
  assert!(
    stdout.contains( &test_path.to_string_lossy().to_string() ) || stdout.contains( "Project:" ),
    "Should display project info. stdout: {}",
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
fn test_show_with_both_params()
{
  // Test: .show session_id::abc project::/path
  // Expected: Shows that session in that project (regression test)

  let test_path = std::env::temp_dir().join( "test-smart-show-both-params" );
  fs::create_dir_all( &test_path ).unwrap();

  let home = std::env::var( "HOME" ).unwrap();
  let storage_root = PathBuf::from( &home ).join( ".claude" );

  use claude_storage_core::encode_path;
  let encoded = encode_path( &test_path ).unwrap();
  let project_dir = storage_root.join( "projects" ).join( &encoded );
  fs::create_dir_all( &project_dir ).unwrap();

  let session_id = "test-both-params";
  let file = project_dir.join( format!( "{}.jsonl", session_id ) );
  fs::write( &file, r#"{"type":"user","text":"test entry"}"# ).unwrap();

  // Execute .show session_id::X project::/path
  let output = common::claude_storage_cmd()
    .args( [
      ".show",
      &format!( "session_id::{}", session_id ),
      &format!( "project::{}", test_path.display() )
    ] )
    .output()
    .unwrap();

  fs::remove_file( &file ).ok();
  fs::remove_dir( &project_dir ).ok();
  fs::remove_dir_all( &test_path ).ok();

  let stdout = String::from_utf8_lossy( &output.stdout );
  let stderr = String::from_utf8_lossy( &output.stderr );

  assert!(
    output.status.success(),
    "Command should succeed (regression test). stderr: {}",
    stderr
  );

  assert!(
    stdout.contains( session_id ),
    "Should display session ID. stdout: {}",
    stdout
  );
}

#[ test ]
fn test_show_error_for_nonexistent_project()
{
  // Test: .show project::/nonexistent/path
  // Expected: Clear error message

  let output = common::claude_storage_cmd()
    .args( [ ".show", "project::/nonexistent/test-path-show-error" ] )
    .output()
    .unwrap();

  let stderr = String::from_utf8_lossy( &output.stderr );

  // Should fail gracefully
  assert!(
    !output.status.success(),
    "Should fail for non-existent project"
  );

  // Should have clear error message
  assert!(
    stderr.contains( "not found" ) || stderr.contains( "Project" ) || stderr.contains( "No project" ),
    "Should have clear error message. stderr: {}",
    stderr
  );
}

#[ test ]
fn test_show_project_parser_accepts_no_args()
{
  // Test: .show.project (parser should accept no arguments)
  // This verifies backward compatibility for the deprecated command

  // Execute .show.project with no arguments
  let output = common::claude_storage_cmd()
    .args( [ ".show.project" ] )
    .output()
    .unwrap();

  let stderr = String::from_utf8_lossy( &output.stderr );

  // Parser should NOT reject with "missing required argument"
  // (It may fail for other reasons like "project not found", which is fine)
  assert!(
    !stderr.contains( "required argument" ),
    "Parser should not require project parameter. stderr: {}",
    stderr
  );

  // May show deprecation warning (verify it's there)
  // Note: deprecation message might be in stdout or stderr
  // Just verify the command was accepted by parser
}

/// Test .show session_id partial UUID matching (Finding #011)
///
/// ## Root Cause
///
/// The .show command's session lookup doesn't support partial UUID matching
/// (first 8 characters) despite the spec explicitly documenting this feature.
/// When users provide `session_id::79f86582` for session `79f86582-1435-442c-935a-13f8d874918a`,
/// the command fails with "Session not found" error.
///
/// The implementation only does exact string matching against full session IDs,
/// without checking if the provided ID is a prefix of any existing session.
///
/// ## Why Not Caught
///
/// Existing tests for .show command used custom test session IDs like "test-session-specific"
/// which don't follow the UUID format. All tests provided the exact full ID that was
/// created, so prefix matching was never exercised. The spec mentions partial UUID support
/// but no tests verified it.
///
/// ## Fix Applied
///
/// Modified session lookup in show_routine to check if provided session_id is either:
/// 1. Exact match (existing behavior for non-UUID IDs like "agent-022ada42")
/// 2. Prefix match (new behavior for partial UUIDs like first 8 chars)
///
/// The fix maintains backward compatibility for all ID formats while adding prefix
/// matching support for UUIDs.
///
/// ## Prevention
///
/// When implementing ID-based lookups:
/// - Always test with both full and partial IDs
/// - Consider UUID prefix matching as standard UX
/// - Test actual UUID format IDs, not just test-friendly strings
/// - Verify spec-documented features have corresponding tests
///
/// ## Pitfall
///
/// **Test data that doesn't match production data patterns leads to missing test coverage**.
/// Using friendly test IDs like "test-session-specific" instead of actual UUIDs meant
/// partial UUID matching was never exercised in tests, despite being documented in the spec.
#[ test ]
fn test_show_partial_uuid_matching()
{
  let test_path = std::env::temp_dir().join( "test-smart-show-partial-uuid" );

  let home = std::env::var( "HOME" ).unwrap();
  let storage_root = PathBuf::from( &home ).join( ".claude" );

  use claude_storage_core::encode_path;
  let encoded = encode_path( &test_path ).unwrap();
  let project_dir = storage_root.join( "projects" ).join( &encoded );

  // Clean up any leftover from previous test runs BEFORE creating
  fs::remove_dir_all( &project_dir ).ok();
  fs::remove_dir_all( &test_path ).ok();

  // Now create fresh directories
  fs::create_dir_all( &test_path ).unwrap();
  fs::create_dir_all( &project_dir ).unwrap();

  // Create session with actual UUID format
  let full_uuid = "79f86582-1435-442c-935a-13f8d874918a";
  let partial_uuid = "79f86582"; // First 8 chars
  let file = project_dir.join( format!( "{}.jsonl", full_uuid ) );
  fs::write( &file, r#"{"type":"user","uuid":"uuid-001","parentUuid":null,"timestamp":"2025-11-29T10:00:00Z","cwd":"/tmp/test","sessionId":"test-session","version":"2.0.0","gitBranch":"master","userType":"external","isSidechain":false,"message":{"role":"user","content":"test partial uuid matching"}}
{"type":"assistant","uuid":"uuid-002","parentUuid":"uuid-001","timestamp":"2025-11-29T10:00:01Z","cwd":"/tmp/test","sessionId":"test-session","version":"2.0.0","gitBranch":"master","userType":"external","isSidechain":false,"message":{"role":"assistant","content":[{"type":"text","text":"this should be findable with partial ID"}]}}"# ).unwrap();

  // Test 1: Full UUID should work
  let output = common::claude_storage_cmd()
    .args( [
      ".show",
      &format!( "session_id::{}", full_uuid ),
      &format!( "project::{}", test_path.display() )
    ] )
    .output()
    .unwrap();

  let stdout_full = String::from_utf8_lossy( &output.stdout );
  let stderr_full = String::from_utf8_lossy( &output.stderr );

  assert!(
    output.status.success(),
    "Full UUID should work. stderr: {}",
    stderr_full
  );

  assert!(
    stdout_full.contains( full_uuid ) || stdout_full.contains( "Session:" ),
    "Should show session with full UUID. stdout: {}",
    stdout_full
  );

  // Test 2: Partial UUID (first 8 chars) should also work
  let output = common::claude_storage_cmd()
    .args( [
      ".show",
      &format!( "session_id::{}", partial_uuid ),
      &format!( "project::{}", test_path.display() )
    ] )
    .output()
    .unwrap();

  // Cleanup must happen BEFORE assertions to ensure cleanup runs even if test fails
  fs::remove_file( &file ).ok();
  fs::remove_dir_all( &project_dir ).ok();  // Use remove_dir_all, not remove_dir
  fs::remove_dir_all( &test_path ).ok();

  let stdout_partial = String::from_utf8_lossy( &output.stdout );
  let stderr_partial = String::from_utf8_lossy( &output.stderr );

  assert!(
    output.status.success(),
    "Partial UUID (first 8 chars) should work. stderr: {}",
    stderr_partial
  );

  assert!(
    stdout_partial.contains( full_uuid ) || stdout_partial.contains( "Session:" ),
    "Should find session using partial UUID. stdout: {}",
    stdout_partial
  );

  assert!(
    stdout_partial.contains( "partial uuid matching" ),
    "Should show session content. stdout: {}",
    stdout_partial
  );
}
