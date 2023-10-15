
// #![ feature( trace_macros ) ]
// #![ feature( type_name_of_val ) ]

///  A struct for testing purpose.
#[ derive( Debug, PartialEq ) ]
pub struct CrateStructForTesting1
{
}

#[ allow( unused_imports ) ]
use ::mod_interface as TheModule;

include!( "../_conditional/local_module.rs" );

mod mod_interface;

#[ path = "./mod_interface/trybuild_test.rs" ]
mod trybuild_test;
