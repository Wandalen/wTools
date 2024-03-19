#![ deny( rust_2018_idioms ) ]
// #![ deny( missing_debug_implementations ) ]
// #![ deny( missing_docs ) ]

//! Trybuild tests.

#[ allow( unused_imports ) ]
use mod_interface as the_module;
#[ allow( unused_imports ) ]
use test_tools::exposed::*;

///  A struct for testing purpose.
#[ derive( Debug, PartialEq ) ]
pub struct CrateStructForTesting1
{
}

/// Test module.
#[ path = "mod.rs" ]
pub mod test;

fn main()
{
}
