//! Common test data generators
//!
//! Reusable test data construction functions to eliminate code duplication across test files.
//!
//! ## Functions
//!
//! - `sample_data()` - Create standard 3x3 table for testing
//!
//! ## Note on `dead_code` suppression
//!
//! Each integration test binary independently compiles `mod inc`, making helpers that are
//! not called in that specific binary appear as dead code. This is a Rust compilation
//! artifact of the integration test model — the functions ARE used across the test suite.
//! Suppression is permanent for shared test infrastructure; remove if test layout changes
//! to per-file helper inclusion.

#![ allow( dead_code ) ]

use data_fmt::{ RowBuilder, TreeNode };

/// Create sample table data for testing
///
/// Returns a 3-column, 2-row table with headers:
/// - NAME, AGE, CITY
/// - Alice, 30, NYC
/// - Bob, 25, LA
///
/// Used across table formatting tests for consistency.
pub fn sample_data() -> TreeNode< String >
{
  RowBuilder::new( vec![ "NAME".into(), "AGE".into(), "CITY".into() ] )
    .add_row( vec![ "Alice".into(), "30".into(), "NYC".into() ] )
    .add_row( vec![ "Bob".into(), "25".into(), "LA".into() ] )
    .build()
}
