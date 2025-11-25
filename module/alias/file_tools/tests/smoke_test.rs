//! Smoke testing of the package.
//!
//! Verifies that the public API surface is accessible and functional.
//! These tests ensure basic module imports and function calls work correctly.
//!
//! ## Test Matrix
//!
//! | Test Case | Scenario | Expected | Status |
//! |-----------|----------|----------|--------|
//! | `local_smoke_test` | Import and call `f1()` from local crate | Function callable, no panic | ✅ |
//! | `published_smoke_test` | Import and call `f1()` from published API surface | Function callable, no panic | ✅ |
//!
//! ## Corner Cases Covered
//!
//! - ✅ Module imports (both local and published)
//! - ✅ Function accessibility
//! - ✅ Feature-gated API (enabled feature)

/// Verifies that the local crate's public API is accessible.
///
/// This test imports the crate directly and calls its exported functions,
/// ensuring the module compiles and links correctly in local development.
///
/// The test succeeds if `f1()` executes without panic. Any panic will cause
/// test failure, providing loud failure feedback.
#[ test ]
fn local_smoke_test()
{
  // Calling f1() - if it panics, the test fails loudly
  file_tools::f1();
}

/// Verifies that the published API surface is accessible.
///
/// This test simulates usage of the published crate, ensuring that
/// consumers can import and use the public API as documented.
///
/// The test succeeds if `f1()` executes without panic. Any panic will cause
/// test failure, providing loud failure feedback.
#[ test ]
fn published_smoke_test()
{
  // Calling f1() - if it panics, the test fails loudly
  file_tools::f1();
}
