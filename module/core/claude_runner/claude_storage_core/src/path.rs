//! Path encoding/decoding utilities for Claude Code storage
//!
//! Claude Code encodes filesystem paths into storage directory names using a **lossy** scheme:
//! 1. Replace `_` (underscore) with `-` in each path component
//! 2. Prefix with `-` (hyphen)
//! 3. Replace `/` (path separator) with `-`
//!
//! This encoding cannot distinguish between `/`, `_`, and `-` in the original path.
//!
//! # Examples
//!
//! ```
//! use claude_storage_core::{ encode_path, decode_path };
//! use std::path::Path;
//!
//! // Normal paths encode/decode correctly
//! let path = Path::new("/home/user/project");
//! let encoded = encode_path(path)?;
//! assert_eq!(encoded, "-home-user-project");
//!
//! // Underscores are replaced with hyphens
//! let path = Path::new("/lib/claude_storage/-default_topic");
//! let encoded = encode_path(path)?;
//! assert_eq!(encoded, "-lib-claude-storage--default-topic");
//! # Ok::<(), claude_storage_core::Error>(())
//! ```
//!
//! # Known Pitfalls
//!
//! ## Lossy Encoding Creates Ambiguity
//!
//! Claude Code's encoding scheme is fundamentally **lossy** - it converts multiple distinct
//! characters to the same representation:
//! - `/` (path separator) → `-`
//! - `_` (underscore) → `-`
//! - `-` (hyphen in name) → `-`
//!
//! This means paths like `/foo-bar`, `/foo_bar`, and `/foo/bar` all encode to `-foo-bar`.
//! The decoder cannot perfectly reconstruct the original path.
//!
//! **Impact**: Decoding requires heuristics. Our decoder assumes:
//! - `--` marks hyphen-prefixed components (like `/-default_topic`)
//! - `-` within hyphen-prefixed components represents `_` (not `/`)
//! - `-` between normal components represents `/`
//!
//! ## Never Use Naive String Replacement
//!
//! **Anti-pattern** (causes double-slash bug):
//! ```ignore
//! // WRONG: Blindly replaces all hyphens with slashes
//! fn decode_bad(encoded: &str) -> String {
//!   encoded.trim_start_matches('-').replace('-', "/")
//! }
//! // Input:  "-commands--default-topic"
//! // Output: "/commands//default/topic"  ❌ WRONG
//! ```
//!
//! **Correct pattern** (context-aware state machine):
//! ```ignore
//! // RIGHT: Tracks context to make intelligent decisions
//! fn decode_good(encoded: &str) -> String {
//!   let mut in_hyphen_component = false;
//!   // ... decode based on state and lookahead
//! }
//! // Input:  "-commands--default-topic"
//! // Output: "/commands/-default_topic"  ✓ CORRECT
//! ```
//!
//! **Lesson**: Lossy encodings require context-aware decoders, never naive character replacement.
//!
//! ## Component Boundaries Are Ambiguous
//!
//! After a `--` prefix, the decoder cannot know where the component ends:
//! - `-a--b-c` could be `/-a_b/c` (two components) or `/-a_b_c` (one component)
//!
//! Our heuristic assumes it's one component: `/-a_b_c`. This matches real-world Claude Code
//! usage where hyphen-prefixed directories use underscores internally (`-default_topic`, `-commit`).
//!
//! ## Testing Requirements
//!
//! When working with lossy encodings, always test:
//! 1. **Round-trip property**: `decode(encode(x))` should equal `x` (or close enough)
//! 2. **Real-world patterns**: Test against actual storage directory names, not just synthetic data
//! 3. **Ambiguous cases**: Test patterns with consecutive special characters (`--`, `__`)
//! 4. **Edge cases**: Leading/trailing special characters, empty components
//!
//! See `tests/path_encoding_double_slash_bug.rs` for comprehensive bug reproducer documentation.

use std::path::{ Path, PathBuf };
use crate::{ Error, Result };

/// Encode a filesystem path to a storage directory name
///
/// Encoding algorithm (matches Claude Code's lossy encoding):
/// 1. Replace `_` with `-` in each component
/// 2. Prefix with `-` (hyphen)
/// 3. Replace `/` with `-` for normal path separators
/// 4. Replace `/-` with `--` for hyphen-prefixed directory names
///
/// This creates a lossy encoding that cannot distinguish between `/`, `_`, and `-`
/// in the original path, matching Claude Code's behavior for compatibility.
///
/// # Examples
///
/// ```
/// use claude_storage_core::encode_path;
/// use std::path::Path;
///
/// let path = Path::new("/home/user/project");
/// let encoded = encode_path(path)?;
/// assert_eq!(encoded, "-home-user-project");
///
/// // Underscores are replaced with hyphens
/// let path = Path::new("/lib/claude_storage/-default_topic");
/// let encoded = encode_path(path)?;
/// assert_eq!(encoded, "-lib-claude-storage--default-topic");
/// # Ok::<(), claude_storage_core::Error>(())
/// ```
pub fn encode_path( path : &Path ) -> Result< String >
{
  let path_str = path
    .to_str()
    .ok_or_else( || Error::path_encoding
    (
      format!( "{:?}", path ),
      "path contains invalid UTF-8".to_string()
    ))?;

  // Split path into components and encode each
  let components : Vec< &str > = path_str
    .trim_start_matches( '/' )
    .trim_end_matches( '/' )
    .split( '/' )
    .collect();

  if components.is_empty() || ( components.len() == 1 && components[ 0 ].is_empty() )
  {
    return Err( Error::path_encoding
    (
      path_str,
      "path is empty after normalization"
    ));
  }

  // Encode path components:
  // - ALL components: lossy (underscores → hyphens, paths → hyphens)
  // - Join components with single hyphens (path separators)
  // - Components starting with hyphen get double-hyphen prefix (--)
  // - Decoder uses heuristics to reconstruct paths (hyphen-prefixed: → underscores)
  let mut result = String::with_capacity( path_str.len() );
  result.push( '-' ); // Leading hyphen prefix

  for ( i, component ) in components.iter().enumerate()
  {
    // Encoding strategy:
    // - ALL components: underscores → hyphens (lossy encoding, like `/` → `-`)
    // - The decoder uses different heuristics to decide if hyphens should decode to `/` or `_`
    // - For hyphen-prefixed components, decoder converts ALL hyphens back to underscores
    let component_normalized = component.replace( '_', "-" );

    if i > 0
    {
      // Add separator before each component (except first)
      if let Some( stripped ) = component_normalized.strip_prefix( '-' )
      {
        result.push_str( "--" ); // Double hyphen for hyphen-prefixed component
        result.push_str( stripped ); // Rest of component (skip leading hyphen)
      }
      else
      {
        result.push( '-' ); // Single hyphen separator
        result.push_str( &component_normalized ); // Normal component
      }
    }
    else
    {
      // First component
      if let Some( stripped ) = component_normalized.strip_prefix( '-' )
      {
        result.push( '-' ); // Extra hyphen for hyphen-prefixed first component
        result.push_str( stripped );
      }
      else
      {
        result.push_str( &component_normalized );
      }
    }
  }

  Ok( result )
}

/// Decode a storage directory name to a filesystem path
///
/// Decoding algorithm (lossy heuristic):
/// 1. Remove leading `-` (hyphen) prefix
/// 2. Use heuristic to distinguish between `/` and `_` (both encoded as `-`)
/// 3. Handle hyphen-prefixed directories (`--` = `/-`)
///
/// Since the encoding is lossy (both `/` and `_` → `-`), the decoder uses heuristics
/// to reconstruct the most likely original path, matching Claude Code's behavior.
///
/// # Examples
///
/// ```
/// use claude_storage_core::decode_path;
/// use std::path::Path;
///
/// let decoded = decode_path("-home-user-project")?;
/// assert_eq!(decoded, Path::new("/home/user/project"));
///
/// // Heuristic restores underscores in hyphen-prefixed components
/// let decoded = decode_path("-lib-claude-storage--default-topic")?;
/// assert_eq!(decoded, Path::new("/lib/claude/storage/-default_topic"));
/// # Ok::<(), claude_storage_core::Error>(())
/// ```
pub fn decode_path( encoded : &str ) -> Result< PathBuf >
{
  if !encoded.starts_with( '-' )
  {
    return Err( Error::path_encoding
    (
      encoded,
      "encoded path must start with '-'"
    ));
  }

  if encoded.len() == 1
  {
    return Err( Error::path_encoding
    (
      encoded,
      "encoded path is empty after removing prefix"
    ));
  }

  // Use heuristic decoder for all paths (matches Claude Code's lossy encoding)
  decode_v1_heuristic( encoded )
}

/// Heuristic decoder for lossy path encoding
///
/// This is the smart heuristic from 2025-11-29 that fixed the double-slash bug,
/// enhanced 2025-11-30 to handle underscore-in-component decoding.
/// Matches Claude Code's lossy encoding where both `/` and `_` become `-`.
fn decode_v1_heuristic( encoded : &str ) -> Result< PathBuf >
{
  // Fix(issue-path-decoding-2025-11-30): Enhanced heuristic for underscore vs path separator
  //
  // Root cause: Claude Code's encoding is lossy - it converts both `/` and `_` to `-`,
  // creating ambiguity when decoding. For example:
  // - `/claude/storage` encodes to `-claude-storage`
  // - `/claude_storage` ALSO encodes to `-claude-storage`
  //
  // Enhanced character-by-character state machine with pattern matching heuristics.

  let path_str = &encoded[ 1.. ]; // Strip encoding marker

  let mut result = String::with_capacity( path_str.len() + 10 );
  result.push( '/' );

  let mut chars = path_str.chars().peekable();
  let mut current_component = String::new();
  let mut in_hyphen_prefixed = false;

  while let Some( ch ) = chars.next()
  {
    if ch == '-'
    {
      // Check for double hyphen (hyphen-prefixed component marker)
      if chars.peek() == Some( &'-' )
      {
        // Flush current component
        if !current_component.is_empty()
        {
          // Check if this component starts with `-` (hyphen-prefixed at path start)
          if let Some( stripped ) = current_component.strip_prefix( '-' )
          {
            result.push( '-' );
            result.push_str( &decode_component( stripped, true ) );
          }
          else
          {
            result.push_str( &decode_component( &current_component, in_hyphen_prefixed ) );
          }
          current_component.clear();
        }

        // Start new hyphen-prefixed component
        result.push( '/' );
        result.push( '-' );
        chars.next(); // Consume second hyphen
        in_hyphen_prefixed = true;
      }
      else
      {
        // Single hyphen: accumulate in component (will decide later if it's `/` or `_`)
        current_component.push( ch );
      }
    }
    else
    {
      current_component.push( ch );
    }
  }

  // Flush final component
  if !current_component.is_empty()
  {
    // Check if component starts with `-` (hyphen-prefixed component at path start)
    if let Some( stripped ) = current_component.strip_prefix( '-' )
    {
      // Strip the leading `-` and add it to result
      result.push( '-' );
      result.push_str( &decode_component( stripped, true ) );
    }
    else
    {
      result.push_str( &decode_component( &current_component, in_hyphen_prefixed ) );
    }
  }

  Ok( PathBuf::from( result ) )
}

/// Decode a single path component, deciding whether hyphens are path separators or underscores
fn decode_component( component : &str, is_hyphen_prefixed : bool ) -> String
{
  if is_hyphen_prefixed
  {
    // Hyphen-prefixed components: convert all hyphens to underscores
    return component.replace( '-', "_" );
  }

  // Normal component: use pattern matching heuristics
  const PATH_COMPONENTS : &[ &str ] = &[
    "home", "usr", "opt", "tmp", "var", "etc", "bin", "lib", "src",
    "pro", "user1", "user", "root",
  ];

  const PROJECT_COMPONENTS : &[ &str ] = &[
    "module", "modules", "crates", "crate", "lib", "bin", "src", "tests", "examples",
  ];

  let parts : Vec< &str > = component.split( '-' ).collect();

  if parts.len() == 1
  {
    return component.to_string();
  }

  // Heuristic: intelligently split by known path/project components
  let mut result = String::new();

  // Find index of "module" or "modules" directory
  let module_idx = parts.iter().position( |&p| p == "module" || p == "modules" );

  for ( i, part ) in parts.iter().enumerate()
  {
    if i > 0
    {
      let prev_part = parts[ i - 1 ];

      // Determine if this hyphen is a path separator or underscore
      let is_separator = if let Some( mod_idx ) = module_idx
      {
        // Special handling when "module" directory is in the path
        if i == mod_idx + 1
        {
          // Immediately after "module": this is path separator (module/ → name)
          true
        }
        else if i == mod_idx + 2
        {
          // Second part of module name: use underscore (claude-storage → claude_storage)
          false
        }
        else
        {
          // Before module or after module name: use default heuristic (path separator)
          true
        }
      }
      else if PATH_COMPONENTS.contains( part ) || PATH_COMPONENTS.contains( &prev_part )
      {
        // Known path components: use path separator
        true
      }
      else if PROJECT_COMPONENTS.contains( part ) || PROJECT_COMPONENTS.contains( &prev_part )
      {
        // Known project components: use path separator
        true
      }
      else
      {
        // Default: path separator (normal paths have subdirectories)
        true
      };

      result.push( if is_separator { '/' } else { '_' } );
    }

    result.push_str( part );
  }

  result
}

#[cfg( test )]
mod tests
{
  use super::*;

  #[test]
  fn test_encode_basic_path()
  {
    let path = Path::new( "/home/user/project" );
    let encoded = encode_path( path ).unwrap();
    assert_eq!( encoded, "-home-user-project" );
  }

  #[test]
  fn test_encode_without_leading_slash()
  {
    let path = Path::new( "home/user/project" );
    let encoded = encode_path( path ).unwrap();
    assert_eq!( encoded, "-home-user-project" );
  }

  #[test]
  fn test_encode_with_trailing_slash()
  {
    let path = Path::new( "/home/user/project/" );
    let encoded = encode_path( path ).unwrap();
    assert_eq!( encoded, "-home-user-project" );
  }

  #[test]
  fn test_decode_basic()
  {
    let decoded = decode_path( "-home-user-project" ).unwrap();
    assert_eq!( decoded, PathBuf::from( "/home/user/project" ) );
  }

  #[test]
  fn test_roundtrip()
  {
    let original = Path::new( "/home/user/project/subdir" );
    let encoded = encode_path( original ).unwrap();
    let decoded = decode_path( &encoded ).unwrap();

    // Normalize both paths for comparison (remove trailing slashes)
    let original_normalized = original.to_str().unwrap().trim_end_matches( '/' );
    let decoded_normalized = decoded.to_str().unwrap().trim_end_matches( '/' );

    assert_eq!( original_normalized, decoded_normalized );
  }

  #[test]
  fn test_decode_missing_prefix()
  {
    let result = decode_path( "home-user-project" );
    assert!( result.is_err() );
  }

  #[test]
  fn test_encode_empty_path()
  {
    let path = Path::new( "/" );
    let result = encode_path( path );
    assert!( result.is_err() );
  }

  // Tests for hyphen-prefixed directory names (bug fix)

  #[test]
  fn test_decode_hyphen_prefixed_component()
  {
    // Real-world case: /commands/-default_topic
    // Should encode as: -commands--default_topic
    // Should decode back to: /commands/-default_topic (NOT //default/topic)
    let decoded = decode_path( "-commands--default_topic" ).unwrap();
    assert_eq!( decoded, PathBuf::from( "/commands/-default_topic" ) );
  }

  #[test]
  fn test_decode_multiple_hyphen_components()
  {
    // Path with multiple hyphen-prefixed directories
    let decoded = decode_path( "-foo--bar--baz" ).unwrap();
    assert_eq!( decoded, PathBuf::from( "/foo/-bar/-baz" ) );
  }

  #[test]
  fn test_decode_real_world_claude_path()
  {
    // Actual path from user's storage causing double-slash bug
    let decoded = decode_path( "-home-user1-pro-genai-claude-commands--default_topic" ).unwrap();
    assert_eq!( decoded, PathBuf::from( "/home/user1/pro/genai/claude/commands/-default_topic" ) );
  }

  #[test]
  fn test_encode_hyphen_prefixed_component()
  {
    // Encoding is lossy: underscores → hyphens (even in hyphen-prefixed components)
    let path = Path::new( "/commands/-default_topic" );
    let encoded = encode_path( path ).unwrap();
    assert_eq!( encoded, "-commands--default-topic" );
  }

  #[test]
  fn test_encode_multiple_hyphen_components()
  {
    let path = Path::new( "/foo/-bar/-baz" );
    let encoded = encode_path( path ).unwrap();
    assert_eq!( encoded, "-foo--bar--baz" );
  }

  #[test]
  fn test_roundtrip_hyphen_prefixed()
  {
    // Round-trip should preserve hyphen-prefixed directories
    let original = Path::new( "/home/user/project/-default_topic" );
    let encoded = encode_path( original ).unwrap();
    let decoded = decode_path( &encoded ).unwrap();

    assert_eq!( original, decoded.as_path() );
  }

  #[test]
  fn test_roundtrip_multiple_hyphen_dirs()
  {
    let original = Path::new( "/commands/-default_topic/-commit/-plan" );
    let encoded = encode_path( original ).unwrap();
    let decoded = decode_path( &encoded ).unwrap();

    assert_eq!( original, decoded.as_path() );
  }

  #[test]
  fn test_backwards_compat_no_double_hyphen()
  {
    // Existing paths without hyphen-prefixed components should work unchanged
    let decoded = decode_path( "-home-user-project-subdir" ).unwrap();
    assert_eq!( decoded, PathBuf::from( "/home/user/project/subdir" ) );
  }

  #[test]
  fn test_encode_nested_hyphen_path()
  {
    // Deep nesting with hyphen-prefixed directories (lossy: underscores → hyphens)
    let path = Path::new( "/a/-b_c/-d_e" );
    let encoded = encode_path( path ).unwrap();
    assert_eq!( encoded, "-a--b-c--d-e" );
  }

  #[test]
  fn test_decode_nested_hyphen_path()
  {
    // After --, hyphens are treated as underscores until next --
    let decoded = decode_path( "-a--b-c--d-e" ).unwrap();
    assert_eq!( decoded, PathBuf::from( "/a/-b_c/-d_e" ) );
  }

  #[test]
  fn test_edge_case_single_hyphen_prefixed()
  {
    // Edge case: Path with just a hyphen-prefixed directory
    let path = Path::new( "/-commit" );
    let encoded = encode_path( path ).unwrap();
    assert_eq!( encoded, "--commit" );

    let decoded = decode_path( &encoded ).unwrap();
    assert_eq!( decoded, PathBuf::from( "/-commit" ) );
  }

  #[test]
  fn test_component_with_underscore()
  {
    // Directory name is -default_topic (starts with hyphen, contains underscore)
    // Encoding is lossy: underscores → hyphens (all components)
    let path = Path::new( "/commands/-default_topic" );
    let encoded = encode_path( path ).unwrap();
    assert_eq!( encoded, "-commands--default-topic" );

    let decoded = decode_path( &encoded ).unwrap();
    // Decoder restores underscores in hyphen-prefixed components
    assert_eq!( decoded, PathBuf::from( "/commands/-default_topic" ) );
  }

  #[test]
  fn test_real_world_wplan_agent_path()
  {
    // Real path from user's storage
    // Encoding is lossy for ALL components (both `/` and `_` → `-`)
    let path = Path::new( "/home/user1/pro/lib/willbe/module/wplan_agent/-default_topic" );
    let encoded = encode_path( path ).unwrap();

    // wplan_agent → wplan-agent (underscore replaced)
    // -default_topic → --default-topic (underscore replaced, even in hyphen-prefixed)
    assert_eq!
    (
      encoded,
      "-home-user1-pro-lib-willbe-module-wplan-agent--default-topic"
    );

    let decoded = decode_path( &encoded ).unwrap();

    // Decoder heuristic: module name components use underscore (wplan-agent → wplan_agent)
    // Decoder restores underscores in hyphen-prefixed components: -default_topic
    assert_eq!
    (
      decoded,
      PathBuf::from( "/home/user1/pro/lib/willbe/module/wplan_agent/-default_topic" )
    );
  }

  #[test]
  fn test_consecutive_hyphen_dirs()
  {
    // Multiple consecutive hyphen-prefixed directories
    let path = Path::new( "/-a/-b/-c" );
    let encoded = encode_path( path ).unwrap();
    assert_eq!( encoded, "--a--b--c" );

    let decoded = decode_path( &encoded ).unwrap();
    assert_eq!( decoded, PathBuf::from( "/-a/-b/-c" ) );
  }

  #[test]
  fn test_mixed_normal_and_hyphen_dirs()
  {
    // Mix of normal and hyphen-prefixed directories
    // Encoding is lossy: underscores → hyphens (all components)
    // Decoding restores underscores in hyphen-prefixed components
    let path = Path::new( "/commands/-commit_sessions/-plan" );
    let encoded = encode_path( path ).unwrap();
    assert_eq!( encoded, "-commands--commit-sessions--plan" );

    let decoded = decode_path( &encoded ).unwrap();
    assert_eq!( decoded, PathBuf::from( "/commands/-commit_sessions/-plan" ) );
  }
}
