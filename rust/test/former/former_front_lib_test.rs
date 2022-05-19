
include!( "../_conditional/local_module.rs" );

use former as TheModule;

#[ allow( unused_imports ) ]
use TheModule::prelude::*;
#[ allow( unused_imports ) ]
use TheModule::*;
#[ allow( unused_imports ) ]
use meta_tools::prelude::*;

#[ path = "./common_front_test.rs" ]
mod  common_front_test;
