//! Verbose Flag and Path Edge Case Tests
//!
//! Tests for edge cases in verbose flag behavior and path parameters.
//!
//! ## Test Coverage
//!
//! | Category | Edge Case | Status |
//! |----------|-----------|--------|
//! | Verbose | verbose(true) | ✓ |
//! | Verbose | verbose(false) | ✓ |
//! | Verbose | verbose not set | ✓ |
//! | Paths | Unicode paths | ✓ |
//! | Paths | Empty path | ✓ |
//! | Paths | Path with shell metacharacters | ✓ |
//! | Paths | Very long path | ✓ |
//! | Paths | Path with newlines | ✓ |

use claude_runner_core::ClaudeCommand;

// ============================================================================
// Verbose Flag Edge Cases
// ============================================================================

#[test]
fn verbose_true_adds_flag() {
  let cmd = ClaudeCommand::new()
    .with_verbose( true );
  let built = cmd.build_command_for_test();
  let debug = format!( "{built:?}" );

  assert!( debug.contains( "--verbose" ) );
  println!( "✓ verbose(true) adds --verbose flag" );
}

#[test]
fn verbose_false_does_not_add_flag() {
  let cmd = ClaudeCommand::new()
    .with_verbose( false );
  let built = cmd.build_command_for_test();
  let debug = format!( "{built:?}" );

  // Should NOT contain --verbose
  assert!( !debug.contains( "--verbose" ), "verbose(false) should NOT add --verbose flag" );
  println!( "✓ verbose(false) correctly does NOT add --verbose flag" );
}

#[test]
fn verbose_not_set_default_behavior() {
  let cmd = ClaudeCommand::new();
  let built = cmd.build_command_for_test();
  let debug = format!( "{built:?}" );

  // By default, verbose should not be set
  assert!( !debug.contains( "--verbose" ), "Default should NOT include --verbose flag" );
  println!( "✓ Default (verbose not called) does NOT add --verbose flag" );
}

#[test]
fn verbose_true_then_false_last_wins() {
  // When called multiple times, last call should win
  // But wait - with_verbose doesn't store a bool, it just pushes to args
  // So calling true then false... let's see what happens
  let cmd = ClaudeCommand::new()
    .with_verbose( true )
    .with_verbose( false );
  let built = cmd.build_command_for_test();
  let debug = format!( "{built:?}" );

  // The first call adds --verbose, second call (false) does nothing
  // So --verbose WILL be present (accumulation behavior, not override)
  // This documents actual behavior
  println!( "verbose(true) then verbose(false): {}", if debug.contains( "--verbose" ) { "flag present" } else { "flag absent" } );

  // Document actual behavior: first true adds flag, subsequent false doesn't remove it
  // This is expected given the implementation: if verbose { push }
  assert!( debug.contains( "--verbose" ), "First verbose(true) adds flag; verbose(false) doesn't remove" );
  println!( "✓ verbose(true) followed by verbose(false): flag remains (accumulation, not override)" );
}

// ============================================================================
// Path Edge Cases - Working Directory
// ============================================================================

#[test]
fn working_directory_unicode_path() {
  let cmd = ClaudeCommand::new()
    .with_working_directory( "/tmp/путь/路径/مسار" );
  let built = cmd.build_command_for_test();
  let debug = format!( "{built:?}" );

  // PathBuf should preserve unicode - debug shows: cd "/tmp/путь/路径/مسار"
  assert!( debug.contains( "cd" ) );
  assert!( debug.contains( "путь" ) );
  println!( "✓ Unicode working directory handled" );
}

#[test]
fn working_directory_empty_string() {
  // Empty string becomes empty PathBuf
  let cmd = ClaudeCommand::new()
    .with_working_directory( "" );
  let built = cmd.build_command_for_test();
  let debug = format!( "{built:?}" );

  // Debug format shows: cd "" && "claude" ...
  assert!( debug.contains( "cd \"\"" ) );
  println!( "✓ Empty working directory path accepted (cd \"\" in command)" );
}

#[test]
fn working_directory_shell_metacharacters() {
  // Test paths with characters that could cause shell issues if improperly escaped
  let cmd = ClaudeCommand::new()
    .with_working_directory( "/tmp/test$HOME;rm -rf /; echo 'pwned'" );
  let built = cmd.build_command_for_test();
  let debug = format!( "{built:?}" );

  // Debug format shows path in cd command
  // The path is preserved literally (not shell-expanded)
  assert!( debug.contains( "cd" ) );
  assert!( debug.contains( "$HOME" ) );
  println!( "✓ Path with shell metacharacters accepted (preserved literally)" );
}

#[test]
fn working_directory_very_long_path() {
  // 4096 char path (common filesystem limit)
  let long_component = "a".repeat( 255 );  // Max path component on most filesystems
  let long_path = format!( "/tmp/{long_component}/{long_component}/{long_component}/{long_component}" );

  let cmd = ClaudeCommand::new()
    .with_working_directory( &long_path );
  let built = cmd.build_command_for_test();
  let debug = format!( "{built:?}" );

  // Debug format shows cd "long_path" && ...
  assert!( debug.contains( "cd" ) );
  assert!( debug.contains( &long_component[ ..50 ] ) );  // Check first 50 chars of component
  println!( "✓ Very long path ({} chars) accepted", long_path.len() );
}

#[test]
fn working_directory_with_newlines() {
  // Paths with newlines (weird but technically valid on some systems)
  let cmd = ClaudeCommand::new()
    .with_working_directory( "/tmp/path\nwith\nnewlines" );
  let built = cmd.build_command_for_test();
  let debug = format!( "{built:?}" );

  // Debug format shows path with escaped newlines: cd "/tmp/path\nwith\nnewlines"
  assert!( debug.contains( "cd" ) );
  assert!( debug.contains( "/tmp/path" ) );
  println!( "✓ Path with newlines accepted" );
}

// ============================================================================
// Path Edge Cases - Session Directory
// ============================================================================

#[test]
fn session_dir_unicode_path() {
  let cmd = ClaudeCommand::new()
    .with_session_dir( "/tmp/セッション/сессия" );
  let built = cmd.build_command_for_test();
  let debug = format!( "{built:?}" );

  assert!( debug.contains( "CLAUDE_CODE_SESSION_DIR" ) );
  println!( "✓ Unicode session directory handled" );
}

#[test]
fn session_dir_empty_string() {
  let cmd = ClaudeCommand::new()
    .with_session_dir( "" );
  let built = cmd.build_command_for_test();
  let debug = format!( "{built:?}" );

  assert!( debug.contains( "CLAUDE_CODE_SESSION_DIR" ) );
  println!( "✓ Empty session directory path accepted" );
}

#[test]
fn session_dir_with_spaces() {
  let cmd = ClaudeCommand::new()
    .with_session_dir( "/tmp/session with many spaces/sub dir" );
  let built = cmd.build_command_for_test();
  let debug = format!( "{built:?}" );

  assert!( debug.contains( "CLAUDE_CODE_SESSION_DIR" ) );
  assert!( debug.contains( "session with many spaces" ) );
  println!( "✓ Session directory with spaces handled" );
}

// ============================================================================
// Model and API Key Edge Cases
// ============================================================================

#[test]
fn model_empty_string() {
  let cmd = ClaudeCommand::new()
    .with_model( "" );
  let built = cmd.build_command_for_test();
  let debug = format!( "{built:?}" );

  // Should add --model followed by empty string arg
  assert!( debug.contains( "--model" ) );
  println!( "✓ Empty model name accepted (adds --model with empty arg)" );
}

#[test]
fn model_with_spaces() {
  let cmd = ClaudeCommand::new()
    .with_model( "model with spaces" );
  let built = cmd.build_command_for_test();
  let debug = format!( "{built:?}" );

  assert!( debug.contains( "--model" ) );
  assert!( debug.contains( "model with spaces" ) );
  println!( "✓ Model name with spaces handled" );
}

#[test]
fn model_with_special_characters() {
  let cmd = ClaudeCommand::new()
    .with_model( "claude-opus-4-5@v1.0-beta" );
  let built = cmd.build_command_for_test();
  let debug = format!( "{built:?}" );

  assert!( debug.contains( "--model" ) );
  assert!( debug.contains( "claude-opus-4-5@v1.0-beta" ) );
  println!( "✓ Model name with special characters handled" );
}

#[test]
fn api_key_empty_string() {
  let cmd = ClaudeCommand::new()
    .with_api_key( "" );
  let built = cmd.build_command_for_test();
  let debug = format!( "{built:?}" );

  assert!( debug.contains( "--api-key" ) );
  println!( "✓ Empty API key accepted (adds --api-key with empty arg)" );
}

#[test]
fn api_key_with_special_characters() {
  // API keys often have special chars
  let cmd = ClaudeCommand::new()
    .with_api_key( "sk-ant-api03-abc123_XYZ-789+test/key==" );
  let built = cmd.build_command_for_test();
  let debug = format!( "{built:?}" );

  assert!( debug.contains( "--api-key" ) );
  assert!( debug.contains( "sk-ant-api03" ) );
  println!( "✓ API key with special characters handled" );
}

// ============================================================================
// Default Trait Implementation
// ============================================================================

#[test]
fn default_trait_same_as_new() {
  let from_new = ClaudeCommand::new();
  let from_default = ClaudeCommand::default();

  let new_debug = format!( "{from_new:?}" );
  let default_debug = format!( "{from_default:?}" );

  assert_eq!( new_debug, default_debug );
  println!( "✓ Default::default() produces same result as new()" );
}
