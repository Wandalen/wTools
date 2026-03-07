//! `StringMatcher` tests - zero-dependency case-insensitive substring matching
//!
//! ## Design
//!
//! `StringMatcher` provides case-insensitive substring matching without regex dependencies.
//! Uses `to_lowercase()` for Unicode-aware matching.
//!
//! ## Key Features
//!
//! - **Case-insensitive**: "`WillBe`" matches "willbe", "WILLBE", "`WiLlBe`"
//! - **Empty pattern matches all**: Empty string matches any text
//! - **Unicode-aware**: Uses `to_lowercase()` for proper Unicode handling
//! - **Zero dependencies**: No regex crate, stdlib only
//!
//! ## Test Coverage
//!
//! 1. Basic substring matching
//! 2. Case-insensitive matching
//! 3. Empty pattern behavior
//! 4. Unicode handling
//! 5. Non-matches
//! 6. Edge cases (empty text, pattern longer than text)

use claude_storage_core::StringMatcher;

#[test]
fn exact_match_same_case()
{
  let matcher = StringMatcher::new( "willbe" );
  assert!( matcher.matches( "willbe" ) );
}

#[test]
fn exact_match_different_case()
{
  let matcher = StringMatcher::new( "WillBe" );
  assert!( matcher.matches( "willbe" ) );
  assert!( matcher.matches( "WILLBE" ) );
  assert!( matcher.matches( "WiLlBe" ) );
}

#[test]
fn substring_match_beginning()
{
  let matcher = StringMatcher::new( "claude" );
  assert!( matcher.matches( "claude_storage" ) );
  assert!( matcher.matches( "CLAUDE_STORAGE" ) );
}

#[test]
fn substring_match_middle()
{
  let matcher = StringMatcher::new( "storage" );
  assert!( matcher.matches( "claude_storage_core" ) );
  assert!( matcher.matches( "/home/user/claude_storage/src" ) );
}

#[test]
fn substring_match_end()
{
  let matcher = StringMatcher::new( "core" );
  assert!( matcher.matches( "claude_storage_core" ) );
  assert!( matcher.matches( "CLAUDE_STORAGE_CORE" ) );
}

#[test]
fn empty_pattern_matches_all()
{
  let matcher = StringMatcher::new( "" );
  assert!( matcher.matches( "anything" ) );
  assert!( matcher.matches( "ANYTHING" ) );
  assert!( matcher.matches( "" ) );
  assert!( matcher.matches( "multiple words here" ) );
}

#[test]
fn empty_text_matches_only_empty_pattern()
{
  let matcher = StringMatcher::new( "willbe" );
  assert!( !matcher.matches( "" ) );

  let empty_matcher = StringMatcher::new( "" );
  assert!( empty_matcher.matches( "" ) );
}

#[test]
fn pattern_longer_than_text()
{
  let matcher = StringMatcher::new( "very_long_pattern" );
  assert!( !matcher.matches( "short" ) );
}

#[test]
fn no_match()
{
  let matcher = StringMatcher::new( "willbe" );
  assert!( !matcher.matches( "claude" ) );
  assert!( !matcher.matches( "storage" ) );
  assert!( !matcher.matches( "wplan" ) );
}

#[test]
fn unicode_matching()
{
  let matcher = StringMatcher::new( "café" );
  assert!( matcher.matches( "café" ) );
  assert!( matcher.matches( "CAFÉ" ) );
  assert!( matcher.matches( "my café is nice" ) );
}

#[test]
fn real_world_path_matching()
{
  let matcher = StringMatcher::new( "willbe" );
  assert!( matcher.matches( "/home/user1/pro/lib/willbe/module/claude_storage" ) );
  assert!( matcher.matches( "/HOME/USER1/PRO/LIB/WILLBE/MODULE/CLAUDE_STORAGE" ) );
  assert!( !matcher.matches( "/home/user1/pro/lib/wplan/src" ) );
}

#[test]
fn real_world_session_id_matching()
{
  let matcher = StringMatcher::new( "default" );
  assert!( matcher.matches( "-default_topic" ) );
  assert!( matcher.matches( "agent-default-session" ) );
  assert!( matcher.matches( "DEFAULT_TOPIC" ) );
  assert!( !matcher.matches( "commit-session" ) );
}

#[test]
fn whitespace_sensitive()
{
  let matcher = StringMatcher::new( "my project" );
  assert!( matcher.matches( "my project" ) );
  assert!( matcher.matches( "MY PROJECT" ) );
  assert!( !matcher.matches( "myproject" ) ); // Whitespace matters
}

#[test]
fn special_characters()
{
  let matcher = StringMatcher::new( "-default_topic" );
  assert!( matcher.matches( "/commands/-default_topic" ) );
  assert!( matcher.matches( "/COMMANDS/-DEFAULT_TOPIC" ) );
  assert!( !matcher.matches( "/commands/default_topic" ) ); // Hyphen matters
}
