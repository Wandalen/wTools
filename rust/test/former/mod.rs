
use wtools::former as TheModule;
use wtools::former;

#[ allow( unused_imports ) ]
use TheModule::prelude::*;
#[ allow( unused_imports ) ]
use TheModule::*;
#[ allow( unused_imports ) ]
use wtools::meta::prelude::*;

#[ path = "./all/basic_runtime_common.rs" ]
mod  basic_runtime_common;

#[ path = "./common_front_test.rs" ]
mod  common_front_test;
