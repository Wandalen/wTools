#![ warn( rust_2018_idioms ) ]
#![ deny( missing_debug_implementations ) ]
#![ deny( missing_docs ) ]

use ::multilayer as TheModule;

// #![ feature( trace_macros ) ]
// #![ feature( type_name_of_val ) ]

include!( "../_conditional/terminal_module.rs" );
#[ allow( unused_imports ) ]
use test_tools::exposed::*;

///  A struct for testing purpose.
#[ derive( Debug, PartialEq ) ]
pub struct CrateStructForTesting1
{
}

mod mod_interface;
