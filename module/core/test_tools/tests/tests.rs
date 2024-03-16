// #![ deny( rust_2018_idioms ) ]
// #![ deny( missing_debug_implementations ) ]
// #![ deny( missing_docs ) ]

#[ allow( unused_imports ) ]
use test_tools as the_module;
#[ allow( unused_imports ) ]
#[ cfg( feature = "enabled" ) ]
#[ cfg( not( feature = "no_std" ) ) ]
use test_tools::exposed::*;

mod inc;
