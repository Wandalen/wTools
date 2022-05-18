
// use meta_tools::*;
// use std::env;

// #[test]
// fn trybuild_tests()
// {
//   println!( "current_dir : {:?}", env::current_dir().unwrap() );
//   // let t = trybuild::TestCases::new();
//   // t.pass( "rust/test/former/test/basic_runtime.rs" );
// }

// include!( "./all/runtime.rs" );

// --

// mod basic_runtime
// {
//   #[cfg( not( feature = "in_wtools" ) )]
//   use meta_tools::*;
//
//   #[cfg( feature = "in_wtools" )]
//   use wtools::*;
//   #[cfg( not( feature = "in_wtools" ) )]
//   mod former
//   {
//     pub use former_runtime as runtime;
//   }
//
//   include!( "./all/basic_runtime_common.rs" );
// }
//
// mod string_slice_runtime { include!( "./all/string_slice_runtime.rs" ); }

// --

include!( "../_conditional/local_module.rs" );

use former_runtime as TheModule;

#[ allow( unused_imports ) ]
use TheModule::prelude::*;
#[ allow( unused_imports ) ]
use TheModule::*;
#[ allow( unused_imports ) ]
use meta_tools::prelude::*;

mod former
{
  pub mod runtime
  {
    pub use former_runtime::*;
  }
}

#[ path = "./all/basic_runtime_common.rs" ]
mod basic_runtime_common;

#[ path = "./all/string_slice_runtime.rs" ]
mod  string_slice_runtime;

// mod basic_runtime
// {
//   #[cfg( not( feature = "in_wtools" ) )]
//   use meta_tools::*;
//
//   #[cfg( feature = "in_wtools" )]
//   use wtools::*;
//   #[cfg( not( feature = "in_wtools" ) )]
//   mod former
//   {
//     pub use former_runtime as runtime;
//   }
//
//   include!( "./all/basic_runtime_common.rs" );
// }
//
// mod string_slice_runtime { include!( "./all/string_slice_runtime.rs" ); }
