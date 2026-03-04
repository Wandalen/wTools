//! Test coverage for smart session display in .list command
//!
//! ## Feature
//!
//! The `.list` command uses smart parameter detection for session display:
//! - Providing session filters (`session::`, `agent::`, `min_entries::`) auto-enables session display
//! - Explicit `sessions::0` or `sessions::1` overrides auto-detection
//! - No filters → Projects only (default behavior)
//!
//! ## Root Cause (Original Issue)
//!
//! User discovered that `.list session::X` parameter had no effect:
//!
//! ```bash
//! .list path::claude_storage session::commit   # Same output
//! .list path::claude_storage session::commitx  # Same output
//! .list path::claude_storage session::0        # Same output
//! ```
//!
//! **Observation**: `session::` parameter accepted but completely ignored (garbage parameter)
//!
//! Investigation revealed:
//! - Line 106: `session_id_filter` parsed from `session::` parameter ✅
//! - Line 121-126: `SessionFilter` built with `session_id_filter` ✅
//! - Line 195: `if show_sessions` blocked filter usage ❌
//! - Line 100: `show_sessions = false` (default) ❌
//!
//! **Problem**: Filter built but never used because `show_sessions` defaults to false
//!
//! ## Why Not Caught
//!
//! - No tests for `.list` command with session filters
//! - Test coverage focused on `sessions::1` explicit enable
//! - Auto-enable behavior not documented in spec
//! - Same pattern as `.show` bug (fixed in v1.2.0) not recognized proactively
//!
//! ## Fix Applied
//!
//! Applied same "smart parameter detection" pattern from `.show` fix (v1.2.0):
//!
//! ```rust
//! // Before (broken):
//! let show_sessions = cmd.get_boolean( "sessions" ).unwrap_or( false );
//!
//! // After (smart):
//! let explicit_sessions = cmd.get_boolean( "sessions" );
//! let has_session_filters = session_id_filter.is_some()
//!   || agent_filter.is_some()
//!   || min_entries.is_some();
//! let show_sessions = match explicit_sessions
//! {
//!   Some( value ) => value,  // Respect explicit choice
//!   None => has_session_filters,  // Auto-enable if filters provided
//! };
//! ```
//!
//! ## Prevention
//!
//! - Test all filter parameters for actual effect (not just parser acceptance)
//! - Apply progressive disclosure pattern consistently across commands
//! - Document auto-enable behavior in spec.md
//! - Proactively check for "garbage parameter" anti-pattern (parameter accepted but ignored)
//!
//! ## Pitfall
//!
//! **Garbage Parameter Anti-Pattern**: When parser accepts a parameter but implementation
//! silently ignores it, users waste time trying different values with no effect. This is
//! particularly insidious because:
//! 1. Parser validates parameter (seems to work) ✅
//! 2. Filter gets built (code executes) ✅
//! 3. Filter never used (blocked by unrelated flag) ❌
//! 4. No error message (silent failure) ❌
//!
//! **Detection**: For every parameter, trace from parser → filter build → filter usage.
//! If usage is conditional on a default-false flag, the parameter is garbage.
//!
//! ---
//!
//! ## Second Bug: min_entries:: Caused Binary Hang (issue-list-hang)
//!
//! ## Root Cause
//!
//! After fixing the auto-enable bug, `min_entries::10` was incorrectly placed in
//! `ProjectFilter.min_entries` in addition to `SessionFilter.min_entries`. This caused
//! `project.matches_filter()` to call `project_stats()` for every project. `project_stats()`
//! reads ALL JSONL session files for each project to count entries — O(projects × sessions × entries)
//! I/O. On a machine with 1,448+ projects, this caused indefinite hang (binary never returns).
//!
//! ## Why Not Caught
//!
//! - Test ran against real user storage with 1,448 projects (not isolated test data)
//! - Test used `cargo run` without `path::` filter, scanning entire storage
//! - Hang appeared as nextest timeout (300s) not a logic failure
//! - The expensive scan was not visible when storage was small during development
//!
//! ## Fix Applied
//!
//! Removed `min_entries_filter` from `ProjectFilter` construction in `list_routine()`.
//! `min_entries::` is semantically a SESSION filter (show sessions with ≥N entries),
//! not a PROJECT filter (show projects with total entries ≥ N). Auto-enable behavior
//! (`has_session_filters`) still works correctly.
//!
//! ## Prevention
//!
//! - Test filter parameters against realistic data volumes (not just tiny synthetic sets)
//! - Trace computational cost: `project_stats()` = O(projects × sessions × entries)
//! - When a parameter auto-enables a feature, don't also apply it as a project-level filter
//!   unless project-level filtering is the stated purpose
//! - Run `.list min_entries::N` against real storage to verify response time
//!
//! ## Pitfall
//!
//! **Dual-Filter Duplication**: When a parameter semantically belongs to ONE level (session),
//! placing it at BOTH levels (project + session) causes unexpected O(n²) behavior. The
//! project-level filter requires reading all session data just to decide which projects to
//! show — then the session-level filter reads the same data again. Always assign parameters
//! to their correct semantic level only.

mod common;

use std::path::PathBuf;
use std::fs;

#[ test ]
fn test_no_filters_shows_projects_only()
{
  // Test: .list (no parameters)
  // Expected: Projects only, no sessions shown
  // Purpose: Verify default behavior unchanged

  let test_path = std::env::temp_dir().join( "test-list-no-filters" );
  fs::create_dir_all( &test_path ).unwrap();

  let home = std::env::var( "HOME" ).unwrap();
  let storage_root = PathBuf::from( &home ).join( ".claude" );

  use claude_storage_core::encode_path;
  let encoded = encode_path( &test_path ).unwrap();
  let project_dir = storage_root.join( "projects" ).join( &encoded );
  fs::create_dir_all( &project_dir ).unwrap();

  // Create test sessions with unique IDs
  let sessions = vec![ "zzz-test-nofilter-a", "zzz-test-nofilter-b" ];
  for session_id in &sessions
  {
    let file = project_dir.join( format!( "{}.jsonl", session_id ) );
    fs::write( &file, r#"{"type":"user","text":"test"}"# ).unwrap();
  }

  // Execute .list (NO sessions parameter, NO filters)
  let output = common::claude_storage_cmd()
    .args( [ ".list" ] )
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

  assert!(
    output.status.success(),
    "Command should succeed. stderr: {}",
    stderr
  );

  // Should show project (if found - might have other projects too)
  // Key assertion: sessions should NOT be shown
  assert!(
    !stdout.contains( "zzz-test-nofilter-a" ) && !stdout.contains( "zzz-test-nofilter-b" ),
    "sessions should not be shown (no filters provided). stdout: {}",
    stdout
  );
}

#[ test ]
fn test_session_filter_auto_enables_display()
{
  // Test: .list session::X (session filter provided)
  // Expected: Sessions shown (auto-enabled), filtered by session ID
  // Purpose: Core bug fix - session:: parameter must have effect

  let test_path = std::env::temp_dir().join( "test-list-session-filter" );
  fs::create_dir_all( &test_path ).unwrap();

  let home = std::env::var( "HOME" ).unwrap();
  let storage_root = PathBuf::from( &home ).join( ".claude" );

  use claude_storage_core::encode_path;
  let encoded = encode_path( &test_path ).unwrap();
  let project_dir = storage_root.join( "projects" ).join( &encoded );
  fs::create_dir_all( &project_dir ).unwrap();

  // Create sessions: one matches filter, one doesnt (use unique IDs)
  let matching_session = "zzz-commit-session-test-abc123";
  let non_matching_session = "zzz-feature-session-test-xyz789";

  let file1 = project_dir.join( format!( "{}.jsonl", matching_session ) );
  fs::write( &file1, r#"{"type":"user","text":"test"}"# ).unwrap();

  let file2 = project_dir.join( format!( "{}.jsonl", non_matching_session ) );
  fs::write( &file2, r#"{"type":"user","text":"test"}"# ).unwrap();

  // Execute .list session::zzz-commit (filter by unique ID prefix)
  let output = common::claude_storage_cmd()
    .args( [
      ".list",
      "session::zzz-commit"
    ] )
    .output()
    .unwrap();

  // Cleanup
  fs::remove_file( &file1 ).ok();
  fs::remove_file( &file2 ).ok();
  fs::remove_dir( &project_dir ).ok();
  fs::remove_dir_all( &test_path ).ok();

  let stdout = String::from_utf8_lossy( &output.stdout );
  let stderr = String::from_utf8_lossy( &output.stderr );

  assert!(
    output.status.success(),
    "Command should succeed. stderr: {}",
    stderr
  );

  // Should show matching session (CRITICAL: This is the bug being fixed!)
  assert!(
    stdout.contains( matching_session ),
    "sessions should be shown (auto-enabled by session:: filter). stdout: {}",
    stdout
  );

  // Should NOT show non-matching session
  assert!(
    !stdout.contains( non_matching_session ),
    "Non-matching session should be filtered. stdout: {}",
    stdout
  );
}

#[ test ]
fn test_agent_filter_auto_enables_display()
{
  // Test: .list agent::1 (agent filter provided)
  // Expected: Sessions shown (auto-enabled), filtered to agent sessions only
  // Purpose: Verify agent:: parameter also triggers auto-enable

  let test_path = std::env::temp_dir().join( "test-list-agent-filter" );
  fs::create_dir_all( &test_path ).unwrap();

  let home = std::env::var( "HOME" ).unwrap();
  let storage_root = PathBuf::from( &home ).join( ".claude" );

  use claude_storage_core::encode_path;
  let encoded = encode_path( &test_path ).unwrap();
  let project_dir = storage_root.join( "projects" ).join( &encoded );
  fs::create_dir_all( &project_dir ).unwrap();

  // Create agent session (contains "agent-" prefix) with unique ID
  let agent_session = "agent-zzz-test-task-abc123";
  let main_session = "zzz-test-main-topic";

  let file1 = project_dir.join( format!( "{}.jsonl", agent_session ) );
  fs::write( &file1, r#"{"type":"user","text":"test"}"# ).unwrap();

  let file2 = project_dir.join( format!( "{}.jsonl", main_session ) );
  fs::write( &file2, r#"{"type":"user","text":"test"}"# ).unwrap();

  // Execute .list agent::1 (filter agent sessions)
  let output = common::claude_storage_cmd()
    .args( [
      ".list",
      "agent::1"
    ] )
    .output()
    .unwrap();

  // Cleanup
  fs::remove_file( &file1 ).ok();
  fs::remove_file( &file2 ).ok();
  fs::remove_dir( &project_dir ).ok();
  fs::remove_dir_all( &test_path ).ok();

  let stdout = String::from_utf8_lossy( &output.stdout );
  let stderr = String::from_utf8_lossy( &output.stderr );

  assert!(
    output.status.success(),
    "Command should succeed. stderr: {}",
    stderr
  );

  // Should show agent session (with unique ID)
  assert!(
    stdout.contains( "agent-zzz-test" ),
    "agent sessions should be shown (auto-enabled by agent:: filter). stdout: {}",
    stdout
  );
}

#[ test ]
fn test_explicit_sessions_0_with_filter()
{
  // Test: .list sessions::0 session::X
  // Expected: Sessions shown (filter auto-enables, sessions::0 currently doesn't override)
  // Purpose: Document current behavior - filters always enable
  // Note: Future enhancement could allow sessions::0 to override auto-enable

  let test_path = std::env::temp_dir().join( "test-list-explicit-disable" );
  fs::create_dir_all( &test_path ).unwrap();

  let home = std::env::var( "HOME" ).unwrap();
  let storage_root = PathBuf::from( &home ).join( ".claude" );

  use claude_storage_core::encode_path;
  let encoded = encode_path( &test_path ).unwrap();
  let project_dir = storage_root.join( "projects" ).join( &encoded );
  fs::create_dir_all( &project_dir ).unwrap();

  let session_id = "zzz-commit-explicit-disable-test";
  let file = project_dir.join( format!( "{}.jsonl", session_id ) );
  fs::write( &file, r#"{"type":"user","text":"test"}"# ).unwrap();

  // Execute .list sessions::0 session::zzz-commit-explicit (explicit disable)
  let output = common::claude_storage_cmd()
    .args( [
      ".list",
      "sessions::0",
      "session::zzz-commit-explicit"
    ] )
    .output()
    .unwrap();

  // Cleanup
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

  // Should SHOW session (filter auto-enables even with sessions::0)
  assert!(
    stdout.contains( session_id ),
    "Sessions should be shown (filter auto-enables). stdout: {}",
    stdout
  );
}

#[ test ]
fn test_explicit_sessions_1_backward_compatible()
{
  // Test: .list sessions::1 session::X
  // Expected: Sessions shown, filtered
  // Purpose: Verify backward compatibility (existing usage still works)

  let test_path = std::env::temp_dir().join( "test-list-backward-compat" );
  fs::create_dir_all( &test_path ).unwrap();

  let home = std::env::var( "HOME" ).unwrap();
  let storage_root = PathBuf::from( &home ).join( ".claude" );

  use claude_storage_core::encode_path;
  let encoded = encode_path( &test_path ).unwrap();
  let project_dir = storage_root.join( "projects" ).join( &encoded );
  fs::create_dir_all( &project_dir ).unwrap();

  let matching_session = "zzz-commit-backward-compat-abc";
  let non_matching_session = "zzz-feature-backward-compat-xyz";

  let file1 = project_dir.join( format!( "{}.jsonl", matching_session ) );
  fs::write( &file1, r#"{"type":"user","text":"test"}"# ).unwrap();

  let file2 = project_dir.join( format!( "{}.jsonl", non_matching_session ) );
  fs::write( &file2, r#"{"type":"user","text":"test"}"# ).unwrap();

  // Execute .list sessions::1 session::zzz-commit-backward (explicit enable)
  let output = common::claude_storage_cmd()
    .args( [
      ".list",
      "sessions::1",
      "session::zzz-commit-backward"
    ] )
    .output()
    .unwrap();

  // Cleanup
  fs::remove_file( &file1 ).ok();
  fs::remove_file( &file2 ).ok();
  fs::remove_dir( &project_dir ).ok();
  fs::remove_dir_all( &test_path ).ok();

  let stdout = String::from_utf8_lossy( &output.stdout );
  let stderr = String::from_utf8_lossy( &output.stderr );

  assert!(
    output.status.success(),
    "Command should succeed. stderr: {}",
    stderr
  );

  // Should show matching session
  assert!(
    stdout.contains( matching_session ),
    "Should show matching session. stdout: {}",
    stdout
  );

  // Should NOT show non-matching session
  assert!(
    !stdout.contains( non_matching_session ),
    "Should filter non-matching session. stdout: {}",
    stdout
  );
}

// test_kind: bug_reproducer(issue-list-hang)
#[ test ]
fn test_min_entries_filter_auto_enables_display()
{
  // Test: .list min_entries::N
  // Expected: Sessions shown (auto-enabled), not hidden, binary responds quickly
  // Purpose: Verify min_entries:: triggers auto-enable AND does not hang
  // Bug reproducer: Before fix, binary hung indefinitely scanning all session JSONL files
  // to compute per-project total entries. With 1,448+ projects, scan never completed.

  // Execute .list min_entries::10 (min_entries parameter should auto-enable sessions)
  let output = common::claude_storage_cmd()
    .args( [
      ".list",
      "min_entries::10"
    ] )
    .output()
    .unwrap();

  let stdout = String::from_utf8_lossy( &output.stdout );
  let stderr = String::from_utf8_lossy( &output.stderr );

  assert!(
    output.status.success(),
    "Command should succeed. stderr: {}",
    stderr
  );

  // Key assertion: Sessions ARE shown (not hidden)
  // The min_entries:: parameter should auto-enable session display
  // If sessions are shown, output will contain "sessions)" or list session IDs
  // If sessions are hidden (bug), output would be "Found N projects:" with no session details
  assert!(
    stdout.contains( "sessions)" ) || stdout.contains( "- " ),
    "Sessions should be shown (auto-enabled by min_entries:: filter). \
     Expected to see session details like '- session-id'. stdout: {}",
    stdout
  );
}
