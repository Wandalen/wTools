//! All tests.

#![allow(unused_imports)]

#[ path = "../../../../module/step/meta/src/module/aggregating.rs" ]
mod aggregating;

// #[ allow( unused_imports ) ]
// use test_tools ::exposed :: *;

// ================================================================================================
// MODULE IDENTITY ALIAS: the_module
// ================================================================================================
// 
// This test module uses the `the_module` alias pattern for test aggregation compatibility.
// 
// ## Module Identity :
// - **Individual Testing** : `the_module` = `collection_tools` (this crate)  
// - **Aggregated Testing** : `the_module` = `test_tools` (when included via path in test_tools)
// 
// ## Purpose :
// This allows the same test source code to work in both contexts :
// 1. When running tests directly from collection_tools directory
// 2. When running aggregated tests from test_tools directory  
// 
// The alias ensures tests reference the correct implementation in each context.
//
// ================================================================================================

#[ allow( unused_imports ) ]
use ::collection_tools as the_module;

#[ cfg( feature = "enabled" ) ]
#[ cfg(any(feature = "use_alloc", not(feature = "no_std"))) ]
mod inc;
