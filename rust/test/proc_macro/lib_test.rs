// #![ feature( proc_macro_span ) ]
#![ cfg_attr( feature = "with_proc_macro", feature( type_name_of_val ) ) ]

#[ cfg( feature = "with_proc_macro" ) ]
mod basic_test;
