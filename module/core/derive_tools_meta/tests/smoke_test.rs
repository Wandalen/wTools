//! Smoke tests for the `derive_tools_meta` crate.
//!
//! Validates basic crate health by ensuring the crate compiles and links correctly
//! in both local development and published package contexts. These tests do not
//! verify specific derive macro behavior - comprehensive derive tests are pending.
//!
//! ## Test Organization
//!
//! - `local_smoke_test`: Verifies crate compiles in local workspace context
//! - `published_smoke_test`: Verifies crate compiles as published dependency
//!
//! ## Coverage Status
//!
//! Current: Minimal (smoke tests only)
//! Needed: Comprehensive derive macro behavior tests (see tests/readme.md)

/// Verifies crate compiles and links correctly in local workspace context.
///
/// This test ensures the crate builds successfully when used as a path dependency
/// within the wTools workspace. It validates that all feature flags, dependencies,
/// and basic functionality work in local development mode.
///
/// Uses `test_tools::smoke_test_for_local_run()` which performs basic compilation
/// and linking checks without exercising specific derive macro functionality.
#[ test ]
fn local_smoke_test()
{
  let _ = ::test_tools ::test ::smoke_test ::smoke_test_for_local_run();
}

/// Verifies crate compiles and links correctly as published crates.io package.
///
/// This test ensures the crate builds successfully when used as a published
/// dependency from crates.io. It validates that workspace dependencies resolve
/// correctly and the crate is usable by external consumers.
///
/// Uses `test_tools::smoke_test_for_published_run()` which simulates the published
/// package context to catch packaging or dependency resolution issues.
#[ test ]
fn published_smoke_test()
{
  let _ = ::test_tools ::test ::smoke_test ::smoke_test_for_published_run();
}
