//! Search functionality tests
//!
//! Tests for content search across sessions.

use claude_storage_core::{ Storage, SearchFilter, EntryType };

#[test]
fn search_basic_case_insensitive()
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

  let mut session = sessions.remove( 0 );

  // Search for common word (case-insensitive by default)
  let filter = SearchFilter::new( "the" );
  let matches = session.search( &filter ).expect( "Search failed" );

  // Should find at least some matches in typical conversation
  if !matches.is_empty()
  {
    // Verify case-insensitive matching
    for m in &matches
    {
      let line = m.full_line().to_lowercase();
      assert!( line.contains( "the" ), "Match should contain 'the': {}", m.full_line() );
    }
  }
}

#[test]
fn search_case_sensitive()
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

  let mut session = sessions.remove( 0 );

  // Search case-sensitive for capitalized word
  let filter = SearchFilter::new( "The" ).case_sensitive( true );
  let matches = session.search( &filter ).expect( "Search failed" );

  // All matches must have exact case
  for m in &matches
  {
    assert!( m.full_line().contains( "The" ), "Case-sensitive match failed: {}", m.full_line() );
    assert!( !m.full_line().eq( &m.full_line().to_lowercase() ) || m.full_line().contains( "The" ) );
  }
}

#[test]
fn search_filter_by_entry_type()
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

  let mut session = sessions.remove( 0 );

  // Search only in user messages
  let filter = SearchFilter::new( "help" ).match_entry_type( EntryType::User );
  let matches = session.search( &filter ).expect( "Search failed" );

  // All matches must be from user entries
  for m in &matches
  {
    assert_eq!( m.entry_type(), EntryType::User, "Expected user entry type" );
  }

  // Search only in assistant messages
  let filter = SearchFilter::new( "the" ).match_entry_type( EntryType::Assistant );
  let matches = session.search( &filter ).expect( "Search failed" );

  // All matches must be from assistant entries
  for m in &matches
  {
    assert_eq!( m.entry_type(), EntryType::Assistant, "Expected assistant entry type" );
  }
}

#[test]
fn search_no_matches()
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

  let mut session = sessions.remove( 0 );

  // Search for extremely unlikely string
  let filter = SearchFilter::new( "xyzabc123definitely_not_in_conversation" );
  let matches = session.search( &filter ).expect( "Search failed" );

  // Should have no matches
  assert!( matches.is_empty(), "Expected no matches for unlikely query" );
}

#[test]
fn search_match_metadata()
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

  let mut session = sessions.remove( 0 );

  let filter = SearchFilter::new( "the" );
  let matches = session.search( &filter ).expect( "Search failed" );

  if !matches.is_empty()
  {
    // Verify match metadata is populated correctly
    for m in &matches
    {
      // Entry index should be non-negative
      let _entry_idx = m.entry_index();

      // Line number should be non-negative
      let _line_num = m.line_number();

      // Entry type should be valid
      let entry_type = m.entry_type();
      assert!( entry_type == EntryType::User || entry_type == EntryType::Assistant );

      // Excerpt should not be empty
      assert!( !m.excerpt().is_empty(), "Excerpt should not be empty" );

      // Full line should not be empty
      assert!( !m.full_line().is_empty(), "Full line should not be empty" );

      // Excerpt should be equal to full line if line is short enough
      if m.full_line().chars().count() <= 150
      {
        assert_eq!( m.excerpt(), m.full_line(), "Short lines should have full excerpt" );
      }
      else
      {
        // Long lines should have truncated excerpt with ellipsis markers
        assert!( m.excerpt().starts_with( "..." ), "Long line excerpt should start with ..." );
        assert!( m.excerpt().ends_with( "..." ), "Long line excerpt should end with ..." );
      }
    }
  }
}

#[test]
fn search_empty_query()
{
  let storage = Storage::new().expect( "Failed to create storage" );
  let projects = storage.list_projects().expect( "Failed to list projects" );

  if projects.is_empty()
  {
    println!( "Skipping test: no projects found" );
    return;
  }

  // Find a session with valid, parseable entries
  let mut found_session = None;
  for project in projects
  {
    let sessions = project.sessions().expect( "Failed to get sessions" );
    for mut session in sessions
    {
      let entry_count = session.count_entries().unwrap_or( 0 );
      if entry_count == 0
      {
        continue;
      }

      // Try empty search to see if entries are parseable
      let filter = SearchFilter::new( "the" );
      let test_matches = session.search( &filter ).unwrap_or_default();

      if !test_matches.is_empty()
      {
        found_session = Some( session );
        break;
      }
    }
    if found_session.is_some()
    {
      break;
    }
  }

  if found_session.is_none()
  {
    println!( "Skipping test: no sessions with parseable entries found" );
    return;
  }

  let mut session = found_session.unwrap();

  // Empty query should match everything (like StringMatcher)
  let filter = SearchFilter::new( "" );
  let matches = session.search( &filter ).expect( "Search failed" );

  // Should match all non-empty lines
  assert!( !matches.is_empty(), "Empty query should match all content" );
}

#[test]
fn search_multiple_sessions()
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

  if sessions.len() < 2
  {
    println!( "Skipping test: need at least 2 sessions" );
    return;
  }

  let filter = SearchFilter::new( "the" );

  // Search across multiple sessions
  for session in &mut sessions
  {
    let matches = session.search( &filter ).expect( "Search failed" );

    // Verify matches are valid for each session
    for m in &matches
    {
      assert!( !m.full_line().is_empty() );
      assert!( m.full_line().to_lowercase().contains( "the" ) );
    }
  }
}
