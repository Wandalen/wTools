//! Corner case tests for `claude_session`.
//!
//! These tests verify edge cases not covered by main integration tests:
//! - Path escaping edge cases (PE-1 through PE-7)
//! - Session detection edge cases (SD-1, SD-6, SD-7)
//! - `Strategy` `FromStr` edge cases (SF-1 through SF-5)
//! - `SessionManager` edge cases (SM-4)

use std::path::PathBuf;
use core::str::FromStr;
use tempfile::TempDir;

use claude_session::{ Strategy, SessionManager, check_session_exists, get_claude_storage_path };

// ============================================================================
// Path Escaping Edge Cases (PE-1 through PE-7)
// ============================================================================

/// PE-1: All special characters should be escaped to hyphens.
/// Spec FR-3: `/_.@#%& ` to `-`
#[ test ]
fn path_escaping_all_special_chars()
{
  let session_dir = PathBuf::from( "/home/user_name.test@example#file%data&info with spaces" );

  if let Some( storage_path ) = get_claude_storage_path( &session_dir )
  {
    let path_str = storage_path.display().to_string();

    // Extract only the escaped portion (after .claude/projects/)
    // The HOME prefix naturally contains slashes which is expected
    let escaped_portion = path_str
      .rsplit( ".claude/projects/" )
      .next()
      .unwrap();

    // Verify all special chars in the session path are escaped
    assert!( !escaped_portion.contains( '/' ), "slash should be escaped: {escaped_portion}" );
    assert!( !escaped_portion.contains( '_' ), "underscore should be escaped: {escaped_portion}" );
    assert!( !escaped_portion.contains( '.' ), "dot should be escaped: {escaped_portion}" );
    assert!( !escaped_portion.contains( '@' ), "at should be escaped: {escaped_portion}" );
    assert!( !escaped_portion.contains( '#' ), "hash should be escaped: {escaped_portion}" );
    assert!( !escaped_portion.contains( '%' ), "percent should be escaped: {escaped_portion}" );
    assert!( !escaped_portion.contains( '&' ), "ampersand should be escaped: {escaped_portion}" );
    assert!( !escaped_portion.contains( ' ' ), "space should be escaped: {escaped_portion}" );
  }
  else
  {
    panic!( "get_claude_storage_path returned None (HOME not set?)" );
  }
}

/// PE-2: Multiple consecutive special characters become multiple hyphens.
#[ test ]
fn path_escaping_consecutive_special_chars()
{
  let session_dir = PathBuf::from( "/tmp//..test" );

  if let Some( storage_path ) = get_claude_storage_path( &session_dir )
  {
    let path_str = storage_path.display().to_string();
    let escaped_portion = path_str
      .rsplit( ".claude/projects/" )
      .next()
      .unwrap();

    // -tmp---test (multiple hyphens expected)
    assert!( escaped_portion.contains( "---" ), "consecutive specials become multiple hyphens: {escaped_portion}" );
  }
  else
  {
    panic!( "get_claude_storage_path returned None" );
  }
}

/// PE-5: Unicode characters should pass through unchanged.
#[ test ]
fn path_escaping_unicode_passthrough()
{
  let session_dir = PathBuf::from( "/tmp/日本語/中文/한국어" );

  if let Some( storage_path ) = get_claude_storage_path( &session_dir )
  {
    let path_str = storage_path.display().to_string();

    assert!( path_str.contains( "日本語" ), "Japanese should pass through: {path_str}" );
    assert!( path_str.contains( "中文" ), "Chinese should pass through: {path_str}" );
    assert!( path_str.contains( "한국어" ), "Korean should pass through: {path_str}" );
  }
  else
  {
    panic!( "get_claude_storage_path returned None" );
  }
}

/// PE-6: Empty path produces empty escaped string.
#[ test ]
fn path_escaping_empty_path()
{
  let session_dir = PathBuf::from( "" );

  if let Some( storage_path ) = get_claude_storage_path( &session_dir )
  {
    // Empty path should produce storage path ending with empty escaped portion
    let path_str = storage_path.display().to_string();
    assert!( path_str.ends_with( "projects/" ), "empty path produces empty escaped: {path_str}" );
  }
  else
  {
    panic!( "get_claude_storage_path returned None" );
  }
}

// ============================================================================
// Session Detection Edge Cases (SD-1, SD-6, SD-7)
// ============================================================================

/// SD-6: Valid `conversation.json` should be detected.
#[ test ]
fn detection_conversation_json()
{
  let temp_dir = TempDir::new().unwrap();
  let session_dir = temp_dir.path().join( "test-session-conv-json" );
  std::fs::create_dir_all( &session_dir ).unwrap();

  // Build Claude storage path
  let home_dir = std::env::var( "HOME" ).unwrap();
  let session_path_str = session_dir.display().to_string();
  let escaped_path : String = session_path_str
    .chars()
    .map( | c | if matches!( c, '/' | '_' | '.' | '@' | '#' | '%' | '&' | ' ' ) { '-' } else { c } )
    .collect();

  let claude_storage = PathBuf::from( &home_dir )
    .join( ".claude" )
    .join( "projects" )
    .join( &escaped_path );

  std::fs::create_dir_all( &claude_storage ).unwrap();

  // Create conversation.json with content
  std::fs::write( claude_storage.join( "conversation.json" ), r#"{"test":true}"# ).unwrap();

  // Should detect conversation.json
  assert!( check_session_exists( &session_dir ), "conversation.json should be detected" );

  // Cleanup
  let _ = std::fs::remove_dir_all( &claude_storage );
}

/// SD-7: Valid `.claude*` files should be detected.
#[ test ]
fn detection_claude_dotfile()
{
  let temp_dir = TempDir::new().unwrap();
  let session_dir = temp_dir.path().join( "test-session-claude-dotfile" );
  std::fs::create_dir_all( &session_dir ).unwrap();

  // Build Claude storage path
  let home_dir = std::env::var( "HOME" ).unwrap();
  let session_path_str = session_dir.display().to_string();
  let escaped_path : String = session_path_str
    .chars()
    .map( | c | if matches!( c, '/' | '_' | '.' | '@' | '#' | '%' | '&' | ' ' ) { '-' } else { c } )
    .collect();

  let claude_storage = PathBuf::from( &home_dir )
    .join( ".claude" )
    .join( "projects" )
    .join( &escaped_path );

  std::fs::create_dir_all( &claude_storage ).unwrap();

  // Create .claude_settings with content
  std::fs::write( claude_storage.join( ".claude_history" ), "test content" ).unwrap();

  // Should detect .claude* file
  assert!( check_session_exists( &session_dir ), ".claude* file should be detected" );

  // Cleanup
  let _ = std::fs::remove_dir_all( &claude_storage );
}

// ============================================================================
// Strategy FromStr Edge Cases (SF-1 through SF-5)
// ============================================================================

/// SF-1: "resume" parses to `Strategy::Resume`
#[ test ]
fn strategy_fromstr_resume()
{
  let result = Strategy::from_str( "resume" );
  assert!( matches!( result, Ok( Strategy::Resume ) ) );
}

/// SF-2: "fresh" parses to `Strategy::Fresh`
#[ test ]
fn strategy_fromstr_fresh()
{
  let result = Strategy::from_str( "fresh" );
  assert!( matches!( result, Ok( Strategy::Fresh ) ) );
}

/// SF-3: "RESUME" (uppercase) returns error
#[ test ]
fn strategy_fromstr_uppercase_fails()
{
  let result = Strategy::from_str( "RESUME" );
  assert!( result.is_err(), "uppercase should fail" );

  let result = Strategy::from_str( "FRESH" );
  assert!( result.is_err(), "uppercase should fail" );
}

/// SF-4: "" (empty) returns error
#[ test ]
fn strategy_fromstr_empty_fails()
{
  let result = Strategy::from_str( "" );
  assert!( result.is_err(), "empty string should fail" );
}

/// SF-5: invalid strings return error with helpful message
#[ test ]
fn strategy_fromstr_invalid_fails()
{
  let result = Strategy::from_str( "invalid" );
  assert!( result.is_err() );

  if let Err( msg ) = result
  {
    assert!( msg.contains( "resume" ), "error should mention valid options: {msg}" );
    assert!( msg.contains( "fresh" ), "error should mention valid options: {msg}" );
  }
}

/// SF-3 additional: Mixed case returns error
#[ test ]
fn strategy_fromstr_mixed_case_fails()
{
  let cases = vec![ "Resume", "Fresh", "RESUME", "FRESH", "reSume", "frEsh" ];

  for case in cases
  {
    let result = Strategy::from_str( case );
    assert!( result.is_err(), "'{case}' should fail (case sensitive)" );
  }
}

// ============================================================================
// SessionManager Edge Cases (SM-4)
// ============================================================================

/// SM-4: `ensure_session` Fresh works when directory doesn't exist.
#[ test ]
fn ensure_session_fresh_creates_new()
{
  let temp = TempDir::new().unwrap();
  let sessions_root = temp.path().join( "sessions" );
  let mgr = SessionManager::new( &sessions_root );

  // Fresh on non-existent should succeed
  let session_dir = mgr.ensure_session( "new-session", Strategy::Fresh ).unwrap();

  assert!( session_dir.exists() );
  assert!( session_dir.ends_with( "-new-session" ) );
}

/// Verify `session_dir` returns correct format with special session names.
#[ test ]
fn session_dir_special_names()
{
  let temp = TempDir::new().unwrap();
  let sessions_root = temp.path().join( "sessions" );
  let mgr = SessionManager::new( &sessions_root );

  // Session name with hyphen
  let dir = mgr.session_dir( "my-debug-session" );
  assert!( dir.ends_with( "-my-debug-session" ) );

  // Session name with numbers
  let dir = mgr.session_dir( "session123" );
  assert!( dir.ends_with( "-session123" ) );

  // Default session name
  let dir = mgr.session_dir( "default" );
  assert!( dir.ends_with( "-default" ) );
}
