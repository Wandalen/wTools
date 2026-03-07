//! Bug fix test for .count command issues (Bug #003a and #003b)
//!
//! ## Root Cause
//!
//! Bug #003a: Default Behavior Mismatch
//! - Location: `claude_storage/src/cli/mod.rs:1018`
//! - The `.count` command defaults to `target::projects` when called without parameters,
//!   counting all projects globally instead of being context-aware like `.show`.
//! - This creates UX inconsistency: `.show` (no params) shows current project with
//!   "Total Entries: N", but `.count` (no params) returns the number of projects globally.
//! - Users expect `.count` to count what `.show` is showing (entries in current project).
//!
//! Bug #003b: Hardcoded UUID Parsing
//! - Location: `claude_storage/src/cli/mod.rs:1040, 1056`
//! - The `count_routine` hardcodes `ProjectId::uuid()` instead of using
//!   `parse_project_parameter()`, exactly the same bug that was already fixed in
//!   `.show` routine (see fix comment at lines 545-557).
//! - This prevents path-based project parameters from working, only UUID parameters work.
//!
//! ## Why Not Caught
//!
//! - No tests for `.count` command default behavior
//! - No tests for `.count` with path-based project parameters
//! - `.count` implementation was copy-pasted before `.show` bug was fixed
//! - Integration tests focused on `.show` command, `.count` overlooked
//!
//! ## Fix Applied
//!
//! Bug #003a: Make .count context-aware
//! - When NO parameters: Load current project from CWD, count total entries
//! - When project parameter: Count entries in that project (implicit target)
//! - Preserve explicit `target::` parameter for other use cases
//!
//! Bug #003b: Use `parse_project_parameter`
//! - Replace `ProjectId::uuid( proj_id )` with `parse_project_parameter( proj_id )?`
//! - Apply same fix pattern as `.show` (lines 693-700, 735-740)
//!
//! ## Prevention
//!
//! - Test `.count` default behavior in CWD project context
//! - Test `.count` with path project parameters
//! - Test `.count` with explicit target parameters
//! - Verify consistency with `.show` command semantics
//!
//! ## Pitfall
//!
//! **Default behavior inconsistency:** When implementing related commands (like
//! `.show` and `.count`), ensure their default behaviors are consistent and
//! context-aware. Users build mental models - if `.show` is context-aware, `.count`
//! should be too. Don't make one global and one local without clear justification.
//!
//! **Copy-paste bugs:** When copying code patterns between commands, ensure you
//! also copy any bug fixes that were applied to the original. Check git history
//! and existing fix comments before copy-pasting.

mod common;

use std::path::PathBuf;
use std::fs;

/// Test Bug #003a: .count should be context-aware like .show
///
/// When called with NO parameters from within a project directory,
/// .count should count entries in that project, not count all projects globally.
// test_kind: bug_reproducer(issue-003a)
#[ test ]
#[ ignore = "Integration test: requires proper project setup - run manually or in CI" ]
fn test_count_default_behavior_context_aware()
{
  // Setup: Create a path project with known number of entries
  let test_project_path = std::env::temp_dir().join( "test-count-context-aware" );
  fs::create_dir_all( &test_project_path ).unwrap();

  let home = std::env::var( "HOME" ).unwrap();
  let storage_root = PathBuf::from( &home ).join( ".claude" );

  // Create project directory
  use claude_storage_core::encode_path;
  let encoded = encode_path( &test_project_path ).unwrap();
  let project_dir = storage_root.join( "projects" ).join( &encoded );
  fs::create_dir_all( &project_dir ).unwrap();

  // Create session with exactly 5 entries
  let session_file = project_dir.join( "test-session.jsonl" );
  fs::write( &session_file, concat!(
    "{\"type\":\"user\",\"text\":\"entry1\"}\n",
    "{\"type\":\"assistant\",\"text\":\"entry2\"}\n",
    "{\"type\":\"user\",\"text\":\"entry3\"}\n",
    "{\"type\":\"assistant\",\"text\":\"entry4\"}\n",
    "{\"type\":\"user\",\"text\":\"entry5\"}\n"
  )).unwrap();

  let output = common::claude_storage_cmd()
    .args( [ ".count" ] )
    .current_dir( &test_project_path )
    .output()
    .expect( "Failed to execute command" );

  // Cleanup
  fs::remove_file( &session_file ).ok();
  fs::remove_dir( &project_dir ).ok();
  fs::remove_dir_all( &test_project_path ).ok();

  // Assertions
  let stderr = String::from_utf8_lossy( &output.stderr );
  let stdout = String::from_utf8_lossy( &output.stdout );
  assert!(
    output.status.success(),
    ".count command should succeed. stderr: {stderr}, stdout: {stdout}"
  );

  let count_str = stdout.trim();

  // Bug #003a: Previously returned number of projects (e.g., 85)
  // Fixed: Now returns number of entries in current project (5)
  assert_eq!(
    count_str, "5",
    "Expected .count (no params) to return 5 entries in current project, got: {count_str}"
  );
}

/// Test Bug #003b: .count should handle path-based project parameters
///
/// The .count command should use `parse_project_parameter()` to handle
/// both UUID and path-based project parameters, not hardcode `ProjectId::uuid()`.
// test_kind: bug_reproducer(issue-003b)
#[ test ]
#[ ignore = "Integration test: requires proper project setup - run manually or in CI" ]
fn test_count_with_path_project_parameter()
{
  // Setup: Create a path project with known number of sessions
  let test_project_path = std::env::temp_dir().join( "test-count-path-param" );
  fs::create_dir_all( &test_project_path ).unwrap();

  let home = std::env::var( "HOME" ).unwrap();
  let storage_root = PathBuf::from( &home ).join( ".claude" );

  // Create project directory
  use claude_storage_core::encode_path;
  let encoded = encode_path( &test_project_path ).unwrap();
  let project_dir = storage_root.join( "projects" ).join( &encoded );
  fs::create_dir_all( &project_dir ).unwrap();

  // Create 3 sessions
  let session1 = project_dir.join( "session1.jsonl" );
  let session2 = project_dir.join( "session2.jsonl" );
  let session3 = project_dir.join( "session3.jsonl" );

  fs::write( &session1, "{\"type\":\"user\",\"text\":\"test\"}\n" ).unwrap();
  fs::write( &session2, "{\"type\":\"user\",\"text\":\"test\"}\n" ).unwrap();
  fs::write( &session3, "{\"type\":\"user\",\"text\":\"test\"}\n" ).unwrap();

  // Execute .count with PATH project parameter
  let mut cmd = common::claude_storage_cmd();
  cmd.args( [ ".count",
    "target::sessions",
    &format!( "project::{}", test_project_path.display() )
  ] );

  let output = cmd.output().unwrap();

  // Cleanup
  fs::remove_file( &session1 ).ok();
  fs::remove_file( &session2 ).ok();
  fs::remove_file( &session3 ).ok();
  fs::remove_dir( &project_dir ).ok();
  fs::remove_dir_all( &test_project_path ).ok();

  // Assertions
  assert!( output.status.success(), ".count command should succeed" );

  let stdout = String::from_utf8_lossy( &output.stdout );
  let count_str = stdout.trim();

  // Bug #003b: Currently returns 0 (fails to find project by path)
  // Should return: 3 (number of sessions)
  assert_eq!(
    count_str, "3",
    "Expected .count with path parameter to return 3 sessions, got: {count_str}"
  );
}

/// Verification test: .count with explicit `target::projects` should still work
///
/// After fixing Bug #003a to make default context-aware, the explicit
/// `target::projects` should still count all projects globally.
// test_kind: bug_reproducer(issue-003a)
#[ test ]
#[ ignore = "Integration test: run manually or in CI" ]
fn test_count_explicit_target_projects()
{
  // Execute .count with explicit target::projects
  let mut cmd = common::claude_storage_cmd();
  cmd.args( [ ".count", "target::projects" ] );

  let output = cmd.output().unwrap();

  // Assertions
  assert!( output.status.success(), ".count command should succeed" );

  let stdout = String::from_utf8_lossy( &output.stdout );
  let count_str = stdout.trim();

  // Should return number of projects globally (e.g., 85)
  // This is still a valid use case, just shouldn't be the DEFAULT
  let count: usize = count_str.parse().expect( "Should be a number" );
  assert!( count > 0, "Should have at least one project" );
}
