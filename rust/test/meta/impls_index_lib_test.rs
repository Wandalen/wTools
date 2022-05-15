#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

// #![ feature( trace_macros ) ]
// #![ feature( type_name_of_val ) ]

// mod impls_index
// {
//   include!{ "./impls_index/mod.rs" }
// }

#[ path = "./impls_index/mod.rs" ]
mod impls_index;
