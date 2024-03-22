// #![ deny( rust_2018_idioms ) ]
// #![ deny( missing_debug_implementations ) ]
// #![ deny( missing_docs ) ]

#[ allow( unused_imports ) ]
use ::meta_tools as the_module;
#[ allow( unused_imports ) ]
use test_tools::exposed::*;
#[ path="../../../../module/step/meta/src/module/aggregating.rs" ]
mod aggregating;

///  A struct for testing purpose.
#[ derive( Debug, PartialEq ) ]
pub struct CrateStructForTesting1
{
}


mod inc;
