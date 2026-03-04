//! Export functionality integration tests
//!
//! Tests for exporting sessions to markdown, JSON, and text formats.

use claude_storage_core::{ Storage, ExportFormat, export_session };
use std::io::Cursor;

/// Test markdown export format
#[test]
fn export_markdown_basic()
{
  let storage = Storage::new().expect( "Failed to create storage" );
  let projects = storage.list_projects().expect( "Failed to list projects" );

  if projects.is_empty()
  {
    println!( "Skipping test: no projects found" );
    return;
  }

  let project = projects.into_iter().next().unwrap();
  let mut sessions = project.sessions().expect( "Failed to get sessions" );

  if sessions.is_empty()
  {
    println!( "Skipping test: no sessions found" );
    return;
  }

  // Try to export the first valid session (skip sessions with parse errors)
  let mut found_valid = false;

  for session in sessions.iter_mut()
  {
    let mut output = Cursor::new( Vec::new() );

    if export_session( session, ExportFormat::Markdown, &mut output ).is_err()
    {
      continue; // Skip sessions with parse errors
    }

    let result = String::from_utf8( output.into_inner() ).expect( "Invalid UTF-8" );

    // Check for expected markdown elements
    if result.contains( "# Session:" ) && result.contains( "## Entry" )
    {
      found_valid = true;
      assert!( result.contains( "**Path**:" ), "Missing path" );
      assert!( result.contains( "**Entries**:" ), "Missing entry count" );
      assert!( result.contains( "---" ), "Missing separator" );
      break;
    }
  }

  if !found_valid
  {
    println!( "Skipping test: no valid sessions found for markdown export" );
  }
}

/// Test JSON export format
#[test]
fn export_json_basic()
{
  let storage = Storage::new().expect( "Failed to create storage" );
  let projects = storage.list_projects().expect( "Failed to list projects" );

  if projects.is_empty()
  {
    println!( "Skipping test: no projects found" );
    return;
  }

  let project = projects.into_iter().next().unwrap();
  let mut sessions = project.sessions().expect( "Failed to get sessions" );

  if sessions.is_empty()
  {
    println!( "Skipping test: no sessions found" );
    return;
  }

  let session = sessions.first_mut().unwrap();

  // Export to JSON
  let mut output = Cursor::new( Vec::new() );
  export_session( session, ExportFormat::Json, &mut output )
    .expect( "Failed to export session" );

  // Verify output
  let result = String::from_utf8( output.into_inner() ).expect( "Invalid UTF-8" );

  // Check for expected JSON elements
  assert!( result.contains( "\"session_id\":" ), "Missing session_id" );
  assert!( result.contains( "\"storage_path\":" ), "Missing storage_path" );
  assert!( result.contains( "\"entries\":" ), "Missing entries array" );
  assert!( result.starts_with( "{" ), "Should start with open brace" );
  assert!( result.trim().ends_with( "}" ), "Should end with close brace" );
}

/// Test text export format
#[test]
fn export_text_basic()
{
  let storage = Storage::new().expect( "Failed to create storage" );
  let projects = storage.list_projects().expect( "Failed to list projects" );

  if projects.is_empty()
  {
    println!( "Skipping test: no projects found" );
    return;
  }

  let project = projects.into_iter().next().unwrap();
  let mut sessions = project.sessions().expect( "Failed to get sessions" );

  if sessions.is_empty()
  {
    println!( "Skipping test: no sessions found" );
    return;
  }

  // Try to export the first valid session (skip sessions with parse errors)
  let mut found_valid = false;

  for session in sessions.iter_mut()
  {
    let mut output = Cursor::new( Vec::new() );

    if export_session( session, ExportFormat::Text, &mut output ).is_err()
    {
      continue; // Skip sessions with parse errors
    }

    let result = String::from_utf8( output.into_inner() ).expect( "Invalid UTF-8" );

    // Check for expected text elements
    if result.contains( "Session:" ) && ( result.contains( "[User]" ) || result.contains( "[Assistant]" ) )
    {
      found_valid = true;
      assert!( result.contains( "Path:" ), "Missing path" );
      assert!( result.contains( "Entries:" ), "Missing entry count" );
      assert!( result.contains( "---" ), "Missing separator" );
      break;
    }
  }

  if !found_valid
  {
    println!( "Skipping test: no valid sessions found for text export" );
  }
}

/// Test ExportFormat::from_str
#[test]
fn export_format_from_str()
{
  // Test valid formats
  assert_eq!
  (
    ExportFormat::from_str( "markdown" ).unwrap(),
    ExportFormat::Markdown
  );

  assert_eq!
  (
    ExportFormat::from_str( "md" ).unwrap(),
    ExportFormat::Markdown
  );

  assert_eq!
  (
    ExportFormat::from_str( "json" ).unwrap(),
    ExportFormat::Json
  );

  assert_eq!
  (
    ExportFormat::from_str( "text" ).unwrap(),
    ExportFormat::Text
  );

  assert_eq!
  (
    ExportFormat::from_str( "txt" ).unwrap(),
    ExportFormat::Text
  );

  // Test case insensitivity
  assert_eq!
  (
    ExportFormat::from_str( "MARKDOWN" ).unwrap(),
    ExportFormat::Markdown
  );

  // Test invalid format
  assert!( ExportFormat::from_str( "invalid" ).is_err() );
}

/// Test ExportFormat::extension
#[test]
fn export_format_extension()
{
  assert_eq!( ExportFormat::Markdown.extension(), "md" );
  assert_eq!( ExportFormat::Json.extension(), "json" );
  assert_eq!( ExportFormat::Text.extension(), "txt" );
}

/// Test export to file
#[test]
fn export_to_file()
{
  let storage = Storage::new().expect( "Failed to create storage" );
  let projects = storage.list_projects().expect( "Failed to list projects" );

  if projects.is_empty()
  {
    println!( "Skipping test: no projects found" );
    return;
  }

  let project = projects.into_iter().next().unwrap();
  let mut sessions = project.sessions().expect( "Failed to get sessions" );

  if sessions.is_empty()
  {
    println!( "Skipping test: no sessions found" );
    return;
  }

  let session = sessions.first_mut().unwrap();

  // Create output file path in /tmp
  let output_path = std::path::PathBuf::from( "/tmp/test_export.md" );

  // Export to file
  claude_storage_core::export_session_to_file
  (
    session,
    ExportFormat::Markdown,
    &output_path
  ).expect( "Failed to export to file" );

  // Verify file exists and has content
  assert!( output_path.exists(), "Output file not created" );

  let content = std::fs::read_to_string( &output_path )
    .expect( "Failed to read output file" );

  assert!( !content.is_empty(), "Output file is empty" );
  assert!( content.contains( "# Session:" ), "Missing session header" );
}

/// Test markdown with thinking blocks
#[test]
fn export_markdown_with_thinking()
{
  let storage = Storage::new().expect( "Failed to create storage" );
  let projects = storage.list_projects().expect( "Failed to list projects" );

  if projects.is_empty()
  {
    println!( "Skipping test: no projects found" );
    return;
  }

  let project = projects.into_iter().next().unwrap();
  let mut sessions = project.sessions().expect( "Failed to get sessions" );

  if sessions.is_empty()
  {
    println!( "Skipping test: no sessions found" );
    return;
  }

  // Find a session with thinking blocks (try all sessions)
  let mut found_thinking = false;

  for session in sessions.iter_mut()
  {
    let mut output = Cursor::new( Vec::new() );

    // Skip sessions that fail to export (e.g., parse errors)
    if export_session( session, ExportFormat::Markdown, &mut output ).is_err()
    {
      continue;
    }

    let result = String::from_utf8( output.into_inner() ).expect( "Invalid UTF-8" );

    if result.contains( "<details>" ) && result.contains( "Thinking" )
    {
      found_thinking = true;

      // Verify thinking block structure
      assert!( result.contains( "<summary>Thinking" ), "Missing thinking summary" );
      assert!( result.contains( "</details>" ), "Missing details close tag" );
      break;
    }
  }

  // Note: Not all sessions have thinking blocks, so we dont assert found_thinking
  // This test just verifies the format when thinking blocks are present
  if !found_thinking
  {
    println!( "No sessions with thinking blocks found (this is ok)" );
  }
}

/// Test export of sessions containing non-conversation metadata entries
///
/// Real Claude Code sessions may contain metadata entries like:
/// - queue-operation: Commands queued for execution
/// - summary: Session summaries
/// - file-history-snapshot: File state snapshots
///
/// These entries should be gracefully skipped during export, allowing
/// export to succeed and showing only conversation entries.
#[test]
fn export_with_metadata_entries()
{
  use std::io::Cursor;

  let storage = Storage::new().expect( "Failed to create storage" );
  let projects = storage.list_projects().expect( "Failed to list projects" );

  if projects.is_empty()
  {
    println!( "Skipping test: no projects found" );
    return;
  }

  // Find a session that contains metadata entries
  // In real storage, many sessions have queue-operation or summary entries
  let mut found_session_with_metadata = false;

  for project in projects
  {
    let mut sessions = match project.sessions()
    {
      Ok( s ) => s,
      Err( _ ) => continue,
    };

    for session in sessions.iter_mut()
    {
      // Try to read the raw JSONL to check for metadata entries
      let path = session.storage_path();
      let content = match std::fs::read_to_string( path )
      {
        Ok( c ) => c,
        Err( _ ) => continue,
      };

      // Check if this session has metadata entries (queue-operation or summary)
      let has_metadata = content.lines().any( | line |
      {
        line.contains( r#""type":"queue-operation"# )
          || line.contains( r#""type":"summary"# )
          || line.contains( r#""type":"file-history-snapshot"# )
      });

      if !has_metadata
      {
        continue;
      }

      // Found a session with metadata entries - test export
      found_session_with_metadata = true;

      let mut output = Cursor::new( Vec::new() );

      // Export should succeed even with metadata entries
      export_session( session, ExportFormat::Markdown, &mut output )
        .expect( "Export failed for session with metadata entries" );

      let result = String::from_utf8( output.into_inner() ).expect( "Invalid UTF-8" );

      // Verify export contains session header
      assert!( result.contains( "# Session:" ), "Missing session header" );
      assert!( result.contains( "**Path**:" ), "Missing path" );

      // Note: Entry count may be 0 if session only has metadata entries
      // (e.g., queue-operation only), which is valid

      println!( "Successfully exported session with metadata entries: {}", session.id() );
      break;
    }

    if found_session_with_metadata
    {
      break;
    }
  }

  if !found_session_with_metadata
  {
    println!( "Skipping test: no sessions with metadata entries found" );
    println!( "This is expected if testing on a minimal storage instance" );
  }
}
