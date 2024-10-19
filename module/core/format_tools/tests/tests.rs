//! Primary tests.

#![ feature( trace_macros ) ]
#![ allow( unused_imports ) ]

use format_tools as the_module;
use test_tools::exposed::*;

#[ cfg( feature = "enabled" ) ]
mod inc;
