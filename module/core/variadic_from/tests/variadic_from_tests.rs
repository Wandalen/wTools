
#[ allow( unused_imports ) ]
use variadic_from as the_module;
use test_tools::exposed::*;

// #[ path = "inc.rs" ]
#[ cfg( feature = "enabled" ) ]
mod inc;

