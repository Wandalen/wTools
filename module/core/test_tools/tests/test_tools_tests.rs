// #![ deny( rust_2018_idioms ) ]
// #![ deny( missing_debug_implementations ) ]
// #![ deny( missing_docs ) ]

#[ allow( unused_imports ) ]
use test_tools as TheModule;
#[ allow( unused_imports ) ]
#[ cfg( feature = "enabled" ) ]
use test_tools::exposed::*;

mod inc;
