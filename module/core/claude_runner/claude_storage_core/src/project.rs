//! Project management - represents a project (directory containing sessions)

use std::
{
  fs,
  path::{ Path, PathBuf },
};

use crate::
{
  Session,
  Error,
  Result,
  stats::ProjectStats,
};

/// Identifier for a project (UUID or filesystem path)
#[derive( Debug, Clone, PartialEq, Eq )]
pub enum ProjectId
{
  /// UUID-based project (web/IDE sessions)
  Uuid( String ),

  /// Path-based project (CLI sessions)
  Path( PathBuf ),
}

/// A project (directory containing session JSONL files)
#[derive( Debug )]
pub struct Project
{
  /// Project identifier
  id : ProjectId,

  /// Storage directory path (e.g., ~/.claude/projects/{id}/)
  storage_dir : PathBuf,
}

impl Project
{
  /// Create a new project reference
  pub fn new( id : ProjectId, storage_dir : PathBuf ) -> Self
  {
    Self
    {
      id,
      storage_dir,
    }
  }

  /// Load project from storage directory
  pub fn load( storage_dir : &Path ) -> Result< Self >
  {
    if !storage_dir.exists()
    {
      return Err( Error::project_not_found
      (
        storage_dir.to_string_lossy().to_string()
      ));
    }

    if !storage_dir.is_dir()
    {
      return Err( Error::invalid_structure
      (
        storage_dir.to_path_buf(),
        "not a directory"
      ));
    }

    // Extract project ID from directory name
    let dir_name = storage_dir
      .file_name()
      .ok_or_else( || Error::invalid_structure
      (
        storage_dir.to_path_buf(),
        "missing directory name"
      ))?
      .to_str()
      .ok_or_else( || Error::invalid_structure
      (
        storage_dir.to_path_buf(),
        "directory name contains invalid UTF-8"
      ))?;

    // Determine if this is a UUID or path-based project
    let id = if dir_name.starts_with( '-' )
    {
      // Path-based project
      let decoded_path = crate::decode_path( dir_name )?;
      ProjectId::Path( decoded_path )
    }
    else
    {
      // UUID-based project
      ProjectId::Uuid( dir_name.to_string() )
    };

    Ok( Self
    {
      id,
      storage_dir : storage_dir.to_path_buf(),
    })
  }

  /// Helper to iterate session files with filtering
  ///
  /// Returns true to continue iteration, false to stop early.
  fn iter_session_files< F >( &self, include_agents : bool, mut action : F ) -> Result< () >
  where
    F : FnMut( PathBuf ) -> Result< bool >,
  {
    // Handle race condition: directory may have been deleted between list_projects()
    // and when we try to read it (e.g., concurrent test cleanup). Treat as empty.
    let entries = match fs::read_dir( &self.storage_dir )
    {
      Ok( entries ) => entries,
      Err( e ) if e.kind() == std::io::ErrorKind::NotFound =>
      {
        // Directory was deleted - treat as empty project (race condition)
        return Ok( () );
      }
      Err( e ) =>
      {
        return Err( Error::io( e, format!( "reading project directory: {:?}", self.storage_dir ) ) );
      }
    };

    for entry in entries
    {
      let entry = entry.map_err( | e | Error::io
      (
        e,
        format!( "reading directory entry in: {:?}", self.storage_dir )
      ))?;

      let path = entry.path();

      // Only include .jsonl files
      if path.extension().and_then( | s | s.to_str() ) == Some( "jsonl" )
      {
        // Apply agent session filter
        if !include_agents
        {
          if let Some( filename ) = path.file_name().and_then( | s | s.to_str() )
          {
            if filename.starts_with( "agent-" )
            {
              continue;
            }
          }
        }

        // Execute action callback, stop if it returns false
        if !action( path )?
        {
          break;
        }
      }
    }

    Ok( () )
  }

  /// Get project ID
  pub fn id( &self ) -> &ProjectId
  {
    &self.id
  }

  /// Get storage directory path
  pub fn storage_dir( &self ) -> &Path
  {
    &self.storage_dir
  }

  /// List all sessions in this project
  pub fn sessions( &self ) -> Result< Vec< Session > >
  {
    let mut sessions = Vec::new();

    self.iter_session_files( false, | path |
    {
      match Session::load( &path )
      {
        Ok( session ) => sessions.push( session ),
        Err( e ) => eprintln!( "Warning: Failed to load session {:?}: {}", path, e ),
      }
      Ok( true )
    })?;

    Ok( sessions )
  }

  /// Check if project has any sessions
  pub fn has_sessions( &self ) -> Result< bool >
  {
    let mut found = false;

    self.iter_session_files( false, | _path |
    {
      found = true;
      Ok( false ) // Stop early after finding first session
    })?;

    Ok( found )
  }

  /// Count sessions without loading them
  pub fn count_sessions( &self ) -> Result< usize >
  {
    let mut count = 0;

    self.iter_session_files( false, | _path |
    {
      count += 1;
      Ok( true )
    })?;

    Ok( count )
  }

  /// List all sessions including agent sessions
  pub fn all_sessions( &self ) -> Result< Vec< Session > >
  {
    let mut sessions = Vec::new();

    self.iter_session_files( true, | path |
    {
      match Session::load( &path )
      {
        Ok( session ) => sessions.push( session ),
        Err( e ) => eprintln!( "Warning: Failed to load session {:?}: {}", path, e ),
      }
      Ok( true )
    })?;

    Ok( sessions )
  }

  /// Compute project statistics
  ///
  /// Aggregates statistics from all sessions (main and agent) in this project.
  pub fn project_stats( &self ) -> Result< ProjectStats >
  {
    let project_id = match &self.id
    {
      ProjectId::Uuid( uuid ) => uuid.clone(),
      ProjectId::Path( path ) => path.to_string_lossy().to_string(),
    };

    let mut stats = ProjectStats::new( project_id );

    // Get all sessions including agents
    let mut all_sessions = self.all_sessions()?;

    stats.session_count = all_sessions.len();

    // Compute stats for each session
    for session in &mut all_sessions
    {
      if session.is_agent_session()
      {
        stats.agent_session_count += 1;
      }
      else
      {
        stats.main_session_count += 1;
      }

      // Get session stats (skip corrupted sessions with warning)
      match session.stats()
      {
        Ok( session_stats ) =>
        {
          stats.total_entries += session_stats.total_entries;
          stats.total_input_tokens += session_stats.total_input_tokens;
          stats.total_output_tokens += session_stats.total_output_tokens;
        },
        Err( e ) =>
        {
          eprintln!( "Warning: Skipping corrupted session {:?}: {}", session.storage_path(), e );
        }
      }
    }

    Ok( stats )
  }

  /// List sessions matching filter
  ///
  /// ## Filtering Logic
  ///
  /// Returns only sessions that match ALL filter conditions (AND logic):
  /// - `agent_only`: Filter by agent/main session type
  /// - `min_entries`: Minimum entry count
  /// - `session_id_substring`: Session ID substring match (case-insensitive)
  ///
  /// ## Examples
  ///
  /// ```rust,no_run
  /// use claude_storage_core::{ Storage, SessionFilter };
  ///
  /// let storage = Storage::new().unwrap();
  /// let mut project = storage.list_projects().unwrap().into_iter().next().unwrap();
  ///
  /// // Filter for agent sessions with 10+ entries
  /// let filter = SessionFilter
  /// {
  ///   agent_only : Some( true ),
  ///   min_entries : Some( 10 ),
  ///   session_id_substring : None,
  /// };
  ///
  /// let sessions = project.sessions_filtered( &filter ).unwrap();
  /// ```
  pub fn sessions_filtered( &mut self, filter : &crate::SessionFilter ) -> Result< Vec< Session > >
  {
    // Optimization: skip filtering if default filter
    if filter.is_default()
    {
      return self.all_sessions();
    }

    let all_sessions = self.all_sessions()?;
    let mut filtered = Vec::new();

    for mut session in all_sessions
    {
      if session.matches_filter( filter )?
      {
        filtered.push( session );
      }
    }

    Ok( filtered )
  }

  /// Check if project matches filter
  ///
  /// ## Filtering Logic
  ///
  /// All filter conditions must match (AND logic):
  /// - `path_substring`: Path substring match (case-insensitive)
  /// - `min_entries`: Minimum total entries across all sessions
  /// - `min_sessions`: Minimum session count
  ///
  /// ## Examples
  ///
  /// ```rust,no_run
  /// use claude_storage_core::{ Storage, ProjectFilter };
  ///
  /// let storage = Storage::new().unwrap();
  /// let project = storage.list_projects().unwrap().into_iter().next().unwrap();
  ///
  /// let filter = ProjectFilter
  /// {
  ///   path_substring : Some( "willbe".to_string() ),
  ///   min_entries : None,
  ///   min_sessions : Some( 5 ),
  /// };
  ///
  /// assert!( project.matches_filter( &filter ).unwrap() );
  /// ```
  pub fn matches_filter( &self, filter : &crate::ProjectFilter ) -> Result< bool >
  {
    // Path substring filter
    if let Some( ref substring ) = filter.path_substring
    {
      let path_str = match &self.id
      {
        ProjectId::Uuid( uuid ) => uuid.clone(),
        ProjectId::Path( path ) => path.to_string_lossy().to_string(),
      };

      let matcher = crate::StringMatcher::new( substring );
      if !matcher.matches( &path_str )
      {
        return Ok( false );
      }
    }

    // Minimum sessions filter
    if let Some( min_sessions ) = filter.min_sessions
    {
      let count = self.count_sessions()?;
      if count < min_sessions
      {
        return Ok( false );
      }
    }

    // Minimum entries filter (requires aggregation)
    if let Some( min_entries ) = filter.min_entries
    {
      let stats = self.project_stats()?;
      if stats.total_entries < min_entries
      {
        return Ok( false );
      }
    }

    Ok( true )
  }
}

impl ProjectId
{
  /// Create a path-based project ID
  pub fn path< P : Into< PathBuf > >( path : P ) -> Self
  {
    ProjectId::Path( path.into() )
  }

  /// Create a UUID-based project ID
  pub fn uuid< S : Into< String > >( uuid : S ) -> Self
  {
    ProjectId::Uuid( uuid.into() )
  }

  /// Get the original path (if path-based project)
  pub fn as_path( &self ) -> Option< &Path >
  {
    match self
    {
      ProjectId::Path( path ) => Some( path ),
      ProjectId::Uuid( _ ) => None,
    }
  }

  /// Get the UUID (if UUID-based project)
  pub fn as_uuid( &self ) -> Option< &str >
  {
    match self
    {
      ProjectId::Uuid( uuid ) => Some( uuid ),
      ProjectId::Path( _ ) => None,
    }
  }
}

#[cfg( test )]
mod tests
{
  use super::*;

  #[test]
  fn test_project_id_path()
  {
    let id = ProjectId::path( "/home/user/project" );
    assert!( id.as_path().is_some() );
    assert!( id.as_uuid().is_none() );
  }

  #[test]
  fn test_project_id_uuid()
  {
    let id = ProjectId::uuid( "550e8400-e29b-41d4-a716-446655440000" );
    assert!( id.as_uuid().is_some() );
    assert!( id.as_path().is_none() );
  }

  #[test]
  fn test_project_new()
  {
    let id = ProjectId::path( "/home/user/project" );
    let project = Project::new( id, PathBuf::from( "/tmp/storage" ) );

    assert!( project.id().as_path().is_some() );
  }
}
