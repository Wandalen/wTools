use super :: *;

// TROUBLESHOOTING: Test Aggregation Pattern
//
// This file includes tests from dependency crates via explicit paths to ensure
// that test_tools re-exports work correctly. If tests are failing to compile :
//
// 1. E0432 errors (unresolved imports) : Check that src/lib.rs namespace modules 
//    (own, orphan, exposed, prelude) are not hidden by cfg gates
//
// 2. E0433 errors (could not find X in the_module) : Check that macros are 
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
#[ path = "../../../../core/error_tools/tests/inc/mod.rs" ]
pub mod error_tests;

/// Collection tools.
#[ path = "../../../../core/collection_tools/tests/inc/mod.rs" ]
pub mod collection_tests;

/// impl and index macros.
/// NOTE: impls_index tests require impls_index crate which is not available in standalone_build mode
/// due to circular dependency: test_tools → impls_index → impls_index_meta → macro_tools → ... → test_tools
#[ cfg( not( feature = "standalone_build" ) ) ]
#[ path = "../../../../core/impls_index/tests/inc/mod.rs" ]
pub mod impls_index_tests;

/// Memory tools.
#[ path = "../../../../experimental/mem_tools/tests/inc/mod.rs" ]
pub mod mem_tools_tests;

/// Typing tools.
// #[ path = "../../../../core/typing_tools/tests/inc/mod.rs" ]
// pub mod typing_tools_tests;  // Disabled - type inference issues with implements! macro
/// Diagnostics tools.
#[ path = "../../../../experimental/diagnostics_tools/tests/inc/mod.rs" ]
pub mod diagnostics_tools_tests;

