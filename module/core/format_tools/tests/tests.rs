//! Primary tests.

// #![ feature( trace_macros ) ]
#![ allow( unused_imports ) ]
#![ allow( clippy::unreadable_literal ) ]
#![ allow( clippy::needless_raw_string_hashes ) ]
#![ allow( clippy::default_trait_access ) ]
#![ allow( clippy::uninlined_format_args ) ]
#![ allow( clippy::ref_option ) ]
#![ allow( clippy::useless_conversion ) ]
#![ allow( clippy::owned_cow ) ]
#![ allow( clippy::type_complexity ) ]
#![ allow( clippy::elidable_lifetime_names ) ]
#![ allow( clippy::redundant_closure ) ]
#![ allow( clippy::println_empty_string ) ]
#![ allow( clippy::field_reassign_with_default ) ]
#![ allow( clippy::never_loop ) ]

use format_tools as the_module;
use test_tools::exposed::*;

#[ cfg( feature = "enabled" ) ]
mod inc;
