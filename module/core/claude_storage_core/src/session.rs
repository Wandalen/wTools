//! Session management - represents a conversation session (JSONL file)

use std::
{
  fs,
  path::{ Path, PathBuf },
};

use crate::
{
  Entry,
  EntryType,
  Error,
  Result,
  stats::SessionStats,
};

/// A conversation session (one JSONL file)
#[derive( Debug )]
pub struct Session
{
  /// Session UUID (filename without .jsonl extension)
  id : String,

  /// Path to the JSONL file
  storage_path : PathBuf,

  /// Loaded entries (lazily loaded)
  entries : Option< Vec< Entry > >,
}

impl Session
{
  /// Create a session reference (doesn't load entries yet)
  #[must_use] 
  pub fn new( id : String, storage_path : PathBuf ) -> Self
  {
    Self
    {
      id,
      storage_path,
      entries : None,
    }
  }

  /// Load session from a JSONL file
  ///
  /// # Errors
  ///
  /// Returns error if the path does not exist, if the filename is missing or
  /// contains invalid UTF-8, or if the file does not have a `.jsonl` extension.
  pub fn load( path : &Path ) -> Result< Self >
  {
    if !path.exists()
    {
      return Err( Error::session_not_found
      (
        path.to_string_lossy().to_string()
      ));
    }

    // Extract session ID from filename
    let filename = path
      .file_name()
      .ok_or_else( || Error::invalid_structure
      (
        path.to_path_buf(),
        "missing filename"
      ))?
      .to_str()
      .ok_or_else( || Error::invalid_structure
      (
        path.to_path_buf(),
        "filename contains invalid UTF-8"
      ))?;

    let id = filename
      .strip_suffix( ".jsonl" )
      .ok_or_else( || Error::invalid_structure
      (
        path.to_path_buf(),
        "not a .jsonl file"
      ))?
      .to_string();

    Ok( Self
    {
      id,
      storage_path : path.to_path_buf(),
      entries : None,
    })
  }

  /// Get session ID
  #[must_use] 
  pub fn id( &self ) -> &str
  {
    &self.id
  }

  /// Get storage path
  #[must_use] 
  pub fn storage_path( &self ) -> &Path
  {
    &self.storage_path
  }

  /// Get entries (loads if not already loaded)
  ///
  /// # Errors
  ///
  /// Returns error if the JSONL file cannot be read from disk.
  ///
  /// # Panics
  ///
  /// Panics if the entries cache is unexpectedly absent after a successful load,
  /// which should never occur under normal operation.
  pub fn entries( &mut self ) -> Result< &Vec< Entry > >
  {
    if self.entries.is_none()
    {
      self.load_entries()?;
    }

    Ok( self.entries.as_ref().unwrap() )
  }

  /// Force reload entries from disk
  ///
  /// # Errors
  ///
  /// Returns error if the JSONL file cannot be read from disk.
  pub fn reload( &mut self ) -> Result< ()>
  {
    self.entries = None;
    self.load_entries()
  }

  /// Load entries from JSONL file
  ///
  /// Gracefully skips non-conversation entries (queue-operation, summary, etc.)
  /// that don't have the standard conversation entry structure.
  ///
  /// ## Graceful Degradation Design
  ///
  /// Real Claude Code sessions contain multiple entry types:
  /// - **Conversation entries** (`type:"user"`, `type:"assistant"`) - Have `uuid` field, parse successfully
  /// - **Metadata entries** (`type:"queue-operation"`, `type:"summary"`) - Lack `uuid` field, skipped
  ///
  /// Production data shows ~3-4% of entries are metadata that should be skipped.
  /// Example: 4901-line session had 4710 conversation entries + 191 metadata entries.
  ///
  /// This is intentional behavior, not a bug. We only need conversation entries for:
  /// - Statistics (entry counts, token usage)
  /// - Export (markdown/text output)
  /// - Search (finding user/assistant messages)
  ///
  /// Metadata entries serve Claude Code's internal operation (command queueing, summaries)
  /// but aren't part of the conversation history we expose.
  fn load_entries( &mut self ) -> Result< ()>
  {
    let content = fs::read_to_string( &self.storage_path )
      .map_err( | e | Error::io
      (
        e,
        format!( "reading session file: {}", self.storage_path.display() )
      ))?;

    let mut entries = Vec::new();

    for line in content.lines()
    {
      if line.trim().is_empty()
      {
        continue; // Skip empty lines
      }

      // Try to parse as conversation entry; silently skip metadata entries
      // (queue-operation, summary, etc.) - graceful degradation, we only need conversation entries
      if let Ok( entry ) = Entry::from_json_line( line )
      {
        entries.push( entry );
      }
    }

    self.entries = Some( entries );
    Ok( () )
  }

  /// Append an entry to the session (append-only operation)
  ///
  /// # Errors
  ///
  /// Returns error if the entry cannot be serialized to JSON, if the session
  /// file cannot be opened for appending, or if writing to the file fails.
  pub fn append_entry( &mut self, entry : &Entry ) -> Result< ()>
  {
    let json_line = entry.to_json_line()?;

    // Append to file
    use std::io::Write;
    let mut file = fs::OpenOptions::new()
      .create( true )
      .append( true )
      .open( &self.storage_path )
      .map_err( | e | Error::io
      (
        e,
        format!( "opening session file for append: {}", self.storage_path.display() )
      ))?;

    writeln!( file, "{json_line}" )
      .map_err( | e | Error::io
      (
        e,
        format!( "appending to session file: {}", self.storage_path.display() )
      ))?;

    // Invalidate cache to force reload
    self.entries = None;

    Ok( () )
  }

  /// Count entries without loading them all into memory
  ///
  /// # Errors
  ///
  /// Returns error if the JSONL file cannot be read from disk.
  pub fn count_entries( &self ) -> Result< usize >
  {
    let content = fs::read_to_string( &self.storage_path )
      .map_err( | e | Error::io
      (
        e,
        format!( "reading session file: {}", self.storage_path.display() )
      ))?;

    Ok( content.lines().filter( | l | !l.trim().is_empty() ).count() )
  }

  /// Filter entries by type (user or assistant)
  ///
  /// Returns only entries matching the specified type. Requires entries to be loaded.
  ///
  /// # Errors
  ///
  /// Returns error if the session file cannot be read when entries are not yet loaded.
  pub fn entries_by_type( &mut self, entry_type : EntryType ) -> Result< Vec< &Entry > >
  {
    let entries = self.entries()?;
    Ok( entries.iter().filter( | e | e.entry_type == entry_type ).collect() )
  }

  /// Filter entries after a specific timestamp
  ///
  /// Returns entries with timestamps after the specified ISO 8601 timestamp string.
  /// Requires entries to be loaded.
  ///
  /// # Errors
  ///
  /// Returns error if the session file cannot be read when entries are not yet loaded.
  pub fn entries_after( &mut self, timestamp : &str ) -> Result< Vec< &Entry > >
  {
    let entries = self.entries()?;
    Ok( entries.iter().filter( | e | e.timestamp.as_str() > timestamp ).collect() )
  }

  /// Detect if this is an agent/sidechain session
  ///
  /// Agent sessions typically have filenames like "agent-{id}.jsonl"
  /// or have isSidechain: true in their entries.
  #[must_use] 
  pub fn is_agent_session( &self ) -> bool
  {
    self.id.starts_with( "agent-" )
  }

  /// Compute session statistics
  ///
  /// Calculates comprehensive statistics including entry counts, token usage,
  /// and timestamps without parsing all entry content (only parses usage metadata).
  ///
  /// # Errors
  ///
  /// Returns error if the session file cannot be read, or if a line contains
  /// invalid JSON that cannot be parsed.
  ///
  /// Fix(issue-session-stats-entry-counting): Entry counting returned 0 for all sessions
  /// Root cause: Code checked for top-level `"role"` field instead of `"type"` field. Claude Code
  ///   v2.0+ format has `"type"` at top level ("user"|"assistant") and `"role"` nested inside
  ///   the `"message"` object, so checking `role` at top level always returned None.
  /// Pitfall: When parsing Claude Code storage, ALWAYS use the top-level `"type"` field to
  ///   determine entry type, not `"role"`. The role field exists but is nested inside message.
  pub fn stats( &mut self ) -> Result< SessionStats >
  {
    use crate::json::parse_json;

    let mut stats = SessionStats::new( self.id.clone() );
    stats.is_agent_session = self.is_agent_session();

    // Read file content
    let content = fs::read_to_string( &self.storage_path )
      .map_err( | e | Error::io
      (
        e,
        format!( "reading session file: {}", self.storage_path.display() )
      ))?;

    // Process each line
    for line in content.lines()
    {
      if line.trim().is_empty()
      {
        continue;
      }

      // Parse just the fields we need for stats
      let json = parse_json( line )
        .map_err( | e | Error::parse( 0, line, &format!( "JSON parse error: {e}" ) ) )?;

      // Extract type - only count conversation entries, skip metadata entries
      // In Claude Code v2.0+, the top-level "type" field indicates entry type ("user" or "assistant")
      // Role is also available nested inside message.role, but we use type for consistency
      if let Some( entry_type ) = json.get_str( "type" )
      {
        match entry_type
        {
          "user" =>
          {
            stats.user_entries += 1;
            stats.total_entries += 1;
          }
          "assistant" =>
          {
            stats.assistant_entries += 1;
            stats.total_entries += 1;
          }
          _ =>
          {
            // Skip non-conversation entries (queue-operation, summary, etc.)
            continue;
          }
        }
      }

      // Extract timestamp
      if let Some( timestamp ) = json.get_str( "timestamp" )
      {
        if stats.first_timestamp.is_none()
        {
          stats.first_timestamp = Some( timestamp.to_string() );
        }
        stats.last_timestamp = Some( timestamp.to_string() );
      }

      // Extract token usage from assistant messages
      if let Some( "assistant" ) = json.get_str( "type" )
      {
        if let Some( message ) = json.get( "message" )
        {
          if let Some( usage ) = message.get( "usage" )
          {
            // Token counts from JSON are always non-negative integers stored as f64.
            // The cast to u64 is safe: values are positive and well within u64 range.
            #[ allow( clippy::cast_possible_truncation, clippy::cast_sign_loss ) ]
            {
              if let Some( input_tokens ) = usage.get_number( "input_tokens" )
              {
                stats.total_input_tokens += input_tokens as u64;
              }

              if let Some( output_tokens ) = usage.get_number( "output_tokens" )
              {
                stats.total_output_tokens += output_tokens as u64;
              }

              if let Some( cache_read ) = usage.get_number( "cache_read_input_tokens" )
              {
                stats.total_cache_read_tokens += cache_read as u64;
              }

              if let Some( cache_creation ) = usage.get_number( "cache_creation_input_tokens" )
              {
                stats.total_cache_creation_tokens += cache_creation as u64;
              }
            }
          }
        }
      }
    }

    Ok( stats )
  }

  /// Check if session matches filter
  ///
  /// ## Filtering Logic (AND composition)
  ///
  /// All filter conditions must match:
  /// - `agent_only`: If set, checks if session is agent session
  /// - `min_entries`: If set, checks if session has at least this many entries
  /// - `session_id_substring`: If set, checks if session ID contains substring (case-insensitive)
  ///
  /// ## Examples
  ///
  /// ```rust,no_run
  /// use claude_storage_core::{ Session, SessionFilter };
  /// use std::path::PathBuf;
  ///
  /// let mut session = Session::new( "agent-abc123".to_string(), PathBuf::from( "/tmp/test.jsonl" ) );
  ///
  /// let filter = SessionFilter
  /// {
  ///   agent_only : Some( true ),
  ///   min_entries : None,
  ///   session_id_substring : None,
  /// };
  ///
  /// assert!( session.matches_filter( &filter ).unwrap() );
  /// ```
  /// # Errors
  ///
  /// Returns error if reading the session file fails when checking entry count.
  pub fn matches_filter( &mut self, filter : &crate::SessionFilter ) -> Result< bool >
  {
    // Agent filter
    if let Some( agent_only ) = filter.agent_only
    {
      if self.is_agent_session() != agent_only
      {
        return Ok( false );
      }
    }

    // Minimum entries filter
    if let Some( min_entries ) = filter.min_entries
    {
      let count = self.count_entries()?;
      if count < min_entries
      {
        return Ok( false );
      }
    }

    // Session ID substring filter
    if let Some( ref substring ) = filter.session_id_substring
    {
      let matcher = crate::StringMatcher::new( substring );
      if !matcher.matches( &self.id )
      {
        return Ok( false );
      }
    }

    Ok( true )
  }

  /// Search session content for query
  ///
  /// Streams through JSONL file line-by-line for memory efficiency.
  /// Returns all matches with context.
  ///
  /// ## Examples
  ///
  /// ```
  /// use claude_storage_core::{ Session, SearchFilter };
  /// use std::path::PathBuf;
  ///
  /// # fn example() -> claude_storage_core::Result< () > {
  /// let mut session = Session::new( "test-id".to_string(), PathBuf::from( "/tmp/test.jsonl" ) );
  /// let filter = SearchFilter::new( "error_tools" );
  /// let matches = session.search( &filter )?;
  ///
  /// for m in matches
  /// {
  ///   println!( "Entry #{}: {}", m.entry_index(), m.excerpt() );
  /// }
  /// # Ok( () )
  /// # }
  /// ```
  ///
  /// # Errors
  ///
  /// Returns error if the session file cannot be opened or if reading a line fails.
  pub fn search( &mut self, filter : &crate::SearchFilter ) -> Result< Vec< crate::SearchMatch > >
  {
    use std::io::{ BufRead, BufReader };
    use std::fs::File;

    let file = File::open( &self.storage_path )?;
    let reader = BufReader::new( file );

    let mut matches = Vec::new();
    let mut entry_index = 0;

    for line in reader.lines()
    {
      let line = line?;

      // Try to extract role and content from JSON without full parsing
      // This is a lightweight check to see if we should parse the full entry
      if !filter.matches_text( &line )
      {
        entry_index += 1;
        continue; // Skip this entry, doesn't contain query
      }

      // Parse entry to get role and content
      match Entry::from_json_line( &line )
      {
        Ok( entry ) =>
        {
          // Check entry type filter
          if let Some( target_type ) = filter.match_entry_type
          {
            if entry.entry_type() != target_type
            {
              entry_index += 1;
              continue;
            }
          }

          // Search in content blocks
          let content_text = entry.content_text();

          // Split into lines and search
          for ( line_num, content_line ) in content_text.lines().enumerate()
          {
            if filter.matches_text( content_line )
            {
              let search_match = crate::SearchMatch::new
              (
                entry_index,
                entry.entry_type(),
                line_num,
                content_line.to_string(),
              );
              matches.push( search_match );
            }
          }
        }
        Err( _e ) =>
        {
          // Skip malformed entries (graceful degradation)
        }
      }

      entry_index += 1;
    }

    Ok( matches )
  }
}

#[cfg( test )]
mod tests
{
  use super::*;
  use std::io::Write;
  use tempfile::NamedTempFile;

  #[test]
  fn test_session_new()
  {
    let session = Session::new
    (
      "test-uuid".to_string(),
      PathBuf::from( "/tmp/test.jsonl" )
    );

    assert_eq!( session.id(), "test-uuid" );
  }

  #[test]
  fn test_session_load_missing_file()
  {
    let result = Session::load( Path::new( "/nonexistent/session.jsonl" ) );
    assert!( result.is_err() );
  }

  #[test]
  fn test_session_load_valid_file()
  {
    // Create temp JSONL file
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!( temp_file, r#"{{"uuid": "test"}}"# ).unwrap();

    // Rename to have .jsonl extension
    let jsonl_path = temp_file.path().with_extension( "jsonl" );
    fs::copy( temp_file.path(), &jsonl_path ).unwrap();

    let result = Session::load( &jsonl_path );

    // Cleanup
    let _ = fs::remove_file( jsonl_path );

    assert!( result.is_ok() );
  }
}
