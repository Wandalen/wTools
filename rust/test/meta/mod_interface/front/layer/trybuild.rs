#![ deny( rust_2018_idioms ) ]
#![ deny( missing_debug_implementations ) ]
#![ deny( missing_docs ) ]

//! Trybuild tests.

use mod_interface as TheModule;
use test_tools::exposed::*;

/// Test module.
#[ path = "mod.rs" ]
pub mod test;

fn main()
{
}
