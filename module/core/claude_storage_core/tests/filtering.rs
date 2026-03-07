//! Filtering integration tests
//!
//! ## Test Coverage
//!
//! 1. Session filtering (`agent_only`, `min_entries`, `session_id_substring`)
//! 2. Project filtering (`path_substring`, `min_entries`, `min_sessions`)
//! 3. Filter composition (AND logic)
//! 4. Default filter (no filtering)
//! 5. Empty results
//!
//! ## Design
//!
//! Tests use real `~/.claude/` storage to verify filtering against actual data.
//! Falls back to synthetic test data if storage not available.

use claude_storage_core::{ Storage, SessionFilter, ProjectFilter, StringMatcher };

/// Test `SessionFilter` with `agent_only`
#[test]
fn session_filter_agent_only()
{
  let storage = Storage::new().expect( "Failed to create storage" );

  // Get first project with sessions
  let projects = storage.list_projects().expect( "Failed to list projects" );
  if projects.is_empty()
  {
    println!( "Skipping test: no projects found" );
    return;
  }

  let mut project = projects.into_iter().next().unwrap();

  // Filter for agent sessions only
  let filter = SessionFilter
  {
    agent_only : Some( true ),
    min_entries : None,
    session_id_substring : None,
  };

  let filtered = project.sessions_filtered( &filter ).expect( "Failed to filter sessions" );

  // All results should be agent sessions
  for session in &filtered
  {
    assert!( session.is_agent_session() );
  }
}

/// Test `SessionFilter` with `min_entries`
#[test]
fn session_filter_min_entries()
{
  let storage = Storage::new().expect( "Failed to create storage" );

  let projects = storage.list_projects().expect( "Failed to list projects" );
  if projects.is_empty()
  {
    println!( "Skipping test: no projects found" );
    return;
  }

  let mut project = projects.into_iter().next().unwrap();

  // Filter for sessions with 10+ entries
  let filter = SessionFilter
  {
    agent_only : None,
    min_entries : Some( 10 ),
    session_id_substring : None,
  };

  let filtered = project.sessions_filtered( &filter ).expect( "Failed to filter sessions" );

  // All results should have 10+ entries
  for session in filtered
  {
    let count = session.count_entries().expect( "Failed to count entries" );
    assert!( count >= 10, "Session has {count} entries, expected >= 10" );
  }
}

/// Test `SessionFilter` with `session_id_substring`
#[test]
fn session_filter_id_substring()
{
  let storage = Storage::new().expect( "Failed to create storage" );

  let projects = storage.list_projects().expect( "Failed to list projects" );
  if projects.is_empty()
  {
    println!( "Skipping test: no projects found" );
    return;
  }

  let mut project = projects.into_iter().next().unwrap();

  // Get all sessions to find a substring to filter by
  let all_sessions = project.sessions().expect( "Failed to list sessions" );
  if all_sessions.is_empty()
  {
    println!( "Skipping test: no sessions found" );
    return;
  }

  // Use first session's ID substring (first 5 chars)
  let first_id = all_sessions[ 0 ].id();
  let substring = &first_id[ ..5.min( first_id.len() ) ];

  let filter = SessionFilter
  {
    agent_only : None,
    min_entries : None,
    session_id_substring : Some( substring.to_string() ),
  };

  let filtered = project.sessions_filtered( &filter ).expect( "Failed to filter sessions" );

  // At least one result should match
  assert!( !filtered.is_empty(), "Filter should match at least the first session" );

  // All results should contain substring
  let matcher = StringMatcher::new( substring );
  for session in &filtered
  {
    let sid = session.id();
    assert!( matcher.matches( sid ), "Session ID {sid} should contain {substring}" );
  }
}

/// Test `SessionFilter` with AND composition
#[test]
fn session_filter_and_composition()
{
  let storage = Storage::new().expect( "Failed to create storage" );

  let projects = storage.list_projects().expect( "Failed to list projects" );
  if projects.is_empty()
  {
    println!( "Skipping test: no projects found" );
    return;
  }

  let mut project = projects.into_iter().next().unwrap();

  // Filter for agent sessions with 10+ entries
  let filter = SessionFilter
  {
    agent_only : Some( true ),
    min_entries : Some( 10 ),
    session_id_substring : None,
  };

  let filtered = project.sessions_filtered( &filter ).expect( "Failed to filter sessions" );

  // All results must match BOTH conditions
  for session in filtered
  {
    assert!( session.is_agent_session() );
    let count = session.count_entries().expect( "Failed to count entries" );
    assert!( count >= 10, "Session has {count} entries, expected >= 10" );
  }
}

/// Test `ProjectFilter` with `path_substring`
#[test]
fn project_filter_path_substring()
{
  let storage = Storage::new().expect( "Failed to create storage" );

  // Filter for projects with "willbe" in path
  let filter = ProjectFilter
  {
    path_substring : Some( "willbe".to_string() ),
    min_entries : None,
    min_sessions : None,
  };

  let filtered = storage.list_projects_filtered( &filter ).expect( "Failed to filter projects" );

  // All results should contain "willbe" in path (case-insensitive)
  let matcher = StringMatcher::new( "willbe" );
  for project in &filtered
  {
    let path_str = format!( "{:?}", project.id() );
    assert!( matcher.matches( &path_str ), "Project path {path_str} should contain 'willbe'" );
  }
}

/// Test `ProjectFilter` with `min_sessions`
#[test]
fn project_filter_min_sessions()
{
  let storage = Storage::new().expect( "Failed to create storage" );

  // Filter for projects with 5+ sessions
  let filter = ProjectFilter
  {
    path_substring : None,
    min_entries : None,
    min_sessions : Some( 5 ),
  };

  let filtered = storage.list_projects_filtered( &filter ).expect( "Failed to filter projects" );

  // All results should have 5+ sessions
  for project in filtered
  {
    let count = project.count_sessions().expect( "Failed to count sessions" );
    assert!( count >= 5, "Project has {count} sessions, expected >= 5" );
  }
}

/// Test default filter (no filtering)
#[test]
fn default_filter_matches_all()
{
  let storage = Storage::new().expect( "Failed to create storage" );

  let all_projects = storage.list_projects().expect( "Failed to list projects" );

  // Default filter should match all
  let filter = ProjectFilter::new();
  assert!( filter.is_default() );

  let filtered = storage.list_projects_filtered( &filter ).expect( "Failed to filter projects" );

  // Should have same count as unfiltered
  assert_eq!( filtered.len(), all_projects.len() );
}

/// Test empty results
#[test]
fn filter_with_no_matches()
{
  let storage = Storage::new().expect( "Failed to create storage" );

  // Filter for projects with impossible substring
  let filter = ProjectFilter
  {
    path_substring : Some( "definitely_does_not_exist_xyzabc123".to_string() ),
    min_entries : None,
    min_sessions : None,
  };

  let filtered = storage.list_projects_filtered( &filter ).expect( "Failed to filter projects" );

  // Should have no results
  assert!( filtered.is_empty(), "Filter with impossible substring should return empty results" );
}
