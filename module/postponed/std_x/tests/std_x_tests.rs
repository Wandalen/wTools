// #![ deny( rust_2018_idioms ) ]
// #![ deny( missing_debug_implementations ) ]
// #![ deny( missing_docs ) ]

// #![ allow( non_snake_case ) ]
#![ cfg_attr( feature = "nightly", feature( type_name_of_val ) ) ]

#[ allow( unused_imports ) ]
use std_x as the_module;
#[ allow( unused_imports ) ]
use test_tools::exposed::*;

///  A struct for testing purpose.
#[ derive( Debug, PartialEq ) ]
pub struct CrateStructForTesting1
{
}

// #[ path = "./mod.rs" ]
// mod tests;

#[ path="../../../../module/core/wtools/tests/wtools_tests.rs" ]
mod wtools;
