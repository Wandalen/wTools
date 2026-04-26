//! Smoke testing of the `file_tools` alias crate.
//!
//! Verifies that `fs_tools` public API is correctly re-exported through `file_tools`.
//! Each test uses a distinct API surface to confirm the re-export is complete.
//!
//! ## Test Matrix
//!
//! | Test Case | Scenario | Expected | Status |
//! |-----------|----------|----------|--------|
//! | `local_smoke_test` | Path search API accessible via re-export | Returns Some for known file | ✅ |
//! | `published_smoke_test` | `TempDir` API accessible via re-export | `full_path()` returns non-empty path | ✅ |
//!
//! ## Corner Cases Covered
//!
//! - ✅ Module re-export accessible (`file_tools::path`, `file_tools::fs`)
//! - ✅ Feature-gated availability (enabled feature required)
//! - ✅ Real assertions — not just compilation checks

/// Verifies path traversal API is accessible through the re-export.
///
/// Calls `file_tools::path::file_upward_find` starting from the current
/// directory and searching for "Cargo.toml". The crate's own Cargo.toml
/// is always reachable upward from the test working directory.
#[ test ]
#[ cfg( feature = "enabled" ) ]
fn local_smoke_test()
{
  let result = file_tools::path::file_upward_find
  (
    std::path::Path::new( "." ),
    "Cargo.toml",
    10,
  );
  assert!( result.is_some(), "Smoke test failed: file_upward_find did not find Cargo.toml" );
}

/// Verifies `TempDir` API is accessible through the re-export.
///
/// Constructs a `file_tools::fs::TempDir`, sets its public path fields,
/// and confirms `full_path()` returns the assembled non-empty path.
/// No directory is created on disk.
#[ test ]
#[ cfg( feature = "enabled" ) ]
fn published_smoke_test()
{
  let mut tmp = file_tools::fs::TempDir::new();
  tmp.base_path = std::env::temp_dir();
  tmp.prefix_path = std::path::PathBuf::from( "file_tools_smoke_" );
  assert!(
    !tmp.full_path().as_os_str().is_empty(),
    "Smoke test failed: TempDir produced an empty path",
  );
}
