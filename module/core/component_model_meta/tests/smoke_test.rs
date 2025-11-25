//! Smoke testing of the package.
//!
//! This module contains basic smoke tests that verify the package can be
//! compiled and its basic functionality works in both local development
//! and published crate contexts.

/// Tests basic functionality in local development environment.
///
/// This test verifies that the crate compiles and basic operations work
/// when run from the local workspace. It uses the `test_tools` smoke test
/// framework which checks compilation, basic imports, and core functionality.
///
/// # Failure Mode
///
/// Test fails loudly with `.expect()` if smoke test returns an error,
/// ensuring any compilation or basic functionality issues are caught.
#[ test ]
fn local_smoke_test()
{
  ::test_tools ::test ::smoke_test ::smoke_test_for_local_run()
    .expect( "local smoke test failed" );
}

/// Tests basic functionality as a published crate.
///
/// This test verifies that the crate works correctly when consumed as
/// a published dependency, checking that all public APIs are accessible
/// and function correctly from an external consumer's perspective.
///
/// # Failure Mode
///
/// Test fails loudly with `.expect()` if smoke test returns an error,
/// ensuring any issues with the published crate interface are caught.
#[ test ]
fn published_smoke_test()
{
  ::test_tools ::test ::smoke_test ::smoke_test_for_published_run()
    .expect( "published smoke test failed" );
}
