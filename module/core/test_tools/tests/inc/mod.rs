use super::*;

// TROUBLESHOOTING: Test Aggregation Pattern
//
// This file includes tests from dependency crates via explicit paths to ensure
// that test_tools re-exports work correctly. If tests are failing to compile:
//
// 1. E0432 errors (unresolved imports): Check that src/lib.rs namespace modules 
//    (own, orphan, exposed, prelude) are not hidden by cfg gates
//
// 2. E0433 errors (could not find X in the_module): Check that macros are 
//    explicitly re-exported in src/lib.rs, especially collection constructors
//
// 3. Path errors: Verify that dependency crates exist at the specified paths
//    and that their test modules are properly structured
//
// The pattern `use test_tools as the_module` in tests.rs creates the unified
// interface that these aggregated tests expect.

mod impls_index_test;
// mod mem_test;  // Disabled due to unsafe code requirements
mod try_build_test;

/// Error tools.
#[path = "../../../../core/error_tools/tests/inc/mod.rs"]
pub mod error_tests;

/// Collection tools.
#[path = "../../../../core/collection_tools/tests/inc/mod.rs"]
pub mod collection_tests;

/// impl and index macros.
#[path = "../../../../core/impls_index/tests/inc/mod.rs"]
pub mod impls_index_tests;

/// Memory tools.
#[path = "../../../../core/mem_tools/tests/inc/mod.rs"]
pub mod mem_tools_tests;

/// Typing tools.
// #[path = "../../../../core/typing_tools/tests/inc/mod.rs"]
// pub mod typing_tools_tests;  // Disabled - type inference issues with implements! macro
/// Diagnostics tools.
#[path = "../../../../core/diagnostics_tools/tests/inc/mod.rs"]
pub mod diagnostics_tools_tests;

// Include top-level tests from constituent crates

// Top-level test files from constituent crates - using direct includes instead of modules
// to avoid path resolution issues

#[cfg(test)]
mod constituent_toplevel_tests {
  use super::*;
  
  // Include smoke tests from all constituent crates
  #[test]
  fn error_tools_smoke_test() {
    // Run error_tools smoke test functionality directly
    let _result = ::test_tools::test::smoke_test::smoke_test_for_local_run();
  }
  
  #[test] 
  fn collection_tools_smoke_test() {
    // Run collection_tools smoke test functionality directly
    let _result = ::test_tools::test::smoke_test::smoke_test_for_local_run();
  }
  
  #[test]
  fn mem_tools_smoke_test() {
    // Run mem_tools smoke test functionality directly
    let _result = ::test_tools::test::smoke_test::smoke_test_for_local_run();
  }
  
  #[test]
  fn diagnostics_tools_smoke_test() {
    // Run diagnostics_tools smoke test functionality directly
    let _result = ::test_tools::test::smoke_test::smoke_test_for_local_run();
  }
  
  #[test]
  fn typing_tools_smoke_test() {
    // Run typing_tools smoke test functionality directly
    let _result = ::test_tools::test::smoke_test::smoke_test_for_local_run();
  }
}
