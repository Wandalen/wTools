//! Directory-based session isolation for Claude Code.
//!
//! # Design
//!
//! Sessions are identified by topic name and isolated in `-{topic}/` directories
//! within a topic-specific sessions directory.
//!
//! ## Session Detection
//!
//! **Modern approach (Claude Code v2.0+):**
//! Use [`check_session_exists()`](crate::check_session_exists) from the `detection` module.
//! This checks Claude's actual storage location at `~/.claude/projects/{escaped-path}/`.
//!
//! **Legacy approach (Claude Code v1.x):**
//! The [`SessionManager::session_exists()`] method checks for `.claude_history` files.
//! This is deprecated and only works for old Claude Code versions.
//!
//! ## Session Strategy
//!
//! - `Resume`: Continue existing session or create new (default)
//! - `Fresh`: Discard existing session and start clean
//!
//! ## Session Location
//!
//! Sessions are created in the current working directory:
//! - Location: `{current_working_directory}/-{session_name}/`
//! - Example: Running from `/project/` creates sessions at `/project/-debug/`
//! - Sessions prefixed with hyphen for git exclusion
//!
//! This ensures Claude Code executes with access to project files.
//!
//! # Examples
//!
//! ## Check Session Existence (Recommended)
//!
//! ```
//! use claude_session::{ SessionManager, Strategy, check_session_exists };
//! use std::path::PathBuf;
//!
//! let temp = tempfile::tempdir().unwrap();
//! let sessions_dir = temp.path().join( "sessions" );
//! let mgr = SessionManager::new( &sessions_dir );
//!
//! // Create session directory
//! let session_dir = mgr.ensure_session( "debug", Strategy::Resume ).unwrap();
//! assert!( session_dir.exists() );
//!
//! // Check if session has conversation history (modern approach)
//! let has_history = check_session_exists( &session_dir );
//! assert!( !has_history );  // New session, no history yet
//! ```

use std::path::{ Path, PathBuf };
use core::str::FromStr;

/// Session creation strategy.
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub enum Strategy
{
  /// Resume existing session or create new.
  Resume,
  /// Discard existing session and create fresh.
  Fresh,
}

impl FromStr for Strategy
{
  type Err = String;

  #[ inline ]
  fn from_str( s : &str ) -> Result< Self, Self::Err >
  {
    match s
    {
      "resume" => Ok( Strategy::Resume ),
      "fresh" => Ok( Strategy::Fresh ),
      _ => Err( format!( "Invalid strategy: '{s}'. Use 'resume' or 'fresh'" ) ),
    }
  }
}

/// Session manager for Claude Code workspaces.
///
/// Manages directory-based session isolation with deterministic
/// storage-based detection.
///
/// Sessions are now stored per-topic in `{topic_dir}/sessions/` rather than
/// globally in `claude_sessions/`.
#[ derive( Debug ) ]
pub struct SessionManager
{
  sessions_base_dir : PathBuf,
}

impl SessionManager
{
  /// Create session manager with sessions base directory.
  ///
  /// # Parameters
  ///
  /// - `sessions_base_dir`: Base directory for sessions (typically `{topic_dir}/sessions/`)
  ///
  /// # Examples
  ///
  /// ```
  /// use claude_session::SessionManager;
  /// use std::path::PathBuf;
  ///
  /// let sessions_dir = PathBuf::from( "/tmp/wplan/abc123/sessions" );
  /// let mgr = SessionManager::new( &sessions_dir );
  /// ```
  #[ inline ]
  pub fn new( sessions_base_dir : impl AsRef< Path > ) -> Self
  {
    Self
    {
      sessions_base_dir : sessions_base_dir.as_ref().to_path_buf(),
    }
  }

  /// Get session directory path for a session name.
  ///
  /// Session directories are prefixed with hyphen for git exclusion:
  /// `{sessions_base_dir}/-{session_name}/`
  ///
  /// # Parameters
  ///
  /// - `session_name`: Name of the session (e.g., "debug", "default")
  ///
  /// # Returns
  ///
  /// Full path to session directory (may not exist yet)
  #[ inline ]
  #[ must_use ]
  pub fn session_dir( &self, session_name : &str ) -> PathBuf
  {
    self.sessions_base_dir
      .join( format!( "-{session_name}" ) )
  }

  /// Check if session exists using storage-based detection.
  ///
  /// **DEPRECATED:** This method checks for `.claude_history` files which
  /// Claude Code v1.x created. Claude Code v2.0+ uses centralized storage
  /// at `~/.claude/projects/` instead.
  ///
  /// Use [`check_session_exists()`](crate::check_session_exists) instead,
  /// which checks Claude's actual storage location.
  ///
  /// # Migration Example
  ///
  /// ```
  /// use claude_session::{ SessionManager, check_session_exists };
  ///
  /// let mgr = SessionManager::new( "/path/to/sessions" );
  /// let session_dir = mgr.session_dir( "my-session" );
  ///
  /// // OLD (deprecated):
  /// // let exists = mgr.session_exists( "my-session" );
  ///
  /// // NEW (recommended):
  /// let exists = check_session_exists( &session_dir );
  /// ```
  ///
  /// # Legacy Behavior
  ///
  /// A session exists if:
  /// 1. Session directory exists: `-{session_name}/`
  /// 2. Claude history file exists: `-{session_name}/.claude_history`
  ///
  /// # Returns
  ///
  /// `true` if `.claude_history` file exists (Claude Code v1.x sessions only)
  #[ deprecated(
    since = "0.2.0",
    note = "Only detects Claude Code v1.x sessions. \
            Use `check_session_exists()` for v2.0+ compatibility."
  ) ]
  #[ inline ]
  #[ must_use ]
  pub fn session_exists( &self, session_name : &str ) -> bool
  {
    let session_dir = self.session_dir( session_name );
    let history_file = session_dir.join( ".claude_history" );

    session_dir.exists() && history_file.exists()
  }

  /// Ensure session directory exists (idempotent).
  ///
  /// Creates or resumes session based on strategy:
  /// - `Resume`: Create if doesn't exist, keep existing if it does
  /// - `Fresh`: Delete existing session, create clean one
  ///
  /// # Parameters
  ///
  /// - `session_name`: Name of the session (e.g., "debug", "default")
  /// - `strategy`: Session creation strategy
  ///
  /// # Returns
  ///
  /// Path to session directory on success
  ///
  /// # Errors
  ///
  /// Returns error if filesystem operations fail
  ///
  /// # Examples
  ///
  /// ```
  /// use claude_session::{ SessionManager, Strategy };
  /// use std::path::PathBuf;
  ///
  /// let sessions_dir = PathBuf::from( "/tmp/test/sessions" );
  /// let mgr = SessionManager::new( &sessions_dir );
  ///
  /// // Resume (create if needed)
  /// let dir = mgr.ensure_session( "default", Strategy::Resume ).unwrap();
  ///
  /// // Fresh (delete and recreate)
  /// let dir = mgr.ensure_session( "default", Strategy::Fresh ).unwrap();
  /// ```
  #[ inline ]
  pub fn ensure_session(
    &self,
    session_name : &str,
    strategy : Strategy,
  ) -> Result< PathBuf, std::io::Error >
  {
    let session_dir = self.session_dir( session_name );

    match strategy
    {
      Strategy::Fresh if session_dir.exists() =>
      {
        // Delete existing session
        std::fs::remove_dir_all( &session_dir )?;
      }
      _ => {}
    }

    // Create directory (idempotent)
    std::fs::create_dir_all( &session_dir )?;

    Ok( session_dir )
  }

  /// Get base sessions directory.
  ///
  /// Returns the configured sessions base directory.
  #[ inline ]
  #[ must_use ]
  pub fn sessions_base_dir( &self ) -> &Path
  {
    &self.sessions_base_dir
  }
}
