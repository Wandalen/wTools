// usual tests

#[ path="../../../../module/step/meta/src/module/aggregating.rs" ]
mod aggregating;

#[ allow( unused_imports ) ]
use test_tools::exposed::*;

#[ allow( unused_imports ) ]
use ::collection_tools as the_module;

#[ cfg( feature = "enabled" ) ]
mod inc;
