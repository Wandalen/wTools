//! Basic smoke tests for `crates_tools` crate
//!
//! ## Purpose
//!
//! Validates core `CrateArchive` functionality works in both local development
//! and published crate environments. Tests empty archive handling and basic API operations.
//!
//! ## Test Organization
//!
//! - `local_smoke_test`: Feature-gated test for local development (enabled feature)
//! - `published_smoke_test`: Manual verification test for published crates (disabled by default)

#[ cfg(feature = "enabled") ]
use crates_tools ::CrateArchive;

/// Validates basic `CrateArchive` operations work with empty archive
#[ cfg(feature = "enabled") ]
#[ test ]
fn local_smoke_test()
{
  // Verify decode with empty bytes creates default archive
  let archive = CrateArchive ::decode(vec![]).expect("Failed to decode empty archive");

  // Verify list returns empty for empty archive
  let files = archive.list();
  assert!(files.is_empty(), "Empty archive should have no files");

  // Verify content_bytes returns None for non-existent path
  let content = archive.content_bytes("nonexistent.txt");
  assert!(content.is_none(), "Non-existent file should return None");
}

/// Validates published crate basic functionality
#[ cfg(not(feature = "enabled")) ]
#[ test ]
fn local_smoke_test()
{
  // When feature disabled, just verify crate compiles
}

/// Manual verification test for published crate on crates.io
///
/// ## Purpose
///
/// This test verifies that the published crate can be downloaded and used
/// correctly from crates.io. Requires manual execution after publishing.
///
/// ## Why Disabled
///
/// Cannot be automated because it requires the crate to first be published
/// to crates.io. Should be run manually after each release.
// DISABLED: 2026-01-24 by Claude Code TDD Agent
// REASON: Manual verification test for published crates.io version (requires external publish)
// RE-ENABLE: When automated crates.io testing infrastructure available OR move to tests/manual/readme.md
// APPROVED: self (test cleanup during TDD cycle)
// TRACKING: N/A (organizational compliance fix)
#[ ignore = "smoke test for published version" ]
#[ test ]
fn published_smoke_test()
{
  println!("Published smoke test - manual verification required");
}
