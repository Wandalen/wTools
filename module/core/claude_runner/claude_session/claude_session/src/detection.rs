//! Claude Code session detection.
//!
//! # Design
//!
//! Claude Code v2.0+ tracks sessions by EXECUTION DIRECTORY.
//! When you run Claude from `/path/to/-session/`, it stores conversation files in
//! `~/.claude/projects/{escaped-path}/` where the path is escaped by replacing
//! special characters with hyphens.
//!
//! # Algorithm
//!
//! From `clm_please`:
//! 1. Build Claude storage path: `~/.claude/projects/{escaped-session-path}/`
//! 2. Check if conversation files exist (`*.jsonl`, `conversation.json`, `.claude*`)
//! 3. Skip agent definition files (`agent-*.jsonl`)
//! 4. Skip empty files (0 bytes)
//!
//! # Examples
//!
//! ```
//! use claude_session::detection;
//! use std::path::PathBuf;
//!
//! let session_dir = PathBuf::from( "/tmp/my-session" );
//! let exists = detection::check_session_exists( &session_dir );
//!
//! if exists
//! {
//!   println!( "Session has conversation history" );
//! }
//! ```

use std::path::{ Path, PathBuf };

/// Get the Claude storage path for a session directory.
///
/// Claude Code v2.0+ stores sessions in `~/.claude/projects/{escaped-path}/`
/// where the path is escaped by replacing special characters with hyphens.
///
/// # Parameters
///
/// - `session_dir`: Session directory to get storage path for
///
/// # Returns
///
/// `Some(PathBuf)` with Claude storage path, or `None` if HOME is not set
///
/// # Examples
///
/// ```
/// use claude_session::detection;
/// use std::path::PathBuf;
///
/// let session_dir = PathBuf::from( "/tmp/my-session" );
/// if let Some( storage ) = detection::get_claude_storage_path( &session_dir )
/// {
///   println!( "Claude stores sessions at: {}", storage.display() );
/// }
/// ```
#[ inline ]
#[ must_use ]
pub fn get_claude_storage_path( session_dir : &Path ) -> Option< PathBuf >
{
  // Get HOME directory
  let home_dir = std::env::var( "HOME" ).ok()?;

  // Build Claude's storage path: ~/.claude/projects/{escaped-session-path}/
  // Claude escapes paths by replacing /_.@#%& and spaces with hyphens
  let session_path_str = session_dir.display().to_string();
  let escaped_path = session_path_str
    .chars()
    .map( | c | if matches!( c, '/' | '_' | '.' | '@' | '#' | '%' | '&' | ' ' ) { '-' } else { c } )
    .collect::< String >();

  Some
  (
    PathBuf::from( &home_dir )
      .join( ".claude" )
      .join( "projects" )
      .join( &escaped_path )
  )
}

/// Check if Claude storage contains conversation history for a session directory.
///
/// Claude Code v2.0+ tracks sessions by EXECUTION DIRECTORY.
/// When you run Claude from `/path/to/-session/`, it stores conversation files in
/// `~/.claude/projects/{escaped-path}/` where the path is escaped by replacing
/// special characters with hyphens.
///
/// # Parameters
///
/// - `session_dir`: Session directory to check for conversation history
///
/// # Returns
///
/// `true` if Claude storage contains conversation files for this session
///
/// # Algorithm
///
/// From `clm_please`:
/// 1. Get HOME directory
/// 2. Build Claude storage path: `~/.claude/projects/{escaped-path}/`
/// 3. Escape path: replace `/_.@#%&` and spaces with hyphens
/// 4. Check for conversation files (`*.jsonl`, `conversation.json`, `.claude*`)
/// 5. Skip agent definition files (`agent-*.jsonl`)
/// 6. Skip empty files (0 bytes - created during crashes)
///
/// # Examples
///
/// ```
/// use claude_session::detection;
/// use std::path::PathBuf;
///
/// let session_dir = PathBuf::from( "/tmp/test-session" );
/// let has_history = detection::check_session_exists( &session_dir );
///
/// if has_history
/// {
///   println!( "Session exists with conversation history" );
/// }
/// else
/// {
///   println!( "New session (no history)" );
/// }
/// ```
#[ inline ]
#[ must_use ]
pub fn check_session_exists( session_dir : &Path ) -> bool
{
  // Get Claude storage path
  let claude_storage = match get_claude_storage_path( session_dir )
  {
    Some( path ) => path,
    None => return false,
  };

  // Check if storage directory exists and has conversation files
  if !claude_storage.exists()
  {
    return false;
  }

  // Look for conversation files (*.jsonl, conversation.json, or .claude* files)
  // Skip agent-*.jsonl files (they're agent definitions, not conversations)
  if let Ok( entries ) = std::fs::read_dir( &claude_storage )
  {
    for entry in entries.flatten()
    {
      if let Some( filename ) = entry.file_name().to_str()
      {
        // Skip agent definition files
        if filename.starts_with( "agent-" )
        {
          continue;
        }

        // Fix(issue-wplan-empty-session-file): Skip empty files in session detection
        //
        // Root cause: Empty conversation files created by Claude Code during initialization
        // were incorrectly treated as valid sessions. When Claude crashes before writing
        // content, 0-byte .jsonl files remain and trigger false positive detection.
        //
        // Pitfall: File existence alone doesn't guarantee valid state. Always validate
        // file size for state detection. Empty files can result from crashes, failed writes,
        // or atomic creation patterns.
        if let Ok( metadata ) = entry.metadata()
        {
          if metadata.len() == 0
          {
            continue;
          }
        }

        if Path::new( filename )
            .extension()
            .map_or( false, | ext | ext.eq_ignore_ascii_case( "jsonl" ) )
          || filename == "conversation.json"
          || filename.starts_with( ".claude" )
        {
          return true;
        }
      }
    }
  }

  false
}
