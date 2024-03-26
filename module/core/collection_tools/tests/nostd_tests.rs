#![ cfg_attr( feature = "no_std", no_std ) ]
// tests without std

#[ allow( unused_imports ) ]
use ::collection_tools as the_module;
#[ allow( unused_imports ) ]
use test_tools::exposed::*;
#[ path="../../../../module/step/meta/src/module/aggregating.rs" ]
mod aggregating;

mod nostd;
// aaa : enable