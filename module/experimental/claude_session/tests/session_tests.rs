//! Integration tests for `SessionManager`.

use claude_session::{ SessionManager, Strategy };
use tempfile::TempDir;

#[ test ]
fn session_dir_format()
{
  let temp = TempDir::new().unwrap();
  let sessions_root = temp.path().join( "sessions" );
  let mgr = SessionManager::new( &sessions_root );

  let dir = mgr.session_dir( "test-session" );

  assert!( dir.ends_with( "sessions/-test-session" ) );
}

#[ test ]
fn session_exists_returns_false_for_nonexistent()
{
  //! Verify deprecated `session_exists()` behavior for nonexistent session.
  //!
  //! **Note:** This tests the deprecated v1.x detection method.
  //! Modern code should use `check_session_exists()` instead.

  let temp = TempDir::new().unwrap();
  let sessions_root = temp.path().join( "sessions" );
  let mgr = SessionManager::new( &sessions_root );

  #[ allow( deprecated ) ]
  {
    assert!( !mgr.session_exists( "nonexistent" ) );
  }
}

#[ test ]
fn session_exists_returns_false_without_history_file()
{
  //! Verify deprecated `session_exists()` requires `.claude_history` file.
  //!
  //! **Note:** This tests the deprecated v1.x detection method.

  let temp = TempDir::new().unwrap();
  let sessions_root = temp.path().join( "sessions" );
  let mgr = SessionManager::new( &sessions_root );

  // Create directory but not history file
  let session_dir = mgr.session_dir( "test" );
  std::fs::create_dir_all( &session_dir ).unwrap();

  #[ allow( deprecated ) ]
  {
    // Should return false (no history file)
    assert!( !mgr.session_exists( "test" ) );
  }
}

#[ test ]
fn session_exists_returns_true_with_history_file()
{
  //! Verify deprecated `session_exists()` detects `.claude_history` file.
  //!
  //! **Note:** This tests the deprecated v1.x detection method.

  let temp = TempDir::new().unwrap();
  let sessions_root = temp.path().join( "sessions" );
  let mgr = SessionManager::new( &sessions_root );

  // Create directory and history file
  let session_dir = mgr.session_dir( "test" );
  std::fs::create_dir_all( &session_dir ).unwrap();
  std::fs::write( session_dir.join( ".claude_history" ), "" ).unwrap();

  #[ allow( deprecated ) ]
  {
    // Should return true
    assert!( mgr.session_exists( "test" ) );
  }
}

#[ test ]
fn ensure_session_creates_directory()
{
  let temp = TempDir::new().unwrap();
  let sessions_root = temp.path().join( "sessions" );
  let mgr = SessionManager::new( &sessions_root );

  let session_dir = mgr.ensure_session( "test", Strategy::Resume ).unwrap();

  assert!( session_dir.exists() );
  assert!( session_dir.ends_with( "-test" ) );
}

#[ test ]
fn ensure_session_is_idempotent()
{
  let temp = TempDir::new().unwrap();
  let sessions_root = temp.path().join( "sessions" );
  let mgr = SessionManager::new( &sessions_root );

  // First call
  let dir1 = mgr.ensure_session( "test", Strategy::Resume ).unwrap();

  // Second call should succeed and return same path
  let dir2 = mgr.ensure_session( "test", Strategy::Resume ).unwrap();

  assert_eq!( dir1, dir2 );
  assert!( dir1.exists() );
}

#[ test ]
fn ensure_session_resume_preserves_existing()
{
  let temp = TempDir::new().unwrap();
  let sessions_root = temp.path().join( "sessions" );
  let mgr = SessionManager::new( &sessions_root );

  // Create session with marker file
  let session_dir = mgr.ensure_session( "test", Strategy::Resume ).unwrap();
  let marker_file = session_dir.join( "marker.txt" );
  std::fs::write( &marker_file, "existing" ).unwrap();

  // Resume should preserve marker
  mgr.ensure_session( "test", Strategy::Resume ).unwrap();

  assert!( marker_file.exists() );
  assert_eq!( std::fs::read_to_string( &marker_file ).unwrap(), "existing" );
}

#[ test ]
fn ensure_session_fresh_deletes_existing()
{
  let temp = TempDir::new().unwrap();
  let sessions_root = temp.path().join( "sessions" );
  let mgr = SessionManager::new( &sessions_root );

  // Create session with marker file
  let session_dir = mgr.ensure_session( "test", Strategy::Resume ).unwrap();
  let marker_file = session_dir.join( "marker.txt" );
  std::fs::write( &marker_file, "old" ).unwrap();

  // Fresh should delete marker
  mgr.ensure_session( "test", Strategy::Fresh ).unwrap();

  assert!( !marker_file.exists() );
}

#[ test ]
fn ensure_session_fresh_creates_clean_directory()
{
  let temp = TempDir::new().unwrap();
  let sessions_root = temp.path().join( "sessions" );
  let mgr = SessionManager::new( &sessions_root );

  // Create session with files
  let session_dir = mgr.ensure_session( "test", Strategy::Resume ).unwrap();
  std::fs::write( session_dir.join( "file1.txt" ), "data1" ).unwrap();
  std::fs::write( session_dir.join( "file2.txt" ), "data2" ).unwrap();

  // Fresh start
  let new_dir = mgr.ensure_session( "test", Strategy::Fresh ).unwrap();

  // Directory should be empty
  let entries : Vec< _ > = std::fs::read_dir( &new_dir ).unwrap().collect();
  assert_eq!( entries.len(), 0 );
}

#[ test ]
fn sessions_base_dir_returns_correct_path()
{
  let temp = TempDir::new().unwrap();
  let sessions_root = temp.path().join( "test-sessions" );
  let mgr = SessionManager::new( &sessions_root );

  let base_dir = mgr.sessions_base_dir();

  assert!( base_dir.ends_with( "test-sessions" ) );
}

#[ test ]
fn modern_detection_supersedes_deprecated_method()
{
  //! Verify that modern `check_session_exists()` works independently
  //! of deprecated `session_exists()` method.
  //!
  //! **Root Cause:** Claude Code v2.0+ changed storage pattern from
  //! `.claude_history` files to `~/.claude/projects/` centralized storage.
  //!
  //! **Fix Applied:** Provide modern detection via `check_session_exists()`
  //! and deprecate old `session_exists()` method.
  //!
  //! **Prevention:** Always use storage-based detection that matches
  //! actual application behavior, not assumed file patterns.
  //!
  //! **Pitfall:** File-based assumptions break when applications change
  //! storage patterns. Detect sessions via actual storage inspection.

  use claude_session::check_session_exists;

  let temp = TempDir::new().unwrap();
  let sessions_root = temp.path().join( "sessions" );
  let mgr = SessionManager::new( &sessions_root );

  // Create session directory
  let session_dir = mgr.ensure_session( "test", Strategy::Resume ).unwrap();

  // No .claude_history file created
  assert!( !session_dir.join( ".claude_history" ).exists() );

  #[ allow( deprecated ) ]
  {
    // Deprecated method returns false (no .claude_history)
    assert!( !mgr.session_exists( "test" ) );
  }

  // Modern method checks actual Claude storage
  // (will be false for test environment, but API works correctly)
  let modern_result = check_session_exists( &session_dir );

  // Both should agree: no conversation history yet
  #[ allow( deprecated ) ]
  {
    assert_eq!( mgr.session_exists( "test" ), modern_result );
  }
}
