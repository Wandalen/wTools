//! All tests.

#![allow(unused_imports)]

// ================================================================================================
// MODULE IDENTITY ALIAS: the_module
// ================================================================================================
// 
// This test module uses the `the_module` alias pattern for test aggregation compatibility.
// 
// ## Module Identity :
// - **Individual Testing** : `the_module` = `error_tools` (this crate)  
// - **Aggregated Testing** : `the_module` = `test_tools` (when included via path in test_tools)
// 
// ## Purpose :
// This allows the same test source code to work in both contexts :
// 1. When running tests directly from error_tools directory (17+ tests)
// 2. When running aggregated tests from test_tools directory (175+ tests via aggregation)
// 
// The alias ensures tests reference the correct implementation in each context.
//
// ================================================================================================

use error_tools as the_module;
// use test_tools ::exposed :: *;

mod inc;

