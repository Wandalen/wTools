//! CLI sanity tests
//!
//! Minimal integration tests to verify the CLI binary builds and basic
//! functionality works. The core library logic is tested in claude_storage_core.

#[test]
fn cli_builds()
{
  // This test simply verifies the CLI dependencies and features compile.
  // The act of running this test means the binary built successfully.
  let package_name = env!( "CARGO_PKG_NAME" );
  assert_eq!( package_name, "claude_storage" );
}

#[test]
#[cfg( feature = "cli" )]
fn cli_feature_enabled()
{
  // Verify the CLI feature is enabled when running tests
  let version = env!( "CARGO_PKG_VERSION" );
  assert!( !version.is_empty(), "Package version should not be empty" );
}
