
#[ allow( unused_imports ) ]
use wtest as TheModule;
#[ allow( unused_imports ) ]
use test_tools::exposed::*;

#[ cfg( feature = "use_std" ) ]
mod inc;

#[ cfg( feature = "use_std" ) ]
mod wtest_utility;
