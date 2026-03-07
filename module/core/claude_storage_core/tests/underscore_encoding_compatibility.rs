//! Test: Underscore Encoding Compatibility with Claude Code
//!
//! ## Root Cause
//!
//! `claude_storage_core` v1.0.1 encoder preserved underscores in path components, while
//! Claude Code's encoder replaces them with hyphens. This created an encoding mismatch:
//!
//! - **Claude Code behavior**: `/claude_storage` → `-claude-storage` (underscore → hyphen)
//! - **Our v1.0.1 behavior**: `/claude_storage` → `-claude_storage` (underscore preserved)
//!
//! This caused paths with underscores to fail when loading from real Claude Code storage:
//!
//! ```text
//! $ claude_storage .show.project /home/user1/pro/lib/willbe/module/claude_storage/-default_topic
//! Error: Project not found: /home/user1/.claude/projects/-home-user1-pro-lib-willbe-module-claude_storage--default_topic
//! ```
//!
//! The storage directory actually exists with hyphens:
//! `/home/user1/.claude/projects/-home-user1-pro-lib-willbe-module-claude-storage--default-topic`
//!
//! Our encoder looked for: `claude_storage` (with underscore)
//! Claude Code created: `claude-storage` (with hyphen)
//!
//! ## Why Not Caught
//!
//! 1. **Insufficient test coverage**: No tests verified compatibility with actual Claude Code storage
//! 2. **Documentation mismatch**: Comments claimed underscores were replaced (line 6), but code didn't implement it
//! 3. **Decoder assumed encoder behavior**: v1 heuristic decoder expected underscores replaced, but encoder didn't do it
//! 4. **No real-world validation**: Tests used synthetic paths without underscores
//!
//! ## Fix Applied
//!
//! Updated `encode_path()` to replace underscores with hyphens in each component before
//! processing, matching Claude Code's lossy encoding scheme:
//!
//! ```rust
//! // In encode_path() function:
//! for (i, component) in components.iter().enumerate() {
//!   // NEW: Replace underscores with hyphens for Claude Code compatibility
//!   let component_normalized = component.replace('_', "-");
//!
//!   // Rest of encoding logic uses component_normalized
//!   // ...
//! }
//! ```
//!
//! ## Prevention
//!
//! 1. **Compatibility tests**: Always test against real Claude Code storage directories
//! 2. **Encoder-decoder contract**: Document and test that encoder behavior matches decoder expectations
//! 3. **Real-world paths**: Include paths with underscores in test suite
//! 4. **Cross-validation**: Verify encoded paths match actual Claude Code directory names
//!
//! ## Pitfall
//!
//! **Lossy encoding requires matching encoder and decoder**. When documentation says the
//! encoding is lossy (replaces multiple characters with same output), both encoder and
//! decoder must agree on the transformation. Decoder expecting transformation that encoder
//! doesn't perform creates an encoding mismatch that breaks compatibility.
//!
//! Specifically: If decoder uses heuristics expecting `_` → `-`, encoder MUST do that
//! transformation. Preserving underscores while decoder expects them replaced creates
//! paths that don't round-trip correctly with real storage.

use claude_storage_core::{ encode_path, decode_path };
use std::path::{ Path, PathBuf };

#[ test ]
fn test_underscore_replaced_in_component()
{
  // Path with underscore in component name
  let path = Path::new( "/lib/claude_storage" );
  let encoded = encode_path( path ).unwrap();

  // Underscore should be replaced with hyphen
  assert_eq!( encoded, "-lib-claude-storage" );

  // Decoder should handle this correctly
  let decoded = decode_path( &encoded ).unwrap();

  // Due to lossy encoding, we get slashes not underscores
  // (decoder can't distinguish `/` from `_`)
  assert_eq!( decoded, PathBuf::from( "/lib/claude/storage" ) );
}

#[ test ]
fn test_underscore_in_hyphen_prefixed_component()
{
  // Path with underscore in hyphen-prefixed component
  let path = Path::new( "/commands/-default_topic" );
  let encoded = encode_path( path ).unwrap();

  // Underscore replaced: "-default_topic" → "-default-topic"
  // Then hyphen-prefix encoding: "-default-topic" → "--default-topic"
  assert_eq!( encoded, "-commands--default-topic" );

  // Decoder should restore hyphen-prefixed component
  let decoded = decode_path( &encoded ).unwrap();

  // Heuristic restores underscore in hyphen-prefixed component
  assert_eq!( decoded, PathBuf::from( "/commands/-default_topic" ) );
}

#[ test ]
fn test_real_claude_storage_path()
{
  // Real path from manual testing
  let path = Path::new( "/home/user1/pro/lib/willbe/module/claude_storage/-default_topic" );
  let encoded = encode_path( path ).unwrap();

  // Should match actual Claude Code storage directory name
  assert_eq!
  (
    encoded,
    "-home-user1-pro-lib-willbe-module-claude-storage--default-topic"
  );

  // Verify this matches the directory that actually exists in ~/.claude/projects/
  // (This test documents the expected behavior, actual existence checked manually)
}

#[ test ]
fn test_multiple_underscores()
{
  // Path with multiple components containing underscores
  let path = Path::new( "/module/wplan_agent/-default_topic" );
  let encoded = encode_path( path ).unwrap();

  // Both underscores replaced
  assert_eq!( encoded, "-module-wplan-agent--default-topic" );
}

#[ test ]
fn test_underscore_and_hyphen_in_same_component()
{
  // Path with both underscore and hyphen
  let path = Path::new( "/foo_bar-baz" );
  let encoded = encode_path( path ).unwrap();

  // Both become hyphens (lossy)
  assert_eq!( encoded, "-foo-bar-baz" );
}

#[ test ]
fn test_round_trip_lossy()
{
  // Demonstrate lossy encoding behavior
  let original_with_underscore = Path::new( "/lib/claude_storage" );
  let original_with_slash = Path::new( "/lib/claude/storage" );

  // Both encode to the same string (lossy)
  let encoded1 = encode_path( original_with_underscore ).unwrap();
  let encoded2 = encode_path( original_with_slash ).unwrap();

  assert_eq!( encoded1, encoded2 );
  assert_eq!( encoded1, "-lib-claude-storage" );

  // Decoding produces one canonical form (decoder prefers slashes)
  let decoded = decode_path( &encoded1 ).unwrap();
  assert_eq!( decoded, PathBuf::from( "/lib/claude/storage" ) );
}

#[ test ]
fn test_compatibility_with_existing_tests()
{
  // Verify this change doesn't break paths without underscores
  let path = Path::new( "/home/user/project" );
  let encoded = encode_path( path ).unwrap();
  assert_eq!( encoded, "-home-user-project" );

  let decoded = decode_path( &encoded ).unwrap();
  assert_eq!( decoded, path );
}

#[ test ]
fn test_underscore_in_deeply_nested_path()
{
  // Real-world complex path
  let path = Path::new( "/home/user1/pro/lib/willbe/module/wplan_agent/tests/test_file.rs" );
  let encoded = encode_path( path ).unwrap();

  // All underscores replaced
  assert_eq!
  (
    encoded,
    "-home-user1-pro-lib-willbe-module-wplan-agent-tests-test-file.rs"
  );
}
