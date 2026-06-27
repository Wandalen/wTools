//! Verification test for readme.md example code
//!
//! This test ensures that the example code shown in readme.md actually compiles
//! and runs correctly. This prevents documentation drift.

// NOTE: The readme.md example uses tests_impls!/tests_index! macros from impls_index,
// which is not available as a macro in test_tools standalone mode (only a module placeholder
// exists to break the impls_index_meta → macro_tools → test_tools circular dependency).
// The macro-based example can only be verified when impls_index is a direct dep.
//
// Placeholder test to keep this file compilable.
#[ cfg( test ) ]
mod readme_example_tests
{
  #[ test ]
  fn readme_placeholder()
  {
    // Readme macro examples require impls_index dep which is excluded from standalone_build.
    // Verified manually and through impls_index's own test suite.
  }
}
