// mod basic_runtime
// {
//   use meta_tools::*;
//   mod former
//   {
//     pub use former_runtime as runtime;
//   }
//   include!( "./all/basic_runtime_common.rs" );
// }
//
// include!( "./common_front_test.rs" );

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
