//! Test aggregator for the `for_each` crate.
//!
//! Imports the public API surface under the `the_module` alias and delegates
//! to the functional test modules in `inc/`.

#![ allow( missing_docs ) ]
#![ allow( non_snake_case ) ]
#![ allow( unused_variables ) ]
#![ allow( unused_assignments ) ]
#![ allow( clippy::manual_string_new ) ]
#![ allow( clippy::needless_raw_string_hashes ) ]
#![ allow( clippy::too_many_lines ) ]
#![ allow( clippy::doc_markdown ) ]

use for_each as the_module;
#[ allow( unused_imports ) ]
use test_tools ::exposed :: *;

pub mod inc;
