//! All test.
//!
//! # Test Compilation Troubleshooting
//! 
//! This file aggregates tests from multiple dependency crates to ensure re-export consistency.
//! If you're seeing compilation errors here, they typically fall into these patterns:
//!
//! ## Common Issues in Aggregated Tests
//!
//! ### E0432: "unresolved imports `test_tools::tests_impls`" 
//! - **Cause:** API modules hidden by cfg gates in src/lib.rs
//! - **Fix:** Remove `#[cfg(not(feature = "doctest"))]` from namespace modules
//! - **Check:** Verify `own`, `orphan`, `exposed`, `prelude` modules are always visible
//!
//! ### E0433: "could not find `heap` in `the_module`"
//! - **Cause:** Collection constructor macros not re-exported 
//! - **Fix:** Add explicit macro re-exports in src/lib.rs
//! - **Check:** Verify `pub use collection_tools::{heap, vec, ...}` exists with proper cfg gates
//!
//! ### Test Organization
//! - Tests are included via `#[path = "..."]` to access dependency test suites
//! - `use test_tools as the_module;` provides the unified access pattern
//! - Aggregated tests verify that re-exports work correctly from consumer perspective
//!

#![allow(unused_imports)]

// #![ deny( rust_2018_idioms ) ]
// #![ deny( missing_debug_implementations ) ]
// #![ deny( missing_docs ) ]

include!("../../../../module/step/meta/src/module/aggregating.rs");

use test_tools as the_module;

// #[ cfg( feature = "enabled" ) ]
// #[ cfg( not( feature = "no_std" ) ) ]
// use test_tools::exposed::*;

mod inc;
