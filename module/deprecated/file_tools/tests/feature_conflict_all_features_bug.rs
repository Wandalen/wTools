//! Test for `--all-features` flag breaking `TempDir` availability (issue-all-features-tempdir-unavailable)
//!
//! ## Root Cause
//!
//! The `file_tools` crate defines both `enabled` and `no_std` features in `Cargo.toml`. `TempDir` requires
//! `#[cfg(all(feature = "enabled", not(feature = "no_std")))]` to be available. When cargo is invoked
//! with `--all-features`, both features activate simultaneously, making the condition evaluate to false
//! and causing `TempDir` to be unavailable. This affects documentation generation, testing, and user code
//! that relies on `--all-features` for comprehensive feature coverage.
//!
//! Technical mechanism: Cargo's `--all-features` activates ALL features defined in `[features]` section
//! without regard for logical conflicts. Since `enabled` and `no_std` are mutually exclusive by design
//! (`TempDir` requires std), both activating breaks the availability condition.
//!
//! Specific failure: `cargo test --doc --all-features` and `cargo run --example X --all-features` both
//! fail to provide `TempDir` because `not(feature = "no_std")` becomes false.
//!
//! ## Why Not Caught
//!
//! Existing tests use explicit feature flags (`--features full`, `--features enabled`) rather than
//! `--all-features`. The workspace test infrastructure (`w3 .test l::3`) runs tests with `--all-features`
//! but `TempDir` tests are feature-gated and silently skip rather than fail loudly. CI/CD documentation
//! generation uses `features = ["full"]` in `[package.metadata.docs.rs]`, avoiding the issue.
//!
//! Test gap: No test validates that `TempDir` remains available when `--all-features` is used, despite
//! this being a common development workflow (`cargo test --all-features`, `cargo doc --all-features`).
//!
//! ## Fix Applied
//!
//! **Fix(issue-all-features-tempdir-unavailable)**: Modified `src/fs/lib.rs:1` crate-level attribute
//!
//! Solution: Modified the `#![cfg_attr(feature = "no_std", no_std)]` attribute to become
//! `#![cfg_attr(all(feature = "no_std", not(feature = "enabled")), no_std)]`. This prioritizes
//! std mode when both features are active, making `--all-features` work correctly.
//!
//! Implementation: When `--all-features` activates both `enabled` and `no_std`, the crate now
//! stays in std mode (not `no_std`), allowing `TempDir` and other std-dependent items to be available.
//! The `no_std` attribute only applies when `no_std` is enabled WITHOUT `enabled`.
//!
//! Verification: `cargo build --all-features` now succeeds with `TempDir` available. `cargo build --features full`
//! continues to work correctly. `cargo build --features no_std` (without enabled) activates `no_std` mode.
//!
//! ## Prevention
//!
//! 1. Add CI test that specifically uses `--all-features` and validates `TempDir` availability
//! 2. Document in `readme.md` that `--all-features` has limitations due to `no_std` conflict
//! 3. Consider using `#[cfg(accessible(...))]` when stabilized for compile-time feature validation
//! 4. Add `cargo-deny` configuration to detect conflicting features in dependencies
//! 5. Use workspace `default-features = false` to force explicit feature selection
//!
//! ## Pitfall
//!
//! Related edge cases:
//! - Any feature-gated API with mutually exclusive features faces similar risk with `--all-features`
//! - Documentation examples marked as `rust,ignore` or `no_run` hide compilation failures
//! - CI may pass while developer workflows with `--all-features` fail
//! - `docs.rs` uses `[package.metadata.docs.rs]` features, potentially diverging from `--all-features` behavior
//! - Feature gates on re-exports (`glob` module) compound the visibility issue
//!
//! Warning signs: Examples compile with `--features X` but fail with `--all-features`, `cfg`-gated
//! items disappearing in generated documentation, silent test skips vs loud test failures.
//!
//! Similar patterns exist in: path module (same cfg), `glob` re-export (`feature = "glob"`), any future
//! std-dependent utilities added to crate. Review all `#[cfg(all(feature = "enabled", not(feature = "no_std")))]`
//! attributes when adding features.

#[ cfg( all( feature = "enabled", not( feature = "no_std" ) ) ) ]
#[ test ]
fn tempdir_available_with_enabled_feature()
{
  use file_tools::TempDir;

  // This test passes when using --features enabled or --features full
  let temp = TempDir::new();
  assert!( temp.base_path.as_os_str().is_empty() );
}

// test_kind: bug_reproducer(issue-all-features-tempdir-unavailable)
#[ cfg( all( feature = "enabled", feature = "no_std" ) ) ]
#[ test ]
fn tempdir_available_with_all_features()
{
  // This test runs when --all-features is used (both enabled and no_std are active)
  // The fix ensures TempDir IS available even with both features active
  use file_tools::TempDir;

  // Verify TempDir is accessible
  let temp = TempDir::new();
  assert!( temp.base_path.as_os_str().is_empty() );

  // Confirm both features are active as expected with --all-features
  #[ allow( clippy::assertions_on_constants ) ]
  const
  {
    assert!(
      cfg!( feature = "enabled" ) && cfg!( feature = "no_std" ),
      "Test should run when both enabled and no_std are active"
    );
  }
}

// Additional validation: glob should still be available with --all-features
#[ cfg( feature = "glob" ) ]
#[ test ]
fn glob_available_with_all_features()
{
  // glob doesn't have the no_std conflict, so it should work
  use file_tools::glob::Pattern;

  let pattern = Pattern::new( "*.rs" ).expect( "valid pattern" );
  assert!( pattern.matches( "test.rs" ) );
}
