#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

// #![ feature( trace_macros ) ]
// #![ feature( type_name_of_val ) ]

include!( "../_conditional/local_module.rs" );

#[ allow( unused_imports ) ]
use former_runtime as TheModule;
#[ allow( unused_imports ) ]
use meta_tools::prelude::*;
use test_tools::*;

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
mod string_slice_runtime;
