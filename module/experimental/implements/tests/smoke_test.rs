//! Smoke testing of the package.
//!
//! Smoke tests are disabled to prevent a circular dependency:
//! `implements` → `test_tools` → `typing_tools` → `implements`.
//! Since `implements` has zero production dependencies, it cannot
//! depend on `test_tools` without creating a cycle. Tests for
//! the package are in `tests/inc/test_cases.rs` instead.

// #[ test ]
// fn local_smoke_test()
// {
//   let _ = ::test_tools ::test ::smoke_test ::smoke_test_for_local_run();
// }
//
// #[ test ]
// fn published_smoke_test()
// {
//   let _ = ::test_tools ::test ::smoke_test ::smoke_test_for_published_run();
// }
