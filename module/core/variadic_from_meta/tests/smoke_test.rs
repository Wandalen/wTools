//! Smoke tests for the `variadic_from_meta` crate.
//!
//! Validates basic crate health by ensuring the proc macro crate compiles and links
//! correctly in both local development and published package contexts.
//!
//! ## Test Organization
//!
//! - `local_smoke_test`: Verifies crate compiles in local workspace context
//! - `published_smoke_test`: Verifies crate compiles as published dependency
//!
//! ## Coverage Status
//!
//! - **Smoke Tests**: ✅ Implemented (this file)
//! - **Comprehensive Tests**: See `derive_test.rs` for conformance checks per [`docs/api/001_variadic_from_derive.md`](../docs/api/001_variadic_from_derive.md)
//!
//! ## Related Tests
//!
//! The `variadic_from` crate (user-facing) contains integration tests at:
//! - `variadic_from/tests/inc/derive_test.rs` - Full test matrix for derive macro
//! - `variadic_from/tests/compile_fail.rs` - Compile-time error validation

/// Verifies crate compiles and links correctly in local workspace context.
///
/// This test ensures the proc macro crate builds successfully when used as a path
/// dependency within the wTools workspace. It validates that all feature flags,
/// dependencies, and basic functionality work in local development mode.
///
/// Uses `test_tools::smoke_test_for_local_run()` which performs basic compilation
/// and linking checks without exercising specific proc macro functionality.
#[ test ]
fn local_smoke_test()
{
  let _ = ::test_tools ::test ::smoke_test ::smoke_test_for_local_run();
}

/// Verifies crate compiles and links correctly as published crates.io package.
///
/// This test ensures the proc macro crate builds successfully when used as a
/// published dependency from crates.io. It validates that workspace dependencies
/// resolve correctly and the crate is usable by external consumers.
///
/// Uses `test_tools::smoke_test_for_published_run()` which simulates the published
/// package context to catch packaging or dependency resolution issues.
#[ test ]
fn published_smoke_test()
{
  let _ = ::test_tools ::test ::smoke_test ::smoke_test_for_published_run();
}
