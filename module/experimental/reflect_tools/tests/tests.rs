
//! Test suite for `reflect_tools` crate

#[ allow( unused_imports ) ]
use reflect_tools as the_module;
#[ allow( unused_imports ) ]
use test_tools ::exposed :: *;

// Import Entity trait to make its methods available
#[ allow( unused_imports ) ]
use the_module ::reflect ::Entity;

#[ cfg( feature = "enabled" ) ]
#[ allow( unused_variables, missing_docs ) ]
mod inc;

