#![ allow( unused_imports ) ]

// #![ cfg_attr( feature = "nightly", feature( type_name_of_val ) ) ]

use wtools as the_module;
use test_tools::exposed::*;

///  A struct for testing purpose.
// #[ derive( Debug, PartialEq ) ]
// pub struct CrateStructForTesting1
// {
// }

#[ cfg( feature = "iter_tools" ) ]
#[ path = "../../../core/iter_tools/tests/tests.rs" ]
mod iter_tools;

#[ cfg( feature = "meta_tools" ) ]
#[ path = "../../../core/meta_tools/tests/meta_tools_tests.rs" ]
mod meta_tools;

#[ cfg( feature = "mem_tools" ) ]
#[ path = "../../../core/mem_tools/tests/mem_tools_tests.rs" ]
mod mem_tools;

#[ cfg( feature = "typing_tools" ) ]
#[ path = "../../../core/typing_tools/tests/tests.rs" ]
mod typing_tools;

#[ cfg( feature = "time_tools" ) ]
#[ path = "../../../core/time_tools/tests/time_tests.rs" ]
mod time_tools;

#[ cfg( feature = "strs_tools" ) ]
#[ path = "../../../core/strs_tools/tests/strs_tools_tests.rs" ]
mod strs_tools;

#[ cfg( feature = "error_tools" ) ]
#[ cfg( not( feature = "error_no_std" ) ) ]
#[ path = "../../../core/error_tools/tests/error_tools_tests.rs" ]
mod error_tools;

#[ cfg( feature = "derive_tools" ) ]
#[ path = "../../../core/derive_tools/tests/tests.rs" ]
mod derive_tools;

#[ cfg( feature = "data_type" ) ]
#[ path = "../../../core/data_type/tests/data_type_tests.rs" ]
mod data_type;

#[ cfg( feature = "diagnostics_tools" ) ]
#[ cfg( not( feature = "meta_tools" ) ) ]
#[ path = "../../../core/diagnostics_tools/tests/diagnostics_tests.rs" ]
mod diag_tools;


#[ cfg( feature = "meta_tools" ) ]
pub use meta_tools::*;
