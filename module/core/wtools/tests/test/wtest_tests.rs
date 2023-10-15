
#[ allow( unused_imports ) ]
use wtest as TheModule;
#[ allow( unused_imports ) ]
use test_tools::exposed::*;

#[ cfg( not( feature = "no_std" ) ) ]
mod inc;

#[ cfg( not( feature = "no_std" ) ) ]
mod wtest_utility;
