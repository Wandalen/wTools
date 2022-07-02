
#[ allow( unused_imports ) ]
use ::mod_interface as TheModule;

// #![ feature( trace_macros ) ]
// #![ feature( type_name_of_val ) ]

include!( "../_conditional/local_module.rs" );

mod mod_interface;

// xxx2
// #[ path = "./mod_interface/trybuild_test.rs" ]
// mod trybuild_test;
