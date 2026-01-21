#![allow(missing_docs)]

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

#[ ignore = "smoke test for published version" ]
#[ test ]
fn published_smoke_test()
{
  println!("Published smoke test - manual verification required");
}
