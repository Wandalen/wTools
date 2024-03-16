
include!( "../../../../module/step/meta/src/module/terminal.rs" );

#[ allow( unused_imports ) ]
use process_tools as the_module;
#[ allow( unused_imports ) ]
use test_tools::exposed::*;

pub const ASSET_PATH : &str = "tests/asset";

#[ cfg( feature = "enabled" ) ]
mod inc;
