//! Main storage interface - entry point for all storage operations

use std::
{
  env,
  fs,
  path::{ Path, PathBuf },
};

use crate::
{
  Project,
  ProjectId,
  Error,
  Result,
  stats::GlobalStats,
};

/// Main storage interface for Claude Code's filesystem database
#[derive( Debug )]
pub struct Storage
{
  /// Root storage directory (default: ~/.claude/)
  root : PathBuf,
}

impl Storage
{
  /// Create a new storage interface using default location (~/.claude/)
  pub fn new() -> Result< Self >
  {
    let home = env::var( "HOME" )
      .map_err( | e | Error::io
      (
        std::io::Error::new
        (
          std::io::ErrorKind::NotFound,
          format!( "HOME environment variable not set: {}", e )
        ),
        "resolving HOME directory"
      ))?;

    let root = PathBuf::from( home ).join( ".claude" );

    Ok( Self { root })
  }

  /// Create a storage interface with custom root directory
  pub fn with_root< P : Into< PathBuf > >( root : P ) -> Self
  {
    Self
    {
      root : root.into(),
    }
  }

  /// Get root directory path
  pub fn root( &self ) -> &Path
  {
    &self.root
  }

  /// Get projects directory path
  pub fn projects_dir( &self ) -> PathBuf
  {
    self.root.join( "projects" )
  }

  /// List all projects in storage
  pub fn list_projects( &self ) -> Result< Vec< Project > >
  {
    let projects_dir = self.projects_dir();

    if !projects_dir.exists()
    {
      return Ok( Vec::new() );
    }

    let entries = fs::read_dir( &projects_dir )
      .map_err( | e | Error::io
      (
        e,
        format!( "reading projects directory: {:?}", projects_dir )
      ))?;

    let mut projects = Vec::new();

    for entry in entries
    {
      let entry = entry.map_err( | e | Error::io
      (
        e,
        format!( "reading directory entry in: {:?}", projects_dir )
      ))?;

      let path = entry.path();

      if path.is_dir()
      {
        match Project::load( &path )
        {
          Ok( project ) => projects.push( project ),
          Err( e ) => eprintln!( "Warning: Failed to load project {:?}: {}", path, e ),
        }
      }
    }

    Ok( projects )
  }

  /// Load a specific project by ID
  pub fn load_project( &self, id : &ProjectId ) -> Result< Project >
  {
    let storage_dir = match id
    {
      ProjectId::Uuid( uuid ) =>
      {
        self.projects_dir().join( uuid )
      }
      ProjectId::Path( path ) =>
      {
        let encoded = crate::encode_path( path )?;
        self.projects_dir().join( encoded )
      }
    };

    Project::load( &storage_dir )
  }

  /// Load project for current working directory
  pub fn load_project_for_cwd( &self ) -> Result< Project >
  {
    let cwd = env::current_dir()
      .map_err( | e | Error::io( e, "getting current directory" ))?;

    // Try exact path first
    match self.load_project( &ProjectId::path( &cwd ) )
    {
      Ok( project ) => Ok( project ),
      Err( _ ) =>
      {
        // If exact path fails, look for topic subdirectories (e.g., /-default_topic, /-commit)
        if let Ok( entries ) = std::fs::read_dir( &cwd )
        {
          // Collect all topic subdirectories starting with hyphen
          let mut topic_dirs : Vec< PathBuf > = entries
            .filter_map( | entry | entry.ok() )
            .filter( | entry |
            {
              if let Ok( file_name ) = entry.file_name().into_string()
              {
                file_name.starts_with( '-' ) && entry.path().is_dir()
              }
              else
              {
                false
              }
            })
            .map( | entry | entry.path() )
            .collect();

          // Sort to prefer -default_topic over other topics
          topic_dirs.sort_by( | a, b |
          {
            let a_name = a.file_name().and_then( | n | n.to_str() ).unwrap_or( "" );
            let b_name = b.file_name().and_then( | n | n.to_str() ).unwrap_or( "" );

            // Prioritize -default_topic
            match ( a_name, b_name )
            {
              ( "-default_topic", _ ) => std::cmp::Ordering::Less,
              ( _, "-default_topic" ) => std::cmp::Ordering::Greater,
              _ => a_name.cmp( b_name ),
            }
          });

          // Try each topic directory
          for topic_dir in topic_dirs
          {
            if let Ok( project ) = self.load_project( &ProjectId::path( &topic_dir ) )
            {
              return Ok( project );
            }
          }
        }

        // If no topic directories found or none have projects, return original error
        Err( Error::project_not_found( format!( "No project found for directory: {}", cwd.display() ) ) )
      }
    }
  }

  /// Load project for a specific filesystem path
  pub fn load_project_for_path< P : AsRef< Path > >( &self, path : P ) -> Result< Project >
  {
    let path = path.as_ref();
    self.load_project( &ProjectId::path( path ) )
  }

  /// Check if a project exists for the given path
  pub fn has_project_for_path< P : AsRef< Path > >( &self, path : P ) -> bool
  {
    let path = path.as_ref();

    match crate::encode_path( path )
    {
      Ok( encoded ) =>
      {
        let storage_dir = self.projects_dir().join( encoded );
        storage_dir.exists() && storage_dir.is_dir()
      }
      Err( _ ) => false,
    }
  }

  /// Check if a project has any sessions
  pub fn has_sessions_for_path< P : AsRef< Path > >( &self, path : P ) -> bool
  {
    match self.load_project_for_path( path )
    {
      Ok( project ) => project.has_sessions().unwrap_or( false ),
      Err( _ ) => false,
    }
  }

  /// Count total projects
  pub fn count_projects( &self ) -> Result< usize >
  {
    let projects_dir = self.projects_dir();

    if !projects_dir.exists()
    {
      return Ok( 0 );
    }

    let entries = fs::read_dir( &projects_dir )
      .map_err( | e | Error::io
      (
        e,
        format!( "reading projects directory: {:?}", projects_dir )
      ))?;

    let mut count = 0;

    for entry in entries
    {
      let entry = entry.map_err( | e | Error::io
      (
        e,
        format!( "reading directory entry in: {:?}", projects_dir )
      ))?;

      if entry.path().is_dir()
      {
        count += 1;
      }
    }

    Ok( count )
  }

  /// Compute global statistics across all projects
  ///
  /// Aggregates statistics from all projects, sessions, and entries in storage.
  /// This provides a comprehensive overview of Claude Code usage.
  pub fn global_stats( &self ) -> Result< GlobalStats >
  {
    let mut stats = GlobalStats::new();

    let projects = self.list_projects()?;
    stats.total_projects = projects.len();

    // Count UUID vs path projects
    for project in &projects
    {
      match project.id()
      {
        ProjectId::Uuid( _ ) => stats.uuid_projects += 1,
        ProjectId::Path( _ ) => stats.path_projects += 1,
      }
    }

    // Aggregate stats from each project
    for project in projects
    {
      let project_stats = project.project_stats()?;

      stats.total_sessions += project_stats.session_count;
      stats.main_sessions += project_stats.main_session_count;
      stats.agent_sessions += project_stats.agent_session_count;
      stats.total_entries += project_stats.total_entries;
      stats.total_input_tokens += project_stats.total_input_tokens;
      stats.total_output_tokens += project_stats.total_output_tokens;

      // Count user vs assistant entries (requires loading entries)
      // For now, we approximate this from total entries
      // This can be optimized later by tracking in ProjectStats

      stats.project_breakdown.insert( project_stats.project_id.clone(), project_stats );
    }

    Ok( stats )
  }

  /// List all path-based projects (excludes UUID projects)
  pub fn list_path_projects( &self ) -> Result< Vec< Project > >
  {
    let all_projects = self.list_projects()?;

    Ok
    (
      all_projects
        .into_iter()
        .filter( | p | matches!( p.id(), ProjectId::Path( _ ) ) )
        .collect()
    )
  }

  /// List all UUID-based projects (excludes path projects)
  pub fn list_uuid_projects( &self ) -> Result< Vec< Project > >
  {
    let all_projects = self.list_projects()?;

    Ok
    (
      all_projects
        .into_iter()
        .filter( | p | matches!( p.id(), ProjectId::Uuid( _ ) ) )
        .collect()
    )
  }

  /// List projects matching filter
  ///
  /// ## Filtering Logic
  ///
  /// Returns only projects that match ALL filter conditions (AND logic):
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
  ///
  /// // Filter for projects with "willbe" in path and 5+ sessions
  /// let filter = ProjectFilter
  /// {
  ///   path_substring : Some( "willbe".to_string() ),
  ///   min_entries : None,
  ///   min_sessions : Some( 5 ),
  /// };
  ///
  /// let projects = storage.list_projects_filtered( &filter ).unwrap();
  /// ```
  pub fn list_projects_filtered( &self, filter : &crate::ProjectFilter ) -> Result< Vec< Project > >
  {
    // Optimization: skip filtering if default filter
    if filter.is_default()
    {
      return self.list_projects();
    }

    let all_projects = self.list_projects()?;
    let mut filtered = Vec::new();

    for project in all_projects
    {
      if project.matches_filter( filter )?
      {
        filtered.push( project );
      }
    }

    Ok( filtered )
  }
}

impl Default for Storage
{
  fn default() -> Self
  {
    Self::new().expect( "Failed to create default storage" )
  }
}

#[cfg( test )]
mod tests
{
  use super::*;

  #[test]
  fn test_storage_new()
  {
    let storage = Storage::new();
    assert!( storage.is_ok() );
  }

  #[test]
  fn test_storage_with_root()
  {
    let storage = Storage::with_root( "/tmp/claude-test" );
    assert_eq!( storage.root(), Path::new( "/tmp/claude-test" ) );
    assert_eq!( storage.projects_dir(), PathBuf::from( "/tmp/claude-test/projects" ) );
  }

  #[test]
  fn test_projects_dir()
  {
    let storage = Storage::with_root( "/tmp/test" );
    assert_eq!( storage.projects_dir(), PathBuf::from( "/tmp/test/projects" ) );
  }
}
