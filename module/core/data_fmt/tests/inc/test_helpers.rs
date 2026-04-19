//! Common test data generators
//!
//! Reusable test data construction functions to eliminate code duplication across test files.
//!
//! ## Functions
//!
//! - `sample_data()` - Create standard 3x3 table for testing


use data_fmt::{ RowBuilder, TreeNode };

/// Create sample table data for testing
///
/// Returns a 3-column, 2-row table with headers:
/// - NAME, AGE, CITY
/// - Alice, 30, NYC
/// - Bob, 25, LA
///
/// Used across table formatting tests for consistency.
#[ allow( dead_code ) ]
pub fn sample_data() -> TreeNode< String >
{
  RowBuilder::new( vec![ "NAME".into(), "AGE".into(), "CITY".into() ] )
    .add_row( vec![ "Alice".into(), "30".into(), "NYC".into() ] )
    .add_row( vec![ "Bob".into(), "25".into(), "LA".into() ] )
    .build()
}
