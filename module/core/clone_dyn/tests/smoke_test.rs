//! Smoke testing of the package.
//!
//! ## Bug Fix Documentation
//!
//! ### Root Cause
//!
//! `test_tools` smoke test functions conditionally skip execution when `WITH_SMOKE`
//! environment variable is not set, returning Ok(()) without validation. This created
//! silent test passes where tests reported success (exit code 0) while actually
//! performing zero validation. Default test invocation via `w3 .test l::3` did not
//! set `WITH_SMOKE`, causing all smoke tests to skip silently.
//!
//! ### Why Not Caught Earlier
//!
//! 1. Test framework design pattern allows conditional skipping as optimization
//! 2. Exit code 0 with "test ... ok" output appears successful in CI/CD
//! 3. No enforcement mechanism for mandatory test execution
//! 4. Performance optimization (skip unless explicitly requested) prioritized over
//!    fail-safe validation
//! 5. Execution time difference (~0.01s skipped vs 13-26s executed) not monitored
//!
//! ### Fix Applied
//!
//! Each smoke test function now explicitly sets `WITH_SMOKE=1` via `std::env::set_var`
//! before calling `test_tools` smoke test functions. This ensures tests always execute
//! actual validation logic (creating temp projects, compiling, running cargo test)
//! instead of silently skipping. Fix applied at test invocation site rather than
//! modifying shared `test_tools` dependency to maintain crate-local control.
//!
//! ### Prevention
//!
//! 1. Monitor test execution duration - smoke tests should take 10+ seconds each
//! 2. Verify "0 skipped" in test output summaries
//! 3. Reject test frameworks that allow silent success without validation
//! 4. Enforce fail-fast pattern: if preconditions not met, fail loudly, never skip
//! 5. Code review checklist: verify conditional test execution always fails when
//!    requirements not satisfied
//!
//! ### Pitfall
//!
//! Conditional test skipping with Ok(()) return creates false confidence. Tests
//! report success while performing zero validation. Always prefer loud failures over
//! silent skips. If test cannot run due to missing preconditions, it must fail with
//! clear error message, never return Ok(()). Test execution time is a critical signal:
//! smoke tests completing instantly indicate skipping, not validation.

#[ test ]
fn local_smoke_test()
{
  // Fix: Ensure smoke tests always run (never skip silently)
  // Root cause: test_tools conditionally skips when WITH_SMOKE unset, returns Ok() without validation
  // Pitfall: Silent skip with Ok() creates false confidence - tests report success while doing nothing
  std::env::set_var( "WITH_SMOKE", "1" );
  ::test_tools ::test ::smoke_test ::smoke_test_for_local_run().unwrap();
}

#[ test ]
fn published_smoke_test()
{
  // Fix: Ensure smoke tests always run (never skip silently)
  // Root cause: test_tools conditionally skips when WITH_SMOKE unset, returns Ok() without validation
  // Pitfall: Silent skip with Ok() creates false confidence - tests report success while doing nothing
  std::env::set_var( "WITH_SMOKE", "1" );
  ::test_tools ::test ::smoke_test ::smoke_test_for_published_run().unwrap();
}
