#![ warn( rust_2018_idioms ) ]
#![ deny( missing_debug_implementations ) ]
#![ deny( missing_docs ) ]

// #![ feature( trace_macros ) ]
// #![ feature( type_name_of_val ) ]

include!( "../_conditional/terminal_module.rs" );

#[ allow( unused_imports ) ]
use test_tools::exposed::*;
// use test_tools::*;

#[ allow( unused_imports ) ]
use former_runtime as TheModule;
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
mod string_slice_runtime;
