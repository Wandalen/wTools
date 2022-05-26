
include!( "../_conditional/local_module.rs" );

use former_runtime as TheModule;
use test_tools::*;

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
