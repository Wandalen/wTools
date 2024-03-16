
include!( "../../../../module/step/meta/src/module/terminal.rs" );

#[ allow( unused_imports ) ]
use {{template_blank}} as TheModule;
#[ allow( unused_imports ) ]
use test_tools::exposed::*;

#[ cfg( feature = "enabled" ) ]
mod inc;
