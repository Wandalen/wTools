//!
//! Bug reproduction and documentation for `tool::http::download`'s malformed request URL.
//!
//! # Root Cause
//!
//! `download` built its request URL via
//! `write!( &mut buf, "https: //static.crates.io/crates/{name}/{name}-{version}.crate" )` —
//! a stray space between `https:` and `//`. Every call failed with `ureq::Error` ("http:
//! invalid uri character") before any request reached the network.
//!
//! # Why Not Caught
//!
//! `download` has zero callers anywhere in willbe's own source and had zero test coverage,
//! so the malformed URL was never exercised. It surfaced only once this test was written to
//! close that coverage gap.
//!
//! # Fix Applied
//!
//! Removed the stray space, matching the already-correct sibling implementation in
//! `crates_tools::CrateArchive::download_crates_io` (`"https://static.crates.io/..."`).
//!
//! # Prevention
//!
//! Any helper function with no production callers is untested by construction — add a
//! direct test (like this one) as soon as such a function is introduced, rather than
//! deferring coverage until a caller appears.
//!
//! # Pitfall
//!
//! A single stray space inside a URL string literal compiles cleanly and is easy to miss
//! in review; it only manifests as a runtime URI-parsing error, and only if the code path
//! is ever actually executed.

use super :: *;
use the_module ::tool ::http ::download;

// test_kind: bug_reproducer(issue-download-url-malformed-space)

#[ test ]
fn download_existing_crate()
{
  // Act
  let bytes = download( "test_experimental_c", "0.1.0" ).unwrap();
  let archive = crates_tools ::CrateArchive ::decode( bytes ).unwrap();

  // Assert
  let mut expected_files: Vec< &std ::path ::Path > =
  vec!
  [
    "test_experimental_c-0.1.0/.cargo_vcs_info.json".as_ref(),
    "test_experimental_c-0.1.0/src/lib.rs".as_ref(),
    "test_experimental_c-0.1.0/Cargo.toml".as_ref(),
    "test_experimental_c-0.1.0/Cargo.toml.orig".as_ref(),
  ];
  expected_files.sort();

  let mut actual_files = archive.list();
  actual_files.sort();

  assert_eq!( expected_files, actual_files );
}
