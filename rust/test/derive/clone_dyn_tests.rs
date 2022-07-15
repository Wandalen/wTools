
#[ allow( unused_imports ) ]
use clone_dyn as TheModule;
#[ allow( unused_imports ) ]
use test_tools::exposed::*;

#[ cfg( any( feature = "use_std", feature = "use_alloc" ) ) ]
mod clone_dyn_test;
