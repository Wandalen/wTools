#![ deny( rust_2018_idioms ) ]
// #![ deny( missing_debug_implementations ) ]
// #![ deny( missing_docs ) ]

//! Trybuild tests.

#[ allow( unused_imports ) ]
use mod_interface as the_module;
#[ allow( unused_imports ) ]
use test_tools::exposed::*;

/// Test module.
#[ path = "mod.rs" ]
pub mod test;

fn main()
{
}
