// #![ feature( type_name_of_val ) ]
// #![ feature( trace_macros ) ]
// #![ feature( concat_idents ) ]

// #[ rustversion::nightly ]
// #![ feature( type_name_of_val ) ]
// #![ feature( inspect_type_of, inspect_to_str_type_of ) ]
#![ cfg_attr( feature = "nightly", feature( type_name_of_val ) ) ]

// #![ cfg_attr( rustversion::nightly, feature( type_name_of_val ) ) ]

mod dt;
mod error;
mod former;
mod meta;
mod options;
mod string;
mod test;
mod time;
mod typing;
// #[ cfg( feature = "proc_macro" ) ]
// mod proc_macro;
// mod vector;
