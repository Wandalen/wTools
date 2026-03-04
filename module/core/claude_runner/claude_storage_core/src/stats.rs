//! Statistics types for Claude Code storage
//!
//! Provides structures for collecting and reporting statistics about
//! sessions and global storage usage.

use std::collections::HashMap;

/// Statistics for a single session
///
/// Provides detailed metrics about conversation entries, token usage,
/// and session characteristics without requiring full entry parsing.
#[derive( Debug, Clone, PartialEq )]
pub struct SessionStats
{
  /// Total number of entries in this session
  pub total_entries : usize,

  /// Number of user messages
  pub user_entries : usize,

  /// Number of assistant messages
  pub assistant_entries : usize,

  /// Total input tokens across all assistant responses
  pub total_input_tokens : u64,

  /// Total output tokens across all assistant responses
  pub total_output_tokens : u64,

  /// Total cache read tokens (prompt caching optimization)
  pub total_cache_read_tokens : u64,

  /// Total cache creation tokens
  pub total_cache_creation_tokens : u64,

  /// Whether this is an agent/sidechain session (sub-conversation)
  pub is_agent_session : bool,

  /// Session ID
  pub session_id : String,

  /// First entry timestamp (ISO 8601 format)
  pub first_timestamp : Option< String >,

  /// Last entry timestamp (ISO 8601 format)
  pub last_timestamp : Option< String >,
}

impl SessionStats
{
  /// Create new empty session statistics
  pub fn new( session_id : String ) -> Self
  {
    Self
    {
      total_entries : 0,
      user_entries : 0,
      assistant_entries : 0,
      total_input_tokens : 0,
      total_output_tokens : 0,
      total_cache_read_tokens : 0,
      total_cache_creation_tokens : 0,
      is_agent_session : false,
      session_id,
      first_timestamp : None,
      last_timestamp : None,
    }
  }
}

/// Global statistics across all projects and sessions
///
/// Aggregates metrics from all sessions in Claude Code storage,
/// providing high-level overview of usage patterns.
#[derive( Debug, Clone, PartialEq )]
pub struct GlobalStats
{
  /// Total number of projects
  pub total_projects : usize,

  /// Number of UUID-based projects (web/IDE sessions)
  pub uuid_projects : usize,

  /// Number of path-based projects (CLI sessions)
  pub path_projects : usize,

  /// Total number of sessions across all projects
  pub total_sessions : usize,

  /// Number of main sessions (non-agent)
  pub main_sessions : usize,

  /// Number of agent/sidechain sessions
  pub agent_sessions : usize,

  /// Total number of entries across all sessions
  pub total_entries : usize,

  /// Total user messages
  pub total_user_entries : usize,

  /// Total assistant messages
  pub total_assistant_entries : usize,

  /// Total input tokens
  pub total_input_tokens : u64,

  /// Total output tokens
  pub total_output_tokens : u64,

  /// Total cache read tokens
  pub total_cache_read_tokens : u64,

  /// Total cache creation tokens
  pub total_cache_creation_tokens : u64,

  /// Per-project breakdown
  pub project_breakdown : HashMap< String, ProjectStats >,
}

impl GlobalStats
{
  /// Create new empty global statistics
  pub fn new() -> Self
  {
    Self
    {
      total_projects : 0,
      uuid_projects : 0,
      path_projects : 0,
      total_sessions : 0,
      main_sessions : 0,
      agent_sessions : 0,
      total_entries : 0,
      total_user_entries : 0,
      total_assistant_entries : 0,
      total_input_tokens : 0,
      total_output_tokens : 0,
      total_cache_read_tokens : 0,
      total_cache_creation_tokens : 0,
      project_breakdown : HashMap::new(),
    }
  }
}

impl Default for GlobalStats
{
  fn default() -> Self
  {
    Self::new()
  }
}

/// Statistics for a single project
#[derive( Debug, Clone, PartialEq )]
pub struct ProjectStats
{
  /// Project identifier (UUID or path)
  pub project_id : String,

  /// Number of sessions in this project
  pub session_count : usize,

  /// Number of main sessions
  pub main_session_count : usize,

  /// Number of agent sessions
  pub agent_session_count : usize,

  /// Total entries in this project
  pub total_entries : usize,

  /// Total input tokens for this project
  pub total_input_tokens : u64,

  /// Total output tokens for this project
  pub total_output_tokens : u64,
}

impl ProjectStats
{
  /// Create new empty project statistics
  pub fn new( project_id : String ) -> Self
  {
    Self
    {
      project_id,
      session_count : 0,
      main_session_count : 0,
      agent_session_count : 0,
      total_entries : 0,
      total_input_tokens : 0,
      total_output_tokens : 0,
    }
  }
}
