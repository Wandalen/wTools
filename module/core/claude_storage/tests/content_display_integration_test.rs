//! Integration test for content-first display (REQ-011)
//!
//! ## Test Organization
//!
//! **Root Cause**: Users were seeing UUIDs instead of conversation content
//!
//! **Why Not Caught**: No prior tests for default display behavior
//!
//! **Fix Applied**: REQ-011 implements content-first display paradigm
//!
//! **Prevention**: This test validates actual CLI output format
//!
//! **Pitfall**: Always test the user-facing output, not just internal functions
//!
//! This integration test validates that:
//! 1. Default `.show session_id::X` displays conversation content
//! 2. `metadata::1` parameter shows only metadata (old behavior)
//! 3. Content format is readable (timestamps, role labels, text)

mod common;

use std::path::Path;

/// Test: .show displays conversation content by default (not UUIDs)
///
/// ## Test Organization
///
/// **Root Cause**: Default behavior showed UUIDs, not content
///
/// **Why Not Caught**: No integration tests for CLI output format
///
/// **Fix Applied**: Content-first display in `format_session_output`
///
/// **Prevention**: Test actual CLI invocation and output parsing
///
/// **Pitfall**: Integration tests must use real storage, not mocks
#[test]
fn show_displays_content_by_default()
{
  // Skip if no real storage available
  let storage_path = Path::new( "/home/user1/.claude/projects" );
  if !storage_path.exists()
  {
    println!( "Skipping test: ~/.claude/projects not found" );
    return;
  }

  // Find any session to test with
  // This test assumes there's at least one session in storage
  // If this assumption is wrong, the test is skipped (not failed)

  let output = common::claude_storage_cmd()
    .args( [ ".list", "sessions::1", "verbosity::0" ] )
    .output()
    .expect( "Failed to execute .list" );

  let list_output = String::from_utf8_lossy( &output.stdout );

  // Extract first NON-EMPTY project path and session ID from output
  // Format: "Path(\"/path/to/project\")\n  - {session-id} (N entries) ..."
  // Fix(issue-004): Skip sessions with 0 entries
  //
  // Root cause: Test assumed any found session would have content, but real storage
  // can have empty sessions (0 entries), causing test to fail when trying to validate
  // chat-log format on empty output.
  //
  // Pitfall: Integration tests using real data must handle all real-world cases,
  // including edge cases like empty sessions. Always validate assumptions.
  let lines : Vec< &str > = list_output.lines().collect();
  let mut project_path = None;
  let mut session_id = None;

  for ( i, line ) in lines.iter().enumerate()
  {
    if line.starts_with( "Path(" ) && i + 1 < lines.len()
    {
      // Extract project path from Path("...") format
      if let Some( start ) = line.find( '"' )
      {
        if let Some( end ) = line.rfind( '"' )
        {
          let path = &line[ start + 1..end ];

          // Check next line for session ID with non-zero entries
          let next_line = lines[ i + 1 ];
          if next_line.trim().starts_with( '-' )
          {
            let parts : Vec< &str > = next_line.split_whitespace().collect();
            // Format: "  - session-id (N entries) ..."
            // parts[0] = "-", parts[1] = session-id, parts[2] = "(N", parts[3] = "entries)"
            if parts.len() >= 4
            {
              // Check if session has entries (skip "(0 entries)")
              if let Some( count_str ) = parts.get( 2 )
              {
                if count_str.starts_with( "(0" )
                {
                  // Skip empty sessions, continue searching
                  continue;
                }
              }

              // Found non-empty session
              project_path = Some( path );
              session_id = Some( parts[ 1 ] );
              break;
            }
          }
        }
      }
    }
  }

  if session_id.is_none() || project_path.is_none()
  {
    println!( "Skipping test: No non-empty sessions found in storage" );
    return;
  }

  let session_id = session_id.unwrap();
  let project_path = project_path.unwrap();
  println!( "Testing with session: {session_id} in project: {project_path}" );

  // Execute .show session_id::X project::Y (default behavior - should show content)
  let output = common::claude_storage_cmd()
    .args( [ ".show", &format!( "session_id::{session_id}" ), &format!( "project::{project_path}" ) ] )
    .output()
    .expect( "Failed to execute .show" );

  let show_output = String::from_utf8_lossy( &output.stdout );

  println!( "Output (first 500 chars):\n{}", &show_output[ ..show_output.len().min( 500 ) ] );

  // Validate content-first display
  assert!( show_output.contains( "Session:" ), "Should show session header" );
  assert!( show_output.contains( "━" ), "Should show separator line" );

  // Should NOT show metadata fields (old behavior)
  // Old format had "Path:", "Agent Session:", "Total Entries:" etc
  // New format should show conversation content instead

  // Validate chat-log format elements
  // Looking for patterns like "[YYYY-MM-DD HH:MM] User:" or "[...] Assistant:"
  let has_timestamp_pattern = show_output.contains( "[20" ) && ( show_output.contains( "] User:" ) || show_output.contains( "] Assistant:" ) );

  assert!( has_timestamp_pattern, "Should contain chat-log format with timestamps and roles" );

  // Ensure it's NOT showing UUIDs as the main content
  // Old broken format: "1. [User] uuid (timestamp)"
  // This pattern should NOT appear in default output
  let has_uuid_list_pattern = show_output.lines().any( | line |
  {
    line.trim().chars().next().is_some_and( | c | c.is_ascii_digit() )
      && line.contains( "[User]" )
      && line.contains( '-' ) // UUIDs contain hyphens
      && !line.contains( ':') // Chat-log format has "User:" not "[User]"
  });

  assert!( !has_uuid_list_pattern, "Should NOT show UUID list format (old entries::1 behavior)" );
}

/// Test: `metadata::1` parameter shows only metadata (old behavior)
///
/// ## Test Organization
///
/// **Root Cause**: Need backward compatibility for metadata-only view
///
/// **Why Not Caught**: New parameter, needs integration test
///
/// **Fix Applied**: `metadata::1` preserves old behavior
///
/// **Prevention**: Test all parameter combinations
///
/// **Pitfall**: Backward compat modes must be tested, not just new defaults
#[test]
fn show_metadata_only_parameter()
{
  // Skip if no real storage available
  let storage_path = Path::new( "/home/user1/.claude/projects" );
  if !storage_path.exists()
  {
    println!( "Skipping test: ~/.claude/projects not found" );
    return;
  }

  // Find any session to test with
  let output = common::claude_storage_cmd()
    .args( [ ".list", "sessions::1", "verbosity::0" ] )
    .output()
    .expect( "Failed to execute .list" );

  let list_output = String::from_utf8_lossy( &output.stdout );

  // Extract first NON-EMPTY project path and session ID from output
  // Fix(issue-004): Same fix as show_displays_content_by_default - skip empty sessions
  let lines : Vec< &str > = list_output.lines().collect();
  let mut project_path = None;
  let mut session_id = None;

  for ( i, line ) in lines.iter().enumerate()
  {
    if line.starts_with( "Path(" ) && i + 1 < lines.len()
    {
      // Extract project path from Path("...") format
      if let Some( start ) = line.find( '"' )
      {
        if let Some( end ) = line.rfind( '"' )
        {
          let path = &line[ start + 1..end ];

          // Check next line for session ID with non-zero entries
          let next_line = lines[ i + 1 ];
          if next_line.trim().starts_with( '-' )
          {
            let parts : Vec< &str > = next_line.split_whitespace().collect();
            if parts.len() >= 4
            {
              // Skip empty sessions
              if let Some( count_str ) = parts.get( 2 )
              {
                if count_str.starts_with( "(0" )
                {
                  continue;
                }
              }

              // Found non-empty session
              project_path = Some( path );
              session_id = Some( parts[ 1 ] );
              break;
            }
          }
        }
      }
    }
  }

  if session_id.is_none() || project_path.is_none()
  {
    println!( "Skipping test: No non-empty sessions found in storage" );
    return;
  }

  let session_id = session_id.unwrap();
  let project_path = project_path.unwrap();
  println!( "Testing metadata::1 with session: {session_id} in project: {project_path}" );

  // Execute .show session_id::X project::Y metadata::1 (old behavior)
  let output = common::claude_storage_cmd()
    .args( [ ".show", &format!( "session_id::{session_id}" ), &format!( "project::{project_path}" ), "metadata::1" ] )
    .output()
    .expect( "Failed to execute .show metadata::1" );

  let show_output = String::from_utf8_lossy( &output.stdout );

  println!( "Metadata output:\n{show_output}" );

  // Validate metadata-only display (old behavior)
  assert!( show_output.contains( "Session:" ), "Should show session header" );
  assert!( show_output.contains( "Path:" ), "Should show path (metadata field)" );
  assert!( show_output.contains( "Total Entries:" ), "Should show entry count (metadata field)" );

  // Should NOT show separator lines (content-first feature)
  assert!( !show_output.contains( "━" ), "Should NOT show separator (content-first feature)" );

  // Should NOT show chat-log format
  let has_chat_format = show_output.contains( "] User:" ) || show_output.contains( "] Assistant:" );
  assert!( !has_chat_format, "Should NOT show chat-log format with metadata::1" );
}

/// Test: `verbosity::0` is equivalent to `metadata::1`
///
/// ## Test Organization
///
/// **Root Cause**: `verbosity::0` should show minimal metadata only
///
/// **Why Not Caught**: New verbosity semantics need validation
///
/// **Fix Applied**: `verbosity::0` triggers metadata-only mode
///
/// **Prevention**: Test verbosity level equivalences
///
/// **Pitfall**: Don't have multiple ways to specify same behavior without tests
#[test]
fn show_verbosity_zero_is_metadata_only()
{
  // Skip if no real storage available
  let storage_path = Path::new( "/home/user1/.claude/projects" );
  if !storage_path.exists()
  {
    println!( "Skipping test: ~/.claude/projects not found" );
    return;
  }

  // Find any session to test with
  let output = common::claude_storage_cmd()
    .args( [ ".list", "sessions::1", "verbosity::0" ] )
    .output()
    .expect( "Failed to execute .list" );

  let list_output = String::from_utf8_lossy( &output.stdout );

  // Extract first NON-EMPTY project path and session ID from output
  // Fix(issue-004): Same fix as show_displays_content_by_default - skip empty sessions
  let lines : Vec< &str > = list_output.lines().collect();
  let mut project_path = None;
  let mut session_id = None;

  for ( i, line ) in lines.iter().enumerate()
  {
    if line.starts_with( "Path(" ) && i + 1 < lines.len()
    {
      // Extract project path from Path("...") format
      if let Some( start ) = line.find( '"' )
      {
        if let Some( end ) = line.rfind( '"' )
        {
          let path = &line[ start + 1..end ];

          // Check next line for session ID with non-zero entries
          let next_line = lines[ i + 1 ];
          if next_line.trim().starts_with( '-' )
          {
            let parts : Vec< &str > = next_line.split_whitespace().collect();
            if parts.len() >= 4
            {
              // Skip empty sessions
              if let Some( count_str ) = parts.get( 2 )
              {
                if count_str.starts_with( "(0" )
                {
                  continue;
                }
              }

              // Found non-empty session
              project_path = Some( path );
              session_id = Some( parts[ 1 ] );
              break;
            }
          }
        }
      }
    }
  }

  if session_id.is_none() || project_path.is_none()
  {
    println!( "Skipping test: No non-empty sessions found in storage" );
    return;
  }

  let session_id = session_id.unwrap();
  let project_path = project_path.unwrap();

  // Execute .show session_id::X project::Y verbosity::0
  let output = common::claude_storage_cmd()
    .args( [ ".show", &format!( "session_id::{session_id}" ), &format!( "project::{project_path}" ), "verbosity::0" ] )
    .output()
    .expect( "Failed to execute .show verbosity::0" );

  let show_output = String::from_utf8_lossy( &output.stdout );

  // Should behave like metadata::1
  assert!( show_output.contains( "Session:" ), "Should show session header" );
  assert!( show_output.contains( "Path:" ) || show_output.contains( "Total Entries:" ), "Should show metadata fields" );

  // Should NOT show content
  let has_chat_format = show_output.contains( "] User:" ) || show_output.contains( "] Assistant:" );
  assert!( !has_chat_format, "verbosity::0 should NOT show chat-log format" );
}
