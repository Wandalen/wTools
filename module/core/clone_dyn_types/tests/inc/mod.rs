//! Test module for additional integration tests.
//!
//! Note: External tests from clone_dyn facade crate are commented out
//! due to test_tools dependency removal (circular dependency issue).
//! Core functionality is tested directly in tests.rs and smoke_test.rs.

#[ allow( unused_imports ) ]
use super :: *;

// External tests from clone_dyn facade crate - disabled due to test_tools dependency
// #[ path = "../../../clone_dyn/tests/inc" ]
// mod tests
// {
//   #[ allow( unused_imports ) ]
//   use super :: *;
//
//   mod basic_manual;
//   // mod basic;
//   // mod parametrized;
// }
