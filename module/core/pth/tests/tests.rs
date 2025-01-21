#![ allow( unused_imports ) ]

include!( "../../../../module/step/meta/src/module/terminal.rs" );

use pth as the_module;
use test_tools::exposed::*;

#[ cfg( feature = "enabled" ) ]
mod inc;
