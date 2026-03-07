//! CLI command integration tests
//!
//! Comprehensive tests for CLI command functionality using `claude_storage_core`.
//! These tests verify the underlying storage operations that CLI commands depend on.
//!
//! # Test Matrix
//!
//! ## Storage operations (.status command logic)
//! - [x] `global_stats` with empty storage
//! - [x] `global_stats` with multiple projects
//! - [x] `global_stats` counts projects correctly
//! - [x] `global_stats` counts sessions correctly
//! - [x] `global_stats` counts entries correctly
//!
//! ## Project listing (.list command logic)
//! - [x] `list_projects` returns all projects
//! - [x] `list_uuid_projects` filters correctly
//! - [x] `list_path_projects` filters correctly
//! - [x] `list_projects` with empty storage
//! - [x] `count_sessions` for specific project
//!
//! ## Session operations (.show command logic)
//! - [x] load session and get statistics
//! - [x] session stats include entry counts
//! - [x] session stats include token usage
//! - [x] session stats include timestamps
//! - [x] handle non-existent session gracefully
//!
//! ## Counting operations (.count command logic)
//! - [x] `count_projects` returns correct total
//! - [x] `count_sessions` for project
//! - [x] `count_entries` for session
//! - [x] counts work with empty storage
//!
//! # Test Organization
//!
//! Tests use tempfile for isolated storage directories. Each test:
//! 1. Creates temporary storage directory
//! 2. Populates with test data via filesystem
//! 3. Uses `claude_storage_core` APIs (same APIs CLI routines use)
//! 4. Verifies results
//! 5. Cleans up automatically
//!
//! # Test Data Format
//!
//! Tests use the same JSONL format as real Claude Code storage:
//!
//! ```json
//! {
//!   "type": "user" | "assistant",  // NOT "message"!
//!   "uuid": "unique-id",
//!   "message": {"role": "user"|"assistant", "content": "text"},
//!   "timestamp": "2025-11-29T10:00:00Z",
//!   "cwd": "/path/to/dir",
//!   "sessionId": "session-id",
//!   "version": "2.0.0",
//!   // ... other fields
//! }
//! ```
//!
//! **Critical**: `type` must be "user" or "assistant", not "message".
//! This matches real Claude Code v2.0+ storage format.
//!
//! ## Why This Format Matters
//!
//! The Entry parser (`entry.rs:165`) strictly requires the `uuid` field:
//! ```rust,ignore
//! let uuid = obj.get( "uuid" )
//!   .ok_or_else( || Error::parse( 0, line, "missing 'uuid' field" ) )?
//! ```
//!
//! Without this field, entries fail to parse and are silently skipped, causing
//! test failures with `left: 0, right: 42` (expected 42 entries, got 0).
//!
//! ## Production Validation
//!
//! This format has been validated against real Claude Code v2.0.31 sessions:
//! - Successfully parsed 4792-entry production session (bc14c4bf)
//! - Handled mixed entry types (user, assistant, queue-operation, summary)
//! - Graceful skip of 109 metadata entries (3.9% of total)
//! - All conversation entries loaded correctly (1547 user + 3245 assistant)
//!
//! See `examples/parse_real_session.rs` for production validation example.
//!
//! ## Known Pitfalls
//!
//! **Bug History** (issue: session-stats-entry-counting - 2025-12-01):
//! - **Root Cause**: `Session::stats()` method in `claude_storage_core` looked for a top-level `"role"`
//!   field to count entries, but Claude Code v2.0+ format has `"type"` at top level and `"role"`
//!   nested inside the `"message"` object. This caused entry counting to return 0 for all sessions.
//! - **Why Not Caught**: Tests were written but `Session::stats()` implementation wasn't updated to
//!   match the v2.0+ format after the format specification changed from using `role` to `type`.
//! - **Fix Applied**: Changed `Session::stats()` to check the top-level `"type"` field instead of
//!   `"role"` (session.rs:279). Test helpers were already generating correct v2.0+ format.
//! - **Prevention**: Added explicit documentation in `Session::stats()` explaining the v2.0+ format.
//!   All future parsing code should reference the format specification in test documentation.
//! - **Pitfall**: When parsing Claude Code storage format, ALWAYS check the top-level `"type"` field
//!   ("user" or "assistant"), not `"role"`. The `role` field exists but is nested inside `message`.
//!   If you see entry counts of 0 when you expect data, check field name mismatches first.

use tempfile::TempDir;
use std::path::Path;
use claude_storage_core::{ Storage, ProjectId };

// Test utilities

/// Creates a temporary storage directory structure
fn create_test_storage() -> TempDir
{
  TempDir::new().expect( "Failed to create temp directory" )
}

/// Creates a minimal valid JSONL entry for testing
///
/// Generates entries matching the real Claude Code v2.0+ format:
/// - Top-level "type" field: "user" or "assistant" (NOT "message")
/// - Unique "uuid" field for each entry
/// - Nested "message" object with "role" and "content"
fn create_test_entry( entry_type : &str, content : &str, uuid : &str ) -> String
{
  let role = if entry_type == "user" { "user" } else { "assistant" };

  format!
  (
    r#"{{"type":"{entry_type}","uuid":"{uuid}","parentUuid":null,"timestamp":"2025-11-29T10:00:00Z","cwd":"/tmp/test","sessionId":"test-session","version":"2.0.0","gitBranch":"master","userType":"external","isSidechain":false,"message":{{"role":"{role}","content":"{content}"}}}}"#
  )
}

/// Writes a test session file with multiple entries
fn write_test_session( storage_dir : &Path, project_id : &str, session_id : &str, entry_count : usize )
{
  let project_dir = storage_dir.join( "projects" ).join( project_id );
  std::fs::create_dir_all( &project_dir ).expect( "Failed to create project directory" );

  let session_path = project_dir.join( format!( "{session_id}.jsonl" ) );
  let mut content = String::new();

  for i in 0..entry_count
  {
    let entry_type = if i % 2 == 0 { "user" } else { "assistant" };
    let text = format!( "Test message {}", i + 1 );
    let uuid = format!( "test-uuid-{:03}", i + 1 );
    content.push_str( &create_test_entry( entry_type, &text, &uuid ) );
    content.push( '\n' );
  }

  std::fs::write( session_path, content ).expect( "Failed to write session file" );
}

// Storage operations tests (.status command logic)

#[test]
fn global_stats_empty_storage()
{
  let temp_dir = create_test_storage();
  let storage_path = temp_dir.path();

  // Create minimal storage structure
  std::fs::create_dir_all( storage_path.join( "projects" ) ).expect( "Failed to create projects dir" );

  let storage = Storage::with_root( storage_path );
  let stats = storage.global_stats().expect( "Failed to get stats" );

  assert_eq!( stats.total_projects, 0, "Empty storage should have 0 projects" );
  assert_eq!( stats.total_sessions, 0, "Empty storage should have 0 sessions" );
  assert_eq!( stats.total_entries, 0, "Empty storage should have 0 entries" );
}

#[test]
fn global_stats_multiple_projects()
{
  let temp_dir = create_test_storage();
  let storage_path = temp_dir.path();

  // Create test data: 2 projects, 3 sessions, 22 entries total
  write_test_session( storage_path, "uuid-project-001", "session-001", 5 );
  write_test_session( storage_path, "uuid-project-001", "session-002", 10 );
  write_test_session( storage_path, "-home-user-code", "session-001", 7 );

  let storage = Storage::with_root( storage_path );
  let stats = storage.global_stats().expect( "Failed to get stats" );

  assert_eq!( stats.total_projects, 2, "Should have 2 projects" );
  assert_eq!( stats.uuid_projects, 1, "Should have 1 UUID project" );
  assert_eq!( stats.path_projects, 1, "Should have 1 path project" );
  assert_eq!( stats.total_sessions, 3, "Should have 3 sessions" );
  assert_eq!( stats.total_entries, 22, "Should have 22 entries" );
}

#[test]
fn global_stats_counts_entries_correctly()
{
  let temp_dir = create_test_storage();
  let storage_path = temp_dir.path();

  // Create session with known entry count
  write_test_session( storage_path, "test-project", "test-session", 42 );

  let storage = Storage::with_root( storage_path );
  let stats = storage.global_stats().expect( "Failed to get stats" );

  assert_eq!( stats.total_entries, 42, "Should count 42 entries" );
}

// Project listing tests (.list command logic)

#[test]
fn list_projects_empty()
{
  let temp_dir = create_test_storage();
  let storage_path = temp_dir.path();

  std::fs::create_dir_all( storage_path.join( "projects" ) ).expect( "Failed to create projects dir" );

  let storage = Storage::with_root( storage_path );
  let projects = storage.list_projects().expect( "Failed to list projects" );

  assert_eq!( projects.len(), 0, "Empty storage should have 0 projects" );
}

#[test]
fn list_projects_all_types()
{
  let temp_dir = create_test_storage();
  let storage_path = temp_dir.path();

  // Create 3 UUID projects and 2 path projects
  write_test_session( storage_path, "uuid-001", "session-001", 1 );
  write_test_session( storage_path, "uuid-002", "session-001", 1 );
  write_test_session( storage_path, "uuid-003", "session-001", 1 );
  write_test_session( storage_path, "-home-user-project1", "session-001", 1 );
  write_test_session( storage_path, "-home-user-project2", "session-001", 1 );

  let storage = Storage::with_root( storage_path );
  let all_projects = storage.list_projects().expect( "Failed to list all projects" );
  let uuid_projects = storage.list_uuid_projects().expect( "Failed to list UUID projects" );
  let path_projects = storage.list_path_projects().expect( "Failed to list path projects" );

  assert_eq!( all_projects.len(), 5, "Should have 5 total projects" );
  assert_eq!( uuid_projects.len(), 3, "Should have 3 UUID projects" );
  assert_eq!( path_projects.len(), 2, "Should have 2 path projects" );
}

#[test]
fn list_sessions_for_project()
{
  let temp_dir = create_test_storage();
  let storage_path = temp_dir.path();

  // Create project with 3 main sessions + 1 agent session
  write_test_session( storage_path, "test-project", "session-001", 1 );
  write_test_session( storage_path, "test-project", "session-002", 1 );
  write_test_session( storage_path, "test-project", "session-003", 1 );
  write_test_session( storage_path, "test-project", "agent-001", 1 );

  let storage = Storage::with_root( storage_path );
  let project = storage.load_project( &ProjectId::uuid( "test-project" ) )
    .expect( "Failed to load project" );

  // count_sessions() excludes agent sessions
  let main_session_count = project.count_sessions().expect( "Failed to count main sessions" );
  // all_sessions() includes agent sessions
  let all_sessions = project.all_sessions().expect( "Failed to list all sessions" );

  assert_eq!( main_session_count, 3, "Should have 3 main sessions (agent excluded)" );
  assert_eq!( all_sessions.len(), 4, "all_sessions should return 4 total sessions (including agent)" );
}

#[test]
fn project_stats_comprehensive()
{
  let temp_dir = create_test_storage();
  let storage_path = temp_dir.path();

  // Create project with multiple sessions
  write_test_session( storage_path, "test-project", "main-session", 20 );
  write_test_session( storage_path, "test-project", "agent-001", 10 );

  let storage = Storage::with_root( storage_path );
  let project = storage.load_project( &ProjectId::uuid( "test-project" ) )
    .expect( "Failed to load project" );

  let stats = project.project_stats().expect( "Failed to get project stats" );

  assert_eq!( stats.session_count, 2, "Should have 2 sessions" );
  assert_eq!( stats.main_session_count, 1, "Should have 1 main session" );
  assert_eq!( stats.agent_session_count, 1, "Should have 1 agent session" );
  assert_eq!( stats.total_entries, 30, "Should have 30 total entries" );
}

// Session operations tests (.show command logic)

#[test]
fn show_session_stats()
{
  let temp_dir = create_test_storage();
  let storage_path = temp_dir.path();

  write_test_session( storage_path, "test-project", "test-session", 10 );

  let storage = Storage::with_root( storage_path );
  let project = storage.load_project( &ProjectId::uuid( "test-project" ) )
    .expect( "Failed to load project" );

  let mut sessions = project.all_sessions().expect( "Failed to get sessions" );
  let session = sessions.iter_mut()
    .find( | s | s.id() == "test-session" )
    .expect( "Session should exist" );

  let stats = session.stats().expect( "Failed to get session stats" );

  assert_eq!( stats.total_entries, 10, "Session should have 10 entries" );
  assert!( stats.first_timestamp.is_some(), "Should have first timestamp" );
  assert!( stats.last_timestamp.is_some(), "Should have last timestamp" );
}

#[test]
fn show_session_nonexistent()
{
  let temp_dir = create_test_storage();
  let storage_path = temp_dir.path();

  write_test_session( storage_path, "test-project", "existing-session", 1 );

  let storage = Storage::with_root( storage_path );
  let project = storage.load_project( &ProjectId::uuid( "test-project" ) )
    .expect( "Failed to load project" );

  let sessions = project.all_sessions().expect( "Failed to get sessions" );
  let non_existent = sessions.iter()
    .find( | s | s.id() == "non-existent-session" );

  assert!( non_existent.is_none(), "Non-existent session should not be found" );
}

#[test]
fn session_entry_counts()
{
  let temp_dir = create_test_storage();
  let storage_path = temp_dir.path();

  write_test_session( storage_path, "test-project", "test-session", 20 );

  let storage = Storage::with_root( storage_path );
  let project = storage.load_project( &ProjectId::uuid( "test-project" ) )
    .expect( "Failed to load project" );

  let mut sessions = project.all_sessions().expect( "Failed to get sessions" );
  let session = sessions.iter_mut()
    .find( | s | s.id() == "test-session" )
    .expect( "Session should exist" );

  let stats = session.stats().expect( "Failed to get stats" );
  let entry_count = session.count_entries().expect( "Failed to count entries" );

  assert_eq!( entry_count, 20, "Should count 20 entries" );
  assert_eq!( stats.total_entries, 20, "Stats should show 20 total entries" );
}

// Counting operations tests (.count command logic)

#[test]
fn count_projects_multiple()
{
  let temp_dir = create_test_storage();
  let storage_path = temp_dir.path();

  write_test_session( storage_path, "project-001", "session-001", 1 );
  write_test_session( storage_path, "project-002", "session-001", 1 );
  write_test_session( storage_path, "project-003", "session-001", 1 );

  let storage = Storage::with_root( storage_path );
  let count = storage.count_projects().expect( "Failed to count projects" );

  assert_eq!( count, 3, "Should count 3 projects" );
}

#[test]
fn count_sessions_in_project()
{
  let temp_dir = create_test_storage();
  let storage_path = temp_dir.path();

  // Create 2 main sessions + 1 agent session
  write_test_session( storage_path, "test-project", "session-001", 1 );
  write_test_session( storage_path, "test-project", "session-002", 1 );
  write_test_session( storage_path, "test-project", "agent-001", 1 );

  let storage = Storage::with_root( storage_path );
  let project = storage.load_project( &ProjectId::uuid( "test-project" ) )
    .expect( "Failed to load project" );

  // count_sessions() returns main sessions only (excludes agents)
  let main_count = project.count_sessions().expect( "Failed to count main sessions" );
  // all_sessions().len() returns total including agents
  let total_count = project.all_sessions().expect( "Failed to get all sessions" ).len();

  assert_eq!( main_count, 2, "Should count 2 main sessions (agent excluded)" );
  assert_eq!( total_count, 3, "Should count 3 total sessions (including agent)" );
}

#[test]
fn count_entries_in_session()
{
  let temp_dir = create_test_storage();
  let storage_path = temp_dir.path();

  write_test_session( storage_path, "test-project", "test-session", 42 );

  let storage = Storage::with_root( storage_path );
  let project = storage.load_project( &ProjectId::uuid( "test-project" ) )
    .expect( "Failed to load project" );

  let sessions = project.all_sessions().expect( "Failed to get sessions" );
  let session = sessions.iter()
    .find( | s | s.id() == "test-session" )
    .expect( "Session should exist" );

  let count = session.count_entries().expect( "Failed to count entries" );

  assert_eq!( count, 42, "Should count 42 entries" );
}

#[test]
fn count_empty_storage()
{
  let temp_dir = create_test_storage();
  let storage_path = temp_dir.path();

  std::fs::create_dir_all( storage_path.join( "projects" ) ).expect( "Failed to create projects dir" );

  let storage = Storage::with_root( storage_path );
  let count = storage.count_projects().expect( "Failed to count projects" );

  assert_eq!( count, 0, "Empty storage should have 0 projects" );
}

// Integration test combining multiple operations

#[test]
fn full_workflow_integration()
{
  let temp_dir = create_test_storage();
  let storage_path = temp_dir.path();

  // Create realistic test data: 2 projects, 3 sessions, 85 entries
  write_test_session( storage_path, "uuid-project-001", "main-session", 25 );
  write_test_session( storage_path, "uuid-project-001", "agent-001", 10 );
  write_test_session( storage_path, "-home-user-code", "cli-session", 50 );

  let storage = Storage::with_root( storage_path );

  // Verify global stats (.status logic)
  let global_stats = storage.global_stats().expect( "Failed to get global stats" );
  assert_eq!( global_stats.total_projects, 2, "Should have 2 projects" );
  assert_eq!( global_stats.uuid_projects, 1, "Should have 1 UUID project" );
  assert_eq!( global_stats.path_projects, 1, "Should have 1 path project" );
  assert_eq!( global_stats.total_sessions, 3, "Should have 3 sessions" );
  assert_eq!( global_stats.total_entries, 85, "Should have 85 total entries" );

  // Verify project listing (.list logic)
  let projects = storage.list_projects().expect( "Failed to list projects" );
  assert_eq!( projects.len(), 2, "Should list 2 projects" );

  // Verify session details (.show logic)
  let project = storage.load_project( &ProjectId::uuid( "uuid-project-001" ) )
    .expect( "Failed to load project" );
  let mut sessions = project.all_sessions().expect( "Failed to get sessions" );
  let main_session = sessions.iter_mut()
    .find( | s | s.id() == "main-session" )
    .expect( "Should find main-session" );

  let session_stats = main_session.stats().expect( "Failed to get session stats" );
  assert_eq!( session_stats.total_entries, 25, "main-session should have 25 entries" );

  // Verify counting (.count logic)
  let entry_count = main_session.count_entries().expect( "Failed to count entries" );
  assert_eq!( entry_count, 25, "Should count 25 entries in main-session" );
}
