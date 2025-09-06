//! All tests.

// #![ deny( rust_2018_idioms ) ]
// #![ deny( missing_debug_implementations ) ]
// #![ deny( missing_docs ) ]

// #![ feature( trace_macros ) ]
// #![ feature( type_name_of_val ) ]

// ================================================================================================
// MODULE IDENTITY ALIAS: the_module
// ================================================================================================
// 
// This test module uses the `the_module` alias pattern for test aggregation compatibility.
// 
// ## Module Identity:
// - **Individual Testing**: `the_module` = `mem_tools` (this crate)  
// - **Aggregated Testing**: `the_module` = `test_tools` (when included via path in test_tools)
// 
// ## Purpose:
// This allows the same test source code to work in both contexts:
// 1. When running tests directly from mem_tools directory
// 2. When running aggregated tests from test_tools directory  
// 
// The alias ensures tests reference the correct implementation in each context.
//
// ================================================================================================

#[ allow( unused_imports ) ]
use mem_tools as the_module;
mod inc;
