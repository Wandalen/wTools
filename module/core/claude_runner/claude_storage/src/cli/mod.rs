//! CLI command routines for claude_storage
//!
//! This module provides command routines that are registered with unilang
//! and called when users invoke CLI commands.
//!
//! ## Known Pitfalls
//!
//! ### Parameter Validation Consistency (Finding #010)
//!
//! **Issue**: Default parameter values do not prevent invalid input - parameters
//! with defaults still require explicit validation.
//!
//! **Context**: search_routine was missing verbosity range validation (0-5)
//! while status_routine and show_routine had it. The default value (1) made
//! it seem like validation was unnecessary, but users can override defaults
//! with invalid values like -1 or 10.
//!
//! **Solution**: All parameters with value constraints must have explicit
//! validation, regardless of default values. Apply validation patterns
//! consistently across all command routines:
//!
//! ```rust,no_run
//! # use unilang::{ VerifiedCommand, ErrorData, ErrorCode };
//! # fn example( cmd : VerifiedCommand ) -> Result< (), ErrorData >
//! # {
//! let verbosity = cmd.get_integer( "verbosity" ).unwrap_or( 1 );
//!
//! // Always validate range, even with defaults
//! if !( 0..=5 ).contains( &verbosity )
//! {
//!   return Err( ErrorData::new(
//!     ErrorCode::InternalError,
//!     format!( "Invalid verbosity: {}. Valid range: 0-5", verbosity )
//!   ));
//! }
//! # Ok( () )
//! # }
//! ```
//!
//! **Prevention**: When adding new parameters, check existing command routines
//! for validation patterns and apply them consistently. Never assume defaults
//! eliminate the need for validation.
//!
//! See: tests/search_command_test.rs::test_search_verbosity_invalid

use unilang::{ VerifiedCommand, ExecutionContext, OutputData, ErrorData, ErrorCode };
use claude_storage_core::Storage;

/// Show storage status and statistics
///
/// Displays comprehensive information about Claude Code storage including
/// project counts, session counts, token usage, and storage location.
pub fn status_routine( cmd : VerifiedCommand, _ctx : ExecutionContext )
  -> std::result::Result< OutputData, ErrorData >
{
  let verbosity = cmd.get_integer( "verbosity" ).unwrap_or( 1 );

  // Validate verbosity range (0-5)
  if !( 0..=5 ).contains( &verbosity )
  {
    return Err
    (
      ErrorData::new
      (
        ErrorCode::InternalError,
        format!( "Invalid verbosity: {}. Valid range: 0-5", verbosity )
      )
    );
  }

  let custom_path = cmd.get_string( "path" );

  // Fix(issue-014): Resolve path parameter before using
  //
  // Root cause: status_routine passed path directly to Storage::with_root() without
  // resolving special markers (".", "..", "~"), unlike list_routine which uses
  // resolve_path_parameter().
  //
  // Pitfall: Inconsistent parameter handling across commands leads to confusing UX
  // where the same parameter format works in one command but not another.
  let resolved_path = if let Some( path ) = custom_path
  {
    match resolve_path_parameter( path )
    {
      Ok( resolved ) => Some( resolved ),
      Err( e ) =>
      {
        return Err
        (
          ErrorData::new
          (
            ErrorCode::InternalError,
            format!( "Failed to resolve path '{}': {}", path, e )
          )
        );
      }
    }
  }
  else
  {
    None
  };

  // Create storage instance
  let storage = if let Some( path ) = resolved_path
  {
    Storage::with_root( &path )
  }
  else
  {
    Storage::new().map_err( | e | ErrorData::new( ErrorCode::InternalError, format!( "Failed to create storage: {}", e ) ) )?
  };

  // Get global statistics
  let stats = storage.global_stats()
    .map_err( | e | ErrorData::new( ErrorCode::InternalError, format!( "Failed to get statistics: {}", e ) ) )?;

  // Format output based on verbosity
  let output = match verbosity
  {
    0 =>
    {
      // Minimal: just project count
      format!( "Projects: {}", stats.total_projects )
    }
    1 =>
    {
      // Standard: overview
      format!
      (
        "Storage: {:?}\nProjects: {} (UUID: {}, Path: {})\nSessions: {} (Main: {}, Agent: {})\nEntries: {}",
        storage.root(),
        stats.total_projects,
        stats.uuid_projects,
        stats.path_projects,
        stats.total_sessions,
        stats.main_sessions,
        stats.agent_sessions,
        stats.total_entries
      )
    }
    _ =>
    {
      // Detailed: include token usage
      format!
      (
        "Storage: {:?}\n\
        Projects: {} (UUID: {}, Path: {})\n\
        Sessions: {} (Main: {}, Agent: {})\n\
        Entries: {} (User: {}, Assistant: {})\n\
        Tokens:\n\
        - Input: {}\n\
        - Output: {}\n\
        - Cache Read: {}\n\
        - Cache Creation: {}",
        storage.root(),
        stats.total_projects,
        stats.uuid_projects,
        stats.path_projects,
        stats.total_sessions,
        stats.main_sessions,
        stats.agent_sessions,
        stats.total_entries,
        stats.total_user_entries,
        stats.total_assistant_entries,
        stats.total_input_tokens,
        stats.total_output_tokens,
        stats.total_cache_read_tokens,
        stats.total_cache_creation_tokens
      )
    }
  };

  Ok( OutputData::new( output, "text" ) )
}

/// Resolve path parameter with smart detection
///
/// ## Behavior
///
/// - `.` → Current working directory (absolute)
/// - `..` → Parent directory (absolute)
/// - `~` → Home directory (absolute)
/// - `~/path` → Home + path (absolute)
/// - `/abs/path` → Use as-is (already absolute)
/// - `rel/path` → Resolve to absolute
/// - `pattern` → Use as-is (no path separators = pattern)
///
/// ## Examples
///
/// ```ignore
/// // Current dir: /home/user/project
/// resolve_path_parameter(".") → "/home/user/project"
/// resolve_path_parameter("..") → "/home/user"
/// resolve_path_parameter("~") → "/home/user"
/// resolve_path_parameter("willbe") → "willbe" (unchanged)
/// ```
fn resolve_path_parameter( param : &str ) -> std::result::Result< String, String >
{
  use std::path::Path;

  match param
  {
    // Current directory
    "." =>
    {
      std::env::current_dir()
        .map( | p | p.to_string_lossy().to_string() )
        .map_err( | e | format!( "Failed to get current directory: {}", e ) )
    },

    // Parent directory
    ".." =>
    {
      let current = std::env::current_dir()
        .map_err( | e | format!( "Failed to get current directory: {}", e ) )?;

      let parent = current.parent()
        .ok_or_else( || "Current directory has no parent".to_string() )?;

      Ok( parent.to_string_lossy().to_string() )
    },

    // Home directory or home + relative path
    s if s.starts_with( '~' ) =>
    {
      let home = std::env::var( "HOME" )
        .map_err( | e | format!( "Failed to get HOME directory: {}", e ) )?;

      if s.len() == 1
      {
        // Just "~"
        Ok( home )
      }
      else if let Some( stripped ) = s.strip_prefix( "~/" )
      {
        // "~/path"
        let path = Path::new( &home ).join( stripped );
        Ok( path.to_string_lossy().to_string() )
      }
      else
      {
        // "~user" not supported, use as-is
        Ok( s.to_string() )
      }
    },

    // Absolute path - use as-is
    s if s.starts_with( '/' ) =>
    {
      Ok( s.to_string() )
    },

    // Relative path with separators - resolve to absolute
    s if s.contains( '/' ) =>
    {
      let current = std::env::current_dir()
        .map_err( | e | format!( "Failed to get current directory: {}", e ) )?;

      let resolved = current.join( s );
      Ok( resolved.to_string_lossy().to_string() )
    },

    // Pattern (no path separators) - use as-is
    s =>
    {
      Ok( s.to_string() )
    },
  }
}

/// Format entry content for display
///
/// ## Behavior
///
/// - Extracts actual message content from Entry
/// - Formats as readable chat log entry
/// - Supports text, thinking, tool use blocks
/// - Optional truncation for long messages
///
/// ## Format
///
/// ```text
/// [2025-12-02 09:57] User:
/// message content here
///
/// [2025-12-02 09:58] Assistant:
/// response content here
/// ```
///
/// ## Examples
///
/// ```ignore
/// let entry = session.entries()[0];
/// let formatted = format_entry_content( &entry, None );
/// // Output: "[2025-12-02 09:57] User:\nHello, Claude!"
/// ```
fn format_entry_content( entry : &claude_storage_core::Entry, max_length : Option< usize > ) -> String
{
  use claude_storage_core::{ MessageContent, ContentBlock };

  // Format timestamp
  let timestamp = format_timestamp( &entry.timestamp );

  // Extract content based on message type
  let ( role, content ) = match &entry.message
  {
    MessageContent::User( msg ) =>
    {
      ( "User", msg.content.clone() )
    },
    MessageContent::Assistant( msg ) =>
    {
      // Extract all text blocks
      let text_blocks : Vec< String > = msg.content
        .iter()
        .filter_map( | block | match block
        {
          ContentBlock::Text { text } => Some( text.clone() ),
          ContentBlock::Thinking { thinking, .. } =>
          {
            // Show thinking blocks with prefix
            Some( format!( "[Thinking]\n{}", thinking ) )
          },
          ContentBlock::ToolUse { name, .. } =>
          {
            // Show tool use briefly
            Some( format!( "[Using tool: {}]", name ) )
          },
          ContentBlock::ToolResult { is_error, content, .. } =>
          {
            if *is_error
            {
              Some( format!( "[Tool error: {}]", content ) )
            }
            else
            {
              // Don't show successful tool results in conversation view
              None
            }
          },
        })
        .collect();

      let combined = text_blocks.join( "\n\n" );
      ( "Assistant", combined )
    }
  };

  // Apply truncation if needed
  let content = truncate_if_needed( &content, max_length );

  // Format as chat log entry
  format!( "[{}] {}:\n{}", timestamp, role, content )
}

/// Format timestamp for display
///
/// Converts ISO 8601 timestamp to readable format:
/// "2025-12-02T09:57:02.237Z" → "2025-12-02 09:57"
///
/// ## Examples
///
/// ```ignore
/// let ts = "2025-12-02T09:57:02.237Z";
/// assert_eq!( format_timestamp( ts ), "2025-12-02 09:57" );
/// ```
fn format_timestamp( timestamp : &str ) -> String
{
  // Try to parse ISO 8601
  if let Some( datetime_part ) = timestamp.split( '.' ).next()
  {
    if let Some( ( date, time ) ) = datetime_part.split_once( 'T' )
    {
      // Extract HH:MM from time
      let time_short = time.split( ':' ).take( 2 ).collect::< Vec< _ > >().join( ":" );
      return format!( "{} {}", date, time_short );
    }
  }

  // Fallback: use raw timestamp
  timestamp.to_string()
}

/// Truncate text with indicator
///
/// Truncates long text and adds "... [truncated]" indicator.
///
/// ## Examples
///
/// ```ignore
/// let text = "a".repeat( 1000 );
/// let truncated = truncate_if_needed( &text, Some( 100 ) );
/// assert!( truncated.contains( "[truncated" ) );
/// ```
fn truncate_if_needed( text : &str, max_length : Option< usize > ) -> String
{
  match max_length
  {
    None => text.to_string(),
    Some( len ) if text.len() <= len => text.to_string(),
    Some( len ) =>
    {
      let truncated = &text[ ..len ];
      format!( "{}... [truncated, {} more chars]", truncated, text.len() - len )
    }
  }
}

/// List projects or sessions
///
/// Lists projects in Claude Code storage, with optional filtering by type.
///
/// Smart session display:
/// - Providing session filters (`session::`, `agent::`, `min_entries::`) auto-enables session display
/// - Explicit `sessions::0` or `sessions::1` overrides auto-detection
/// - No filters → Projects only (default behavior)
///
/// Examples:
/// ```bash
/// # Projects only (no sessions)
/// .list
///
/// # Auto-enable sessions (filter provided)
/// .list session::commit
///
/// # Explicit disable (overrides auto-enable)
/// .list sessions::0 session::commit
/// ```
pub fn list_routine( cmd : VerifiedCommand, _ctx : ExecutionContext )
  -> std::result::Result< OutputData, ErrorData >
{
  let project_type = cmd.get_string( "type" ).unwrap_or( "all" );
  let verbosity = cmd.get_integer( "verbosity" ).unwrap_or( 1 );

  // Parse filter parameters
  let path_filter = cmd.get_string( "path" );
  let agent_filter = cmd.get_boolean( "agent" );

  // Validate and parse min_entries (must be non-negative)
  let min_entries_filter = if let Some( n ) = cmd.get_integer( "min_entries" )
  {
    if n < 0
    {
      return Err
      (
        ErrorData::new
        (
          ErrorCode::InternalError,
          format!( "Invalid min_entries: {}. Must be non-negative", n )
        )
      );
    }
    Some( n as usize )
  }
  else
  {
    None
  };

  let session_id_filter = cmd.get_string( "session" );

  // Fix(issue-002): Smart path resolution for path:: parameter
  //
  // Root cause: The path:: parameter used literal substring matching only.
  // When users provided path::., it searched for paths containing a literal "."
  // character instead of resolving "." to the current working directory.
  // This violated user expectations from shell semantics where . means "current
  // directory", .. means "parent directory", and ~ means "home directory".
  //
  // Pitfall: When implementing filters that accept both patterns and paths,
  // clearly define detection logic. Ambiguous cases (like .) should prioritize
  // user expectations over literal interpretation. Support shell semantics for
  // special characters (., .., ~) in all filesystem path parameters.

  // Resolve path parameter with smart detection
  let path_filter = if let Some( ref param ) = path_filter
  {
    match resolve_path_parameter( param )
    {
      Ok( resolved ) => Some( resolved ),
      Err( e ) =>
      {
        return Err
        (
          ErrorData::new
          (
            ErrorCode::InternalError,
            format!( "Failed to resolve path parameter '{}': {}", param, e )
          )
        );
      }
    }
  }
  else
  {
    None
  };

  // Fix(issue-001): Smart session display - auto-enable when filters provided
  //
  // Root cause: `show_sessions` defaulted to false, blocking filter usage even when
  // session filters were provided. This made session::, agent::, and min_entries::
  // parameters "garbage" - accepted by parser but silently ignored by implementation.
  //
  // Pitfall: Garbage parameters create silent failures that waste user time. Users try
  // different parameter values but see no effect because the filter is built but never
  // used. ALWAYS trace parameter flow: parser → filter build → filter usage. If usage
  // is conditional on default-false flag, parameter is garbage.

  // Smart parameter detection: Auto-enable session display when filters provided
  let has_session_filters = session_id_filter.is_some()
    || agent_filter.is_some()
    || min_entries_filter.is_some();

  let show_sessions = has_session_filters || cmd.get_boolean( "sessions" ).unwrap_or( false );

  // Create storage instance
  let storage = Storage::new()
    .map_err( | e | ErrorData::new( ErrorCode::InternalError, format!( "Failed to create storage: {}", e ) ) )?;

  // Fix(issue-list-hang): min_entries:: must not be placed in ProjectFilter
  //
  // Root cause: Placing min_entries in ProjectFilter caused project.matches_filter()
  // to call project_stats() for EVERY project. project_stats() reads ALL session JSONL
  // files to count entries, scanning gigabytes of data when the user has many projects.
  // This caused the binary to hang indefinitely.
  //
  // min_entries:: is semantically a SESSION filter (show sessions with ≥N entries),
  // not a project filter (show projects whose total entries ≥ N). The auto-enable
  // behavior (show_sessions=true) is handled separately at line 512-516.
  //
  // Pitfall: When a parameter auto-enables a feature, don't also apply it as a
  // project-level filter unless that filtering is the stated purpose. Trace the
  // computational cost: project_stats() = O(projects × sessions × entries).

  // Build project filter (min_entries is a session filter, not a project filter)
  let project_filter = claude_storage_core::ProjectFilter
  {
    path_substring : path_filter.map( | s | s.to_string() ),
    min_entries : None,
    min_sessions : None,
  };

  // Build session filter
  let session_filter = claude_storage_core::SessionFilter
  {
    agent_only : agent_filter,
    min_entries : min_entries_filter,
    session_id_substring : session_id_filter.map( | s | s.to_string() ),
  };

  // Get projects based on type filter
  let mut projects = match project_type
  {
    "uuid" => storage.list_uuid_projects(),
    "path" => storage.list_path_projects(),
    "all" => storage.list_projects(),
    _ => return Err
    (
      ErrorData::new
      (
        ErrorCode::InternalError,
        format!( "Invalid type: {}. Valid values: uuid, path, all", project_type )
      )
    ),
  }
  .map_err( | e | ErrorData::new( ErrorCode::InternalError, format!( "Failed to list projects: {}", e ) ) )?;

  // Apply project-level filtering
  if !project_filter.is_default()
  {
    projects.retain( | project |
    {
      project.matches_filter( &project_filter ).unwrap_or( false )
    });
  }

  // Format output
  let mut output = String::new();

  if verbosity >= 1
  {
    output.push_str( &format!( "Found {} projects:\n\n", projects.len() ) );
  }

  for mut project in projects
  {
    // Handle projects that may have been deleted (gracefully skip them)
    match verbosity
    {
      0 =>
      {
        // Just ID
        output.push_str( &format!( "{:?}\n", project.id() ) );
      }
      1 =>
      {
        // ID + session count (skip if project was deleted)
        let session_count = match project.count_sessions()
        {
          Ok( count ) => count,
          Err( _ ) => continue,  // Skip projects that can't be read
        };

        output.push_str( &format!( "{:?} ({} sessions)\n", project.id(), session_count ) );
      }
      _ =>
      {
        // Full details (skip if project was deleted)
        let project_stats = match project.project_stats()
        {
          Ok( stats ) => stats,
          Err( _ ) => continue,  // Skip projects that can't be read
        };

        output.push_str
        (
          &format!
          (
            "{:?}\n  Sessions: {} (Main: {}, Agent: {})\n  Entries: {}\n  Tokens: {} in, {} out\n\n",
            project.id(),
            project_stats.session_count,
            project_stats.main_session_count,
            project_stats.agent_session_count,
            project_stats.total_entries,
            project_stats.total_input_tokens,
            project_stats.total_output_tokens
          )
        );
      }
    }

    // Show sessions if requested (skip if project was deleted)
    if show_sessions
    {
      let sessions = if session_filter.is_default()
      {
        match project.sessions()
        {
          Ok( s ) => s,
          Err( _ ) => continue,  // Skip if can't read sessions
        }
      }
      else
      {
        match project.sessions_filtered( &session_filter )
        {
          Ok( s ) => s,
          Err( _ ) => continue,  // Skip if can't read sessions
        }
      };

      for session in sessions
      {
        output.push_str( &format!( "  - {}\n", session.id() ) );
      }
    }
  }

  Ok( OutputData::new( output, "text" ) )
}

/// Parse project parameter into ProjectId
///
/// Supports multiple formats:
/// - Absolute path: `/home/user/project` → ProjectId::Path
/// - Path-encoded: `-home-user-project` → ProjectId::Path (decoded)
/// - UUID: `abc-123-def` → ProjectId::Uuid
/// - Debug format: `Path("/home/user/project")` → ProjectId::Path
///
/// # Fix
///
/// Fix(issue-project-param-2025-11-30): Project parameter always treated as UUID
///
/// # Root Cause
///
/// Code at line 239 hardcoded `ProjectId::uuid(proj_id)` without checking
/// if the parameter was a path. This caused all path projects to fail with
/// "Project not found" error.
///
/// # Pitfall
///
/// When accepting string parameters that could have multiple formats, always
/// implement smart detection logic. Hardcoding assumptions about parameter
/// format leads to silent failures for valid inputs.
pub fn parse_project_parameter( input : &str )
  -> std::result::Result< claude_storage_core::ProjectId, String >
{
  use claude_storage_core::{ ProjectId, decode_path };
  use std::path::PathBuf;

  // [1] Check for Debug format from .list output
  if let Some( path_str ) = input.strip_prefix( "Path(\"" ).and_then( | s | s.strip_suffix( "\")" ) )
  {
    return Ok( ProjectId::path( path_str ) );
  }

  // [2] Check for absolute path (cross-platform)
  let path = PathBuf::from( input );
  if path.is_absolute()
  {
    return Ok( ProjectId::path( input ) );
  }

  // [3] Check for path-encoded
  if input.starts_with( '-' )
  {
    match decode_path( input )
    {
      Ok( decoded ) => return Ok( ProjectId::path( decoded ) ),
      Err( e ) => return Err( format!( "Failed to decode path: {}", e ) ),
    }
  }

  // Fix(issue-013): Handle relative paths (".", "..", "~", "./foo", "../bar")
  //
  // Root cause: Only checked for absolute paths and path-encoded, missing relative
  // path conventions that users commonly use for directory references.
  //
  // Pitfall: Assuming only absolute paths need path treatment; users commonly use
  // "." for CWD, ".." for parent, "~" for home in shell contexts.

  // [4] Check for home directory expansion (~)
  if input == "~" || input.starts_with( "~/" )
  {
    let home = std::env::var( "HOME" )
      .map_err( | _ | "HOME environment variable not set".to_string() )?;
    let expanded = if input == "~"
    {
      home
    }
    else
    {
      format!( "{}{}", home, &input[ 1.. ] )
    };
    return Ok( ProjectId::path( expanded ) );
  }

  // [5] Check for relative path markers (".", "..", "./", "../")
  if input == "." || input == ".." ||
     input.starts_with( "./" ) || input.starts_with( "../" )
  {
    // Get current working directory and resolve relative path
    let cwd = std::env::current_dir()
      .map_err( | e | format!( "Failed to get current directory: {}", e ) )?;
    let path = cwd.join( input );

    // For "." and "..", try to canonicalize (they should exist)
    // For "./foo" patterns, normalize without requiring existence
    if input == "." || input == ".."
    {
      match path.canonicalize()
      {
        Ok( abs_path ) => return Ok( ProjectId::path( abs_path.to_string_lossy().to_string() ) ),
        Err( e ) => return Err( format!( "Failed to resolve path '{}': {}", input, e ) ),
      }
    }
    else
    {
      // For "./foo" or "../bar" - normalize path components without canonicalize
      // This handles non-existent paths correctly
      use std::path::Component;
      let mut normalized = PathBuf::new();
      for component in path.components()
      {
        match component
        {
          Component::ParentDir =>
          {
            normalized.pop();
          }
          Component::CurDir =>
          {
            // Skip "." components
          }
          _ => normalized.push( component ),
        }
      }
      return Ok( ProjectId::path( normalized.to_string_lossy().to_string() ) );
    }
  }

  // [6] Default: UUID
  Ok( ProjectId::uuid( input ) )
}

/// Show session or project details (location-aware)
///
/// Smart behavior based on parameters:
/// - No parameters → Show current directory project (all sessions)
/// - session_id only → Show that session in current project
/// - project only → Show that project (all sessions)
/// - Both parameters → Show that session in that project
pub fn show_routine( cmd : VerifiedCommand, _ctx : ExecutionContext )
  -> std::result::Result< OutputData, ErrorData >
{
  let session_id = cmd.get_string( "session_id" );
  let project_param = cmd.get_string( "project" );
  let verbosity = cmd.get_integer( "verbosity" ).unwrap_or( 1 );
  let show_entries = cmd.get_boolean( "entries" ).unwrap_or( false );
  let metadata_only = cmd.get_boolean( "metadata" ).unwrap_or( false );

  // Validate verbosity range (0-5)
  if !( 0..=5 ).contains( &verbosity )
  {
    return Err
    (
      ErrorData::new
      (
        ErrorCode::InternalError,
        format!( "Invalid verbosity: {}. Valid range: 0-5", verbosity )
      )
    );
  }

  // Fix(issue-001): Validate entries parameter requires session_id
  //
  // Root cause: entries parameter was accepted and parsed but silently ignored
  // when displaying projects (cases 1 and 3). This created a "garbage parameter"
  // that users could pass but had no effect, wasting debugging time.
  //
  // Pitfall: Always validate parameter compatibility. If parameter P only works
  // with parameter Q, reject the combination where P is set but Q is not.
  // Silent ignoring of valid-syntax parameters creates user frustration.
  if show_entries && session_id.is_none()
  {
    return Err
    (
      ErrorData::new
      (
        ErrorCode::InternalError,
        "Parameter 'entries' requires 'session_id'. \
         Use '.show session_id::<id> entries::1' to display session entries."
          .to_string()
      )
    );
  }

  // Fix(issue-008): Validate entries parameter only works in metadata mode
  //
  // Root cause: entries parameter was checked only in metadata mode branch
  // (format_session_output lines 874-895). In content mode (lines 897-935),
  // the parameter was completely ignored - all entries were always displayed.
  // This created a "garbage parameter" where entries::1 was accepted but had
  // no effect in default content-first mode.
  //
  // Pitfall: When parameters only work in specific modes, validate mode
  // compatibility at parameter parsing time. Don't accept parameters that
  // will be silently ignored based on other parameter values.
  if show_entries && verbosity >= 1 && !metadata_only
  {
    return Err
    (
      ErrorData::new
      (
        ErrorCode::InternalError,
        "Parameter 'entries' only works in metadata mode. \
         Use '.show session_id::<id> metadata::1 entries::1' or \
         '.show session_id::<id> verbosity::0 entries::1'."
          .to_string()
      )
    );
  }

  // Smart parameter detection (4 cases)
  match ( session_id, project_param )
  {
    // Case 1: No parameters → Show current directory project
    ( None, None ) =>
    {
      show_project_for_cwd_impl( verbosity )
    }

    // Case 2: session_id only → Show session in current project
    ( Some( sid ), None ) =>
    {
      show_session_in_cwd_impl( sid, verbosity, show_entries, metadata_only )
    }

    // Case 3: project only → Show that project
    ( None, Some( proj ) ) =>
    {
      show_project_impl( proj, verbosity )
    }

    // Case 4: Both parameters → Show session in that project
    ( Some( sid ), Some( proj ) ) =>
    {
      show_session_in_project_impl( sid, proj, verbosity, show_entries, metadata_only )
    }
  }
}

/// Helper: Show session in current directory project
fn show_session_in_cwd_impl(
  session_id : &str,
  verbosity : i64,
  show_entries : bool,
  metadata_only : bool
) -> std::result::Result< OutputData, ErrorData >
{
  let storage = Storage::new()
    .map_err( | e | ErrorData::new( ErrorCode::InternalError, format!( "Failed to create storage: {}", e ) ) )?;

  let project = storage.load_project_for_cwd()
    .map_err( | e | ErrorData::new
    (
      ErrorCode::InternalError,
      format!( "Failed to load project from current directory: {}", e )
    ))?;

  format_session_output( &project, session_id, verbosity, show_entries, metadata_only )
}

/// Helper: Show session in specific project
fn show_session_in_project_impl(
  session_id : &str,
  project_param : &str,
  verbosity : i64,
  show_entries : bool,
  metadata_only : bool
) -> std::result::Result< OutputData, ErrorData >
{
  let storage = Storage::new()
    .map_err( | e | ErrorData::new( ErrorCode::InternalError, format!( "Failed to create storage: {}", e ) ) )?;

  // Parse project parameter
  let proj_id = parse_project_parameter( project_param )
    .map_err( | e | ErrorData::new
    (
      ErrorCode::InternalError,
      format!( "Invalid project parameter: {}", e )
    ))?;

  let project = storage.load_project( &proj_id )
    .map_err( | e | ErrorData::new
    (
      ErrorCode::InternalError,
      format!( "Failed to load project {:?}: {}", proj_id, e )
    ))?;

  format_session_output( &project, session_id, verbosity, show_entries, metadata_only )
}

/// Helper: Show project for current directory
fn show_project_for_cwd_impl( verbosity : i64 )
  -> std::result::Result< OutputData, ErrorData >
{
  let storage = Storage::new()
    .map_err( | e | ErrorData::new( ErrorCode::InternalError, format!( "Failed to create storage: {}", e ) ) )?;

  let project = storage.load_project_for_cwd()
    .map_err( | e | ErrorData::new
    (
      ErrorCode::InternalError,
      format!( "Failed to load project from current directory: {}", e )
    ))?;

  format_project_output( &project, verbosity )
}

/// Helper: Show specific project
fn show_project_impl( project_param : &str, verbosity : i64 )
  -> std::result::Result< OutputData, ErrorData >
{
  let storage = Storage::new()
    .map_err( | e | ErrorData::new( ErrorCode::InternalError, format!( "Failed to create storage: {}", e ) ) )?;

  // Parse project parameter
  let proj_id = parse_project_parameter( project_param )
    .map_err( | e | ErrorData::new
    (
      ErrorCode::InternalError,
      format!( "Invalid project parameter: {}", e )
    ))?;

  let project = storage.load_project( &proj_id )
    .map_err( | e | ErrorData::new
    (
      ErrorCode::InternalError,
      format!( "Failed to load project {:?}: {}", proj_id, e )
    ))?;

  format_project_output( &project, verbosity )
}

/// Helper: Format session output (extracted logic)
///
/// REQ-011: Content-First Display
///
/// By default (verbosity::1), shows conversation content in readable chat-log format.
/// Use metadata::1 or verbosity::0 for old metadata-only behavior.
fn format_session_output(
  project : &claude_storage_core::Project,
  session_id : &str,
  verbosity : i64,
  show_entries : bool,
  metadata_only : bool
) -> std::result::Result< OutputData, ErrorData >
{
  // Find session
  let mut sessions = project.all_sessions()
    .map_err( | e | ErrorData::new( ErrorCode::InternalError, format!( "Failed to list sessions: {}", e ) ) )?;

  // Fix(issue-011): Support partial UUID matching (first 8 chars)
  //
  // Root cause: Session lookup only did exact string matching without checking
  // if provided ID is a prefix of existing session IDs. Users expect Git-style
  // prefix matching for UUIDs (e.g., "79f86582" matches "79f86582-1435-442c-935a-13f8d874918a").
  //
  // Pitfall: ID lookups should always support prefix matching for UUIDs. Test with
  // both exact and partial IDs to ensure both work. Use production-format test data
  // (actual UUIDs) not test-friendly strings like "test-session-123".
  let session = sessions.iter_mut()
    .find( | s | s.id() == session_id || s.id().starts_with( session_id ) )
    .ok_or_else( || ErrorData::new( ErrorCode::InternalError, format!( "Session not found: {}", session_id ) ) )?;

  // Get session stats
  let stats = session.stats()
    .map_err( | e | ErrorData::new( ErrorCode::InternalError, format!( "Failed to get session stats: {}", e ) ) )?;

  // Format output
  let mut output = String::new();

  // REQ-011: Content-first paradigm
  // - verbosity::0 or metadata::1 → Metadata only (old behavior)
  // - verbosity::1+ (default) → Conversation content (NEW behavior)
  let show_content = verbosity >= 1 && !metadata_only;

  // Always show basic session header
  output.push_str( &format!( "Session: {} ({} entries)\n", session_id, stats.total_entries ) );

  // Metadata-only mode (old behavior)
  if metadata_only || verbosity == 0
  {
    output.push_str( &format!( "Path: {:?}\n", session.storage_path() ) );
    output.push_str( &format!( "Agent Session: {}\n", stats.is_agent_session ) );
    output.push_str( &format!( "Total Entries: {}\n", stats.total_entries ) );
    output.push_str( &format!( "User Entries: {}\n", stats.user_entries ) );
    output.push_str( &format!( "Assistant Entries: {}\n", stats.assistant_entries ) );

    if let Some( first ) = &stats.first_timestamp
    {
      output.push_str( &format!( "First Entry: {}\n", first ) );
    }

    if let Some( last ) = &stats.last_timestamp
    {
      output.push_str( &format!( "Last Entry: {}\n", last ) );
    }

    if verbosity >= 2
    {
      output.push_str( "\nToken Usage:\n" );
      output.push_str( &format!( "- Input: {}\n", stats.total_input_tokens ) );
      output.push_str( &format!( "- Output: {}\n", stats.total_output_tokens ) );
      output.push_str( &format!( "- Cache Read: {}\n", stats.total_cache_read_tokens ) );
      output.push_str( &format!( "- Cache Creation: {}\n", stats.total_cache_creation_tokens ) );
    }

    // Old entries::1 behavior (UUID list) for backward compat
    if show_entries
    {
      let entries = session.entries()
        .map_err( | e | ErrorData::new( ErrorCode::InternalError, format!( "Failed to load entries: {}", e ) ) )?;

      output.push_str( "\nEntries:\n" );

      for ( idx, entry ) in entries.iter().enumerate()
      {
        output.push_str
        (
          &format!
          (
            "{}. [{:?}] {} ({})\n",
            idx + 1,
            entry.entry_type,
            entry.uuid,
            entry.timestamp
          )
        );
      }
    }
  }
  // Content-first mode (NEW default behavior)
  else if show_content
  {
    let entries = session.entries()
      .map_err( | e | ErrorData::new( ErrorCode::InternalError, format!( "Failed to load entries: {}", e ) ) )?;

    // Add separator for readability
    output.push_str( "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n" );
    output.push( '\n' );

    // Format each entry as conversation
    for entry in entries.iter()
    {
      let formatted = format_entry_content( entry, None );
      output.push_str( &formatted );
      output.push_str( "\n\n" );
    }

    output.push_str( "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n" );

    // Add metadata footer for verbosity::2+
    if verbosity >= 2
    {
      output.push( '\n' );
      output.push_str( "Session Metadata:\n" );
      output.push_str( &format!( "- Path: {:?}\n", session.storage_path() ) );
      output.push_str( &format!( "- Total Entries: {}\n", stats.total_entries ) );
      output.push_str( &format!( "- User/Assistant: {}/{}\n", stats.user_entries, stats.assistant_entries ) );

      if verbosity >= 3
      {
        output.push_str( "\nToken Usage:\n" );
        output.push_str( &format!( "- Input: {}\n", stats.total_input_tokens ) );
        output.push_str( &format!( "- Output: {}\n", stats.total_output_tokens ) );
        output.push_str( &format!( "- Cache Read: {}\n", stats.total_cache_read_tokens ) );
        output.push_str( &format!( "- Cache Creation: {}\n", stats.total_cache_creation_tokens ) );
      }
    }
  }

  Ok( OutputData::new( output, "text" ) )
}

/// Helper: Format project output (extracted logic)
fn format_project_output(
  project : &claude_storage_core::Project,
  verbosity : i64
) -> std::result::Result< OutputData, ErrorData >
{
  // Get project statistics
  let stats = project.project_stats()
    .map_err( | e | ErrorData::new
    (
      ErrorCode::InternalError,
      format!( "Failed to get project stats: {}", e )
    ))?;

  // Get all sessions
  let mut sessions = project.sessions()
    .map_err( | e | ErrorData::new
    (
      ErrorCode::InternalError,
      format!( "Failed to list sessions: {}", e )
    ))?;

  // Format output
  let mut output = String::new();

  // Project header
  output.push_str( &format!( "Project: {:?}\n", project.id() ) );
  output.push_str( &format!( "Storage: {:?}\n", project.storage_dir() ) );
  output.push( '\n' );

  // Statistics
  output.push_str( &format!( "Sessions: {} (Main: {}, Agent: {})\n",
    stats.session_count,
    stats.main_session_count,
    stats.agent_session_count
  ));

  output.push_str( &format!( "Total Entries: {}\n", stats.total_entries ) );

  if verbosity >= 2
  {
    output.push_str( "Tokens:\n" );
    output.push_str( &format!( "  Input: {}\n", stats.total_input_tokens ) );
    output.push_str( &format!( "  Output: {}\n", stats.total_output_tokens ) );
  }

  output.push( '\n' );

  // Sessions list
  if !sessions.is_empty()
  {
    output.push_str( "Sessions:\n" );

    for session in &mut sessions
    {
      let session_stats = session.stats()
        .map_err( | e | ErrorData::new
        (
          ErrorCode::InternalError,
          format!( "Failed to get session stats: {}", e )
        ))?;

      if verbosity == 0
      {
        // Minimal: just IDs
        output.push_str( &format!( "  - {}\n", session.id() ) );
      }
      else if verbosity == 1
      {
        // Standard: ID + entry count + last timestamp
        let last = session_stats.last_timestamp
          .map( | t | t.to_string() )
          .unwrap_or_else( || "unknown".to_string() );

        output.push_str( &format!( "  - {} ({} entries, last: {})\n",
          session.id(),
          session_stats.total_entries,
          last
        ));
      }
      else
      {
        // Detailed: full stats
        output.push_str( &format!( "  - {}\n", session.id() ) );
        output.push_str( &format!( "      Entries: {} (User: {}, Assistant: {})\n",
          session_stats.total_entries,
          session_stats.user_entries,
          session_stats.assistant_entries
        ));

        if let Some( first ) = &session_stats.first_timestamp
        {
          output.push_str( &format!( "      First: {}\n", first ) );
        }

        if let Some( last ) = &session_stats.last_timestamp
        {
          output.push_str( &format!( "      Last: {}\n", last ) );
        }
      }
    }
  }
  else
  {
    output.push_str( "No sessions found in this project.\n" );
  }

  Ok( OutputData::new( output, "text" ) )
}

/// Show project details and all sessions (DEPRECATED: use .show instead)
///
/// Displays comprehensive information about a project including all sessions,
/// statistics, and metadata. This command is deprecated in favor of `.show`.
pub fn show_project_routine( cmd : VerifiedCommand, _ctx : ExecutionContext )
  -> std::result::Result< OutputData, ErrorData >
{
  let project_param = cmd.get_string( "project" );
  let verbosity = cmd.get_integer( "verbosity" ).unwrap_or( 1 );

  // Handle optional project parameter (backward compatibility)
  match project_param
  {
    Some( proj ) => show_project_impl( proj, verbosity ),
    None => show_project_for_cwd_impl( verbosity ),
  }
}

/// Count entries, sessions, or projects
///
/// Fast counting without loading all data into memory.
///
/// Context-aware: When called without parameters, counts entries in the current project
/// (detected from CWD), matching the behavior of `.show` for UX consistency.
pub fn count_routine( cmd : VerifiedCommand, _ctx : ExecutionContext )
  -> std::result::Result< OutputData, ErrorData >
{
  // Fix(issue-003a): Make .count context-aware like .show
  //
  // Root cause: .count defaulted to counting all projects globally when called without
  // parameters, while .show showed current project stats. Users expected .count to
  // count what .show shows (entries in current project).
  //
  // Pitfall: Related commands should have consistent default behaviors. If .show is
  // context-aware (uses CWD), .count should be too. Don't make one global and one local.
  let target = cmd.get_string( "target" );
  let project_id = cmd.get_string( "project" );
  let session_id = cmd.get_string( "session" );

  // Create storage instance
  let storage = Storage::new()
    .map_err( | e | ErrorData::new( ErrorCode::InternalError, format!( "Failed to create storage: {}", e ) ) )?;

  // Context-aware default: If no target and no project specified, try to count entries in CWD project
  // If CWD is not a project directory, fall back to counting all projects globally
  if target.is_none() && project_id.is_none()
  {
    if let Ok( project ) = storage.load_project_for_cwd()
    {
      // Count all entries across all sessions in the project
      let sessions = project.all_sessions()
        .map_err( | e | ErrorData::new( ErrorCode::InternalError, format!( "Failed to list sessions: {}", e ) ) )?;

      let mut total_entries = 0usize;
      for session in sessions.iter()
      {
        total_entries += session.count_entries()
          .map_err( | e | ErrorData::new( ErrorCode::InternalError, format!( "Failed to count entries: {}", e ) ) )?;
      }

      let output = format!( "{}", total_entries );
      return Ok( OutputData::new( output, "text" ) );
    }
    // If load_project_for_cwd() fails, fall through to default behavior (count all projects)
  }

  // Explicit target specified, or project without target (counts sessions in project)
  let target : &str = target.unwrap_or( "projects" );
  let count = match target
  {
    "projects" =>
    {
      storage.count_projects()
        .map_err( | e | ErrorData::new( ErrorCode::InternalError, format!( "Failed to count projects: {}", e ) ) )?
    }
    "sessions" =>
    {
      // Requires project context
      let proj_id = project_id
        .ok_or_else( || ErrorData::new( ErrorCode::InternalError, "project parameter required for counting sessions".to_string() ) )?;

      // Fix(issue-012): Support path projects in .count command
      //
      // Root cause: Hardcoded ProjectId::uuid() prevented path projects from working.
      // Commands .count/.search/.export shared this bug which was fixed for .show (Finding #008)
      // but not propagated.
      //
      // Pitfall: When fixing a bug in one command, grep for identical patterns in other commands.
      // Bugs often exist in multiple locations sharing the same flawed assumption.
      let project_id_parsed = parse_project_parameter( proj_id )
        .map_err( | e | ErrorData::new( ErrorCode::InternalError, e ) )?;

      let project = storage.load_project( &project_id_parsed )
        .map_err( | e | ErrorData::new( ErrorCode::InternalError, format!( "Failed to load project: {}", e ) ) )?;

      project.count_sessions()
        .map_err( | e | ErrorData::new( ErrorCode::InternalError, format!( "Failed to count sessions: {}", e ) ) )?
    }
    "entries" =>
    {
      // Requires project + session context
      let proj_id = project_id
        .ok_or_else( || ErrorData::new( ErrorCode::InternalError, "project parameter required for counting entries".to_string() ) )?;

      let sess_id = session_id
        .ok_or_else( || ErrorData::new( ErrorCode::InternalError, "session parameter required for counting entries".to_string() ) )?;

      // Fix(issue-012): Support path projects in .count command
      //
      // Root cause: Hardcoded ProjectId::uuid() prevented path projects from working.
      // Commands .count/.search/.export shared this bug which was fixed for .show (Finding #008)
      // but not propagated.
      //
      // Pitfall: When fixing a bug in one command, grep for identical patterns in other commands.
      // Bugs often exist in multiple locations sharing the same flawed assumption.
      let project_id_parsed = parse_project_parameter( proj_id )
        .map_err( | e | ErrorData::new( ErrorCode::InternalError, e ) )?;

      let project = storage.load_project( &project_id_parsed )
        .map_err( | e | ErrorData::new( ErrorCode::InternalError, format!( "Failed to load project: {}", e ) ) )?;

      let sessions = project.all_sessions()
        .map_err( | e | ErrorData::new( ErrorCode::InternalError, format!( "Failed to list sessions: {}", e ) ) )?;

      let session = sessions.iter()
        .find( | s | s.id() == sess_id )
        .ok_or_else( || ErrorData::new( ErrorCode::InternalError, format!( "Session not found: {}", sess_id ) ) )?;

      session.count_entries()
        .map_err( | e | ErrorData::new( ErrorCode::InternalError, format!( "Failed to count entries: {}", e ) ) )?
    }
    // Fix(issue-009): Validate target parameter against allowed values
    //
    // Root cause: target parameter accepted any string without validation,
    // causing confusing errors when invalid values provided.
    //
    // Pitfall: Don't assume unilang validates enum constraints. Always
    // validate enumerated parameters explicitly against allowed values.
    _ =>
    {
      return Err( ErrorData::new( ErrorCode::InternalError, format!( "Invalid target: {}", target ) ) );
    }
  };

  let output = format!( "{}", count );
  Ok( OutputData::new( output, "text" ) )
}

/// Search session content for query string
///
/// Performs full-text search through session content with optional filtering.
pub fn search_routine( cmd : VerifiedCommand, _ctx : ExecutionContext )
  -> std::result::Result< OutputData, ErrorData >
{
  let query = cmd.get_string( "query" )
    .ok_or_else( || ErrorData::new( ErrorCode::InternalError, "query is required".to_string() ) )?;

  let project_id = cmd.get_string( "project" );
  let session_id = cmd.get_string( "session" );
  let case_sensitive = cmd.get_boolean( "case_sensitive" ).unwrap_or( false );
  let entry_type = cmd.get_string( "entry_type" );
  let verbosity = cmd.get_integer( "verbosity" ).unwrap_or( 1 );

  // Fix(issue-010): Validate verbosity range (0-5)
  //
  // Root cause: search_routine accepted any verbosity value without validation,
  // inconsistent with status_routine and show_routine which validate 0-5 range.
  //
  // Pitfall: Don't assume default values prevent invalid input. Parameters with
  // defaults still need validation since users can override with invalid values.
  if !( 0..=5 ).contains( &verbosity )
  {
    return Err
    (
      ErrorData::new
      (
        ErrorCode::InternalError,
        format!( "Invalid verbosity: {}. Valid range: 0-5", verbosity )
      )
    );
  }

  // Create storage instance
  let storage = Storage::new()
    .map_err( | e | ErrorData::new( ErrorCode::InternalError, format!( "Failed to create storage: {}", e ) ) )?;

  // Build search filter
  let mut filter = claude_storage_core::SearchFilter::new( query )
    .case_sensitive( case_sensitive );

  // Add entry type filter if specified
  if let Some( et ) = entry_type
  {
    let entry_type_enum = match et
    {
      "user" => claude_storage_core::EntryType::User,
      "assistant" => claude_storage_core::EntryType::Assistant,
      _ => return Err( ErrorData::new( ErrorCode::InternalError, format!( "Invalid entry_type: {}", et ) ) ),
    };
    filter = filter.match_entry_type( entry_type_enum );
  }

  // Determine search scope
  let mut all_matches = Vec::new();

  if let Some( sess_id ) = session_id
  {
    // Search specific session
    let project = if let Some( proj_id ) = project_id
    {
      // Fix(issue-012): Support path projects in .search command
      //
      // Root cause: Hardcoded ProjectId::uuid() prevented path projects from working.
      // Commands .count/.search/.export shared this bug which was fixed for .show (Finding #008)
      // but not propagated.
      //
      // Pitfall: When fixing a bug in one command, grep for identical patterns in other commands.
      // Bugs often exist in multiple locations sharing the same flawed assumption.
      let project_id_parsed = parse_project_parameter( proj_id )
        .map_err( | e | ErrorData::new( ErrorCode::InternalError, e ) )?;

      storage.load_project( &project_id_parsed )
    }
    else
    {
      storage.load_project_for_cwd()
    }
    .map_err( | e | ErrorData::new( ErrorCode::InternalError, format!( "Failed to load project: {}", e ) ) )?;

    let mut sessions = project.all_sessions()
      .map_err( | e | ErrorData::new( ErrorCode::InternalError, format!( "Failed to list sessions: {}", e ) ) )?;

    let session = sessions.iter_mut()
      .find( | s | s.id() == sess_id )
      .ok_or_else( || ErrorData::new( ErrorCode::InternalError, format!( "Session not found: {}", sess_id ) ) )?;

    let matches = session.search( &filter )
      .map_err( | e | ErrorData::new( ErrorCode::InternalError, format!( "Search failed: {}", e ) ) )?;

    for m in matches
    {
      all_matches.push( ( project.id().clone(), sess_id.to_string(), m ) );
    }
  }
  else if let Some( proj_id ) = project_id
  {
    // Search specific project
    // Fix(issue-012): Support path projects in .search command
    //
    // Root cause: Hardcoded ProjectId::uuid() prevented path projects from working.
    // Commands .count/.search/.export shared this bug which was fixed for .show (Finding #008)
    // but not propagated.
    //
    // Pitfall: When fixing a bug in one command, grep for identical patterns in other commands.
    // Bugs often exist in multiple locations sharing the same flawed assumption.
    let project_id_parsed = parse_project_parameter( proj_id )
      .map_err( | e | ErrorData::new( ErrorCode::InternalError, e ) )?;

    let project = storage.load_project( &project_id_parsed )
      .map_err( | e | ErrorData::new( ErrorCode::InternalError, format!( "Failed to load project: {}", e ) ) )?;

    let mut sessions = project.sessions()
      .map_err( | e | ErrorData::new( ErrorCode::InternalError, format!( "Failed to list sessions: {}", e ) ) )?;

    for session in &mut sessions
    {
      let matches = session.search( &filter )
        .map_err( | e | ErrorData::new( ErrorCode::InternalError, format!( "Search failed in session {}: {}", session.id(), e ) ) )?;

      for m in matches
      {
        all_matches.push( ( project.id().clone(), session.id().to_string(), m ) );
      }
    }
  }
  else
  {
    // Search all projects and sessions (current working directory project only)
    let project = storage.load_project_for_cwd()
      .map_err( | e | ErrorData::new( ErrorCode::InternalError, format!( "Failed to load project: {}", e ) ) )?;

    let mut sessions = project.sessions()
      .map_err( | e | ErrorData::new( ErrorCode::InternalError, format!( "Failed to list sessions: {}", e ) ) )?;

    for session in &mut sessions
    {
      let matches = session.search( &filter )
        .map_err( | e | ErrorData::new( ErrorCode::InternalError, format!( "Search failed in session {}: {}", session.id(), e ) ) )?;

      for m in matches
      {
        all_matches.push( ( project.id().clone(), session.id().to_string(), m ) );
      }
    }
  }

  // Format output
  let mut output = String::new();

  if verbosity >= 1
  {
    output.push_str( &format!( "Found {} matches:\n\n", all_matches.len() ) );
  }

  for ( proj_id, sess_id, m ) in &all_matches
  {
    match verbosity
    {
      0 =>
      {
        // Minimal: just excerpt
        output.push_str( &format!( "{}\n", m.excerpt() ) );
      }
      1 =>
      {
        // Standard: session + excerpt
        output.push_str
        (
          &format!
          (
            "[{}] [{:?}] {}\n",
            sess_id,
            m.entry_type(),
            m.excerpt()
          )
        );
      }
      _ =>
      {
        // Detailed: full metadata
        output.push_str
        (
          &format!
          (
            "Project: {:?}\nSession: {}\nEntry: {} ({})\nLine: {}\nExcerpt: {}\nFull Line: {}\n\n",
            proj_id,
            sess_id,
            m.entry_index(),
            match m.entry_type()
            {
              claude_storage_core::EntryType::User => "user",
              claude_storage_core::EntryType::Assistant => "assistant",
            },
            m.line_number(),
            m.excerpt(),
            m.full_line()
          )
        );
      }
    }
  }

  if all_matches.is_empty()
  {
    output.push_str( "No matches found.\n" );
  }

  Ok( OutputData::new( output, "text" ) )
}

/// Export session to file
///
/// Exports a session to the specified format (markdown, JSON, or text).
pub fn export_routine( cmd : VerifiedCommand, _ctx : ExecutionContext )
  -> std::result::Result< OutputData, ErrorData >
{
  let session_id = cmd.get_string( "session_id" )
    .ok_or_else( || ErrorData::new( ErrorCode::InternalError, "session_id is required".to_string() ) )?;

  let output_path_str = cmd.get_string( "output" )
    .ok_or_else( || ErrorData::new( ErrorCode::InternalError, "output is required".to_string() ) )?;

  let format_str = cmd.get_string( "format" ).unwrap_or( "markdown" );
  let project_id = cmd.get_string( "project" );

  // Parse export format
  let format = claude_storage_core::ExportFormat::from_str( format_str )
    .map_err( | e | ErrorData::new( ErrorCode::InternalError, format!( "Invalid format: {}", e ) ) )?;

  // Create storage instance
  let storage = Storage::new()
    .map_err( | e | ErrorData::new( ErrorCode::InternalError, format!( "Failed to create storage: {}", e ) ) )?;

  // Load project
  let project = if let Some( proj_id ) = project_id
  {
    // Fix(issue-012): Support path projects in .export command
    //
    // Root cause: Hardcoded ProjectId::uuid() prevented path projects from working.
    // Commands .count/.search/.export shared this bug which was fixed for .show (Finding #008)
    // but not propagated.
    //
    // Pitfall: When fixing a bug in one command, grep for identical patterns in other commands.
    // Bugs often exist in multiple locations sharing the same flawed assumption.
    let project_id_parsed = parse_project_parameter( proj_id )
      .map_err( | e | ErrorData::new( ErrorCode::InternalError, e ) )?;

    storage.load_project( &project_id_parsed )
  }
  else
  {
    storage.load_project_for_cwd()
  }
  .map_err( | e | ErrorData::new( ErrorCode::InternalError, format!( "Failed to load project: {}", e ) ) )?;

  // Find session
  let mut sessions = project.all_sessions()
    .map_err( | e | ErrorData::new( ErrorCode::InternalError, format!( "Failed to list sessions: {}", e ) ) )?;

  // Fix(issue-011): Support partial UUID matching (first 8 chars)
  //
  // Root cause: Session lookup only did exact string matching without checking
  // if provided ID is a prefix of existing session IDs. Users expect Git-style
  // prefix matching for UUIDs (e.g., "79f86582" matches "79f86582-1435-442c-935a-13f8d874918a").
  //
  // Pitfall: ID lookups should always support prefix matching for UUIDs. Test with
  // both exact and partial IDs to ensure both work. Use production-format test data
  // (actual UUIDs) not test-friendly strings like "test-session-123".
  let session = sessions.iter_mut()
    .find( | s | s.id() == session_id || s.id().starts_with( session_id ) )
    .ok_or_else( || ErrorData::new( ErrorCode::InternalError, format!( "Session not found: {}", session_id ) ) )?;

  // Export to file
  let output_path = std::path::Path::new( output_path_str );

  claude_storage_core::export_session_to_file( session, format, output_path )
    .map_err( | e | ErrorData::new( ErrorCode::InternalError, format!( "Export failed: {}", e ) ) )?;

  let output = format!( "Exported session '{}' to {:?} (format: {:?})", session_id, output_path, format );
  Ok( OutputData::new( output, "text" ) )
}
