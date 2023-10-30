// #![ cfg_attr( all(), feature( module_is_terminal ) ) ]
// #![ feature( trace_macros ) ]
// #![ feature( type_name_of_val ) ]

///  A struct for testing purpose.
#[ derive( Debug, PartialEq ) ]
pub struct CrateStructForTesting1
{
}

#[ allow( unused_imports ) ]
use ::mod_interface as TheModule;
#[ allow( unused_imports ) ]
use test_tools::exposed::*;
#[ path="../../../../module/step/meta/src/module/terminal.rs" ]
mod terminal;

mod inc;
