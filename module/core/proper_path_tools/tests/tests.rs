#![ allow( unused_imports ) ]

include!( "../../../../module/step/meta/src/module/terminal.rs" );

use proper_path_tools as the_module;
use test_tools::exposed::*;

#[ cfg( feature = "enabled" ) ]
mod inc;
