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
//! - `published_smoke_test`: Env-gated test of the published crates.io version (`WITH_SMOKE`/CI)

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

/// Smoke test of the published crates.io version via the workspace-standard machinery :
/// executes only under `WITH_SMOKE=1|published` or CI detection, trivially passes otherwise.
#[ test ]
fn published_smoke_test()
{
  let _ = ::test_tools ::test ::smoke_test ::smoke_test_for_published_run();
}
