//! Test propagation from core `interval_adapter` crate.
//!
//! This file uses `#[path]` to include tests from the core `interval_adapter` crate,
//! verifying that winterval's re-exports are complete and function correctly.
//!
//! ## Purpose
//!
//! - Validates facade re-exports match core crate API
//! - Ensures `the_module` alias works (tests use `winterval` instead of `interval_adapter`)
//! - Provides regression guard against incomplete re-exports
//!
//! ## Test Organization
//!
//! All actual test implementations are in `interval_adapter/tests/inc/mod.rs`.
//! This file propagates those tests to run against winterval's public API.
//!

#[ allow(unused_imports) ]
use winterval as the_module;
#[ allow(unused_imports) ]
use test_tools::exposed :: *;

#[ path = "../../interval_adapter/tests/inc/mod.rs" ]
mod inc;
