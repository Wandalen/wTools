#![ allow( unused_imports ) ]

#![ cfg_attr( feature = "nightly", feature( type_name_of_val ) ) ]

use wtools as TheModule;
use test_tools::exposed::*;

///  A struct for testing purpose.
#[ derive( Debug, PartialEq ) ]
pub struct CrateStructForTesting1
{
}

#[ path = "../../../core/iter_tools/tests/inc/mod.rs" ]
mod iter_tools;
#[ path = "../../../core/meta_tools/tests/inc/mod.rs" ]
mod meta_tools;
#[ path = "../../../core/mem_tools/tests/inc/mod.rs" ]
mod mem_tools;
#[ path = "../../../core/typing_tools/tests/inc/mod.rs" ]
mod typing_tools;
#[ path = "../../../core/time_tools/tests/inc/mod.rs" ]
mod time_tools;
#[ path = "../../../core/strs_tools/tests/inc/mod.rs" ]
mod strs_tools;
#[ path = "../../../core/error_tools/tests/inc/mod.rs" ]
mod error_tools;
#[ path = "../../../core/derive_tools/tests/inc/mod.rs" ]
mod derive_tools;
#[ path = "../../../core/data_type/tests/inc/mod.rs" ]
mod data_type;
#[ path = "../../../core/diagnostics_tools/tests/inc/mod.rs" ]
mod diagnostics_tools;
#[ path = "../../../core/diagnostics_tools/tests/inc/mod.rs" ]
mod diagnostics_tools;

// #[ path = "./mod.rs" ]
// mod tests;
