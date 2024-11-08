//! Main tests
#![ allow( unused_imports ) ]

///  A struct for testing purpose.
#[ derive( Debug, PartialEq ) ]
pub struct CrateStructForTesting1
{
}

use ::mod_interface as the_module;
use test_tools::exposed::*;
#[ path="../../../../module/step/meta/src/module/terminal.rs" ]
mod terminal;

mod inc;
