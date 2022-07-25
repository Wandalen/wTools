#![ allow( unused_imports ) ]

#![ cfg_attr( feature = "nightly", feature( type_name_of_val ) ) ]

use wtools as TheModule;
use test_tools::exposed::*;

///  A struct for testing purpose.
#[ derive( Debug, PartialEq ) ]
pub struct CrateStructForTesting1
{
}

#[ path = "./mod.rs" ]
mod tests;
