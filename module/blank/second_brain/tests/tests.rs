
include!( "../../../../module/step/meta/src/module/terminal.rs" );

#[ allow( unused_imports ) ]
use second_brain as the_module;
#[ allow( unused_imports ) ]
use test_tools::exposed::*;

#[ cfg( feature = "enabled" ) ]
mod inc;
