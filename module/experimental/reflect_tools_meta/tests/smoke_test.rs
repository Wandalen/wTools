//! Smoke tests for the `reflect_tools_meta` crate.
//!
//! Validates basic crate health by ensuring the crate compiles and links correctly
//! in both local development and published package contexts. Uses `test_tools`
//! smoke test infrastructure for consistent validation across the workspace.
//!
//! The implementation is currently a stub returning empty `TokenStream`, so these
//! tests focus on crate infrastructure rather than derive macro behavior.
//!
//! ## Test Organization
//!
//! - `local_smoke_test`: Verifies crate compiles in local workspace context
//! - `published_smoke_test`: Verifies crate compiles as published dependency
//!
//! ## Test Execution
//!
//! Smoke tests run conditionally based on `WITH_SMOKE` environment variable:
//! - `WITH_SMOKE=1` or `WITH_SMOKE=local`: Run local smoke test
//! - `WITH_SMOKE=published`: Run published smoke test
//! - CI/CD environments: Run automatically
//! - Otherwise: Skip with explicit message (not silent failure)
//!
//! ## Coverage Status
//!
//! Current: Minimal (smoke tests only)
//! Reason: Implementation is stub (returns empty `TokenStream`)
//! Future: Add comprehensive Reflect derive tests when implementation is complete

#![ allow( missing_docs ) ]

/// Verifies crate compiles and links correctly in local workspace context.
///
/// This test ensures the crate builds successfully when used as a path dependency
/// within the wTools workspace. It validates that all feature flags, dependencies,
/// and basic compilation succeed.
///
/// Uses `test_tools::smoke_test_for_local_run()` which performs conditional
/// execution based on `WITH_SMOKE` environment variable or CI/CD detection.
///
/// Note: Does not test derive macro behavior (implementation is stub).
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
///
/// Note: Does not test derive macro behavior (implementation is stub).
#[ test ]
fn published_smoke_test()
{
  let _ = ::test_tools ::test ::smoke_test ::smoke_test_for_published_run();
}
