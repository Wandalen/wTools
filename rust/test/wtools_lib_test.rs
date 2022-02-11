// #![ feature( type_name_of_val ) ]
// #![ feature( trace_macros ) ]
// #![ feature( concat_idents ) ]
// #![ feature( type_name_of_val ) ]
// #![ feature( inspect_type_of, inspect_to_str_type_of ) ]

#![ cfg_attr( feature = "nightly", feature( type_name_of_val ) ) ]
// #![ cfg_attr( rustversion::nightly, feature( type_name_of_val ) ) ]

include!( "./mod.rs" );
