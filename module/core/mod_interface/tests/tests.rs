
///  A struct for testing purpose.
#[ derive( Debug, PartialEq ) ]
pub struct CrateStructForTesting1
{
}

#[ allow( unused_imports ) ]
use ::mod_interface as the_module;
#[ allow( unused_imports ) ]
use test_tools::exposed::*;
#[ path="../../../../module/step/meta/src/module/terminal.rs" ]
mod terminal;

mod inc;
